---
description: Create a new SpecLoop scenario markdown file.
allowed-tools: Read, Bash
---

# New SpecLoop Scenario

Use the scaffolded helper:

```bash
node --experimental-strip-types scripts/specloop-new-scenario.ts --name "$ARGUMENTS"
```

Then edit the generated file so the target, steps, expected outcome, and evidence paths are explicit.
