# SP-02 — Rust CLI MVP

## Intent

Provide a useful Rust CLI baseline that proves the local SpecLoop lifecycle:
initialize context, validate it, generate a report contract, list reports, scaffold
optional agent assets, and export schemas.

## Write Scope

- `crates/specloop-cli/`
- `crates/specloop-core/`
- `crates/specloop-config/`
- `crates/specloop-reporter/`
- `crates/specloop-templates/`

## Acceptance Criteria

- [x] `specloop init` creates `.specloop/`, `specloop.yaml`, `AGENTS.md`, and `CLAUDE.md`.
- [x] `specloop validate` passes on generated defaults.
- [x] `specloop run` writes a Markdown report.
- [x] `specloop report` lists generated reports.
- [x] `specloop scaffold all` writes optional `.claude` agents and skill assets.
- [x] `specloop doctor` reports local readiness.

## Verify

```bash
cargo test --workspace
cargo run -p specloop-cli -- init --force
cargo run -p specloop-cli -- validate
cargo run -p specloop-cli -- run --action "open landing page"
cargo run -p specloop-cli -- report
```
