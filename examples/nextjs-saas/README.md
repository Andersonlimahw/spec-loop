# Next.js SaaS Example

This example documents the shape of a small runnable browser smoke pass.

## Scenario

Validate a public landing page and auth entry flow against:

- product spec;
- business rules;
- critical flow;
- console/network evidence;
- expected CTA visibility.

## Run

```bash
specloop init
specloop scaffold scripts
specloop run --action "open landing page"
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local
specloop report
```

Start the product app on `http://localhost:3000` before running the script. The
v0 CLI still generates the report contract; the scaffolded Playwright helper
captures browser evidence for the local smoke path.
