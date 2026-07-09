# Getting Started

## Build

```bash
cargo build --workspace
```

## Run The CLI Locally

```bash
cargo run -p specloop-cli -- init --force
cargo run -p specloop-cli -- doctor
cargo run -p specloop-cli -- validate
cargo run -p specloop-cli -- run --action "open landing page"
cargo run -p specloop-cli -- report
```

## Install

```bash
cargo install --path crates/specloop-cli
specloop init
```

From a product repo, install the CLI from Git:

```bash
cargo install --git https://github.com/Andersonlimahw/spec-loop specloop-cli
```

## Plugin Install

Local checkout:

```bash
codex plugin add /path/to/spec-loop
```

Marketplace-style install:

```bash
codex plugin marketplace add https://github.com/Andersonlimahw/spec-loop
codex plugin add spec-loop
```

Claude-compatible hosts can use:

```bash
claude plugin marketplace add https://github.com/Andersonlimahw/spec-loop
claude plugin add spec-loop
```

The plugin loads skills and browser MCP defaults. Install the CLI with `cargo install`
until binary releases are available.

## Browser Scenario Examples

Local apps must be started first:

```bash
npm run dev
specloop scaffold scripts
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local
```

Remote read-only example:

```bash
specloop scaffold scripts
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target lemon
```

On first use, the helper installs Playwright and Chromium under
`.specloop/runtime/` if they are not already available.

The remote example opens `https://lemon.dev.br/pt`, scrolls to the footer, saves
`.specloop/screenshots/lemon-landing.png`, clicks `Conteúdo`, and saves
`.specloop/screenshots/lemon-blog.png`.

## Generated Files

- `.specloop/product.md`
- `.specloop/specs.md`
- `.specloop/business-rules.md`
- `.specloop/critical-flows.md`
- `.specloop/risk-register.md`
- `.specloop/scenarios/*.md`
- `.specloop/loops/*.md`
- `specloop.yaml`
- `schemas/*.schema.json`
