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
    'Usage: node --experimental-strip-types scripts/specloop-new-scenario.ts --name "pricing smoke" --url "http://localhost:3000"',
  );
  process.exit(1);
}

const slug = name
  .toLowerCase()
  .replace(/[^a-z0-9]+/g, "-")
  .replace(/^-|-$/g, "");
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

const force = args.get("force") === "true";
await mkdir(dirname(path), { recursive: true });
try {
  await writeFile(path, content, { flag: force ? "w" : "wx" });
  console.log(`scenario: ${path}`);
} catch (error: any) {
  if (error.code === "EEXIST") {
    console.error(`Error: file already exists, open '${path}'. Use --force to overwrite.`);
    process.exit(1);
  }
  throw error;
}
