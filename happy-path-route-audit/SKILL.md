---
name: happy-path-route-audit
description: "Validate happy paths across app routes and core user flows, especially onboarding and post-onboarding transitions, with explicit pass/fail checks."
---

# Happy Path Route Audit

Use this skill when a product appears broken after onboarding or navigation changes.

## Scope

Audit these route classes:
- root/entry routes
- onboarding routes
- primary workspace routes
- department/feature routes
- settings and recovery routes

## Workflow

1. Enumerate routes under test.
2. Define one happy-path scenario per route.
3. Execute flows end-to-end in order.
4. Record pass/fail with exact failing step.
5. Open bug list grouped by blocker vs non-blocker.

## Required checks

- Route loads without runtime/hydration errors.
- Primary action is visible and clickable.
- Successful submit advances to next expected state.
- URL/state remain consistent after refresh.
- Flow can continue into at least one downstream route.

## Output

Return:
1. Route matrix with pass/fail
2. Blocking defects
3. Minimal reproduction steps
4. Fix priority order

