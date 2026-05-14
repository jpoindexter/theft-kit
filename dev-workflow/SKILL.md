---
name: dev-workflow
description: >-
  Use before starting implementation work in a managed coding-agent sandbox.
  Covers the repository checkout, branch constraints, already-warmed dev
  server/GIC browser setup, browser verification, commit/push timing, default
  PR ownership, and final handoff evidence.
---

# Dev Workflow

Read this before implementing in a coding-agent sandbox. It keeps the launch
prompt small while preserving the operational rules that make app changes
reviewable.

## Sandbox Map

- The app repository checkout is `/workspace/repo`.
- `/workspace/work` is parent-agent scratch and artifacts, not the repo.
- Read the repo's own `AGENTS.md`, `CLAUDE.md`, README, package scripts, and env
  examples before assuming commands or structure.
- Check the injected skills folder for task-relevant skills and read the
  relevant `SKILL.md` files before using them.

## Design Context For UI Work

Before planning or editing browser-visible UI, site, page, styling, brand, or
frontend implementation work, check these design-context files and read any
that exists:

1. `/workspace/work/artifacts/DESIGN.md`
2. `/workspace/library/design/DESIGN.md`
3. `/workspace/repo/DESIGN.md` or `/workspace/repo/artifacts/DESIGN.md`

Treat an existing `DESIGN.md` as the visual and brand source of truth for color,
type, spacing, component treatment, page rhythm, and interaction tone. If the
task also uses `web-design-guidelines`, read `DESIGN.md` before choosing
annotation templates so the selected template fills gaps instead of overriding
first-party direction. If no usable `DESIGN.md` exists, continue from the
repo's existing design system and state the gap briefly.

## Branch Rules

- Stay on the branch assigned in the launch message. Do not create another
  branch unless the task explicitly asks for one.
- If you are explicitly asked to create a branch, use the `cofounder-cto/`
  prefix.
- Stage and commit only intentional changes. Do not sweep unrelated dirty files
  into a commit.

## Local Supabase

Repo-backed product app sandboxes start a best-effort background warmup after
clone. The helper `gic-app-warmup /workspace/repo` runs
`ensure-supabase-up.sh` when available, writes `.env`, runs `bun install` when
needed, starts the dev server on port 3000, and calls `gic-browser prepare`.
Assume the dev server and prepared GIC browser are already available for the
agent unless the warmup evidence says otherwise. Before manually redoing
startup, inspect `/home/user/.cache/gic-app-warmup/warmup.json` and
`/home/user/.cache/gic-app-warmup/warmup.log`. The helper runs as `user` with
`HOME=/home/user`, so do not use `~` for these files from terminal shells that
may have a different home.

Rerun this flow when the warmup failed, the repo has a `supabase/` directory and
needs a reset, the app is missing Supabase env vars, or browser verification is
blocked by auth/database state:

1. Run `gic-app-warmup /workspace/repo`, setting `GIC_APP_WARMUP_PORT` first if
   the app must use a port other than 3000.
2. If Supabase needs a manual reset after a real warmup attempt, run
   `supabase status -o json` from `/workspace/repo` and copy local values into
   the app env:
   - `NEXT_PUBLIC_SUPABASE_URL` from `API_URL`
   - `NEXT_PUBLIC_SUPABASE_ANON_KEY` from `ANON_KEY`
   - `SUPABASE_SERVICE_ROLE_KEY` from `SERVICE_ROLE_KEY`
3. If Docker is unavailable, run `docker info`, wait briefly if it is still
   starting, and report the exact blocker if it stays unavailable.

Do not use placeholder Supabase values unless local Supabase fails after a real
attempt. Report the exact blocker if that happens.

If warmup or local dev startup fails because of repository-owned code,
Supabase migrations or seed data, dependency installation, app env, or the dev
server itself, fix that root cause in the repo and rerun
`gic-app-warmup /workspace/repo`. A migration-order error, missing table, bad
seed, install failure, or app boot crash is not an acceptable reason to skip
browser verification. After fixing it, rerun `gic-browser prepare` and
`agent-browser` against the changed flow. Only report a blocker when the browser
tooling/session itself fails, Docker remains unavailable after checking it, or
an external credential/input is genuinely required.

## Browser Verification

If the user explicitly asks for browser testing, UI verification, design
verification, or local app verification, do that in the sandbox first before
long repo archaeology.

For browser-visible app work, create an early live preview checkpoint. After
minimal repo orientation, check whether `gic-app-warmup` already has the app and
prepared browser ready, then get the current URL before changing code:

- `gic-browser preview-url`
- `agent-browser --cdp "$(gic-browser cdp-url)" get url`

If the prepared browser is not ready, rerun the helper or start the app with the repo's commands, then run `gic-browser prepare --path /target-route` before broad implementation or final verification. If the target route is not obvious yet, use the closest entry route and record that assumption. Keep the dev server and prepared browser session available so the user can inspect progress while you continue changing code, then reuse the same flow for final verification.

Before the first dev-server attempt, check dependency readiness. If
`package.json` exists and `node_modules` is missing, start the repo's package
manager install immediately before waiting on the dev server:

- Bun: use `bun install --frozen-lockfile` when a Bun lockfile exists, otherwise
  `bun install`.
- npm: use `npm ci` when `package-lock.json` exists, otherwise `npm install`.
- pnpm: use `pnpm install --frozen-lockfile` when `pnpm-lock.yaml` exists,
  otherwise `pnpm install`.
- Yarn: use `yarn install --frozen-lockfile` when `yarn.lock` exists, otherwise
  `yarn install`.

Do not wait for a failed `next dev`, `vite`, or equivalent app boot to discover
missing dependencies. If `node_modules` already exists or an install is already
running, do not reinstall.

After implementation, browser verification is required before commit, push, PR
creation, or final completion when the task:

- changes browser-visible behavior
- adds or modifies a page, route, modal, navigation item, form, dashboard, or
  app shell
- fixes a bug whose symptom appears in the browser
- explicitly asks for UI/browser/local app verification

Lint, typecheck, build, tests, a pushed branch, or an opened PR do not replace a
browser verification attempt. A curl response alone is not browser verification
when browser tooling is available.

Before browser verification, use a test plan from the parent agent or your
implementation summary. If none exists, write a compact plan with scenarios,
preconditions, steps, and success criteria.

## Auth Walls And Local Test Data

If browser verification lands on a login page, organization setup screen,
missing-account state, or missing-seed-data state, treat that as local QA setup
work instead of stopping at the blocker.

- For generic local QA, use the repo's seeded local test account or documented
  local login helper when one exists. Do not use the user's personal account
  unless the task depends on their org, session, permissions, or private state,
  or the user explicitly asks for it.
- If no usable local account, organization, or fixture exists, create local-only
  test data through the app's normal local setup path, seed data, or local
  Supabase/app APIs, then rerun the browser verification.
- If you edit `supabase/seed.sql` or need seed changes to apply, reset only the
  disposable local Supabase stack before retrying. Never reset a linked or
  remote Supabase project for browser QA.
- Continue the same browser scenario after auth setup and report the account or
  fixture path used at a high level. Do not include credentials in final output.
- Only report auth as a blocker when external credentials, user-specific state,
  or a non-local identity provider is genuinely required after the local setup
  paths have been tried.

## Browser Tooling

Follow the Browser Verification section in `/home/user/.codex/AGENTS.md` when
it exists. When `gic-browser` and `agent-browser` are on `PATH`, they are the
required workflow in coding-agent sandboxes.

- Assume the warmup already started the app and prepared the GIC browser. Check
  `/home/user/.cache/gic-app-warmup/warmup.json` and get the current preview
  URL before starting another server.
- Use the prepared browser first. Prepare again with
  `gic-browser prepare --path /target-route` only when a route is known and the
  current browser is not already on the needed route.
- Use one-shot commands such as:
  - `gic-browser preview-url`
  - `agent-browser --cdp "$(gic-browser cdp-url)" get url`
  - `agent-browser --cdp "$(gic-browser cdp-url)" snapshot -i`
  - `agent-browser --cdp "$(gic-browser cdp-url)" get text "body"`
  - `agent-browser --cdp "$(gic-browser cdp-url)" errors`
- Exercise at least one representative interaction for interactive UI, such as
  clicking a changed button/link, switching a tab/month/filter, submitting a
  small form, or opening the route from the page that links to it.
- If the prepared page, snapshot, or body text is empty, stay in the wrapper
  workflow: check `get url`, `get title`, and `errors`, wait for the app to
  settle, rerun `gic-browser prepare --path ...`, and retry.
- Run `gic-browser release` after the browser outcome is confirmed.

Do not substitute Playwright or ad hoc browser scripts while the wrappers are
available. Only report a browser blocker when the wrapper tooling/session
itself fails or a missing external input prevents verification.

## Commit, Push, And PRs

- Commit and push meaningful progress only after required browser verification
  passes with `agent-browser` against the prepared GIC browser, or the exact
  browser-tooling blocker is documented.
- Before pushing browser-visible work, look at the actual running app in the
  prepared browser session. Confirm the changed route or interaction behaves as
  intended; do not treat lint, typecheck, tests, build output, curl, or a local
  server log as a substitute for observing the app.
- Before committing browser-visible work, run
  `agent-browser --cdp "$(gic-browser cdp-url)" get url` and include that URL
  with the verification evidence.
- Create the PR yourself unless the task explicitly assigns PR ownership
  elsewhere. If it says the parent agent creates the PR, do not create one.
- When you create the PR, include the original task and verification evidence.
- Never skip hooks with `--no-verify`.

## Final Handoff

Include the highest-signal evidence:

- changed behavior and key files
- lint/typecheck/test commands and results
- browser scenarios attempted and observed outcomes, or the exact blocker
- commit/push/PR status, following the task's ownership instruction
