use specloop_safety::sanitize_log;
use specloop_schemas::{Finding, RunReport, Severity};

pub fn render_finding_markdown(finding: &Finding) -> String {
    let steps = finding
        .reproduction_steps
        .iter()
        .enumerate()
        .map(|(index, step)| format!("{}. {step}", index + 1))
        .collect::<Vec<_>>()
        .join("\n");

    let evidence = finding
        .evidence
        .iter()
        .map(|item| {
            format!(
                "- {:?}: `{}` — {}",
                item.kind,
                clean(&item.location),
                clean(&item.summary)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"---
id: {id}
severity: {severity:?}
status: {status:?}
area: {area}
---

# {title}

## Expected
{expected}

## Actual
{actual}

## Reproduction
{steps}

## Evidence
{evidence}

## Suspected Cause
{cause}

## Suggested Fix
{fix}

## Suggested Regression Test
{test_title}

{test_assertion}
"#,
        id = finding.id,
        severity = finding.severity,
        status = finding.status,
        area = clean(&finding.area),
        title = clean(&finding.title),
        expected = clean(&finding.expected_behavior),
        actual = clean(&finding.actual_behavior),
        steps = steps,
        evidence = evidence,
        cause = clean(&finding.suspected_cause),
        fix = clean(&finding.suggested_fix),
        test_title = clean(&finding.suggested_regression_test.title),
        test_assertion = clean(&finding.suggested_regression_test.assertion),
    )
}

pub fn render_run_report_markdown(report: &RunReport) -> String {
    let mut output = format!(
        r#"# SpecLoop Run Report — {run_id}

| Field | Value |
|---|---|
| Project | {project} |
| Target URL | {target_url} |
| Mode | {mode:?} |
| Status | {status:?} |
| Started | {started_at} |

## Summary
{summary}

## Findings

| Severity | ID | Title | Status |
|---|---|---|---|
"#,
        run_id = report.run_id,
        project = clean(&report.project),
        target_url = clean(&report.target_url),
        mode = report.mode,
        status = report.status,
        started_at = clean(&report.started_at),
        summary = clean(&report.summary),
    );

    if report.findings.is_empty() {
        output.push_str("| - | - | No findings in this run yet. | - |\n");
        return output;
    }

    for finding in &report.findings {
        output.push_str(&format!(
            "| {} | {} | {} | {:?} |\n",
            severity_label(&finding.severity),
            finding.id,
            clean(&finding.title),
            finding.status
        ));
    }

    output
}

fn clean(value: &str) -> String {
    sanitize_log(value)
}

fn severity_label(severity: &Severity) -> &'static str {
    match severity {
        Severity::P0 => "P0",
        Severity::P1 => "P1",
        Severity::P2 => "P2",
        Severity::P3 => "P3",
    }
}

#[cfg(test)]
mod tests {
    use specloop_schemas::{
        Evidence, EvidenceKind, Finding, FindingStatus, FindingType, RegressionKind,
        RegressionTestSuggestion, Severity,
    };

    use super::*;

    #[test]
    fn finding_markdown_sanitizes_evidence() {
        let finding = Finding {
            id: "F-1".to_string(),
            title: "Leaked token".to_string(),
            severity: Severity::P0,
            finding_type: FindingType::Security,
            area: "auth".to_string(),
            persona: "user".to_string(),
            related_spec: "SPEC-1".to_string(),
            related_business_rule: "BR-1".to_string(),
            expected_behavior: "No token=abc".to_string(),
            actual_behavior: "Saw token=def token=ghi".to_string(),
            reproduction_steps: vec!["Open console".to_string()],
            evidence: vec![Evidence {
                kind: EvidenceKind::Console,
                location: "console token=loc".to_string(),
                summary: "secret=abc token=xyz".to_string(),
            }],
            suspected_cause: "token=bug".to_string(),
            suggested_fix: "redact".to_string(),
            suggested_regression_test: RegressionTestSuggestion {
                title: "No logs".to_string(),
                kind: RegressionKind::Unit,
                steps: vec![],
                assertion: "token=nope".to_string(),
            },
            status: FindingStatus::Open,
            created_at: "now".to_string(),
        };

        let rendered = render_finding_markdown(&finding);

        assert!(!rendered.contains("token=def"));
        assert!(!rendered.contains("secret=abc"));
        assert!(rendered.contains("token=[REDACTED]"));
    }
}
