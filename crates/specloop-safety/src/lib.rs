pub const DESTRUCTIVE_ACTIONS: &[&str] = &[
    "delete",
    "destroy",
    "drop table",
    "truncate",
    "purchase",
    "pay now",
    "checkout",
    "transfer",
    "refund",
    "send email",
    "submit payment",
    "confirm order",
];

pub const SENSITIVE_MARKERS: &[&str] = &[
    "api_key=",
    "apikey=",
    "authorization: bearer ",
    "password=",
    "secret=",
    "token=",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SafetyFinding {
    pub rule: String,
    pub message: String,
}

pub fn detect_destructive_action(input: &str) -> Option<SafetyFinding> {
    let lower = input.to_lowercase();
    DESTRUCTIVE_ACTIONS
        .iter()
        .find(|action| lower.contains(**action))
        .map(|action| SafetyFinding {
            rule: "block-destructive-action".to_string(),
            message: format!("blocked potentially destructive action: {action}"),
        })
}

pub fn sanitize_log(input: &str) -> String {
    let mut output = input.to_string();
    for marker in SENSITIVE_MARKERS {
        output = redact_marker(&output, marker);
    }
    output
}

pub fn production_write_warning(environment: &str, requested_write: bool) -> Option<SafetyFinding> {
    if requested_write && environment.eq_ignore_ascii_case("production") {
        return Some(SafetyFinding {
            rule: "block-production-write".to_string(),
            message: "production writes require an explicit override outside the default loop"
                .to_string(),
        });
    }

    None
}

fn redact_marker(input: &str, marker: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut rest = input;

    loop {
        let lower = rest.to_lowercase();
        let Some(start) = lower.find(marker) else {
            output.push_str(rest);
            break;
        };

        let value_start = start + marker.len();
        let value_len = rest[value_start..]
            .find(char::is_whitespace)
            .unwrap_or(rest.len() - value_start);
        let value_end = value_start + value_len;

        output.push_str(&rest[..start]);
        output.push_str(&rest[start..value_start]);
        output.push_str("[REDACTED]");
        rest = &rest[value_end..];
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catches_destructive_action() {
        let finding = detect_destructive_action("click pay now in production");

        assert!(finding.is_some());
    }

    #[test]
    fn sanitizes_tokens() {
        let clean = sanitize_log("token=abc123 ok token=xyz");

        assert_eq!(clean, "token=[REDACTED] ok token=[REDACTED]");
    }
}
