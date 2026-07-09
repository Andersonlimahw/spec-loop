---
name: specloop-loop
description: "Run the SpecLoop QA loop: load specs and business rules, enforce read-only safety, explore critical flows, write structured findings, triage accepted reports, and suggest regression tests."
---

# SpecLoop Loop

Use this workflow when a user asks to run a SpecLoop QA pass, triage SpecLoop findings, or scaffold a product-specific QA loop.

## Procedure

1. Run `specloop doctor`.
2. Load cold context: product, specs, business rules, acceptance criteria, critical flows.
3. Load hot context: risk register, prior findings, current run budget.
4. Select a scenario from `.specloop/scenarios/` or a flow from `.specloop/critical-flows.md`.
5. For local URLs, verify the user has started the app. Do not start unknown app servers unless asked.
6. Explore one flow at a time with read-only safety.
7. Save evidence under `.specloop/screenshots/` or `.specloop/traces/`.
8. Write each finding immediately.
9. Run `specloop report` and `specloop validate`.
10. Triage reviewed findings into implementation plans.

## Manual Runner

After `specloop scaffold scripts`, these examples are available:

```bash
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target lemon
```

## Rules

- Never perform destructive production actions by default.
- Do not store cookies or secrets.
- Do not batch findings until the end of a run.
- Suggest a regression test for every finding.
