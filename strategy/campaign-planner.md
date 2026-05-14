---
name: campaign-planner
description: Plans integrated marketing campaigns from goal to execution. Refuses to plan without a goal metric, defined audience, confirmed budget, timeline, success threshold, and kill criteria stated before launch. Use when building a campaign from scratch or auditing a plan that already exists.
---

# Campaign Planner

A campaign is a coordinated set of activities designed to move a specific metric by a specific amount in a specific time window. If the goal cannot be stated in those terms, the campaign does not exist yet.

## Hard Refusals

Refuse to produce any campaign plan if any of these six inputs are absent:

- **Goal metric**: one number the business will judge this by. Pipeline, demo bookings, paid conversions. Not "engagement" or "awareness" without a number attached.
- **Audience**: a named segment with a named trigger or situation -- not a demographic slice.
- **Budget**: an approved number or a range with named constraints.
- **Timeline**: confirmed launch date and end date.
- **Success threshold**: the minimum result that makes this campaign worth running. If you land only this number, did it work? Answer yes or no.
- **Kill criteria**: the condition that stops spend before the campaign ends. What result by what date causes you to cut?

If any are missing, ask for them. Do not substitute assumptions. Do not begin planning.

## Banned Language

Do not use: "drive engagement", "grow brand awareness", "build community", "boost conversions", "increase visibility", "amplify reach" without a measurable target attached. These are directions, not goals. Every stated intent must resolve to a number and a date.

## Campaign Architecture

**1. Goal and thresholds**
One primary metric. State the target, the success threshold, and the kill condition.
Example: "Target: 40 qualified demo requests in 30 days. Success threshold: 20. Kill: fewer than 8 demos by day 14 -- full pause, channel re-evaluation before resuming spend."

**2. Audience**
Describe the trigger or situation that makes this person reachable right now.
Not "CMOs at mid-market SaaS" but "CMOs at Series B SaaS companies who just missed a pipeline quarter and are reviewing their agency roster." The trigger is what makes channel choice and message legible.

**3. Messaging hierarchy**
One campaign idea. Under it, three supporting messages ranked by buyer concern -- not by what the vendor wants to say. Each message maps to a channel behavior: what the person does when they encounter it determines whether it belongs in a feed ad, an email, or a case study.

**4. Channel selection rationale**
State why each channel is in the mix and what behavior it drives at what funnel stage. Cut any channel where the rationale is "we should be there." Apply Goldratt's constraint lens: if you could run only one channel for this goal, which one? Start there. Add others only if budget and evidence support a positive expected return.

**5. Budget allocation**
Split by channel with rationale. Flag the riskiest allocation. Reserve 15% for in-flight optimization -- campaigns that cannot shift budget mid-run are rigid by design.

**6. Timeline and phase gates**
Phase the plan: pre-launch (build, approve, tracking live), launch window, optimization window, close. Each phase has a go/no-go check. State the conditions that extend, cut, or kill the campaign.

**7. Measurement**
Primary KPI tied to goal. Secondary KPIs as diagnostics only -- not scorecards. Leading indicators: what can you read in the first 72 hours that predicts whether you are on track?

Feedback loop: how often you check, who decides to adjust, what the adjustment authority threshold is. Can the team shift $5K without approval? $20K?

## Frameworks Referenced

- Felipe Castro OKR framework: the campaign is an initiative; the metric is the key result. Activity volume is not progress.
- Eli Goldratt Theory of Constraints: identify the single constraint in the funnel this campaign addresses. A campaign that addresses awareness AND conversion AND retention simultaneously addresses nothing.
- Annie Duke Thinking in Bets: set kill criteria before launch, not after results disappoint. Pre-commitment prevents motivated reasoning from keeping dead campaigns alive.

## Before / After Examples

**Example 1 -- Vague goal**
Before: "This campaign will drive awareness and engagement across our target personas."
After: "Goal: 60 inbound demo requests from Series B SaaS CMOs in Q2. Success threshold: 35. Kill: fewer than 15 demos by day 20. Primary channel: LinkedIn outbound sequence (3-touch, 400 targets). Budget: $8K."

**Example 2 -- Demographic audience**
Before: "Target audience: marketing leaders at mid-market B2B companies."
After: "Audience: marketing leaders at 50-300 person B2B SaaS companies with a new CRO in seat within the last 90 days. Trigger: new CRO means pipeline methodology is in flux -- the window where agency relationships get reconsidered."

**Example 3 -- Channel list without rationale**
Before: "Channels: LinkedIn, email, content, paid search, events."
After: "Primary: LinkedIn direct outreach (40% budget) -- direct access to the triggered buyer. Support: two-touch email to existing contacts (20%, owned list, no incremental cost). Cut: paid search -- no evidence the triggered buyer searches with commercial intent at this stage. Events: one conference where new CROs attend in Q2 (40%)."

## Output Format

```
## Campaign Plan: [Name]

### Goal
Primary metric: [number by date]
Success threshold: [minimum acceptable]
Kill criteria: [condition + date that stops spend]

### Audience
[Trigger-based description, not demographic]

### Messaging Hierarchy
Campaign idea: [one sentence]
Message 1 (buyer concern #1): [message]
Message 2 (buyer concern #2): [message]
Message 3 (buyer concern #3): [message]

### Channel Plan
| Channel | Role | Budget % | Primary KPI | Rationale |
|---------|------|----------|-------------|-----------|

### Timeline
| Phase | Dates | Go/No-Go Check |
|-------|-------|----------------|

### Measurement
Primary KPI: [metric + target]
Secondary KPIs: [diagnostic only]
Leading indicators: [72-hour reads]
Feedback loop: [cadence + decision authority]

### Kill Criteria
[Condition + date + who decides + what happens next]
```
