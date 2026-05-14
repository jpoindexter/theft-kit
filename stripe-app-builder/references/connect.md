## Stripe Connect

Use this reference only when the user explicitly needs:
- app fees
- marketplace or platform fund flows
- payout management
- connected accounts
- platform onboarding for many sellers

This is not the default path for a normal generated SaaS app that simply wants to accept payments with the user's own Stripe account.

Source:
- Stripe upstream `references/connect.md`

This file is deliberately a narrower wrapper around Stripe's upstream Connect
guidance because Connect is not the default path for `superoptimizers`.

## Table Of Contents

- Accounts v2 API
- Controller properties
- Charge types
- Integration guides
- Superoptimizers overlay

## Accounts V2 API

Reach for Connect when the product itself is becoming a payments platform.

Examples:
- multi-seller marketplace
- split payments
- platform-collected fees
- platform-managed seller onboarding
- platform-managed payouts

For new Connect platforms, use Accounts v2. Avoid legacy `type`-based account
creation for new integrations unless the user explicitly asks for that older
path.

## Controller Properties

When a real Connect platform is being built, describe accounts by explicit
responsibility settings, dashboard access, and capabilities rather than by old
"Standard / Express / Custom" labels.

## Charge Types

Choose one charge type per integration and avoid mixing them casually.

Stripe upstream guidance is:
- start with destination charges for many platforms
- use direct charges when the connected account should be the direct charge owner
- do not use the Charges API for Connect fund flows

## Integration Guides

Before coding a real Connect platform, review:
- Stripe SaaS platforms and marketplaces guide
- Stripe interactive platform guide
- Stripe Connect integration design guidance

## When Connect Is Not The Default

Do not switch to Connect just because the user said:
- "add Stripe"
- "I need subscriptions"
- "I want a pricing page"
- "customers should pay for my SaaS"

Those default to user-owned Stripe billing inside the generated app.

## Superoptimizers Overlay

If Connect is truly required:
- confirm that the user wants platform-style money movement
- scope the implementation separately from default SaaS billing
- document the tradeoff clearly before coding
