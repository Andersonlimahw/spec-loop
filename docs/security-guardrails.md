# Security Guardrails

SpecLoop is read-only by default.

## Default Blocks

- delete;
- purchase;
- payment;
- transfer;
- destructive mutation;
- production writes without explicit override;
- cookie/session persistence by default.

## Sensitive Data

Reports should capture location and shape, never raw secret values. The current
safety crate redacts obvious secret markers such as `token=`, `secret=`, and
`authorization: bearer`.

## MCP And Browser Runners

Chrome DevTools MCP and Playwright are powerful. Treat them as untrusted until
the project profile explicitly marks them trusted and the run target is known.
