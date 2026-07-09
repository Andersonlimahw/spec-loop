# ADR-0002 — Docs First On GitHub Pages

## Status

Accepted

## Context

SpecLoop is open-core and educational. Early adoption depends on trust,
explainability, examples, and protocol clarity more than conversion-focused
landing pages.

## Decision

Ship documentation first with Astro/Starlight and GitHub Pages. A separate Vercel
landing page waits until the CLI, demo, and paid offer are validated.

## Consequences

- Good: low-cost publishing and strong developer onboarding.
- Good: coherent with the existing AI Native Developer editorial style.
- Bad: weaker short-term lead capture.
- Risk: docs can drift from CLI behavior; CI must build docs and CLI together.
