---
name: stripe-app-builder
description: "Use when the user wants to add, review, debug, or plan Stripe-powered revenue flows in a generated app. Trigger on requests involving Stripe, checkout, subscriptions, pricing pages, billing portal, webhooks, entitlements, free trials, paywalls, upgrade flows, or connecting Stripe to superoptimizers. This skill is for superoptimizers as the control plane that scaffolds billing into the user's app. It is not the default path for Connect marketplace or payout-management work."
metadata:
  version: 1.0.0
---

# Stripe App Builder

Source:
- Stripe upstream skill: `https://github.com/stripe/ai/blob/main/skills/stripe-best-practices/SKILL.md`

Last reviewed:
- 2026-03-19

This skill is the router for Stripe work in `superoptimizers`.
It intentionally follows Stripe's upstream routing model first, then adds a
`superoptimizers` overlay for how that guidance should be applied in generated
apps.

Default mental model:
- `superoptimizers` is the control plane.
- The generated app is the billing runtime.
- Users usually bring their own Stripe account.
- Do not default to Stripe Connect marketplace flows.

## Read This Before Coding

Stripe upstream guidance says to route integration work by domain first. Do that
here too: pick the narrowest domain below, then read the matching reference
before you answer questions or write code.

| Building... | Recommended API or surface | Read first |
|---|---|---|
| Connect Stripe to the builder or verify setup | Control-plane connection flow | [references/connection.md](references/connection.md) |
| One-time payments | Checkout Sessions | [references/payments.md](references/payments.md) |
| Custom payment form with embedded UI | Checkout Sessions + Payment Element | [references/payments.md](references/payments.md) |
| Saving a payment method for later | Setup Intents | [references/payments.md](references/payments.md) |
| Subscriptions or recurring billing | Billing APIs + Checkout Sessions | [references/billing.md](references/billing.md) |
| Webhook design, idempotency, reconciliation | Generated-app webhook layer | [references/webhooks.md](references/webhooks.md) |
| Pricing page, upgrade UX, free trial messaging | Generated-app pricing and entitlements | [references/pricing-pages.md](references/pricing-pages.md) |
| Connect platform or marketplace | Accounts v2 and Connect guidance | [references/connect.md](references/connect.md) |

If the task is implementation-heavy, also use the `stripe-generated-app-scaffold` skill before coding.

## Upstream Stripe Defaults

These come directly from the shape of Stripe's upstream skill and should be the
default stance unless the task clearly needs something else.

- Prefer higher-level Stripe APIs first.
- Use Checkout Sessions for most on-session payments.
- Use Billing APIs plus Checkout Sessions for subscriptions.
- Use Setup Intents to save a payment method for later.
- For new Connect platforms, use Accounts v2.
- Avoid deprecated payment surfaces unless there is a very specific migration need.

## Superoptimizers Overlay

After applying Stripe's upstream guidance, apply these product-specific rules.

- Stripe platform tools are CLI-only in Engineer. Use
  `cofounder run <tool_name> --key value` from the terminal for Stripe
  control-plane and connected-account operations; do not expect direct Stripe
  tool calls in the model tool list.
- Default to user-owned Stripe integrations for SaaS apps.
- Do not make Stripe a signup blocker unless the user explicitly wants it in onboarding.
- Keep control-plane state and generated-app billing state separate.
- Keep actual secrets in the platform's secret store. Keep only non-secret linkage and health metadata in the control plane.
- Start every Stripe setup or billing action by running
  `cofounder run get_stripe_connection_status` and inspecting
  `credential_slots` plus `env.sync`.
- Test credentials power preview, staging, and development sandboxes. Live
  credentials power production only.
- If live credentials are missing and the user wants production readiness, ask
  whether they want to connect production keys before repo research, coding
  delegation, production-impacting env, webhook, product, or billing work. When
  they agree, run `cofounder run connect_stripe --mode live`; if they decline,
  keep the work explicitly test/non-production scoped.
- Only run `cofounder run sync_stripe_env` when the relevant slot's
  `env.sync.synced` is false or `env.sync.missing_keys` is non-empty. Do not
  sync again when the slot is already synced and has no missing keys.
- Run `cofounder run create_managed_stripe_product` when the task is to create
  a Stripe product plus first price and sync the resulting IDs into the managed
  app env.
- Run `cofounder run get_stripe_webhook_destination` before
  `cofounder run add_stripe_webhook`; never guess the managed app webhook URL.
- Prefer Checkout Sessions first. Use Embedded Checkout second. Use Payment Element only when the user actually needs a custom embedded payment form.
- Prefer Billing APIs plus Customer Portal for subscriptions.
- Do not recommend Charges API, Sources API, Tokens API, legacy Card Element, or deprecated plan-based subscription surfaces.
- Do not put Stripe secrets directly in source files, examples, or prompts.
- Do not recommend Connect unless the user explicitly needs app fees, payouts, multi-seller platform flows, or connected-account operations.

## Responsibility Split

Control plane owns:
- prompting for Stripe connection when payments are relevant
- storing secret references and connection metadata
- verifying test/live setup
- deciding whether billing scaffolding can proceed
- activating Stripe skills and specialist planning
- syncing managed Stripe env vars only when status shows sync is incomplete
- creating managed products/prices through reviewable approval artifacts

Generated app owns:
- checkout and billing portal routes
- pricing page and upgrade UX
- webhook endpoint and idempotency
- customer and subscription persistence
- entitlement and feature-gating logic

## Traps To Avoid

- Do not scaffold buyer billing state into `superoptimizers`.
- Do not assume Connect just because the user said "Stripe."
- Do not build recurring billing with manual PaymentIntent renewal loops.
- Do not recommend raw Charges API for new work.
- Do not recommend Sources API for saving cards or future payment methods.
- Do not recommend Card Element as the default embedded UI path.
- Do not ask the user to paste raw secrets into repo files.
- Do not treat webhook delivery as optional for subscriptions and entitlements.
- Do not skip test-mode validation before suggesting a live cutover.
- Do not use raw Stripe product/price creation when the managed app needs the
  resulting IDs synced into env vars.
