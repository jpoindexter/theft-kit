## Stripe Billing

Use this reference for subscriptions, free trials, upgrades, downgrades, billing portal, and recurring SaaS billing flows.

Source:
- Stripe upstream `references/billing.md`

This file stays intentionally close to Stripe's upstream billing guidance and
adds a generated-app overlay for `superoptimizers`.

## Table Of Contents

- When to use Billing APIs
- Recommended frontend pairing
- Traps to avoid
- Generated-app overlay

## When To Use Billing APIs

Prefer:
- Billing APIs for product and subscription modeling
- Checkout Sessions in `subscription` mode for initial signup
- Billing Portal for self-serve billing management

Do not build subscription systems around manual recurring PaymentIntents.

If the user has recurring revenue, usage-based billing, or seat-based pricing,
plan the integration with Stripe Billing first rather than starting from raw
payment primitives.

Review Stripe's subscription use cases and SaaS integration guidance when the
pricing model is ambiguous.

## Recommended Frontend Pairing

Use Stripe Checkout for the initial payment frontend where possible.

Checkout Sessions in `subscription` mode should be the default starting point
for:
- new paid signup
- free trial to paid flows
- standard SaaS pricing pages

Use Customer Portal for:
- upgrades
- downgrades
- cancellation
- payment method updates
- self-serve billing management

## Traps To Avoid

- Do not build manual subscription renewal loops using raw PaymentIntents.
- Do not use the deprecated `plan` object for new work. Use Prices instead.

## Generated-App Overlay

After applying Stripe's billing defaults, scaffold the runtime shape into the
generated app.

The generated app should own:
- the pricing page
- the subscription checkout route
- the billing portal route
- local subscription persistence
- entitlement and feature-gating rules

Typical routes:
- `POST /api/billing/checkout`
- `POST /api/billing/portal`
- `GET /api/billing/subscription`
- `POST /api/webhooks/stripe`

Typical tables:
- `billing_customers`
- `subscriptions`
- `stripe_webhook_events`

### Plan Modeling Rules

- Use stable internal plan keys such as `free`, `pro`, and `team`.
- Map plan keys to Stripe Price IDs via config, not hard-coded JSX strings scattered through the app.
- Keep access control keyed off the app's local subscription record, updated from Stripe events.

### Billing Portal

For a default SaaS flow, include a billing portal route and UI affordance by default.

Use the billing portal for:
- payment method updates
- plan changes when appropriate
- cancellation or reactivation
- invoice history

### Avoid

- Do not make the control plane the source of truth for buyer subscription state.
- Do not use deprecated plan-based subscription surfaces for new scaffolds.
- Do not bury billing portal access in obscure settings when the user asked for self-serve billing.
