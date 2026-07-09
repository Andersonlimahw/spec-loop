#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateFile {
    pub path: &'static str,
    pub content: &'static str,
}

pub fn init_files() -> Vec<TemplateFile> {
    let mut files = vec![
        TemplateFile {
            path: ".specloop/product.md",
            content: PRODUCT,
        },
        TemplateFile {
            path: ".specloop/specs.md",
            content: SPECS,
        },
        TemplateFile {
            path: ".specloop/personas.md",
            content: PERSONAS,
        },
        TemplateFile {
            path: ".specloop/business-rules.md",
            content: BUSINESS_RULES,
        },
        TemplateFile {
            path: ".specloop/critical-flows.md",
            content: CRITICAL_FLOWS,
        },
        TemplateFile {
            path: ".specloop/acceptance-criteria.md",
            content: ACCEPTANCE_CRITERIA,
        },
        TemplateFile {
            path: ".specloop/risk-register.md",
            content: RISK_REGISTER,
        },
        TemplateFile {
            path: ".specloop/prompts/explorer.md",
            content: EXPLORER_PROMPT,
        },
        TemplateFile {
            path: "AGENTS.md",
            content: AGENTS,
        },
        TemplateFile {
            path: "CLAUDE.md",
            content: CLAUDE,
        },
    ];
    files.extend(scenario_files());
    files.extend(loop_files());
    files
}

pub fn scaffold_agent_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            path: ".claude/agents/specloop-qa-explorer.md",
            content: QA_EXPLORER_AGENT,
        },
        TemplateFile {
            path: ".claude/agents/specloop-report-triage.md",
            content: REPORT_TRIAGE_AGENT,
        },
        TemplateFile {
            path: ".claude/agents/specloop-scenario-writer.md",
            content: SCENARIO_WRITER_AGENT,
        },
    ]
}

pub fn scaffold_skill_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            path: ".claude/skills/specloop-loop/SKILL.md",
            content: LOOP_SKILL,
        },
        TemplateFile {
            path: ".claude/skills/specloop-scenario/SKILL.md",
            content: SCENARIO_SKILL,
        },
    ]
}

pub fn scaffold_script_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            path: "scripts/specloop-browser-smoke.ts",
            content: BROWSER_SMOKE_SCRIPT,
        },
        TemplateFile {
            path: "scripts/specloop-new-scenario.ts",
            content: NEW_SCENARIO_SCRIPT,
        },
        TemplateFile {
            path: "scripts/specloop-new-loop.ts",
            content: NEW_LOOP_SCRIPT,
        },
    ]
}

pub fn scaffold_command_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            path: ".claude/commands/specloop-run-scenario.md",
            content: RUN_SCENARIO_COMMAND,
        },
        TemplateFile {
            path: ".claude/commands/specloop-new-scenario.md",
            content: NEW_SCENARIO_COMMAND,
        },
    ]
}

pub fn scaffold_scenario_files() -> Vec<TemplateFile> {
    scenario_files()
}

pub fn scaffold_loop_files() -> Vec<TemplateFile> {
    loop_files()
}

fn scenario_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            path: ".specloop/scenarios/localhost-3000-smoke.md",
            content: LOCALHOST_SCENARIO,
        },
        TemplateFile {
            path: ".specloop/scenarios/lemon-content-screenshots.md",
            content: LEMON_SCENARIO,
        },
    ]
}

fn loop_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            path: ".specloop/loops/manual-browser-qa-loop.md",
            content: MANUAL_BROWSER_LOOP,
        },
        TemplateFile {
            path: ".specloop/loops/ITERATION_LOG.md",
            content: ITERATION_LOG,
        },
    ]
}

const PRODUCT: &str = r#"# Product Context

Describe the product, target users, environments, and surfaces that SpecLoop may inspect.

## Cache Class

Cold context. Stable product facts can be prompt-cached or loaded once per run.
"#;

const SPECS: &str = r#"# Product Specs

Write expected product behavior as verifiable specs.

| ID | Area | Expected behavior | Acceptance criteria |
|---|---|---|---|
| SPEC-001 | Example | The app loads without console errors. | AC-001 |
"#;

const PERSONAS: &str = r#"# Personas

| ID | Name | Goals | Constraints |
|---|---|---|---|
| P-001 | Visitor | Understand the product. | No authenticated actions. |
"#;

const BUSINESS_RULES: &str = r#"# Business Rules

Rules that define correct product behavior.

| ID | Rule | Severity if broken |
|---|---|---|
| BR-001 | Production runs are read-only unless explicitly approved. | P0 |
"#;

const CRITICAL_FLOWS: &str = r#"# Critical Flows

| ID | Persona | Flow | Expected outcome |
|---|---|---|---|
| FLOW-001 | P-001 | Open landing page. | Page renders and primary CTA is visible. |
"#;

const ACCEPTANCE_CRITERIA: &str = r#"# Acceptance Criteria

| ID | Spec | Verification |
|---|---|---|
| AC-001 | SPEC-001 | Browser console has no uncaught errors. |
"#;

const RISK_REGISTER: &str = r#"# Risk Register

Hot context. Update this after each run with fresh risks and mitigations.

| ID | Risk | Impact | Mitigation |
|---|---|---|---|
| RISK-001 | Agent writes real production data. | P0 | Read-only default and write approval gate. |
"#;

const EXPLORER_PROMPT: &str = r#"# SpecLoop Explorer Prompt

You are a provider-agnostic QA agent. Load cold context first, then hot run context.

1. Read product specs, business rules, acceptance criteria, critical flows, and risks.
2. Keep production read-only.
3. Explore one flow at a time.
4. Compare actual behavior with expected specs.
5. Write one structured finding immediately when evidence appears.
6. Suggest a regression test for every accepted finding.

For browser evidence, prefer the explicit scenario files in `.specloop/scenarios/`.
If the target is local, the app must already be running. If the target is remote,
keep the session read-only and save screenshots under `.specloop/screenshots/`.
"#;

const AGENTS: &str = r#"# SpecLoop Agent Instructions

- Treat `.specloop/product.md`, `.specloop/specs.md`, `.specloop/business-rules.md`, and `.specloop/critical-flows.md` as cold context.
- Treat `.specloop/risk-register.md`, `.specloop/findings/`, and `.specloop/reports/` as hot run context.
- Treat `.specloop/scenarios/` and `.specloop/loops/` as executable runbooks.
- Never perform destructive browser actions by default.
- Local targets require the user or harness to start the app first.
- Record findings as structured evidence, not vibes.
"#;

const CLAUDE: &str = r#"# Claude Compatibility

SpecLoop is provider-agnostic. These notes exist only to help Claude-compatible harnesses run the same protocol.

- Use `specloop doctor` before a run.
- Use `specloop validate` before accepting generated context.
- Use `specloop scaffold all` to generate optional `.claude/agents/`, `.claude/skills/`, `.claude/commands/`, and `scripts/` assets.
- To test a local app, start it first, then ask the agent to run a `.specloop/scenarios/` file.
- To test a remote app, ask the agent to run the scenario directly and keep browser actions read-only.
"#;

const QA_EXPLORER_AGENT: &str = r#"---
name: specloop-qa-explorer
description: |
  Use this agent when running provider-agnostic SpecLoop exploratory QA over a web app from specs, business rules, critical flows, and browser evidence. Examples:

  <example>
  Context: A project has `.specloop/specs.md` and needs a read-only QA pass.
  user: "Run a SpecLoop pass against the landing flow."
  assistant: "I will use the specloop-qa-explorer to load the specs, inspect the flow, and write structured findings."
  <commentary>
  The request asks for exploratory QA grounded in SpecLoop context.
  </commentary>
  </example>

  <example>
  Context: A report needs fresh evidence for one critical flow.
  user: "Check whether FLOW-001 still matches the business rules."
  assistant: "I will use the specloop-qa-explorer for that single critical flow and keep the run read-only."
  <commentary>
  The request targets a specific flow/spec compliance check.
  </commentary>
  </example>
model: inherit
color: blue
tools: ["Read", "Write", "Grep", "Bash", "mcp__chrome_devtools__list_pages", "mcp__chrome_devtools__select_page", "mcp__chrome_devtools__new_page", "mcp__chrome_devtools__navigate_page", "mcp__chrome_devtools__resize_page", "mcp__chrome_devtools__take_snapshot", "mcp__chrome_devtools__take_screenshot", "mcp__chrome_devtools__wait_for", "mcp__chrome_devtools__click_at", "mcp__chrome_devtools__hover", "mcp__chrome_devtools__fill", "mcp__chrome_devtools__type_text", "mcp__chrome_devtools__press_key", "mcp__chrome_devtools__evaluate_script", "mcp__chrome_devtools__list_console_messages", "mcp__chrome_devtools__list_network_requests", "mcp__chrome_devtools__handle_dialog"]
---

You are the SpecLoop QA explorer.

Process:
1. Load cold context from `.specloop/product.md`, `.specloop/specs.md`, `.specloop/business-rules.md`, and `.specloop/critical-flows.md`.
2. Load hot context from `.specloop/risk-register.md`, `.specloop/findings/`, and `.specloop/reports/`.
3. Pick one scenario from `.specloop/scenarios/` or one flow from `.specloop/critical-flows.md`.
4. If the target is local, verify the app is already running before opening a browser.
5. Exercise one flow at a time with read-only safety.
6. Capture screenshots/traces in the configured output folders when the scenario asks for evidence.
7. Compare observed behavior with specs and business rules.
8. Write one finding per evidence-backed mismatch immediately.
9. Never perform destructive production writes.

Output: run summary, findings count by severity, and paths written.
"#;

const REPORT_TRIAGE_AGENT: &str = r#"---
name: specloop-report-triage
description: |
  Use this agent when converting reviewed SpecLoop findings into deduplicated implementation plans, priorities, and regression-test suggestions. Examples:

  <example>
  Context: A SpecLoop run has several accepted finding files.
  user: "Triage the accepted findings from this run."
  assistant: "I will use specloop-report-triage to dedupe root causes, rank severity, and write implementation-plan stubs."
  <commentary>
  The request is triage, not browser exploration or implementation.
  </commentary>
  </example>

  <example>
  Context: The user reviewed findings and rejected noisy reports.
  user: "Turn the remaining findings into prioritized work."
  assistant: "I will use specloop-report-triage and skip rejected findings."
  <commentary>
  The agent should process reviewed findings into actionable work.
  </commentary>
  </example>
model: inherit
color: yellow
tools: ["Read", "Write", "Grep"]
---

You are the SpecLoop report triage agent.

Process:
1. Read `_INDEX.md` and reviewed finding files.
2. Skip rejected findings.
3. Dedupe by root cause.
4. Prioritize P0 to P3 by business impact.
5. Emit one implementation plan per root cause.

Output: prioritized task list, accepted/rejected counts, and suggested regression tests.
"#;

const LOOP_SKILL: &str = r#"---
name: specloop-loop
description: "Run the SpecLoop QA loop: load specs and business rules, enforce read-only safety, explore critical flows, write structured findings, triage accepted reports, and suggest regression tests."
---

# SpecLoop Loop

Use this workflow when a user asks to run a SpecLoop QA pass, triage SpecLoop findings, or scaffold a product-specific QA loop.

## Procedure

1. Run `specloop doctor`.
2. Load cold context: product, specs, business rules, acceptance criteria, critical flows.
3. Load hot context: risk register, prior findings, current run budget.
4. Select a scenario from `.specloop/scenarios/` or a flow from `.specloop/critical-flows.md`.
5. For local URLs, verify the user has started the app. Do not start unknown app servers unless asked.
6. Explore one flow at a time with read-only safety.
7. Save evidence under `.specloop/screenshots/` or `.specloop/traces/`.
8. Write each finding immediately.
9. Run `specloop report` and `specloop validate`.
10. Triage reviewed findings into implementation plans.

## Manual Runner

After `specloop scaffold scripts`, these examples are available:

```bash
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target lemon
```

## Rules

- Never perform destructive production actions by default.
- Do not store cookies or secrets.
- Do not batch findings until the end of a run.
- Suggest a regression test for every finding.
"#;

const SCENARIO_SKILL: &str = r#"---
name: specloop-scenario
description: Create, edit, or run SpecLoop scenario files under `.specloop/scenarios/`, including local localhost checks, remote landing/blog screenshot flows, and manual browser QA runbooks.
---

# SpecLoop Scenario

Use this skill when the user asks to create a QA scenario, add a browser flow, or run a specific `.specloop/scenarios/*.md` file.

## Procedure

1. Read `specloop.yaml` for target URL and output folders.
2. Read the chosen scenario file.
3. Confirm whether the target is local or remote.
4. For local targets, require the app to already be running unless the user asked you to start it.
5. Execute only read-only browser steps by default.
6. Save requested screenshots under `.specloop/screenshots/`.
7. Record findings as structured evidence, one file per issue.

## Create A Scenario

After `specloop scaffold scripts`:

```bash
node --experimental-strip-types scripts/specloop-new-scenario.ts --name "pricing smoke" --url "http://localhost:3000"
```

Keep scenarios small: one persona, one target, one expected outcome, and explicit evidence paths.
"#;

const SCENARIO_WRITER_AGENT: &str = r#"---
name: specloop-scenario-writer
description: |
  Use this agent when creating or updating SpecLoop scenario and loop markdown files. Examples:

  <example>
  Context: A product has `.specloop/` initialized but no browser scenario for a new route.
  user: "Create a scenario for the pricing landing page."
  assistant: "I will use specloop-scenario-writer to create a small read-only scenario with explicit evidence paths."
  <commentary>
  The request is scenario authoring, not browser execution.
  </commentary>
  </example>

  <example>
  Context: The user wants a repeatable QA loop for manual runs.
  user: "Add a manual loop for our checkout smoke pass."
  assistant: "I will use specloop-scenario-writer to draft the loop contract and verification gates."
  <commentary>
  The request needs reusable markdown assets under `.specloop/loops/`.
  </commentary>
  </example>
model: inherit
color: green
tools: ["Read", "Write", "Grep"]
---

You are the SpecLoop scenario writer.

Process:
1. Read `specloop.yaml` and existing `.specloop/scenarios/` or `.specloop/loops/`.
2. Create one focused scenario or loop file.
3. Include target URL, persona, steps, expected outcome, evidence paths, and safety notes.
4. Keep production actions read-only unless the user gives explicit approval.
5. Prefer extending an existing scenario over duplicating the same flow.

Output: created or updated file path and a short run command.
"#;

const LOCALHOST_SCENARIO: &str = r#"# Localhost 3000 Smoke Scenario

## Target

- URL: `http://localhost:3000`
- Environment: local
- Persona: visitor

## Operator Setup

Start the app first in the project being tested:

```bash
npm run dev
```

SpecLoop does not assume it may start arbitrary app servers. Once the app is up, ask the agent to run this scenario or run the script generated by `specloop scaffold scripts`.

## Steps

1. Open `http://localhost:3000`.
2. Wait for the page to render.
3. Capture a full-page screenshot.
4. Check console errors and failed network requests.

## Evidence

- Screenshot: `.specloop/screenshots/local-landing.png`
- Optional trace: `.specloop/traces/local-landing.json`

## Expected Outcome

The landing page renders without uncaught console errors and the primary content is visible.

## Script

```bash
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local
```
"#;

const LEMON_SCENARIO: &str = r#"# Lemon Content Screenshot Scenario

## Target

- URL: `https://lemon.dev.br/pt`
- Environment: production remote, read-only
- Persona: visitor

## Steps

1. Open `https://lemon.dev.br/pt`.
2. Navigate through the landing page down to the footer.
3. Save a full-page screenshot of the landing.
4. Click the `Conteudo` / `Conteúdo` navigation item.
5. Wait for the blog/content page to render.
6. Save a full-page screenshot of the blog.
7. Check console errors and failed network requests.

## Evidence

- Landing screenshot: `.specloop/screenshots/lemon-landing.png`
- Blog screenshot: `.specloop/screenshots/lemon-blog.png`
- Optional trace: `.specloop/traces/lemon-content.json`

## Expected Outcome

The landing page reaches the footer, the content navigation opens the blog page, and both screenshots are saved.

## Script

```bash
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target lemon
```
"#;

const MANUAL_BROWSER_LOOP: &str = r#"# Manual Browser QA Loop

## Goal

Run one small, evidence-backed browser scenario at a time and leave reusable artifacts behind.

## Loop

1. Select one `.specloop/scenarios/*.md` file.
2. Load cold context: product, specs, business rules, acceptance criteria, critical flows.
3. Load hot context: risk register, previous findings, previous reports.
4. Execute only the scenario steps. Keep production read-only.
5. Save screenshots and traces under `.specloop/screenshots/` and `.specloop/traces/`.
6. Write one finding per confirmed mismatch under `.specloop/findings/`.
7. Run `specloop validate` and `specloop report`.
8. Append a short note to `.specloop/loops/ITERATION_LOG.md`.
9. If a step was repeated manually, extract a script, skill, agent, or MCP tool.

## Local vs Remote

- Local target: start the app first, then run the scenario.
- Remote target: ask the agent to run the scenario directly, but keep it read-only.

## Quality Gate

- Scenario stayed in scope.
- No destructive browser actions.
- Evidence paths exist.
- Findings are structured evidence, not impressions.
"#;

const ITERATION_LOG: &str = r#"# SpecLoop Iteration Log

Append one entry per loop run.

## Template

```md
### <date> - <scenario>

- Target:
- Screens covered:
- Evidence:
- Findings:
- Repeated manual steps:
- Assets extracted:
- Next run:
```
"#;

const BROWSER_SMOKE_SCRIPT: &str = r#"#!/usr/bin/env -S node --experimental-strip-types
import { spawnSync } from "child_process";
import { mkdir, writeFile } from "fs/promises";
import { dirname, join, resolve } from "path";
import { pathToFileURL } from "url";

const args = new Map();
for (let i = 2; i < process.argv.length; i += 1) {
  const arg = process.argv[i];
  if (!arg.startsWith("--")) continue;
  const key = arg.slice(2);
  const value = process.argv[i + 1] && !process.argv[i + 1].startsWith("--") ? process.argv[++i] : "true";
  args.set(key, value);
}

function runOrThrow(command, args, options = {}) {
  const result = spawnSync(command, args, { stdio: "inherit", cwd: options.cwd });
  if (result.status !== 0) {
    throw new Error(`failed to run ${command} ${args.join(" ")}`);
  }
}

const target = args.get("target") ?? "local";
const outDir = args.get("out") ?? ".specloop/screenshots";
const localUrl = args.get("url") ?? "http://localhost:3000";
const lemonUrl = args.get("url") ?? "https://lemon.dev.br/pt";

async function loadPlaywright() {
  try {
    return await import("playwright");
  } catch {
    const runtimeDir = resolve(".specloop/runtime");
    const entrypoint = () =>
      pathToFileURL(join(runtimeDir, "node_modules", "playwright", "index.mjs")).href;
    try {
      return await import(entrypoint());
    } catch {
      // Install below.
    }
    await mkdir(runtimeDir, { recursive: true });
    await writeFile(
      join(runtimeDir, "package.json"),
      '{ "private": true, "type": "module", "dependencies": {} }\n',
      { flag: "wx" },
    ).catch((error) => {
      if (error?.code !== "EEXIST") throw error;
    });
    console.log("Installing Playwright under .specloop/runtime with npm...");
    runOrThrow("npm", ["install", "--prefix", runtimeDir, "playwright"]);
    runOrThrow("npx", ["playwright", "install", "chromium"], { cwd: runtimeDir });
    return import(entrypoint());
  }
}

async function ensureParent(path) {
  await mkdir(dirname(path), { recursive: true });
}

async function quietNetwork(page) {
  try {
    await page.waitForLoadState("networkidle", { timeout: 5000 });
  } catch {
    await page.waitForTimeout(1000);
  }
}

async function save(page, path) {
  await ensureParent(path);
  await page.screenshot({ path, fullPage: true });
  console.log(`screenshot: ${path}`);
}

function compact(value) {
  return value.length > 240 ? `${value.slice(0, 237)}...` : value;
}

const { chromium } = await loadPlaywright();
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
const consoleErrors = [];
const failedRequests = [];

page.on("console", (message) => {
  if (message.type() === "error") consoleErrors.push(message.text());
});
page.on("requestfailed", (request) => {
  failedRequests.push(`${request.method()} ${request.url()} ${request.failure()?.errorText ?? ""}`.trim());
});

try {
  if (target === "lemon") {
    await page.goto(lemonUrl, { waitUntil: "domcontentloaded", timeout: 30000 });
    await quietNetwork(page);
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
    await page.waitForTimeout(750);
    await save(page, join(outDir, "lemon-landing.png"));

    await page.evaluate(() => window.scrollTo(0, 0));
    await page.waitForTimeout(300);
    const clickedContent = await page.evaluate(() => {
      const normalize = (value) =>
        value
          .normalize("NFD")
          .replace(/\p{Diacritic}/gu, "")
          .trim()
          .toLowerCase();
      const link = Array.from(document.querySelectorAll("a")).find(
        (candidate) => normalize(candidate.textContent ?? "") === "conteudo",
      );
      if (!(link instanceof HTMLAnchorElement)) return false;
      link.click();
      return true;
    });
    if (!clickedContent) {
      await page.goto(new URL("/pt/blog", lemonUrl).toString(), {
        waitUntil: "domcontentloaded",
        timeout: 30000,
      });
    }
    await page.waitForURL(/\/blog/, { timeout: 10000 }).catch(() => {});
    await quietNetwork(page);
    await save(page, join(outDir, "lemon-blog.png"));
  } else {
    await page.goto(localUrl, { waitUntil: "domcontentloaded", timeout: 15000 });
    await quietNetwork(page);
    await save(page, join(outDir, "local-landing.png"));
  }
} finally {
  await browser.close();
}

if (consoleErrors.length) {
  console.log("console-errors:");
  for (const error of consoleErrors.slice(0, 20)) console.log(`- ${compact(error)}`);
}
if (failedRequests.length) {
  console.log("failed-requests:");
  for (const request of failedRequests.slice(0, 20))
    console.log(`- ${compact(request)}`);
}
"#;

const NEW_SCENARIO_SCRIPT: &str = r#"#!/usr/bin/env -S node --experimental-strip-types
import { mkdir, writeFile } from "fs/promises";
import { dirname, join } from "path";

const args = new Map();
for (let i = 2; i < process.argv.length; i += 1) {
  const arg = process.argv[i];
  if (!arg.startsWith("--")) continue;
  const key = arg.slice(2);
  const value = process.argv[i + 1] && !process.argv[i + 1].startsWith("--") ? process.argv[++i] : "true";
  args.set(key, value);
}

const name = args.get("name");
if (!name) {
  console.error('Usage: node --experimental-strip-types scripts/specloop-new-scenario.ts --name "pricing smoke" --url "http://localhost:3000"');
  process.exit(1);
}

const slug = name.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
const url = args.get("url") ?? "http://localhost:3000";
const persona = args.get("persona") ?? "visitor";
const path = args.get("path") ?? join(".specloop", "scenarios", `${slug}.md`);

const content = `# ${name}

## Target

- URL: \`${url}\`
- Persona: ${persona}

## Steps

1. Open \`${url}\`.
2. Wait for the page to render.
3. Exercise the primary read-only flow.
4. Save screenshots or traces as evidence.
5. Check console errors and failed network requests.

## Evidence

- Screenshot: \`.specloop/screenshots/${slug}.png\`

## Expected Outcome

Describe the observable result that proves this scenario passed.

## Safety

Keep production read-only. Stop before create, update, delete, payment, email, or irreversible actions.
`;

await mkdir(dirname(path), { recursive: true });
await writeFile(path, content, { flag: "wx" });
console.log(`scenario: ${path}`);
"#;

const NEW_LOOP_SCRIPT: &str = r#"#!/usr/bin/env -S node --experimental-strip-types
import { mkdir, writeFile } from "fs/promises";
import { dirname, join } from "path";

const args = new Map();
for (let i = 2; i < process.argv.length; i += 1) {
  const arg = process.argv[i];
  if (!arg.startsWith("--")) continue;
  const key = arg.slice(2);
  const value = process.argv[i + 1] && !process.argv[i + 1].startsWith("--") ? process.argv[++i] : "true";
  args.set(key, value);
}

const name = args.get("name");
if (!name) {
  console.error('Usage: node --experimental-strip-types scripts/specloop-new-loop.ts --name "checkout smoke" --trigger "manual"');
  process.exit(1);
}

const slug = name.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
const trigger = args.get("trigger") ?? "manual";
const path = args.get("path") ?? join(".specloop", "loops", `${slug}.md`);

const content = `# ${name}

## Trigger

${trigger}

## Goal

Describe the single value this loop verifies or improves.

## Phases

1. Select scenario and load context.
2. Execute read-only browser steps.
3. Save evidence.
4. Write findings immediately.
5. Validate and report.
6. Triage accepted findings.
7. Reflect and extract reusable assets.

## Quality Gate

- No destructive production actions.
- Evidence files exist.
- Findings include reproduction steps and suggested regression tests.
- Repeated manual steps are converted into script, skill, agent, or MCP assets.
`;

await mkdir(dirname(path), { recursive: true });
await writeFile(path, content, { flag: "wx" });
console.log(`loop: ${path}`);
"#;

const RUN_SCENARIO_COMMAND: &str = r#"---
description: Run one SpecLoop scenario with read-only browser safety.
allowed-tools: Read, Bash, Grep
---

# Run SpecLoop Scenario

1. Run `specloop doctor`.
2. Read the requested `.specloop/scenarios/*.md` file.
3. If it targets localhost, confirm the app is already running.
4. Run the matching script when available, for example:

```bash
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target local
node --experimental-strip-types scripts/specloop-browser-smoke.ts --target lemon
```

5. Save findings immediately under `.specloop/findings/`.
6. Run `specloop validate` and summarize evidence paths.
"#;

const NEW_SCENARIO_COMMAND: &str = r#"---
description: Create a new SpecLoop scenario markdown file.
allowed-tools: Read, Bash
---

# New SpecLoop Scenario

Use the scaffolded helper:

```bash
node --experimental-strip-types scripts/specloop-new-scenario.ts --name "$ARGUMENTS"
```

Then edit the generated file so the target, steps, expected outcome, and evidence paths are explicit.
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_core_context_files() {
        let paths = init_files()
            .into_iter()
            .map(|file| file.path)
            .collect::<Vec<_>>();

        assert!(paths.contains(&".specloop/product.md"));
        assert!(paths.contains(&".specloop/specs.md"));
        assert!(paths.contains(&".specloop/scenarios/lemon-content-screenshots.md"));
        assert!(paths.contains(&".specloop/loops/manual-browser-qa-loop.md"));
    }

    #[test]
    fn scaffolds_manual_assets() {
        let script_paths = scaffold_script_files()
            .into_iter()
            .map(|file| file.path)
            .collect::<Vec<_>>();
        let skill_paths = scaffold_skill_files()
            .into_iter()
            .map(|file| file.path)
            .collect::<Vec<_>>();
        let agent_paths = scaffold_agent_files()
            .into_iter()
            .map(|file| file.path)
            .collect::<Vec<_>>();

        assert!(script_paths.contains(&"scripts/specloop-browser-smoke.ts"));
        assert!(script_paths.contains(&"scripts/specloop-new-scenario.ts"));
        assert!(script_paths.contains(&"scripts/specloop-new-loop.ts"));
        assert!(skill_paths.contains(&".claude/skills/specloop-scenario/SKILL.md"));
        assert!(agent_paths.contains(&".claude/agents/specloop-scenario-writer.md"));
    }
}
