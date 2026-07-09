use std::env;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use specloop_core::{
    DoctorReport, InitOptions, RunOptions, ScaffoldKind, doctor_project, export_schemas,
    init_project, list_reports, run_project, scaffold_project, validate_project,
};

#[derive(Debug, Parser)]
#[command(name = "specloop")]
#[command(about = "Spec-driven QA loops for AI-native teams")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize .specloop context and specloop.yaml.
    Init {
        /// Overwrite existing generated files.
        #[arg(long)]
        force: bool,
    },
    /// Check local SpecLoop readiness.
    Doctor {
        /// Emit JSON.
        #[arg(long)]
        json: bool,
    },
    /// Create a safe run report contract.
    Run {
        /// Mark the run as executed. The default is a plan-only dry run.
        #[arg(long)]
        execute: bool,
        /// Optional natural-language action to safety-check before the run.
        #[arg(long)]
        action: Option<String>,
    },
    /// List generated reports.
    Report {
        /// Emit JSON.
        #[arg(long)]
        json: bool,
    },
    /// Validate config and required context files.
    Validate {
        /// Emit JSON.
        #[arg(long)]
        json: bool,
    },
    /// Scaffold optional compatibility assets.
    Scaffold {
        #[arg(value_enum, default_value_t = ScaffoldArg::All)]
        kind: ScaffoldArg,
        /// Overwrite existing generated files.
        #[arg(long)]
        force: bool,
    },
    /// Export versioned JSON schemas.
    Export {
        #[arg(value_enum, default_value_t = ExportArg::Schemas)]
        target: ExportArg,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum ScaffoldArg {
    Agents,
    Skills,
    Scripts,
    Commands,
    Scenarios,
    Loops,
    All,
}

#[derive(Clone, Debug, ValueEnum)]
enum ExportArg {
    Schemas,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = env::current_dir()?;

    match cli.command {
        Commands::Init { force } => {
            let summary = init_project(root, InitOptions { force })?;
            println!("initialized SpecLoop");
            println!("config: {}", summary.config_path.display());
            println!("files: {}", summary.files_written.len());
            println!("directories: {}", summary.directories_created.len());
        }
        Commands::Doctor { json } => {
            let report = doctor_project(root);
            if json {
                println!("{}", doctor_json(&report));
            } else {
                for check in report.checks {
                    println!(
                        "{} {} — {}",
                        if check.ok { "ok" } else { "fail" },
                        check.name,
                        check.detail
                    );
                }
            }
        }
        Commands::Run { execute, action } => {
            let summary = run_project(
                root,
                RunOptions {
                    dry_run: !execute,
                    requested_action: action,
                },
            )?;
            println!("run: {}", summary.run_id);
            println!("report: {}", summary.report_path.display());
        }
        Commands::Report { json } => {
            let reports = list_reports(root)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&reports)?);
            } else if reports.is_empty() {
                println!("no reports yet");
            } else {
                for report in reports {
                    println!("{}", report.display());
                }
            }
        }
        Commands::Validate { json } => {
            let issues = validate_project(root)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&issues)?);
            } else if issues.is_empty() {
                println!("validation passed");
            } else {
                for issue in issues {
                    println!("fail {issue}");
                }
            }
        }
        Commands::Scaffold { kind, force } => {
            let written = scaffold_project(root, kind.into(), force)?;
            println!("scaffolded {} file(s)", written.len());
            for path in written {
                println!("{}", path.display());
            }
        }
        Commands::Export { target } => match target {
            ExportArg::Schemas => {
                let written = export_schemas(root)?;
                println!("exported {} schema file(s)", written.len());
            }
        },
    }

    Ok(())
}

impl From<ScaffoldArg> for ScaffoldKind {
    fn from(value: ScaffoldArg) -> Self {
        match value {
            ScaffoldArg::Agents => ScaffoldKind::Agents,
            ScaffoldArg::Skills => ScaffoldKind::Skills,
            ScaffoldArg::Scripts => ScaffoldKind::Scripts,
            ScaffoldArg::Commands => ScaffoldKind::Commands,
            ScaffoldArg::Scenarios => ScaffoldKind::Scenarios,
            ScaffoldArg::Loops => ScaffoldKind::Loops,
            ScaffoldArg::All => ScaffoldKind::All,
        }
    }
}

fn doctor_json(report: &DoctorReport) -> String {
    let checks = report
        .checks
        .iter()
        .map(|check| {
            serde_json::json!({
                "name": check.name,
                "ok": check.ok,
                "detail": check.detail,
            })
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&checks).unwrap_or_else(|_| "[]".to_string())
}
