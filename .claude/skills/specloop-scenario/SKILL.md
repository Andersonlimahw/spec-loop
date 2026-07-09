---
name: specloop-scenario
description: Create, edit, or run SpecLoop scenario files under `.specloop/scenarios/`, including local localhost checks, remote landing/blog screenshot flows, and manual browser QA runbooks.
---

# SpecLoop Scenario

Use this skill when the user asks to create a QA scenario, add a browser flow, or run a specific `.specloop/scenarios/*.md` file.

## Procedure

1. Read `specloop.yaml` for target URL and output folders.
2. Read the chosen scenario file.
3. Confirm whether the target is local or remote.
4. For local targets, require the app to already be running unless the user asked you to start it.
5. Execute only read-only browser steps by default.
6. Save requested screenshots under `.specloop/screenshots/`.
7. Record findings as structured evidence, one file per issue.

## Create A Scenario

After `specloop scaffold scripts`:

```bash
node --experimental-strip-types scripts/specloop-new-scenario.ts --name "pricing smoke" --url "http://localhost:3000"
```

Keep scenarios small: one persona, one target, one expected outcome, and explicit evidence paths.
