---
title: "02 - The Core Loop"
description: "How one SpecLoop run works."
---

A SpecLoop run has a stable shape:

1. Load cold context.
2. Load hot run context.
3. Check guardrails.
4. Execute or receive runner evidence.
5. Compare evidence with specs.
6. Write findings immediately.
7. Render reports.
8. Suggest regression tests.

## Why Immediate Findings

Agentic browser runs can fail halfway through. Writing each finding immediately
keeps evidence durable and makes the run resumable.

## Pass, Fail, Drift

SpecLoop should distinguish:

- `pass`: behavior matches spec;
- `fail`: behavior violates spec;
- `drift`: product behavior changed and the spec may need review.
