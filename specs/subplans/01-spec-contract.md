# SP-01 — Spec Contract

## Intent

Define the stable data contracts that make SpecLoop more than a prompt pack:
config, manifest, specs, findings, reports, personas, business rules, critical
flows, risks, and regression-test suggestions.

## Write Scope

- `crates/specloop-schemas/`
- `schemas/`
- `.specloop/schemas/`

## Acceptance Criteria

- [x] `SpecLoopConfig` has safe defaults.
- [x] Finding schema includes severity, type, related spec, business rule, evidence,
      suggested fix, suggested regression test, status, and created timestamp.
- [x] JSON schemas can be exported by CLI.
- [x] Unit tests verify all schema names resolve.

## Verify

```bash
cargo test -p specloop-schemas
cargo run -p specloop-cli -- export schemas
```
