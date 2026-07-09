# SpecLoop — Refined Product Idea

## Product

SpecLoop is a Rust CLI and open-core protocol for spec-driven agentic QA loops.
It helps AI-native teams validate real product behavior against product specs,
business rules, acceptance criteria, and critical flows.

SpecLoop is provider-agnostic. It can be used by Claude Code, Codex, Cursor,
custom agents, shell runners, Playwright, Chrome DevTools MCP, or future runners
without becoming coupled to any one harness.

## Fixed Naming

| Surface | Value |
|---|---|
| Product | SpecLoop |
| Repository | `spec-loop` |
| Package | `spec-loop` |
| CLI | `specloop` |
| Config | `specloop.yaml` |
| Local directory | `.specloop/` |
| Tagline | Spec-driven QA loops for AI-native teams |

## MVP Thesis

Given a versioned product spec and a safe local project profile, `specloop`
should initialize a project, validate the QA context, generate a run contract,
collect or receive evidence, and render structured reports that distinguish
pass/fail/drift.

The MVP must prove one thing well:

```txt
Product Specs + Business Rules + Critical Flows + Safe Runner Contracts
= repeatable QA loop with structured findings and regression suggestions
```

## Current MVP Scope

- Rust Cargo workspace.
- CLI commands: `init`, `doctor`, `run`, `report`, `validate`, `scaffold`, `export`.
- Versioned JSON schemas for config, manifest, specs, findings, reports, and run assets.
- Safe defaults: read-only, no destructive actions, no cookie/session persistence.
- Local `.specloop/` context scaffold.
- Optional Claude-compatible agents and skill assets, without provider lock-in.
- Docs-first open-core positioning.

## Explicitly Out Of Scope For v0

- Dashboard SaaS.
- Remote runners or fleet orchestration.
- Automated code implementation of findings.
- Full MCP adapter runtime.
- Broad runner abstraction beyond shell/Playwright/Chrome DevTools contracts.
- Commercial premium kit implementation.

## Memory And Caching Model

SpecLoop follows a cold/hot context split inspired by cache-augmented generation
and AI memory systems.

Cold context is stable and cacheable by content hash:

- base loop prompt;
- schema contracts;
- adapter contracts;
- project profile;
- product specs;
- business rules;
- critical flows;
- canonical few-shots.

Hot context is run-specific and retrieved on demand:

- target URL and environment;
- current run objective;
- recent findings;
- traces, screenshots, console, and network summaries;
- relevant prior run facts with provenance.

Durable memory should be typed facts with source, confidence, and optional TTL.
Raw transcripts, full DOM dumps, secrets, and unsanitized logs must not become
long-term memory.

## Open-Core Strategy

The open core contains the CLI, schemas, local protocol, examples, templates,
docs, and compatibility assets. The author retains exclusive monetization rights
for paid kits, hosted SaaS, services, and enterprise/self-hosted offerings.

The initial license is PolyForm Noncommercial 1.0.0, with clear contribution
terms and a planned dual-license path for commercial customers.

## Success Criteria

- `cargo fmt --all -- --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `cargo test --workspace` passes.
- `specloop init` creates a valid `.specloop/` workspace.
- `specloop validate` passes on generated defaults.
- `specloop run` writes a Markdown run report.
- `specloop scaffold all` writes optional agents and skills.
- Docs explain how the protocol remains provider-agnostic and safe by default.
