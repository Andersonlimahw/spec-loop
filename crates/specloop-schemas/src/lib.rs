use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SCHEMA_NAMES: &[&str] = &[
    "manifest",
    "config",
    "spec",
    "finding",
    "run-report",
    "persona",
    "business-rule",
    "acceptance-criteria",
    "critical-flow",
    "risk",
    "regression-test-suggestion",
];

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct SpecLoopConfig {
    pub schema_version: String,
    pub project: ProjectConfig,
    pub agent: AgentConfig,
    pub browser: BrowserConfig,
    pub safety: SafetyConfig,
    pub outputs: OutputsConfig,
}

impl Default for SpecLoopConfig {
    fn default() -> Self {
        Self {
            schema_version: "0.1".to_string(),
            project: ProjectConfig::default(),
            agent: AgentConfig::default(),
            browser: BrowserConfig::default(),
            safety: SafetyConfig::default(),
            outputs: OutputsConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct ProjectConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    pub target_url: String,
    pub environment: Environment,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "My SaaS".to_string(),
            project_type: ProjectType::WebApp,
            target_url: "http://localhost:3000".to_string(),
            environment: Environment::Local,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectType {
    #[default]
    WebApp,
    Api,
    MobileApp,
    Library,
    Other,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Environment {
    #[default]
    Local,
    Staging,
    Production,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct AgentConfig {
    pub mode: AgentMode,
    pub provider_agnostic: bool,
    pub cold_context_cache: Vec<String>,
    pub hot_context_sources: Vec<String>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            mode: AgentMode::Exploratory,
            provider_agnostic: true,
            cold_context_cache: vec![
                ".specloop/product.md".to_string(),
                ".specloop/specs.md".to_string(),
                ".specloop/business-rules.md".to_string(),
                ".specloop/critical-flows.md".to_string(),
            ],
            hot_context_sources: vec![
                ".specloop/risk-register.md".to_string(),
                ".specloop/findings".to_string(),
                ".specloop/reports".to_string(),
            ],
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentMode {
    #[default]
    Exploratory,
    Regression,
    Triage,
    Documentation,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct BrowserConfig {
    pub engine: BrowserEngine,
    pub fallback: BrowserEngine,
    pub allow_browser_writes: bool,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            engine: BrowserEngine::ChromeDevtoolsMcp,
            fallback: BrowserEngine::Playwright,
            allow_browser_writes: false,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BrowserEngine {
    #[default]
    ChromeDevtoolsMcp,
    Playwright,
    Shell,
    None,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct SafetyConfig {
    pub read_only_by_default: bool,
    pub block_destructive_actions: bool,
    pub require_approval_for_writes: bool,
    pub block_production_writes: bool,
    pub sanitize_logs: bool,
    pub persist_cookies: bool,
    pub trusted_mcp_servers: Vec<String>,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            read_only_by_default: true,
            block_destructive_actions: true,
            require_approval_for_writes: true,
            block_production_writes: true,
            sanitize_logs: true,
            persist_cookies: false,
            trusted_mcp_servers: vec!["chrome-devtools-mcp".to_string()],
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct OutputsConfig {
    pub findings_dir: String,
    pub reports_dir: String,
    pub screenshots_dir: String,
    pub traces_dir: String,
}

impl Default for OutputsConfig {
    fn default() -> Self {
        Self {
            findings_dir: ".specloop/findings".to_string(),
            reports_dir: ".specloop/reports".to_string(),
            screenshots_dir: ".specloop/screenshots".to_string(),
            traces_dir: ".specloop/traces".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(default)]
pub struct Manifest {
    pub schema_version: String,
    pub project: ProjectConfig,
    pub specs: Vec<ProductSpec>,
    pub personas: Vec<Persona>,
    pub business_rules: Vec<BusinessRule>,
    pub critical_flows: Vec<CriticalFlow>,
    pub risks: Vec<Risk>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            schema_version: "0.1".to_string(),
            project: ProjectConfig::default(),
            specs: vec![],
            personas: vec![],
            business_rules: vec![],
            critical_flows: vec![],
            risks: vec![],
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct ProductSpec {
    pub id: String,
    pub title: String,
    pub area: String,
    pub expected_behavior: String,
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct Persona {
    pub id: String,
    pub name: String,
    pub goals: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct BusinessRule {
    pub id: String,
    pub title: String,
    pub rule: String,
    pub severity_if_broken: Severity,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct AcceptanceCriterion {
    pub id: String,
    pub spec_id: String,
    pub statement: String,
    pub verification: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct CriticalFlow {
    pub id: String,
    pub title: String,
    pub persona_id: String,
    pub steps: Vec<String>,
    pub expected_outcome: String,
    pub blocked_actions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct Risk {
    pub id: String,
    pub title: String,
    pub impact: Severity,
    pub mitigation: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct RegressionTestSuggestion {
    pub title: String,
    pub kind: RegressionKind,
    pub steps: Vec<String>,
    pub assertion: String,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RegressionKind {
    Unit,
    Integration,
    #[default]
    Playwright,
    Manual,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct Evidence {
    pub kind: EvidenceKind,
    pub location: String,
    pub summary: String,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceKind {
    Screenshot,
    Trace,
    Console,
    Network,
    Dom,
    Stdout,
    Stderr,
    #[default]
    Note,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    #[serde(rename = "type")]
    pub finding_type: FindingType,
    pub area: String,
    pub persona: String,
    pub related_spec: String,
    pub related_business_rule: String,
    pub expected_behavior: String,
    pub actual_behavior: String,
    pub reproduction_steps: Vec<String>,
    pub evidence: Vec<Evidence>,
    pub suspected_cause: String,
    pub suggested_fix: String,
    pub suggested_regression_test: RegressionTestSuggestion,
    pub status: FindingStatus,
    pub created_at: String,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
pub enum Severity {
    P0,
    P1,
    #[default]
    P2,
    P3,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FindingType {
    Bug,
    Ux,
    BusinessRule,
    SpecMismatch,
    Security,
    Accessibility,
    Performance,
    DataQuality,
    #[default]
    AiBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FindingStatus {
    #[default]
    Open,
    Triaged,
    Rejected,
    InProgress,
    Fixed,
    Verified,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct RunReport {
    pub run_id: String,
    pub project: String,
    pub target_url: String,
    pub mode: AgentMode,
    pub status: RunStatus,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub findings: Vec<Finding>,
    pub summary: String,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    #[default]
    Planned,
    Running,
    Blocked,
    Completed,
}

pub fn schema_for_name(name: &str) -> Option<Value> {
    let schema = match name {
        "manifest" => schema_for!(Manifest),
        "config" => schema_for!(SpecLoopConfig),
        "spec" => schema_for!(ProductSpec),
        "finding" => schema_for!(Finding),
        "run-report" => schema_for!(RunReport),
        "persona" => schema_for!(Persona),
        "business-rule" => schema_for!(BusinessRule),
        "acceptance-criteria" => schema_for!(AcceptanceCriterion),
        "critical-flow" => schema_for!(CriticalFlow),
        "risk" => schema_for!(Risk),
        "regression-test-suggestion" => schema_for!(RegressionTestSuggestion),
        _ => return None,
    };

    serde_json::to_value(schema).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_safe() {
        let config = SpecLoopConfig::default();

        assert!(config.safety.read_only_by_default);
        assert!(config.safety.block_destructive_actions);
        assert!(!config.safety.persist_cookies);
    }

    #[test]
    fn exposes_all_named_schemas() {
        for name in SCHEMA_NAMES {
            assert!(schema_for_name(name).is_some(), "{name}");
        }
    }
}
