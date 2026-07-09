# Manual Browser QA Loop

## Goal

Run one small, evidence-backed browser scenario at a time and leave reusable artifacts behind.

## Loop

1. Select one `.specloop/scenarios/*.md` file.
2. Load cold context: product, specs, business rules, acceptance criteria, critical flows.
3. Load hot context: risk register, previous findings, previous reports.
4. Execute only the scenario steps. Keep production read-only.
5. Save screenshots and traces under `.specloop/screenshots/` and `.specloop/traces/`.
6. Write one finding per confirmed mismatch under `.specloop/findings/`.
7. Run `specloop validate` and `specloop report`.
8. Append a short note to `.specloop/loops/ITERATION_LOG.md`.
9. If a step was repeated manually, extract a script, skill, agent, or MCP tool.

## Local vs Remote

- Local target: start the app first, then run the scenario.
- Remote target: ask the agent to run the scenario directly, but keep it read-only.

## Quality Gate

- Scenario stayed in scope.
- No destructive browser actions.
- Evidence paths exist.
- Findings are structured evidence, not impressions.
