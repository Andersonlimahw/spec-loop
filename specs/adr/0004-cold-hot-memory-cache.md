# ADR-0004 — Cold/Hot Memory And Prompt Cache Split

## Status

Accepted

## Context

Agentic QA can waste context on stable specs and raw run artifacts. The AI
Engineering Guidebook pattern of CAG/RAG plus explicit agent memory suggests
separating stable, high-value knowledge from run-specific retrieval.

## Decision

Represent cold context and hot context separately:

- cold context: product, specs, business rules, acceptance criteria, critical flows,
  adapter contracts, prompt version;
- hot context: current target, run objective, recent findings, trace/log summaries,
  screenshots, scratchpad.

Cold context can be content-addressed and prompt-cached. Hot context is retrieved
per run under a fixed budget and only promoted to durable memory as typed facts.

## Consequences

- Good: lower repeated context cost.
- Good: safer memory because raw transcripts and secrets are not durable facts.
- Bad: cache invalidation becomes product logic.
- Risk: stale cached context can hide spec drift; invalidate on spec, prompt, or adapter contract changes.
