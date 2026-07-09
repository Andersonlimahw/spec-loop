#!/usr/bin/env -S node --experimental-strip-types
import { mkdir, writeFile } from "fs/promises";
import { dirname, join } from "path";

function parseArgs(argv: string[]) {
  const args = new Map<string, string>();
  for (let i = 2; i < argv.length; i += 1) {
    const arg = argv[i];
    if (!arg.startsWith("--")) continue;
    const key = arg.slice(2);
    const value =
      argv[i + 1] && !argv[i + 1].startsWith("--") ? argv[++i] : "true";
    args.set(key, value);
  }
  return args;
}

const args = parseArgs(process.argv);
const name = args.get("name");
if (!name) {
  console.error(
    'Usage: node --experimental-strip-types scripts/specloop-new-loop.ts --name "checkout smoke" --trigger "manual"',
  );
  process.exit(1);
}

const slug = name
  .toLowerCase()
  .replace(/[^a-z0-9]+/g, "-")
  .replace(/^-|-$/g, "");
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

const force = args.get("force") === "true";
await mkdir(dirname(path), { recursive: true });
try {
  await writeFile(path, content, { flag: force ? "w" : "wx" });
  console.log(`loop: ${path}`);
} catch (error: any) {
  if (error.code === "EEXIST") {
    console.error(`Error: file already exists, open '${path}'. Use --force to overwrite.`);
    process.exit(1);
  }
  throw error;
}
