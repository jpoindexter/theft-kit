---
name: debugging-vercel-builds
description: "Use when a Vercel build or deployment fails. Covers inspecting build logs, triaging common failure causes (missing env vars, lockfile mismatches, TypeScript/lint errors, wrong framework config, project/team mismatch), and the secure credential flow for missing Stripe or database secrets. Trigger on Vercel build errors, deploy failures, or when the user reports a broken deployment."
metadata:
  version: 1.0.0
---

# Debugging Vercel Builds

Use this skill when a Vercel build or deployment fails. The goal is to identify
the root cause before changing code or asking the user for secrets.

## Step 1: Inspect the Build Error

Start with the Vercel build output:

- `vercel-list-deployments` to find the failed deployment ID, target, branch,
  and timestamp
- `vercel-get-deployment-events` with the deployment ID to read the build log
- Look for the first error, not the last — cascading failures start from one
  root cause

If the project ID is unknown, use `vercel-list-projects` first.

## Step 2: Classify the Failure

Read the build log and match it to one of these categories before acting:

### Missing environment variables

Symptoms: `process.env.X is undefined`, `Missing required environment variable`,
`NEXT_PUBLIC_*` is empty at build time, Stripe/Supabase client initialization
crashes.

Triage:

1. Run `vercel-get-project-env-vars` for the deployment target that failed
   (production, preview, or development). Check names, targets, and branch
   scope — never secret values.
2. Compare the env var names against what the app code expects.
3. Identify exactly which keys are missing and which deployment target they
   are missing from.

Response rules:

- **Name the missing keys explicitly** and explain why the app needs each one
  (e.g., "`NEXT_PUBLIC_SUPABASE_URL` is required at build time for the Supabase
  client to initialize; without it the client constructor throws").
- **Never ask the user to paste secrets into chat.** Secrets must flow through
  secure tooling, not conversation text.
- For **Stripe credentials** (`STRIPE_SECRET_KEY`,
  `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`, `STRIPE_WEBHOOK_SECRET`, or any
  `STRIPE_PRICE_ID_*` / `STRIPE_PRODUCT_ID_*` var):
  - Ask the user to connect Stripe using `cofounder run connect_stripe`
    instead of pasting keys.
  - Distinguish test mode vs live mode: test keys come from a Stripe Sandbox
    or test-mode dashboard; live keys come from the live-mode dashboard.
  - If you have access to Stripe tools, run
    `cofounder run get_stripe_connection_status` first to check whether Stripe
    is already connected and which credential slots are populated before asking
    the user to reconnect.
  - Once connected, use `cofounder run sync_stripe_env` only when
    `env.sync.synced` is false or `env.sync.missing_keys` is non-empty.
- For **database/Supabase secrets** (`NEXT_PUBLIC_SUPABASE_URL`,
  `NEXT_PUBLIC_SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`,
  `DATABASE_URL`, `DIRECT_URL`):
  - If you have access to `add-secret` / `get-secret`, use that secure tool
    flow to provision them.
  - If you do not have `add-secret`, tell the user
    which keys are missing and why, then ask them to add the values through
    the platform's secret management or escalate to Engineer.
  - Explain which target (production, preview, development) needs the value.
- For **any other secret** (API keys, OAuth secrets, signing keys):
  - If you have access to `add-secret` / `get-secret`, use that secure tool
    flow.
  - If you do not have `add-secret`, name the missing keys and explain what
    they are for, then ask the user to add them through the platform's secret
    management or escalate to the Engineer Agent.
  - Never ask the user to paste secret values into chat.

### Wrong env var mode or scope

Symptoms: build succeeds but runtime uses wrong values; preview deployment reads
production keys or vice versa; staging has no env vars at all.

Triage:

1. Check `vercel-get-project-env-vars` and compare the `target` array on each
   var against the deployment that failed.
2. Verify branch-scoped env vars if the project uses Vercel's branch-based
   preview scoping.
3. Confirm `NEXT_PUBLIC_*` vars are set — these must be present at build time,
   not only at runtime.

### Missing Stripe test or live keys

Symptoms: Stripe client throws `No API key provided`, checkout redirects fail,
webhook signature verification fails.

Triage:

1. Determine whether the failing deployment is production (needs live keys) or
   preview/staging (needs test keys).
2. Check `vercel-get-project-env-vars` for `STRIPE_SECRET_KEY`,
   `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`, and `STRIPE_WEBHOOK_SECRET`.
3. Verify mode alignment: test keys (starting with `sk_test_` / `pk_test_`)
   must not be on production, and live keys (`sk_live_` / `pk_live_`) must not
   be on preview unless intentionally configured.
4. If keys are missing, direct the user to connect Stripe via
   `cofounder run connect_stripe` — never accept pasted keys.

### Missing database or Supabase URL/keys

Symptoms: `connection refused`, `ECONNREFUSED`, `Invalid URL`,
`supabaseUrl is required`, Prisma/Drizzle migration fails during build.

Triage:

1. Check `vercel-get-project-env-vars` for `NEXT_PUBLIC_SUPABASE_URL`,
   `NEXT_PUBLIC_SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`,
   `DATABASE_URL`, and `DIRECT_URL`.
2. Confirm the values point to the correct Supabase project (not localhost or
   a deleted project).
3. Use the secure `add-secret` / `get-secret` flow to provision missing keys.

### Failed install or build command

Symptoms: `npm ERR!`, `bun install failed`, `pip install` errors, exit code 1
during the install phase.

Triage:

1. Check whether `installCommand` or `buildCommand` in `vercel.json` or
   project settings matches the actual package manager.
2. Verify the lockfile matches the package manager (`package-lock.json` for
   npm, `bun.lockb` / `bun.lock` for Bun, `pnpm-lock.yaml` for pnpm, `yarn.lock`
   for Yarn).
3. Check for private registry or scoped package auth issues.

### Lockfile or package manager mismatch

Symptoms: `Lockfile was generated with a different package manager`, multiple
lockfiles detected, Vercel auto-detects the wrong package manager.

Triage:

1. Check which lockfiles exist in the repo root and in the framework root
   directory.
2. Remove extra lockfiles or set the correct `installCommand` in Vercel
   project settings or `vercel.json`.
3. If the project uses Bun, confirm Vercel is configured for Bun, not npm.

### TypeScript, lint, or build errors

Symptoms: `tsc` errors, ESLint errors, `next build` type failures, `Type error:`
in the build log.

Triage:

1. Read the specific error messages — these are app code issues, not
   configuration problems.
2. Do not "fix" code if the root cause is a missing env var that causes a type
   error (e.g., `process.env.X` being `undefined` when the code expects
   `string`). Fix the env var first.
3. If the error is genuine app code, fix it with direct repository coding tools
   using the exact error message, file, and line number.

### Missing framework config or wrong root directory

Symptoms: `No framework detected`, `Could not find a Next.js config`,
build runs from the wrong directory.

Triage:

1. Check `vercel.json` or Vercel project settings for `framework` and
   `rootDirectory`.
2. Verify the root directory contains the expected config file
   (`next.config.js`, `next.config.ts`, `vite.config.ts`, etc.).
3. For monorepos, confirm the root directory is set to the app package, not
   the repo root.

### Vercel project or team mismatch

Symptoms: deployment targets the wrong project, env vars belong to a different
project, `vercel link` points to a stale project.

Triage:

1. Check `.vercel/project.json` in the repo for `projectId` and `orgId`.
2. Compare against the Vercel dashboard project.
3. Re-link if the IDs do not match.

### Domain or env mismatch for webhook URLs

Symptoms: webhooks work in preview but fail in production, or vice versa;
`APP_URL` or `NEXT_PUBLIC_APP_URL` points to the wrong domain.

Triage:

1. Check `vercel-get-project-env-vars` for `APP_URL`,
   `NEXT_PUBLIC_APP_URL`, `NEXTAUTH_URL`, or similar.
2. Confirm the value matches the actual domain serving each deployment target.
3. For Stripe webhooks, verify the webhook endpoint URL matches the
   production domain, not a preview URL.

## Stripe Payments Notes

When this skill is activated for Engineer-owned Stripe or payments work:

- If Stripe credentials are missing, run
  `cofounder run get_stripe_connection_status --include_payments_implementation true`
  first to check existing connection state before asking the user to reconnect.
- Distinguish test mode vs live mode credentials. Test keys must come from a
  Stripe Sandbox or test-mode dashboard. Live keys come from the live-mode
  dashboard.
- Use `cofounder run connect_stripe --mode test` or
  `cofounder run connect_stripe --mode live` when credentials need to be
  connected.
- Only run `cofounder run sync_stripe_env` when `env.sync.synced` is false or
  `env.sync.missing_keys` is non-empty.
- Do not assume missing Stripe env vars mean Stripe is not connected — the
  connection may exist but env sync may be pending.
- For non-Stripe secrets (database URLs, Supabase keys, other API keys), use
  the secure secret tool flow when available. Otherwise name the missing keys
  and explain why they are needed without asking the user to paste values into
  chat.

## Engineer Agent Notes

When this skill is activated by the Engineer Agent:

- **Identify whether the problem is app code vs configuration before changing
  code.** Most Vercel build failures are caused by missing env vars, wrong
  project settings, or lockfile issues — not by app code bugs.
- Do not "fix" code when the root cause is missing configuration. A type error
  caused by `process.env.X` being `undefined` is an env var problem, not a
  TypeScript problem.
- If the error is genuinely in app code, fix it with a narrow brief: the exact
  error, file, line, and what the fix should achieve.
- If env vars need to be added, use the secure tool flow — do not hardcode
  fallback values, add placeholder strings, or commit secrets.

## Output Shape

Report:

- the failure category from the list above
- the specific error message or log lines that identify the cause
- exactly which keys, files, or settings are wrong or missing
- the recommended fix with secure tool references for any credentials
- verification steps: what to check after the fix (redeploy, check build log,
  confirm env var presence)
