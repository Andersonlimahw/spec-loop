---
name: specloop-mobile-ui-loop
description: "Run an agentic testing loop for Mobile applications (SwiftUI, iOS, Android) that navigates the UI, analyzes accessibility, takes screenshots, and verifies visual specs."
---

# SpecLoop Mobile UI Loop

Use this skill to automate a UI and visual verification loop for mobile and native macOS applications using Computer Use or similar automation tools (e.g. Appium, Maestro).

## Procedure

1. **Load Context**: Read `.specloop/product.md` and design specs to understand the expected UI state.
2. **Setup Environment**: Ensure the simulator, emulator, or native app is running locally.
3. **Navigate & Investigate**: Interact with the UI safely to reach the target scenarios defined in `.specloop/scenarios/`.
4. **Analyze Accessibility**: Inspect the Accessibility (AX) tree, ARIA equivalents, and tap targets. Verify proper labels and contrast.
5. **Capture Evidence**: Take screenshots and save them under `.specloop/screenshots/`.
6. **Record Findings**: Log visual regressions, accessibility violations, or layout issues as structured findings.
7. **Report**: Run `specloop report` to aggregate the mobile QA results.

## Rules

- Computer Use actions should be read-only by default (no purchasing, no destructive account actions).
- Always verify accessibility alongside visual layout.
- If the app crashes, capture the device crash logs to `.specloop/traces/` before completing the iteration.
