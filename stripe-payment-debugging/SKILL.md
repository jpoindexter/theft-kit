---
name: stripe-payment-debugging
description: "Use this skill when Engineer needs to investigate broken Stripe payments in a generated app: failed checkout, missing subscription access, webhook delivery failures, invoice payment failures, Vercel endpoint errors, missing Stripe env vars, wrong price IDs, or test/live mode mismatches. Start here before changing billing code."
metadata:
  version: 1.0.0
---

# Stripe Payment Debugging

Use this skill when a Stripe-powered payment flow is failing or the app state
does not match Stripe state. The goal is to identify the failing boundary before
writing code or asking the user to reconnect Stripe.

Stripe platform tools are CLI-only in Engineer. Use
`cofounder run <tool_name> --key value` from the terminal for Stripe
control-plane and connected-account operations; do not expect direct Stripe tool
calls in the model tool list.

## Default Investigation Order

1. Define the symptom and scope: checkout cannot start, payment fails, webhook
   fails, subscription/entitlement is stale, billing portal fails, or only one
   environment is broken.
2. Immediately start the external evidence checks, preferably in parallel
   and before repo code inspection:
   - Stripe webhook deliveries/events with
     `cofounder run stripe-list-events`, starting with
     `--delivery_success false` and the narrowest relevant time window
   - Vercel runtime logs/deployment events for the generated app's checkout,
     billing portal, and Stripe webhook routes
   - Vercel env-var metadata with `vercel-get-project-env-vars` for the
     deployment target that is actually serving traffic; check names, targets,
     and branch scope, never secret values
3. Run `cofounder run get_stripe_connection_status` with
   `--include_payments_implementation true`; use
   `--refresh_payments_implementation true` when code or env recently changed.
   Inspect `credential_slots` and `env.sync` before any env-var action. Test
   credentials power preview, staging, and development sandboxes; live
   credentials power production only.
4. Check Vercel deployments, endpoint logs, and env-var names for the generated
   app runtime.
5. Compare configured price IDs, webhook endpoint URL, event types, and
   test/live mode across Stripe, Vercel, and app code.
6. If code changes must start immediately, keep the Stripe delivery, Vercel
   runtime-log, and Vercel env-var checks running in parallel and feed those
   findings into the coding brief.
7. Only then propose a code fix, env sync, webhook rotation, data repair, or
   repo-agent delegation.

Only run `cofounder run sync_stripe_env` when the relevant slot's
`env.sync.synced` is false or `env.sync.missing_keys` is non-empty. Do not sync
again when the slot is already synced and complete. Do not issue refunds or
mutate Stripe while debugging unless the user explicitly asks for that action
and it is reviewable.

## Evidence To Gather

Capture enough evidence to map cause to boundary:

- failing user action, route, environment, and approximate timestamp
- Stripe event IDs, event types, object IDs, request IDs, and livemode
- Vercel project ID, deployment ID, target, branch, runtime log errors, and
  affected route
- relevant env-var names and targets, never secret values
- price IDs used by the UI/API and the product/price they map to in Stripe
- local app records for customer, subscription, entitlement, or processed
  webhook event when available

## Stripe Checks

Start with `cofounder run stripe-list-events`:

- default `delivery_success=false` to find failed webhook deliveries
- filter by types when useful:
  - `checkout.session.completed`
  - `payment_intent.payment_failed`
  - `payment_intent.succeeded`
  - `invoice.payment_failed`
  - `invoice.paid`
  - `customer.subscription.created`
  - `customer.subscription.updated`
  - `customer.subscription.deleted`
- use `created_after` and `created_before` around the user's timestamp
- inspect `pending_webhooks`, `request.id`, `idempotency_key`, `livemode`, and
  the nested object ID

Then use the narrow Stripe read tool that matches the object, through
`cofounder run`:

- `stripe-get-customer` for customer/account linkage
- `stripe-get-subscription` or `stripe-list-subscriptions` for entitlement
  state
- `stripe-list-invoices` for payment collection and dunning state
- `stripe-list-charges` for card/network failure evidence
- `stripe-list-products` to confirm active products and price metadata

Do not issue refunds or mutate Stripe while debugging unless the user explicitly
asks and the intended action is reviewable.

## Vercel Checks

Use Vercel to answer whether the runtime endpoint is broken:

- Check runtime logs and env-var metadata early, in parallel with Stripe
  webhook deliveries when possible, before assuming the repo code is the
  primary source of truth.
- `vercel-list-projects` if the project ID is unknown
- `vercel-list-deployments` for the target environment and branch
- `vercel-get-deployment-events` with `status_code="5xx"` for endpoint or
  build/runtime failures
- `vercel-get-runtime-logs` on the relevant deployment ID; look for
  `/api/billing/checkout`, `/api/billing/portal`, `/api/webhooks/stripe`, or the
  app's actual Stripe routes
- `vercel-get-project-env-vars` for production, preview, staging, and
  branch-scoped env-var names

Useful Vercel failure patterns:

- webhook endpoint returns 404, 405, 500, timeout, or redirects
- body parsing consumed the raw request before Stripe signature verification
- endpoint exists in preview but not production, or vice versa
- deployment target lacks `STRIPE_SECRET_KEY`, `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`,
  `STRIPE_WEBHOOK_SECRET`, `APP_URL`, or the expected price ID env vars
- preview uses production Stripe keys, or production uses test keys
- old deployment is still serving after env changes; redeploy is needed

## Env And Price ID Checks

Check names and mode alignment, not secret values.

Required or common env vars:

- `STRIPE_SECRET_KEY`
- `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `APP_URL`
- `STRIPE_PRICE_ID_BASIC`
- `STRIPE_PRICE_ID_PRO`
- `STRIPE_PRICE_ID_TEAM`
- generated managed IDs like `STRIPE_PRODUCT_ID_*` and
  `STRIPE_PRICE_ID_*_MONTHLY`, `*_YEARLY`, or `*_METERED`

Confirm:

- server code only reads secret keys on the server
- client code only reads `NEXT_PUBLIC_*`
- price IDs belong to the same Stripe mode as the keys
- UI-selected plan, API `price_id`, and Stripe product/price agree
- old price IDs were not left hard-coded in components, config, tests, or
  server routes
- Vercel env vars are present for the deployment target that is actually
  serving traffic

Only run `cofounder run sync_stripe_env` when
`cofounder run get_stripe_connection_status` shows `env.sync.synced` is false
or `env.sync.missing_keys` is non-empty.

## Webhook Failure Checklist

When Stripe says delivery failed or app state is stale:

- run `cofounder run get_stripe_webhook_destination` and compare it to the
  Stripe endpoint URL and Vercel deployment URL
- confirm the endpoint path matches the generated app implementation
- confirm required event types include the app's minimum set
- confirm `STRIPE_WEBHOOK_SECRET` belongs to that exact Stripe webhook endpoint
- confirm webhook signature verification uses the raw body
- check idempotency storage for duplicate or failed event processing
- check whether the handler returns 2xx only after durable processing or after
  safely queueing work
- replay the specific Stripe event only after the root cause is fixed

For subscriptions, the minimum useful event set is:

- `checkout.session.completed`
- `customer.subscription.created`
- `customer.subscription.updated`
- `customer.subscription.deleted`
- `invoice.paid`
- `invoice.payment_failed`

For one-time purchases, `checkout.session.completed` is usually the minimum.

## Common Root Causes

- missing or target-scoped Vercel env var
- test/live key mismatch
- stale Vercel deployment after env changes
- wrong price ID or inactive price
- Checkout success page trusts query params but the server never verified the
  session
- webhook points to preview while users pay on production, or production while
  testing preview
- webhook secret copied from a different endpoint
- endpoint method/path mismatch
- raw body unavailable for signature verification
- local subscription/entitlement row never updated after webhook success
- duplicate event handling fails because idempotency is missing or too broad
- app creates a new Stripe customer instead of reusing the stored customer
- billing portal uses the wrong customer ID

## Output Shape

Report:

- the most likely failing boundary
- evidence from Stripe, Vercel, env-var metadata, app code, or app data
- customer impact and whether money moved
- recommended next action: redeploy, sync env, rotate webhook secret, update
  endpoint, fix price mapping, repair local state, or use direct repository coding tools
- verification plan, including the specific Stripe event or checkout path to
  re-test

If code changes are needed, keep the implementation brief narrow and grounded
in the Stripe/Vercel evidence above. If speed requires starting code changes
sooner, explicitly run the Vercel runtime-log, Vercel env-var, and Stripe
webhook delivery checks in parallel and fold the findings into the fix as they
arrive. Include the failing route, expected env names, event IDs, webhook
endpoint, price ID mapping, and exact verification steps.
