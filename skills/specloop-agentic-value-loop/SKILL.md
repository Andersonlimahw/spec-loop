---
name: specloop-agentic-value-loop
description: "Run a repeatable, agent-driven workflow enforcing the 7 mandatory phases (Idea, Plan, Implement, Test, Gate, Ship, Reflect) to keep projects improving while maintaining SpecLoop quality standards."
---

# SpecLoop Agentic Value Loop

Use this workflow to transform documented automations into executable Value Loops. A loop is a repeatable iteration that delivers an increment of user value, validated by a non-negotiable SpecLoop "quality gate".

## Procedure

1. **Idea**: Define the value increment for this iteration.
2. **Plan**: Read cold context (specs, business rules, architecture) and plan the changes. Map out any risks in `.specloop/risk-register.md`.
3. **Implement**: Execute the plan securely, adhering to read-only safety when interacting with production and local contexts.
4. **Test**: Use `specloop-scenario` or specific test tools to validate the implementation locally.
5. **Gate**: Run `specloop doctor`, `specloop validate`, and any language-specific quality gates (e.g. `cargo fmt`, `npm run lint`).
6. **Ship**: Create the PR/Commit, run `specloop report` to document the iteration, and ensure changes are integrated.
7. **Reflect**: Triage findings and document lessons learned for the next iteration.

## Rules

- Never skip the Gate phase.
- Use `specloop report` to track iterations.
- If a gate fails, the loop must return to Implement or Plan.
