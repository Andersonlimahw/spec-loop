# SpecLoop Agent Instructions

- Treat `.specloop/product.md`, `.specloop/specs.md`, `.specloop/business-rules.md`, and `.specloop/critical-flows.md` as cold context.
- Treat `.specloop/risk-register.md`, `.specloop/findings/`, and `.specloop/reports/` as hot run context.
- Treat `.specloop/scenarios/` and `.specloop/loops/` as executable runbooks.
- Never perform destructive browser actions by default.
- Local targets require the user or harness to start the app first.
- Record findings as structured evidence, not vibes.
