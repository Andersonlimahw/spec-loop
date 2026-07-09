# SpecLoop

Spec-driven QA loops for AI-native teams.

SpecLoop is a Rust CLI and open-core protocol for running provider-agnostic QA
loops from product specs, business rules, acceptance criteria, critical flows,
and structured findings.

It is built for teams that use coding agents but still need deterministic,
auditable product-quality loops.

## Install From Source

```bash
cargo install --path crates/specloop-cli
```

This installs the `specloop` command.

From another project, install the CLI directly from Git:

```bash
cargo install --git https://github.com/Andersonlimahw/spec-loop specloop-cli
```

## Install As Agent Plugin

Local checkout:

```bash
codex plugin add /path/to/spec-loop
```

Marketplace-style install, once the marketplace entry is published:

```bash
codex plugin marketplace add https://github.com/Andersonlimahw/spec-loop
codex plugin add spec-loop
```

Claude-compatible hosts can use the same intent:

```bash
claude plugin marketplace add https://github.com/Andersonlimahw/spec-loop
claude plugin add spec-loop
```

The plugin manifest exposes SpecLoop skills and browser MCP defaults. The CLI
binary is still installed with `cargo install` until a packaged binary release is
published.

## Quick Start

```bash
specloop init
specloop doctor
specloop validate
specloop run --action "open landing page"
specloop report
```

Generated project context lives in `.specloop/`. `specloop init` now creates the
default scenario and loop runbooks too:

- `.specloop/scenarios/localhost-3000-smoke.md`
- `.specloop/scenarios/lemon-content-screenshots.md`
- `.specloop/loops/manual-browser-qa-loop.md`
- `.specloop/loops/ITERATION_LOG.md`

Use `--force` when you intentionally want generated files overwritten:

```bash
specloop init --force
```

For browser scenarios, there are two steps:

1. If the target is local, start the app yourself, for example `npm run dev` on
   `http://localhost:3000`.
2. Ask the agent to run a scenario, or run the scaffolded Playwright helper.

```bash
specloop scaffold scripts
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target lemon
```

The local target requires a real app already running on `localhost:3000` unless
you pass a different URL:

```bash
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local --url http://localhost:3000
```

On first use, the helper installs Playwright and Chromium under
`.specloop/runtime/` if they are not already available. The scripts require
Node.js 22+ (`node --experimental-strip-types`) and npm; no Bun project setup is
needed.

The `lemon` example opens `https://lemon.dev.br/pt`, scrolls the landing to the
footer, saves `.specloop/screenshots/lemon-landing.png`, clicks `Conteúdo`, and
saves `.specloop/screenshots/lemon-blog.png`.

## Scaffold Optional Assets

`specloop scaffold all` writes 14 optional compatibility assets:

- 3 agents under `.claude/agents/`
- 2 skills under `.claude/skills/`
- 3 helper scripts under `scripts/`
- 2 slash commands under `.claude/commands/`
- 2 scenarios under `.specloop/scenarios/`
- 2 loop files under `.specloop/loops/`

You can scaffold only one group when needed:

```bash
specloop scaffold agents
specloop scaffold skills
specloop scaffold scripts
specloop scaffold commands
specloop scaffold scenarios
specloop scaffold loops
```

The script scaffold also includes small authoring helpers:

```bash
node --experimental-strip-types scripts/specloop-new-scenario.ts --name "pricing smoke" --url "http://localhost:3000"
node --experimental-strip-types scripts/specloop-new-loop.ts --name "checkout smoke" --trigger "manual"
```

## What The MVP Includes

- Rust Cargo workspace.
- CLI commands: `init`, `doctor`, `run`, `report`, `validate`, `scaffold`, `export`.
- Scaffold targets: `agents`, `skills`, `scripts`, `commands`, `scenarios`,
  `loops`, and `all`.
- Versioned JSON schemas for configs, specs, findings, reports, and QA assets.
- Read-only safety defaults.
- Cold/hot memory model for prompt caching and run retrieval.
- Optional Claude-compatible agents and skill assets.
- Docs-first product structure for GitHub Pages.

## Safety Defaults

SpecLoop defaults to dry-run, read-only operation. Use `specloop run --execute`
only when you intentionally want a run marked as executed. It blocks common
destructive actions, does not persist cookies by default, redacts obvious secret
markers in rendered finding/report fields, and separates local/staging/production
intent in configuration.

## License

This project is source-available under PolyForm Noncommercial 1.0.0. Personal,
educational, research, nonprofit, and internal noncommercial use are allowed.
Commercial use, resale, hosted competing services, paid bundles, and enterprise
redistribution require a separate commercial license from Anderson Lima.

See [LICENSE](LICENSE).

## Docs

The docs app lives in `apps/docs` and follows an Astro/Starlight docs-first model.

## Status

SpecLoop is in v0 MVP. The browser runtime is still contract-first, with
scaffolded Playwright helpers for local and remote smoke scenarios. Full Chrome
DevTools MCP integration is the next execution layer.
