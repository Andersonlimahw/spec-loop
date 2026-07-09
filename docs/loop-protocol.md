# Loop Protocol

SpecLoop runs a repeatable QA loop:

1. Read project context.
2. Load product specs.
3. Load business rules.
4. Load acceptance criteria.
5. Load critical flows.
6. Check safety guardrails.
7. Start or receive evidence from a runner.
8. Compare observed behavior with expected behavior.
9. Register each finding immediately.
10. Classify severity.
11. Suggest fix and regression test.
12. Generate final report.

The MVP implements the local contract, schemas, report generation, and safety
checks. Browser runtime adapters are deliberately separated so the protocol stays
provider-agnostic.

In practice:

- `specloop run` creates the run/report contract.
- `.specloop/scenarios/*.md` describes what to verify.
- an agent with browser tools or `scripts/specloop-browser-smoke.ts` captures
  screenshots/traces.
- `specloop report` lists generated reports.

For local targets, start the app first. For remote targets, keep the browser
session read-only.
