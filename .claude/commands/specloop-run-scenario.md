---
description: Run one SpecLoop scenario with read-only browser safety.
allowed-tools: Read, Bash, Grep
---

# Run SpecLoop Scenario

1. Run `specloop doctor`.
2. Read the requested `.specloop/scenarios/*.md` file.
3. If it targets localhost, confirm the app is already running.
4. Run the matching script when available, for example:

```bash
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target lemon
```

5. Save findings immediately under `.specloop/findings/`.
6. Run `specloop validate` and summarize evidence paths.
