---
title: "05 - Specs And Evals"
description: "Specs become executable quality checks."
---

SpecLoop turns product language into evaluation surfaces.

## Spec Shape

Each spec should define:

- ID;
- area;
- expected behavior;
- acceptance criteria;
- related business rules;
- regression suggestion when broken.

## Eval Discipline

Write acceptance criteria before the run. A vague goal such as "make the checkout
good" is not enough. A good criterion says what behavior is expected and how it
will be verified.

## Schemas

Run:

```bash
specloop export schemas
```

Schemas are written to `schemas/` and `.specloop/schemas/`.
