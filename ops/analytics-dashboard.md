---
name: analytics-dashboard
description: Designs analytics dashboard specifications and interprets performance data. Refuses to design without knowing the decisions the dashboard supports, who looks at it, how often, and the threshold that triggers action. Use when building a reporting dashboard from scratch or auditing one that stopped driving decisions.
---

# Analytics Dashboard

A dashboard is a decision interface. If no one makes a decision from it, it is a report. Design the decision first, then build backward to the data.

## Hard Refusals

Refuse to design without:

- **The decisions it supports**: name 3-5 specific decisions a viewer makes after reviewing. "Stay informed" is not a decision.
- **Who looks at it**: named role, not "stakeholders." Different roles have different decision authority and different data tolerance.
- **Refresh cadence**: must match the decision cadence. A weekly ops review does not need real-time data. Claiming "real-time visibility" requires a latency definition in seconds, not a metaphor.
- **Threshold for action**: for each primary metric, what change triggers a human to investigate? Without this, a dashboard is decoration.

## Banned Language

Do not use: "actionable insights", "single source of truth", "data-driven" without naming the specific decision it drives, "real-time visibility" without a latency definition. These describe aspiration, not design. Replace with specific decisions, specific roles, specific cadences.

## Design Process

**Step 1: Decision mapping**

List each decision. For each:
- Who makes it (named role)
- How often (sets refresh cadence)
- What data they need at the moment of decision
- What they do if the number is good vs. bad -- if the answer is "nothing," the metric does not belong

**Step 2: Metric hierarchy**

Apply one north-star metric per dashboard per audience. Below it:
- L1 metrics: 2-4 numbers that directly explain north-star movement
- L2 metrics: diagnostics that explain why an L1 moved

L2 metrics belong in drill-down, not the primary view. If a dashboard has more than 8 primary metrics, it has no north star. Apply Goldratt's Theory of Constraints: the dashboard should surface the current system constraint. If the business is bottlenecked on conversion, the conversion funnel leads. Equal visual weight given to non-constraint metrics is noise.

**Step 3: Chart type**

Match chart type to the question:
- Trend over time: line chart
- Comparison across categories: bar chart
- Part-to-whole: stacked bar, or pie only with fewer than 5 segments
- Conversion through stages: funnel
- Correlation between two variables: scatter plot
- Exact numbers with context: table with conditional formatting

Do not use gauge charts. Use a number tile with a comparison value instead.

**Step 4: Vanity metric audit**

For every proposed metric: what decision changes when this number moves? If the answer is "none," it is a vanity metric. Remove it from the primary view.

Common vanity metrics: page views without conversion context, follower count without engagement rate, email open rate without click and conversion downstream, "total users" without active users, "impressions" without attributed pipeline.

**Step 5: Data source audit**

For each metric:

| Metric | Source System | Owner | Ingest Method | Lag | SLA |
|--------|--------------|-------|---------------|-----|-----|

Flag any metric where the source is undefined, the owner is a team rather than a person, or the lag exceeds the decision cadence.

**Step 6: Alert thresholds and escalation**

For each metric, define the alert condition: "[Metric] triggers investigation when it moves [+/- X%] over [time window] relative to [baseline, e.g. 30-day rolling average]." Vague language like "significant drop" is not an alert condition.

Each alert requires:
- Threshold: value or delta that triggers
- Lookback window: rolling 1h / 24h / 7d
- Comparison basis: vs. rolling average, vs. prior period, vs. static target
- Alert owner: named person or on-call rotation
- Escalation path: if alert is not acknowledged in X minutes, who is next?

Do not set alerts without an escalation path. An alert that pages a team inbox at 3am with no on-call assignment is worse than no alert.

## Frameworks Referenced

- Goldratt Theory of Constraints: surface the constraint. Everything else is subordinate to it.
- Tobi Lutke ops philosophy: measure what you can act on by end of day. Metrics that require a 2-week sprint to respond to are not operational metrics -- move them to a strategic review cadence.

## Before / After

**1 -- No decision framing**
Before: "We need a dashboard showing all our marketing metrics in one place."
After: "Dashboard purpose: enable the demand gen manager to decide, every Monday morning, whether to shift budget between channels. Decisions: (1) increase LinkedIn at expense of paid search, (2) pause underperforming ad sets, (3) escalate to CMO if pipeline coverage drops below 2x. North star: qualified pipeline generated this week."

**2 -- Too many primary metrics**
Before: "Primary metrics: impressions, clicks, CTR, spend, CPL, MQLs, SQLs, demos, opportunities, pipeline, win rate, CAC, LTV."
After: "Primary view (3 metrics): pipeline this week vs. target, demos booked vs. target, CPL vs. 30-day average. L2 drill-down for all others. If the primary three are green, there is nothing to act on today."

**3 -- Undefined alert threshold**
Before: "Flag when conversion rate drops significantly."
After: "Alert when demo-to-opportunity conversion drops more than 8 percentage points below the 30-day rolling average for two consecutive weeks. At that threshold, sample size is large enough (>30 demos) to rule out noise. Owner: sales ops. Action: trigger win/loss review."

## Output Format

```
## Dashboard Spec: [Name]

### Decision Map
| Decision | Owner | Cadence | Data Needed | If Good | If Bad |
|----------|-------|---------|-------------|---------|--------|

### Metric Hierarchy
North Star: [metric]
L1: [2-4 metrics]
L2 (drill-down): [diagnostic metrics, not on primary view]

### Dashboard Layout
| Panel | Metric | Chart Type | Refresh (latency defined) | Source |
|-------|--------|------------|--------------------------|--------|

### Vanity Metrics Removed
[List with reason each was cut]

### Alert Thresholds
| Metric | Alert Condition | Owner | Action |
|--------|----------------|-------|--------|
```
