---
name: specloop-backend-api-loop
description: "Run a backend validation loop that generates Postman collections, executes RESTful tests, and validates API compliance using OpenAPI/Swagger specs or code routes."
---

# SpecLoop Backend API Loop

Use this skill when a user wants to run a SpecLoop QA pass over a Backend API, validate RESTful compliance, or execute Postman/Newman tests automatically.

## Procedure

1. **Analyze Context**: Read OpenAPI specs (`openapi.yaml`, `openapi.json`), Swagger files, or code route handlers (Express, Fastify, Rust Axum/Actix).
2. **Identify Test Scenarios**: Map out endpoints, paths, HTTP methods, and auth requirements. Design tests for Success (200, 201, 204) and Failure edge-cases (400, 401, 422).
3. **Generate Assets**: Create a Postman Collection (v2.1.0) and Environment file with embedded JS test assertions (schema validation, headers, status codes).
4. **Execute**: Run the collection locally or against a staging environment using Newman or similar test runners.
5. **Report**: Save execution logs to `.specloop/traces/` and record findings as structured evidence using `specloop report`.
6. **Reflect**: Triage validation failures into the `.specloop/risk-register.md` or directly into implementation plans.

## Rules

- Do not execute destructive actions on production endpoints without explicit approval.
- Ensure test assertions validate not just status codes, but also JSON schema structures and security headers.
- Store sensitive auth tokens in environments, never hardcoded in the collection.
