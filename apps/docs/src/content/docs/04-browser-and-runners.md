---
title: "04 - Browser And Runners"
description: "Chrome DevTools MCP, Playwright, and shell runner boundaries."
---

SpecLoop keeps runner execution behind contracts.

The runtime surface has two layers:

- scaffolded Playwright helpers for repeatable local and remote smoke scenarios;
- contract-first agent/MCP execution for broader browser QA loops.

The primary runner targets are:

- Chrome DevTools MCP for real browser inspection;
- Playwright for deterministic replay and regression paths;
- shell for local validation commands and fixtures.

## Guardrail First

Runners must receive safety policy before actions:

- read-only by default;
- no purchase, payment, transfer, delete, or destructive mutation;
- no cookie persistence by default;
- warnings for untrusted MCP servers.

## Why Contract First

Contract-first runners keep the CLI useful before every adapter is implemented
and reduce provider lock-in.
