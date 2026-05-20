---
name: stripe-generated-app-scaffold
description: "Use when the task is to scaffold concrete Stripe billing code into a generated app. Trigger on requests to add a pricing page, checkout route, subscription system, billing portal, webhook handler, Stripe env vars, entitlement checks, or trial-to-paid flow. This skill assumes the user has connected their own Stripe to superoptimizers and the generated app will own runtime billing state."
metadata:
  version: 1.0.0
---

# Stripe Generated App Scaffold

This skill defines the default billing scaffold that Engineers should add to a generated SaaS app after Stripe is connected.

Use it together with `stripe-app-builder`:
- `stripe-app-builder` decides the right billing pattern
- `stripe-generated-app-scaffold` turns that decision into concrete app code

## Preconditions

Before scaffolding:
- confirm Stripe is connected or the user explicitly asked for placeholder setup
- run Stripe tools through the CLI with `cofounder run <tool_name> --key value`;
  do not expect direct Stripe tool calls in Engineer's model tool list
- run `cofounder run get_stripe_connection_status` and inspect
  `credential_slots` plus `env.sync` before any env, product, webhook, or
  billing action
- treat test credentials as preview, staging, and development-sandbox only, and
  live credentials as production only
- confirm which billing pattern is needed:
  - one-time checkout
  - subscriptions
  - billing portal
  - pricing page
  - trial-to-paid
- confirm the app has a user model that can own customers and entitlements

If Stripe is not connected, the scaffold can still lay down placeholders, but it must mark missing values clearly.

Only run `cofounder run sync_stripe_env` when the relevant slot's
`env.sync.synced` is false or `env.sync.missing_keys` is non-empty. Do not sync
again when the slot is already synced and complete.

## Runtime Ownership

The generated app owns:
- checkout session creation
- billing portal session creation
- Stripe webhook handling
- customer and subscription persistence
- entitlement checks and feature gates

The control plane does not own buyer billing state.

## Required Env Vars

Default env vars:
- `STRIPE_SECRET_KEY`
- `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `APP_URL`

Often needed for subscriptions:
- `STRIPE_PRICE_ID_BASIC`
- `STRIPE_PRICE_ID_PRO`
- `STRIPE_PRICE_ID_TEAM`

If values are not present, add clear placeholder comments and helpful runtime errors.

## Default Backend Surface

For one-time checkout:
- `POST /api/billing/checkout`
- `GET /api/billing/session-status`

For subscriptions:
- `POST /api/billing/checkout`
- `POST /api/billing/portal`
- `GET /api/billing/subscription`
- `POST /api/webhooks/stripe`

Keep the server responsible for:
- reading env vars
- creating Stripe sessions
- verifying webhook signatures
- verifying checkout success state

## Default Frontend Surface

For a recurring SaaS app, scaffold:
- `/pricing`
- `/settings/billing`
- `PricingTable`
- `UpgradeButton`
- `ManageBillingButton`
- `CurrentPlanCard`

Common optional surfaces:
- `/checkout/success`
- `/checkout/cancel`
- paywall or upgrade modal

## Default Data Model

Recommended tables:

### `billing_customers`
- app user id
- Stripe customer id
- email copy if useful
- created and updated timestamps

### `subscriptions`
- app user id
- Stripe customer id
- Stripe subscription id
- plan key
- normalized status
- `cancel_at_period_end`
- current period end timestamp
- created and updated timestamps

### `stripe_webhook_events`
- Stripe event id
- event type
- processing status
- payload JSON
- processed timestamp
- error message
- created timestamp

## Scaffold Rules

- Prefer Checkout Sessions for initial payment or subscription signup.
- Run `cofounder run create_managed_stripe_product` for the
  product-plus-first-price flow when the resulting product and price IDs need
  to be synced into managed app env vars.
- Run `cofounder run get_stripe_webhook_destination` before
  `cofounder run add_stripe_webhook`; never invent the webhook URL.
- Keep Stripe secrets out of source files, examples, and prompts. Use secure env
  or secret tooling, and commit only names/placeholders plus clear runtime
  errors.
- Prefer Billing Portal for self-serve billing management.
- Use local subscription rows for app feature access.
- Gate premium features off the app's local entitlement state, updated from Stripe events.
- Verify checkout success server-side instead of trusting query params alone.
- Make the webhook handler idempotent.

## Minimum Webhook Event Set

For subscription scaffolds:
- `checkout.session.completed`
- `customer.subscription.created`
- `customer.subscription.updated`
- `customer.subscription.deleted`
- `invoice.paid`
- `invoice.payment_failed`

For one-time checkout:
- `checkout.session.completed`

## Pattern Selection

### Simple subscription SaaS
Add:
- pricing page
- subscription checkout route
- webhook handler
- billing portal route
- entitlement helper

### Free trial to paid
Add:
- trial messaging on pricing page
- local subscription state that handles trial status cleanly
- upgrade CTA where value is experienced
- billing portal affordance

### One-time purchase
Add:
- purchase button
- one-time checkout route
- success page
- local order or fulfillment hook if the app needs it

## Traps To Avoid

- Do not put Stripe secret values in the repo.
- Do not hard-code raw price IDs throughout the UI.
- Do not use webhook absence as an excuse to skip local billing state.
- Do not build recurring billing on top of manual PaymentIntent retries.
- Do not make the control plane the source of truth for generated-app subscriptions.
