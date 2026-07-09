use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use specloop_schemas::SpecLoopConfig;

pub const CONFIG_FILE: &str = "specloop.yaml";
pub const CONFIG_EXAMPLE_FILE: &str = "specloop.yaml.example";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigIssue {
    pub field: String,
    pub message: String,
}

pub fn default_config() -> SpecLoopConfig {
    SpecLoopConfig::default()
}

pub fn load_config(root: impl AsRef<Path>) -> Result<SpecLoopConfig> {
    let path = root.as_ref().join(CONFIG_FILE);
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config {}", path.display()))?;
    serde_yaml::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
}

pub fn write_config_example(root: impl AsRef<Path>, force: bool) -> Result<PathBuf> {
    let path = root.as_ref().join(CONFIG_EXAMPLE_FILE);
    write_config_file(&path, &default_config(), force)?;
    Ok(path)
}

pub fn write_default_config(root: impl AsRef<Path>, force: bool) -> Result<PathBuf> {
    let path = root.as_ref().join(CONFIG_FILE);
    write_config_file(&path, &default_config(), force)?;
    Ok(path)
}

pub fn validate_config(config: &SpecLoopConfig) -> Vec<ConfigIssue> {
    let mut issues = Vec::new();

    if config.project.name.trim().is_empty() {
        issues.push(issue("project.name", "project name must not be empty"));
    }

    if config.project.target_url.trim().is_empty() {
        issues.push(issue("project.target_url", "target URL must not be empty"));
    }

    if !config.safety.read_only_by_default {
        issues.push(issue(
            "safety.read_only_by_default",
            "SpecLoop must be read-only by default",
        ));
    }

    if !config.safety.block_destructive_actions {
        issues.push(issue(
            "safety.block_destructive_actions",
            "destructive actions must be blocked by default",
        ));
    }

    if config.safety.persist_cookies {
        issues.push(issue(
            "safety.persist_cookies",
            "cookies and sessions must not be persisted by default",
        ));
    }

    issues
}

fn write_config_file(path: &Path, config: &SpecLoopConfig, force: bool) -> Result<()> {
    if path.exists() && !force {
        return Ok(());
    }

    let yaml = serde_yaml::to_string(config).context("failed to serialize default config")?;
    fs::write(path, yaml).with_context(|| format!("failed to write {}", path.display()))
}

fn issue(field: &str, message: &str) -> ConfigIssue {
    ConfigIssue {
        field: field.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_safe_defaults() {
        assert!(validate_config(&default_config()).is_empty());
    }
}
