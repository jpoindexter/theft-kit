## Stripe Payments

Use this reference for one-time purchases, simple checkout flows, and payment blocks inside generated apps.

Source:
- Stripe upstream `references/payments.md`

This file is intentionally close to Stripe's upstream advice, with only a small
generated-app overlay added.

## Table Of Contents

- API hierarchy
- Integration surfaces
- Payment Element guidance
- Saving payment methods
- Dynamic payment methods
- Deprecated APIs and migration paths
- Generated-app overlay

## API Hierarchy

Prefer:
- Checkout Sessions for on-session payments
- PaymentIntents for off-session payments or when the merchant truly needs to
  model checkout state independently
- SetupIntents for saving payment methods
- higher-level Stripe surfaces over lower-level raw primitives whenever possible

Avoid:
- Charges API
- Sources API
- Card Element for new work

Integrations should only use Checkout Sessions, PaymentIntents, SetupIntents,
or higher-level billing surfaces unless there is a very specific migration need.

## Integration Surfaces

Prioritize Stripe-hosted or embedded Checkout where possible. Use this order of
preference:

1. Payment Links for simple no-code products
2. Checkout for most web apps
3. Payment Element for advanced embedded customization

When using Payment Element, prefer backing it with Checkout Sessions over a raw
PaymentIntent when that surface can satisfy the task.

## Payment Element Guidance

If the task needs inspection of card details before payment or other advanced
server-finalized payment flows, prefer Confirmation Tokens over older
Stripe.js tokenization helpers.

Do not recommend `createPaymentMethod` or `createToken` as the default modern
path if the task is really a Payment Element or Checkout design question.

## Saving Payment Methods

Use Setup Intents to save a payment method for later use.

Do not use the deprecated Sources API to save cards to customers.

## Dynamic Payment Methods

Prefer dynamic payment methods configured in Stripe Dashboard over hard-coding
specific `payment_method_types` everywhere.

## Deprecated APIs And Migration Paths

Never recommend the Charges API for new work.

| API | Status | Use instead |
|---|---|---|
| Charges API | Never use for new work | Checkout Sessions or PaymentIntents |
| Sources API | Deprecated | Setup Intents |
| Tokens API | Outdated | Setup Intents or Checkout Sessions |
| Card Element | Legacy | Payment Element |

## Generated-App Overlay

After following Stripe's upstream choice of API, apply these generated-app
rules.

The generated app should usually own:
- a pricing or purchase surface
- a server route that creates the Checkout Session
- a success page that verifies the session
- local order or fulfillment hooks if the product needs them

Typical runtime env vars:
- `STRIPE_SECRET_KEY`
- `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`
- `APP_URL`

Typical server routes:
- `POST /api/billing/checkout`
- `GET /api/billing/session-status`

### Block-Level Recommendations

Use small builder primitives:
- `BuyButton`
- `PricingTable`
- `OrderSuccess`

Keep block inputs simple:
- plan key
- price lookup key
- success path
- cancel path

Resolve Stripe IDs on the server whenever possible instead of pushing raw Stripe object IDs into the UI layer.

### Success-Page Rule

Do not trust frontend state alone after redirect.

On the success page:
- read the returned session identifier
- verify the session with Stripe on the server
- render the final success or failure state from verified data

### Avoid

- Do not create raw PaymentIntents for a basic SaaS checkout flow if Checkout Sessions would solve it more safely.
- Do not couple purchase success directly to client-only state without server verification.
