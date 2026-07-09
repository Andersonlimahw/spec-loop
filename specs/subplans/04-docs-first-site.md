# SP-04 — Docs First Site

## Intent

Ship an educational docs-first site inspired by the AI Native Developer ebook:
clear sidebar, short progressive chapters, Mermaid-friendly mental models, and
GitHub Pages deployment before any standalone commercial landing page.

## Write Scope

- `apps/docs/`
- `.github/workflows/docs.yml`

## Acceptance Criteria

- [x] Astro/Starlight app exists in `apps/docs`.
- [x] Sidebar covers mental model, core loop, agents, browser runners, specs/evals,
      CLI workflows, examples, and open-core roadmap.
- [x] GitHub Pages workflow builds the docs app.
- [x] PDF generation is documented as a later phase, not a blocker for MVP.

## Verify

```bash
cd apps/docs
npm install
npm run build
```
