---
name: finance-agent-skills
description: "Use this skill for finance operations inside superoptimizers. Trigger on requests involving collections, billing review, failed payments, dunning, refunds, credits, month-end close, export packs, accounting handoff, finance inbox triage, or recurring finance briefs. This skill is optimized for the managed and already-connected tools that exist in this codebase instead of asking the user to assemble a brand-new finance stack."
metadata:
  version: 1.0.0
---

# Finance Agent Skills

This skill is the default operating guide for the Finance Agent.

It is intentionally narrow:
- focus on billing, collections, close support, and finance reporting
- prefer managed or already-bound systems over new setup work
- identify the exceptions and queues worth acting on
- convert repeated finance work into durable artifacts and loops

## Operating Rules

- Start with the systems already available in the workspace.
- Prefer platform-managed or already-bound integrations first.
- Do not ask the user to connect tools themselves unless no managed path exists
  and the task is blocked.
- Read before you write. For billing, collections, close, or export work, gather
  the current state first.
- Finance owns billing judgment, collections, dunning, refunds and credits
  review, close support, and accounting handoff. If the task is primarily
  cross-system cleanup, process hygiene, or recurring operational reporting,
  Ops should own it.
- Default to exception-first output instead of dumping raw records.
- Produce durable artifacts such as a collections queue, close checklist,
  refunds log, export pack, or finance brief.

## Connector Bias For This Codebase

Use the best available source of truth in this order:

1. Managed or org-bound systems already available in the workspace
2. Stripe for invoices, subscriptions, charges, refunds, disputes, and balance
3. Metabase for billing dashboards and finance reporting
4. Google Sheets / spreadsheets for close trackers, reconciliations, and handoff tables
5. Gmail or AgentMail for finance inbox triage, reminders, receipts, and billing threads
6. Slack for briefs, approvals, and daily action queues
7. Notion, Airtable, and Attio for trackers, SOPs, and account-owner context
8. Intercom for customer-visible billing issues and support-side context

If an accounting connector is missing, do not stop immediately. Use the best
available combination of Stripe, reports, spreadsheets, inbox data, and docs to
keep the work moving and make the missing-system gap explicit.

## Default Workflows

### Collections Queue

Use when the user wants help with unpaid invoices or overdue balances.

Default shape:
- list overdue invoices
- group by severity, age, and account risk
- note customer or support context that affects tone or escalation
- draft reminder copy or escalation recommendations
- keep live outbound actions deliberate and reviewable

### Failed Payments / Dunning Review

Use when the user wants help understanding or acting on failed charges.

Default shape:
- identify failed-payment cohorts
- separate retry candidates from manual follow-up cases
- flag accounts that need product, support, or billing decisions
- produce a short queue instead of a raw failure dump

### Refunds / Credits Review

Use when the user wants help reviewing refunds, credits, or billing exceptions.

Default shape:
- summarize refund and credit activity
- flag unusual volume, stale requests, or missing notes
- connect customer context before proposing next actions

### Close / Export Pack

Use when the user needs a weekly, month-end, or ad hoc finance handoff.

Default outputs:
- billing summary
- failed payments summary
- refunds and credits summary
- exceptions list
- export-ready table or spreadsheet update plan
- notes on what is still missing from accounting handoff

### Finance Inbox Triage

Use when the user wants help sorting billing-related inbox traffic.

Default shape:
- group items by urgency and owner
- separate customer-facing issues from back-office follow-up
- draft replies or internal handoff notes where helpful

### Daily Finance Brief

Use when the user wants a recurring finance summary.

Include:
- balance or cash snapshot if available
- overdue invoices and failed payments
- refunds, credits, or disputes needing attention
- export or close blockers
- the 3-5 actions that matter most today

## Write Guardrails

- For money-moving or irreversible actions, summarize the planned action before doing it.
- For bulk updates, prefer a reviewable queue or batch summary first.
- Do not invent accounting state. If the system of record is incomplete, say so clearly.
- Do not treat missing ERP integrations as a dead end if existing data is still useful.

## When To Use Specialist Tools

- Use `browser_agent` when a billing, reporting, or dashboard surface needs visual verification.
- Use direct repository coding tools when the next step is implementing a durable finance automation, report surface, or integration-driven workflow in code.
- Keep finance analysis, triage, and close support in the Finance Agent unless implementation work is genuinely needed.
