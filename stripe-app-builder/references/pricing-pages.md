## Pricing Pages

Use this reference for pricing page design, upgrade CTAs, free-trial flows, and entitlement-aware monetization UX in generated apps.

### Default SaaS Structure

Most generated SaaS apps should start with:
- a clear pricing page
- a free plan or free trial message when relevant
- a default paid tier such as `pro`
- a higher team or business tier only when the product actually needs it

### Recommended Components

- `PricingTable`
- `UpgradeButton`
- `CurrentPlanCard`
- `ManageBillingButton`
- paywall or upgrade modal for gated features

### Upgrade UX Rules

- Put upgrade CTAs near the blocked feature or usage limit.
- Explain the unlocked value, not just the price.
- Link back to the billing portal or current plan view after purchase.
- Use the app's local subscription and entitlement state to decide what to show.

### User-Facing Copy Rules

- When writing user-facing copy such as UI text, emails, error messages, or onboarding flows, do not include implementation details about how Stripe data is stored or synced.
- Bad example, do not write this: "Subscription status is server-authoritative. Meridian only grants Pro access from the local subscription table synced by Stripe webhooks."
- Good example, write this instead: "Your Pro subscription is active." Or omit the explanation entirely when the implementation detail is not useful to the user.
- The user does not need to know about webhook sync, local subscription tables, or server-authoritative state. Keep copy focused on what the user cares about: subscription status, billing, and access level.

### Trial Guidance

When the user asks for trial-to-paid:
- make the trial messaging explicit on the pricing page
- include post-trial behavior in the subscription state model
- surface upgrade prompts before the trial ends
- make billing portal access easy to find

### Avoid

- Do not hard-code price copy in many places with no shared plan config.
- Do not gate features off Stripe redirects alone without local entitlement checks.
