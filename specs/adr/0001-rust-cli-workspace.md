# ADR-0001 — Rust Cargo Workspace For The CLI

## Status

Accepted

## Context

SpecLoop must be safe for local automation, easy to distribute as a binary, and
strict about filesystem/process boundaries. The product also needs clean internal
boundaries: CLI, config, schemas, safety, reporter, templates, and core workflow.

## Decision

Use a Rust Cargo workspace with focused crates:

- `specloop-cli`
- `specloop-core`
- `specloop-config`
- `specloop-schemas`
- `specloop-safety`
- `specloop-reporter`
- `specloop-templates`

## Consequences

- Good: strong typing, easy binary distribution, clear internal seams.
- Good: schemas and safety rules can be tested independently.
- Bad: docs/frontend contributors need Rust setup for CLI work.
- Risk: too many crates too early; keep each crate small and only split further when needed.
