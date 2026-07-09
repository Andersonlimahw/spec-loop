# SpecLoop PLAN

Status: in-progress  
Owner: Codex orchestrator  
Method: Spec-Driven Development with subagent-ready vertical slices

## Execution Map

```text
EXEC-MAP v1
intent:   build-code
executor: codex
effort:   high
time:     ~2-4h
tokens:   ~80k-140k
skills:   [senior-prompt-engineer, skills-selector, smart-dispatch, sdd, git-worktree, git-save, Agent Development, skill-creator]
models:   {plan: quality, impl: balanced, mechanical: budget}
agents:   [planner, docs-template-reviewer, implementation-reviewer]
mcp:      [chrome-devtools optional, playwright optional]
notes:    Notion tools unavailable in this session; local PLAN/specs are the source of truth.
```

## Product Goal

Ship a first usable SpecLoop MVP: a Rust CLI and local protocol that can initialize
spec-driven QA context, validate safety defaults, generate a run report, export
schemas, and scaffold optional agent/skill compatibility assets.

## Subplans Index

| ID | Subplan | Type | Status | Owner | Verify |
|---|---|---|---|---|---|
| SP-01 | [Spec Contract](specs/subplans/01-spec-contract.md) | AFK | done | Orchestrator | schemas exported and tested |
| SP-02 | [Rust CLI MVP](specs/subplans/02-rust-cli-mvp.md) | AFK | done | Orchestrator | init/validate/run/report work |
| SP-03 | [Safety And Memory](specs/subplans/03-safety-memory.md) | AFK | done | Orchestrator | destructive actions blocked, cold/hot model documented |
| SP-04 | [Docs First Site](specs/subplans/04-docs-first-site.md) | AFK | done | Docs subagent + Orchestrator | docs app scaffold exists |
| SP-05 | [Open Core Operations](specs/subplans/05-open-core-ops.md) | AFK | done | Orchestrator | license, contribution, security, CI |

## Subagent Development Protocol

Each subplan can be assigned to a fresh implementer subagent with this contract:

1. Read only `PLAN.md`, the specific `specs/subplans/<id>.md`, and files listed in
   that subplan.
2. Make changes only inside the declared write scope.
3. Run the verify command listed in the subplan.
4. Report `DONE`, `DONE_WITH_CONCERNS`, `NEEDS_CONTEXT`, or `BLOCKED`.
5. After implementation, a reviewer checks spec compliance, then code quality.

## Quality Gates

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo run -p specloop-cli -- init --force`
- `cargo run -p specloop-cli -- validate`
- `cargo run -p specloop-cli -- run --action "open landing page"`
- `cargo run -p specloop-cli -- report`

## Architecture Decisions

- Rust Cargo workspace for binary distribution and strong local automation.
- Docs-first publication on GitHub Pages using Astro/Starlight.
- PolyForm Noncommercial 1.0.0 for the public repo until a dual-license program is formalized.
- Browser execution is contract-first in v0; Chrome DevTools MCP and Playwright runtime adapters evolve after the run/report loop is stable.
- Memory is explicit system design: cold cacheable context plus hot per-run retrieval.

## Definition Of Done

- MVP CLI commands compile and pass tests.
- SDD specs and ADRs exist.
- Open-core legal/security docs exist.
- CI validates Rust and docs.
- Work is committed on `codex/specloop-mvp`, merged to `main`, and the linked worktree is removed.
