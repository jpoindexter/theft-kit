## Stripe Webhooks

Use this reference for webhook event selection, idempotency, reconciliation, and sync architecture.

### Source Of Truth Split

Use Stripe as the source of truth for payment and subscription events.

Use the generated app database as the source of truth for:
- local entitlements
- feature access
- app-specific subscription summaries
- webhook processing status

### Minimum Subscription Event Set

For a recurring SaaS scaffold, subscribe to:
- `checkout.session.completed`
- `customer.subscription.created`
- `customer.subscription.updated`
- `customer.subscription.deleted`
- `invoice.paid`
- `invoice.payment_failed`

Optional but useful:
- `payment_method.attached`
- `payment_method.detached`
- `customer.updated`
- `billing_portal.session.created`

### Processing Rules

- Verify the webhook signature before any business logic.
- Record the Stripe event ID before processing domain writes.
- Skip duplicate event IDs safely.
- Update the app's local subscription or order state from the event payload.
- Return a retryable failure on transient write errors if the scaffold expects Stripe retries.

### Reconciliation Rule

Do not rely on webhooks alone for revenue-critical correctness.

Recommend:
- a manual resync path
- a periodic reconciliation job for recent or active subscriptions

### Avoid

- Do not let duplicate events create duplicate access grants.
- Do not treat client redirects as a substitute for webhook-driven state updates.
- Do not store webhook secrets in source files.
