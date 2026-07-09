# Contributing

Contributions are welcome.

By contributing, you agree that your contribution is provided under the project
license and that Anderson Lima may offer the project or derivative commercial
licenses separately.

## Development

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Rules

- Keep provider and harness integration optional.
- Preserve read-only safety defaults.
- Add or update schemas before changing report/config shape.
- Keep docs and CLI behavior in sync.
- Do not commit secrets, real cookies, production traces, or private customer data.

## Good First Areas

- More example specs.
- Reporter improvements.
- Playwright adapter experiments.
- Docs clarifications.
- Schema test fixtures.
