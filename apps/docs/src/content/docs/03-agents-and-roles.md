---
title: "03 - Agents And Roles"
description: "How agents fit without provider lock-in."
---

SpecLoop treats agents as optional executors of a protocol.

The protocol is stable even when the harness changes:

- Claude Code can use `.claude/agents` and `.claude/skills`.
- Codex can read `AGENTS.md` and run the CLI.
- Custom systems can consume JSON schemas and reports.

## Suggested Roles

- QA explorer: exercises flows and writes findings.
- Report triage: dedupes and prioritizes accepted findings.
- Implementation planner: turns accepted findings into fix plans.
- Reviewer: checks spec compliance and code quality.

The MVP scaffolds compatibility assets, but does not require them.
