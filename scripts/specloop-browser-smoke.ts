#!/usr/bin/env -S node --experimental-strip-types
import { spawnSync } from "child_process";
import { mkdir, writeFile } from "fs/promises";
import { dirname, join, resolve } from "path";
import { pathToFileURL } from "url";

type Args = Map<string, string>;

function parseArgs(argv: string[]): Args {
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

function runOrThrow(command: string, args: string[], options?: { cwd?: string }) {
  const result = spawnSync(command, args, {
    stdio: "inherit",
    cwd: options?.cwd,
  });
  if (result.status !== 0) {
    throw new Error(`failed to run ${command} ${args.join(" ")}`);
  }
}

async function ensureRuntimePackage(runtimeDir: string) {
  await mkdir(runtimeDir, { recursive: true });
  await writeFile(
    join(runtimeDir, "package.json"),
    '{ "private": true, "type": "module", "dependencies": {} }\n',
    { flag: "wx" },
  ).catch((error: unknown) => {
    if (!(error instanceof Error) || !("code" in error) || error.code !== "EEXIST") {
      throw error;
    }
  });
}

async function importRuntimePlaywright(runtimeDir: string) {
  const entrypoint = pathToFileURL(
    join(runtimeDir, "node_modules", "playwright", "index.mjs"),
  ).href;
  return import(entrypoint);
}

async function loadPlaywright() {
  try {
    return await import("playwright");
  } catch {
    const runtimeDir = resolve(".specloop/runtime");
    try {
      return await importRuntimePlaywright(runtimeDir);
    } catch {
      // Install below.
    }

    await ensureRuntimePackage(runtimeDir);
    console.log("Installing Playwright under .specloop/runtime with npm...");
    runOrThrow("npm", ["install", "--prefix", runtimeDir, "playwright"]);
    runOrThrow("npx", ["playwright", "install", "chromium"], { cwd: runtimeDir });
    return importRuntimePlaywright(runtimeDir);
  }
}

async function ensureParent(path: string) {
  await mkdir(dirname(path), { recursive: true });
}

async function quietNetwork(page: {
  waitForLoadState(state: "networkidle", options: { timeout: number }): Promise<void>;
  waitForTimeout(ms: number): Promise<void>;
}) {
  try {
    await page.waitForLoadState("networkidle", { timeout: 5000 });
  } catch {
    await page.waitForTimeout(1000);
  }
}

async function save(
  page: { screenshot(options: { path: string; fullPage: boolean }): Promise<void> },
  path: string,
) {
  await ensureParent(path);
  await page.screenshot({ path, fullPage: true });
  console.log(`screenshot: ${path}`);
}

function compact(value: string) {
  return value.length > 240 ? `${value.slice(0, 237)}...` : value;
}

const args = parseArgs(process.argv);
const target = args.get("target") ?? "local";
const outDir = args.get("out") ?? ".specloop/screenshots";
const localUrl = args.get("url") ?? "http://localhost:3000";
const lemonUrl = args.get("url") ?? "https://lemon.dev.br/pt";

const { chromium } = await loadPlaywright();
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
const consoleErrors: string[] = [];
const failedRequests: string[] = [];

page.on("console", (message: { type(): string; text(): string }) => {
  if (message.type() === "error") consoleErrors.push(message.text());
});
page.on(
  "requestfailed",
  (request: {
    method(): string;
    url(): string;
    failure(): { errorText: string } | null;
  }) => {
    failedRequests.push(
      `${request.method()} ${request.url()} ${request.failure()?.errorText ?? ""}`.trim(),
    );
  },
);

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
      const normalize = (value: string) =>
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
