## Stripe Connection

Use this reference when the task is about connecting Stripe to `superoptimizers`, verifying credentials, or deciding whether billing scaffolding can proceed.

### Control-Plane Goal

The control plane should know:
- whether Stripe is connected for this app
- whether the connection is valid in test mode, live mode, or both
- where the secret values live
- whether the generated app runtime has the env vars it needs
- whether the webhook setup is healthy enough for scaffolding

The control plane should not be the source of truth for customer subscriptions or entitlements.

### Preferred Product Flow

- Do not force Stripe during signup by default.
- Prompt for Stripe when the user asks for payments, pricing, subscriptions, checkout, or billing.
- Show a dedicated setup surface such as `StripeConnectionPanel` or `PaymentsSetupPanel`.
- Verify credentials before declaring the app billing-ready.
- Sync the required runtime env vars into the generated app only after verification passes.

### What To Store Where

Store secret values in the platform's secure secret system.

Store non-secret metadata locally:
- `app_id`
- `org_id`
- connection type such as `manual_key` or `oauth`
- mode such as `test`, `live`, or `both`
- `stripe_account_id` when available
- secret references or names
- webhook health
- last verification timestamp
- latest human-readable error

### Verification Checklist

Before the planner or Engineer scaffolds billing:
- verify the secret key can authenticate against Stripe
- verify the publishable key matches the intended mode
- verify the generated app has or can receive the required env vars
- verify the webhook endpoint contract is known
- mark the app `scaffold ready` only when the above checks pass

### Control-Plane Capability Summary

The control plane should be able to answer:
- can we scaffold one-time checkout?
- can we scaffold subscriptions?
- can we scaffold a billing portal?
- is the webhook setup missing or unhealthy?
- should the Stripe specialist planner refuse to proceed until setup is fixed?

### Avoid

- Do not rely on Vercel secret presence alone as the only queryable state.
- Do not inspect or print secret values during repository coding.
- Do not assume live mode just because test mode works.
