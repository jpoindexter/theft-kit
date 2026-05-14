---
name: gic-coding-agent-skills
description: "Use this bundled GIC coding skill for platform-managed implementation playbooks that require code-aware planning or direct repository coding. It consolidates the prior coding-oriented default skills into one shared guide for site rollout, analytics instrumentation, schema markup implementation, Vercel Workflows implementation, and Stripe billing scaffolding."
metadata:
  version: 1.0.0
---

# GIC Repository Coding Skills

This bundled skill replaces the older coding-oriented default skill entries.
Use it when the next useful step requires implementation planning or code
changes rather than strategy-only advice.

## Quick Route By Agent

- `Engineer`: use this bundle for bounded marketing-site, analytics, schema,
  SEO, workflow, or Stripe implementation work that should not become a broad
  product refactor. For SEO execution, use `site-operator` for marketing-site
  changes, `schema-markup` for structured data, and `analytics-tracking` when
  the SEO change also needs measurement.
- `Marketing Agent`: usually pair this bundle with `gic-skills` when strategy is done
  and the next step is shipping copy, metadata, tracking, or page changes.
- `Engineer`: for Stripe or payments work, use `stripe-generated-app-scaffold`
  after `stripe-app-builder` in `gic-skills` has already chosen the billing
  shape.

## Quick Route By Legacy Skill Name

- Frontend taste and page judgment: `taste-first-frontend`
- Marketing-site rollout: `site-operator`
- Measurement and analytics: `analytics-tracking`
- Structured data: `schema-markup`
- Durable orchestration on Vercel: `vercel-workflows`
- Stripe implementation: `stripe-generated-app-scaffold`
- Shader/WebGL visuals for apps, HTML artifacts, iframe UI artifacts, or
  HyperFrames: `shader-authoring`

## Core Rules

- Keep secrets out of prompts, commits, and source files. Use runtime config,
  secret storage, and placeholder env names where needed.
- Prefer the smallest working slice that proves the path forward.
- Verify success in code and runtime terms: files changed, routes added, events
  firing, schema validated, previews checked, or webhook behavior confirmed.
- Keep implementation grounded in the upstream product/marketing/SEO strategy
  selected via `gic-skills`.
- When a prompt mentions one of the replaced legacy skill names, jump directly
  to the matching subsection below instead of treating the bundle as ambiguous.

## Product PR Workflow

Use this workflow whenever an agent with direct repository coding tools is
shipping code:

- Treat the pushed branch or PR as a checkpoint, not the end of the task.
- If creating a PR reports `pull_request_already_exists`, reuse the returned
  existing PR number and URL instead of retrying PR creation.
- After a PR exists, check `github-list-check-runs` and
  `github-get-merge-blockers` before waiting or merging.
- If CI, typecheck, build, tests, browser verification, human review comments,
  or review-bot comments fail, fix the concrete failures and keep iterating
  until the PR is clean or a genuine external blocker remains.
- If browser verification is blocked because the local app, Supabase
  migrations/seed data, dependencies, env, or dev server do not start, treat
  that as a repository failure to fix and retest. Do not accept it as a
  browser-tooling blocker.
- When checks pass and review comments are resolved, call
  `github-merge-pull-request` for the PR. That tool is the policy gate for the
  user's auto-merge setting: if auto-merge is enabled it should merge; if
  auto-merge is disabled it will return the disabled-auto-merge response. Do
  not bypass that response with `gh pr merge`, raw GitHub API calls, or other
  merge workarounds.
- Only report completion after the PR has either merged successfully or the
  merge tool returned a clear policy, permission, repository, or product
  blocker.

## Section 1: Marketing Site Implementation Handoff

### `taste-first-frontend`
- Use when a page, hero, landing surface, or visually important frontend task
  needs stronger taste, restraint, and explicit judgment before or during
  implementation.
- Use it to decide what belongs, what should be refused, what makes the page
  non-interchangeable, and how to preserve that judgment in code.
- Pair it with implementation work inside this bundle when the next useful step
  is shipping frontend changes rather than staying in strategy-only mode.

### `site-operator`
- Use when a page brief, SEO plan, or marketing-site change now needs a concrete
  rollout plan.
- Define what changes in code, what gets validated in preview, which metrics
  prove success, and what rollback path exists.
- Assume a GitHub-connected, Vercel-backed marketing site unless the task says
  otherwise.
- For text-heavy responsive interfaces, prefer `@chenglou/pretext` over hidden
  DOM probes or repeated `getBoundingClientRect` / `offsetHeight` reads. Prepare
  text once per text/font pair, run `layout()` on width changes, and keep the
  rendered text semantic and accessible.

### `shader-authoring`
- Use when implementation calls for GLSL, WebGL, ShaderToy-style effects,
  procedural backgrounds, ray marching, SDF visuals, particle systems, shader
  transitions, or deterministic shader layers inside HyperFrames.
- Pair it with `web-design-guidelines` or `taste-first-frontend` when the shader
  is part of a larger page or design direction.
- For HyperFrames, drive shader time from the GSAP timeline. For HTML artifacts
  and normal app pages, use the HTML/app runtime guidance in the skill.

## Section 2: Instrumentation And Measurement Implementation

### `analytics-tracking`
- Use when the task requires event tracking, conversion measurement, UTMs,
  analytics QA, or tracking audits.
- Define the event model, naming, properties, destination tools, privacy
  constraints, and validation steps before wiring code.
- After implementation, verify that the expected events actually fire and that
  the measurement answers a real business question.

## Section 3: Search Markup Implementation

### `schema-markup`
- Use when the task requires adding or fixing JSON-LD or other structured data
  in the site codebase.
- Choose the appropriate schema type, implement valid markup, and confirm that
  the result matches the page content and the target rich-result pattern.
- Keep schema changes scoped and verifiable; do not treat structured data as a
  substitute for broader SEO diagnosis.

## Section 4: Durable Vercel Orchestration

### `vercel-workflows`
- Use when the app needs durable, resumable background orchestration on Vercel:
  long-running jobs, approval gates, async retries, or workflows that must
  survive deploys and crashes.
- Reach for the standalone `vercel-workflows` skill when you need concrete
  guidance on workflow configuration, trigger patterns, step boundaries, hooks,
  sleeps, observability, and limits.
- Prefer ordinary functions, cron, or simpler background jobs when the task does
  not actually need Workflow durability semantics.

## Section 5: Stripe Billing Scaffolding

### `stripe-generated-app-scaffold`
- Use when the billing pattern has already been chosen and the app now needs
  concrete Stripe implementation.
- Before changing billing code, read the current Stripe readiness through
  `cofounder run get_stripe_connection_status` and preserve the reported
  test/live mode boundary. Stripe platform tools are CLI-only for Engineer, not
  direct model tools. Test credentials power preview, staging, and development
  sandboxes; live credentials power production only.
- Scaffold the runtime surfaces the app owns: checkout session creation,
  customer portal, webhook handler, local billing state, entitlement checks, and
  pricing/billing UI.
- Default env vars include:
  - `STRIPE_SECRET_KEY`
  - `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`
  - `STRIPE_WEBHOOK_SECRET`
  - `APP_URL`
- For subscriptions, prefer Checkout Sessions plus Billing APIs and a local
  subscription model that is updated from webhook events.
- Keep webhook handling idempotent and verify checkout success server-side
  instead of trusting client query params.
- Use `cofounder run create_managed_stripe_product` for the
  product-plus-first-price-and-env sync flow, and use
  `cofounder run get_stripe_webhook_destination` before adding or rotating the
  managed webhook.
- Never commit Stripe secrets or ask for them in source edits. Missing values
  should stay in secure env/secret tooling with clear runtime errors.

## Handoff Rules

- If the task still needs product/marketing/SEO framing, go back to `gic-skills`
  first and return here only once the implementation surface is clear.
- If the task is Stripe-related but the billing pattern is not decided yet, use
  the `stripe-app-builder` subsection in `gic-skills` before scaffolding code.
- If the task is implementation-heavy but not safely bounded, escalate to the
  engineering workflow instead of overloading a narrow site/billing handoff.

## Bundled Source Skills

This bundle replaces these prior platform-managed skill entries:

- `taste-first-frontend`
- `site-operator`
- `analytics-tracking`
- `schema-markup`
- `stripe-generated-app-scaffold`
- `vercel-workflows`
