---
title: "06 - CLI And Workflows"
description: "The current Rust CLI commands."
---

## Commands

```bash
specloop init
specloop doctor
specloop validate
specloop run
specloop report
specloop scaffold
specloop export
```

Most commands support the current working directory as the project root. Generated
files are skipped when they already exist; use `--force` on `init` or `scaffold`
when you intentionally want to overwrite generated assets.

## Typical Flow

```bash
specloop init
specloop validate
specloop run --action "open landing page"
specloop report
```

`init` creates the core `.specloop/` context, schemas, starter scenarios, and
starter loop files:

- `.specloop/scenarios/localhost-3000-smoke.md`
- `.specloop/scenarios/lemon-content-screenshots.md`
- `.specloop/loops/manual-browser-qa-loop.md`
- `.specloop/loops/ITERATION_LOG.md`

`run` is a dry-run contract by default. Use `specloop run --execute` only when a
configured runner actually executed the flow and safety checks allow it.

For browser work, `run` does not magically start or drive your app. Use it to
record the run contract, then run a scenario with an agent or scaffolded script.
Local apps must already be running.

## Scaffold

```bash
specloop scaffold all
specloop scaffold agents
specloop scaffold skills
specloop scaffold scripts
specloop scaffold commands
specloop scaffold scenarios
specloop scaffold loops
```

`specloop scaffold all` writes 14 optional compatibility assets: agents, skills,
commands, scripts, scenarios, and loops. They are helpers, not product lock-in.

`specloop scaffold scripts` writes:

- `scripts/specloop-browser-smoke.ts`
- `scripts/specloop-new-scenario.ts`
- `scripts/specloop-new-loop.ts`

Run them with Node.js 22+ using `node --experimental-strip-types` (no Bun or root
`package.json` required). Playwright installs on demand under `.specloop/runtime/`.

The authoring helpers create focused markdown runbooks:

```bash
node --experimental-strip-types scripts/specloop-new-scenario.ts --name "pricing smoke" --url "http://localhost:3000"
node --experimental-strip-types scripts/specloop-new-loop.ts --name "checkout smoke" --trigger "manual"
```

## Browser Examples

```bash
# Local app already running on localhost:3000
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local

# Local app running somewhere else
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local --url http://localhost:5173

# Remote read-only example
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target lemon
```

The `lemon` target opens `https://lemon.dev.br/pt`, scrolls the landing to the
footer, saves `.specloop/screenshots/lemon-landing.png`, clicks `Conteúdo`, and
saves `.specloop/screenshots/lemon-blog.png`.

The `local` target does not start your app. It expects a running app and saves
`.specloop/screenshots/local-landing.png`.

On first use, the helper installs Playwright and Chromium under
`.specloop/runtime/` if they are not already available.

## Plugin Install

```bash
codex plugin marketplace add https://github.com/Andersonlimahw/spec-loop
codex plugin add spec-loop
```

The plugin exposes SpecLoop skills and browser MCP defaults. Install the CLI
binary separately with `cargo install` until packaged releases are published.
