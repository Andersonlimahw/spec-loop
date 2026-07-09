# SP-03 — Safety And Memory

## Intent

Make safety and context economy first-class product constraints.

SpecLoop must block risky actions by default and distinguish cacheable cold
context from hot run context.

## Write Scope

- `crates/specloop-safety/`
- `crates/specloop-schemas/`
- `.specloop/*.md`
- `docs/security-guardrails.md`
- `docs/memory-caching.md`

## Acceptance Criteria

- [x] Destructive action phrases are detected.
- [x] Secret-like markers are redacted from logs.
- [x] Config defaults are read-only and do not persist cookies.
- [x] Cold context cache and hot context sources are represented in config.
- [x] Memory/caching docs define invalidation boundaries.

## Verify

```bash
cargo test -p specloop-safety
cargo run -p specloop-cli -- validate
```

## Cold Context

- base loop prompt;
- schema contracts;
- adapter contracts;
- project profile;
- product specs;
- business rules;
- critical flows.

## Hot Context

- run objective;
- target URL and current environment;
- fresh traces/screenshots/log summaries;
- recent findings;
- short-lived scratchpad.
