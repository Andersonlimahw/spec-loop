# SP-05 — Open Core Operations

## Intent

Prepare the project for open-source collaboration while preserving the author's
exclusive commercial monetization rights.

## Write Scope

- `README.md`
- `LICENSE`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `CODE_OF_CONDUCT.md`
- `CHANGELOG.md`
- `.github/workflows/ci.yml`

## Acceptance Criteria

- [x] README explains product, CLI, quick start, safety defaults, and docs.
- [x] License is noncommercial with explicit commercial licensing route.
- [x] Contribution and security policies exist.
- [x] CI runs Rust format, clippy, tests, and docs build.

## Verify

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
