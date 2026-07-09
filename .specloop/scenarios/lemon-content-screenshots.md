# Lemon Content Screenshot Scenario

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
