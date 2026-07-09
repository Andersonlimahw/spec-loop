# Security Policy

## Supported Versions

SpecLoop is pre-1.0. Security fixes target the current `main` branch.

## Reporting A Vulnerability

Do not open a public issue for sensitive vulnerabilities.

Contact Anderson Lima through https://lemon.dev.br with:

- affected version or commit;
- reproduction steps;
- impact;
- whether secrets, PII, production actions, or browser sessions are involved.

## Safety Principles

- Read-only by default.
- No production writes without explicit approval outside the default loop.
- No cookie/session persistence by default.
- No raw secret values in reports.
- MCP servers and browser runners must be treated as untrusted until configured.
