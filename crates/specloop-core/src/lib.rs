use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use specloop_config::{
    CONFIG_FILE, load_config, validate_config, write_config_example, write_default_config,
};
use specloop_reporter::render_run_report_markdown;
use specloop_safety::{detect_destructive_action, production_write_warning};
use specloop_schemas::{RunReport, RunStatus, SCHEMA_NAMES, SpecLoopConfig, schema_for_name};
use specloop_templates::{
    TemplateFile, init_files, scaffold_agent_files, scaffold_command_files, scaffold_loop_files,
    scaffold_scenario_files, scaffold_script_files, scaffold_skill_files,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InitOptions {
    pub force: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitSummary {
    pub config_path: PathBuf,
    pub files_written: Vec<PathBuf>,
    pub directories_created: Vec<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoctorCheck {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoctorReport {
    pub checks: Vec<DoctorCheck>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunOptions {
    pub dry_run: bool,
    pub requested_action: Option<String>,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            dry_run: true,
            requested_action: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunSummary {
    pub report_path: PathBuf,
    pub run_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScaffoldKind {
    Agents,
    Skills,
    Scripts,
    Commands,
    Scenarios,
    Loops,
    All,
}

pub fn init_project(root: impl AsRef<Path>, options: InitOptions) -> Result<InitSummary> {
    let root = root.as_ref();
    let dirs = [
        ".specloop/findings",
        ".specloop/reports",
        ".specloop/screenshots",
        ".specloop/traces",
        ".specloop/prompts",
        ".specloop/skills",
        ".specloop/scenarios",
        ".specloop/loops",
        ".specloop/schemas",
    ];

    let mut directories_created = Vec::new();
    for dir in dirs {
        let path = root.join(dir);
        fs::create_dir_all(&path)
            .with_context(|| format!("failed to create {}", path.display()))?;
        directories_created.push(path);
    }

    let config_path = write_default_config(root, options.force)?;
    write_config_example(root, options.force)?;

    let mut files_written = Vec::new();
    for file in init_files() {
        if write_template(root, &file, options.force)? {
            files_written.push(root.join(file.path));
        }
    }

    export_schemas(root)?;

    Ok(InitSummary {
        config_path,
        files_written,
        directories_created,
    })
}

pub fn doctor_project(root: impl AsRef<Path>) -> DoctorReport {
    let root = root.as_ref();
    let checks = vec![
        DoctorCheck {
            name: "config".to_string(),
            ok: root.join(CONFIG_FILE).exists(),
            detail: CONFIG_FILE.to_string(),
        },
        DoctorCheck {
            name: "specloop-dir".to_string(),
            ok: root.join(".specloop").is_dir(),
            detail: ".specloop/".to_string(),
        },
        DoctorCheck {
            name: "chrome-devtools-mcp".to_string(),
            ok: executable_on_path("chrome-devtools-mcp"),
            detail: "optional runtime adapter; install/configure chrome-devtools-mcp before browser runs".to_string(),
        },
        DoctorCheck {
            name: "node".to_string(),
            ok: executable_on_path("node"),
            detail: "required for scaffolded Node TypeScript helper scripts (Node 22+ with --experimental-strip-types)".to_string(),
        },
        DoctorCheck {
            name: "playwright".to_string(),
            ok: executable_on_path("playwright")
                || root.join("node_modules/.bin/playwright").exists()
                || root
                    .join(".specloop/runtime/node_modules/.bin/playwright")
                    .exists(),
            detail: "optional deterministic fallback; helper can install Playwright under .specloop/runtime".to_string(),
        },
    ];

    DoctorReport { checks }
}

pub fn validate_project(root: impl AsRef<Path>) -> Result<Vec<String>> {
    let root = root.as_ref();
    let config = load_config(root)?;
    let mut issues = validate_config(&config)
        .into_iter()
        .map(|issue| format!("{}: {}", issue.field, issue.message))
        .collect::<Vec<_>>();

    for required in [
        ".specloop/product.md",
        ".specloop/specs.md",
        ".specloop/personas.md",
        ".specloop/business-rules.md",
        ".specloop/acceptance-criteria.md",
        ".specloop/critical-flows.md",
        ".specloop/risk-register.md",
    ] {
        if !root.join(required).exists() {
            issues.push(format!("{required}: required context file is missing"));
        }
    }

    Ok(issues)
}

pub fn run_project(root: impl AsRef<Path>, options: RunOptions) -> Result<RunSummary> {
    let root = root.as_ref();
    let config = load_config(root)?;

    if let Some(action) = &options.requested_action
        && let Some(finding) = detect_destructive_action(action)
    {
        bail!("{}", finding.message);
    }

    let environment = format!("{:?}", config.project.environment);
    if let Some(warning) = production_write_warning(&environment, !options.dry_run) {
        bail!("{}", warning.message);
    }

    let run_id = format!("run-{}", unix_timestamp_nanos());
    let report = RunReport {
        run_id: run_id.clone(),
        project: config.project.name.clone(),
        target_url: config.project.target_url.clone(),
        mode: config.agent.mode.clone(),
        status: if options.dry_run {
            RunStatus::Planned
        } else {
            RunStatus::Completed
        },
        started_at: unix_timestamp().to_string(),
        finished_at: if options.dry_run {
            None
        } else {
            Some(unix_timestamp().to_string())
        },
        findings: vec![],
        summary: "MVP run contract generated. Browser evidence comes from `.specloop/scenarios/`, an agent browser tool, or `scripts/specloop-browser-smoke.ts`; local apps must already be running.".to_string(),
    };

    let reports_dir = root.join(&config.outputs.reports_dir);
    fs::create_dir_all(&reports_dir)
        .with_context(|| format!("failed to create {}", reports_dir.display()))?;
    let report_path = reports_dir.join(format!("{run_id}.md"));
    fs::write(&report_path, render_run_report_markdown(&report))
        .with_context(|| format!("failed to write {}", report_path.display()))?;

    Ok(RunSummary {
        report_path,
        run_id,
    })
}

pub fn list_reports(root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let config = load_config(root)?;
    let reports_dir = root.join(&config.outputs.reports_dir);

    if !reports_dir.exists() {
        return Ok(vec![]);
    }

    let mut reports = fs::read_dir(&reports_dir)
        .with_context(|| format!("failed to read {}", reports_dir.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>();
    reports.sort();
    Ok(reports)
}

pub fn scaffold_project(
    root: impl AsRef<Path>,
    kind: ScaffoldKind,
    force: bool,
) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let files = match kind {
        ScaffoldKind::Agents => scaffold_agent_files(),
        ScaffoldKind::Skills => scaffold_skill_files(),
        ScaffoldKind::Scripts => scaffold_script_files(),
        ScaffoldKind::Commands => scaffold_command_files(),
        ScaffoldKind::Scenarios => scaffold_scenario_files(),
        ScaffoldKind::Loops => scaffold_loop_files(),
        ScaffoldKind::All => {
            let mut files = scaffold_agent_files();
            files.extend(scaffold_skill_files());
            files.extend(scaffold_script_files());
            files.extend(scaffold_command_files());
            files.extend(scaffold_scenario_files());
            files.extend(scaffold_loop_files());
            files
        }
    };

    let mut written = Vec::new();
    for file in files {
        if write_template(root, &file, force)? {
            written.push(root.join(file.path));
        }
    }

    Ok(written)
}

pub fn export_schemas(root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let schemas_dir = root.join("schemas");
    let local_schemas_dir = root.join(".specloop/schemas");
    fs::create_dir_all(&schemas_dir)
        .with_context(|| format!("failed to create {}", schemas_dir.display()))?;
    fs::create_dir_all(&local_schemas_dir)
        .with_context(|| format!("failed to create {}", local_schemas_dir.display()))?;

    let mut written = Vec::new();
    for name in SCHEMA_NAMES {
        let Some(schema) = schema_for_name(name) else {
            continue;
        };
        let json = serde_json::to_string_pretty(&schema)?;
        for dir in [&schemas_dir, &local_schemas_dir] {
            let path = dir.join(format!("{name}.schema.json"));
            fs::write(&path, &json)
                .with_context(|| format!("failed to write {}", path.display()))?;
            written.push(path);
        }
    }

    Ok(written)
}

pub fn load_project_config(root: impl AsRef<Path>) -> Result<SpecLoopConfig> {
    load_config(root)
}

fn write_template(root: &Path, file: &TemplateFile, force: bool) -> Result<bool> {
    let path = root.join(file.path);
    if path.exists() && !force {
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&path, file.content)
        .with_context(|| format!("failed to write {}", path.display()))?;
    Ok(true)
}

fn executable_on_path(name: &str) -> bool {
    let Some(paths) = env::var_os("PATH") else {
        return false;
    };

    env::split_paths(&paths).any(|path| path.join(name).exists())
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn unix_timestamp_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_creates_safe_project() {
        let temp = tempfile::tempdir().expect("tempdir");
        let summary = init_project(temp.path(), InitOptions::default()).expect("init");

        assert!(summary.config_path.exists());
        assert!(temp.path().join(".specloop/product.md").exists());
        assert!(
            temp.path()
                .join(".specloop/scenarios/lemon-content-screenshots.md")
                .exists()
        );
        assert!(temp.path().join("schemas/finding.schema.json").exists());
    }

    #[test]
    fn scaffold_scripts_writes_browser_smoke_runner() {
        let temp = tempfile::tempdir().expect("tempdir");
        let written =
            scaffold_project(temp.path(), ScaffoldKind::Scripts, false).expect("scaffold");

        assert_eq!(written.len(), 3);
        assert!(
            temp.path()
                .join("scripts/specloop-new-scenario.ts")
                .exists()
        );
        assert!(temp.path().join("scripts/specloop-new-loop.ts").exists());
        assert!(
            temp.path()
                .join("scripts/specloop-browser-smoke.ts")
                .exists()
        );
    }
}
