---
name: project-tracker
description: Structures and maintains project tracking for engagements. Refuses to set up tracking without milestone definitions with completion criteria, explicit dependency mapping, and slack analysis. Use when setting up a new engagement or auditing one that has drifted.
---

# Project Tracker

A project plan without milestone definitions is a schedule. A schedule without dependency mapping is a wish list. A wish list with names and dates is a blame document waiting to be written.

## Hard Refusals

Refuse to set up or review tracking without:

- **Milestone definitions with completion criteria**: what does it mean to complete each milestone? If the answer is "the work is done," the milestone is not defined. Each milestone needs a binary test: an observable state that is either true or false on the due date.
- **Dependencies**: which milestones block others? A milestone plan with no dependency map assumes everything is parallel. If that is not true, the plan is wrong from day one.
- **Slack analysis**: for each critical path milestone, how much buffer exists between its completion and the next dependent milestone? Zero-buffer milestones are delivery risks that should be named, not discovered.

## Milestone Definition Standard

Each milestone requires:
- Name and number
- Description of what changes in the world when it is complete
- Completion criteria: specific, binary, observable
- Owner: one named person (not a team)
- Due date
- Upstream dependencies: what must be complete before this starts
- Downstream dependencies: what does this unblock

"Phase 1 complete" is not a milestone. "Client has approved discovery synthesis document and verbal kickoff for Phase 2 has occurred" is a milestone.

## Dependency Mapping

Build a dependency table before the milestone plan is considered current:

| Milestone | Depends On | Blocks |
|-----------|-----------|--------|

Flag:
- Milestones with external dependencies (client approvals, third-party inputs) -- these have no internal buffer management
- Milestones with a single owner who also owns an upstream dependency (compresses available recovery window if that person is blocked)
- Long chains of sequential dependencies with no parallel paths (single-threaded delivery is fragile)

## Slack Analysis

For each critical path milestone:
- Slack = (due date - today) minus estimated remaining work in working days
- Milestones with slack below 20% of remaining duration are amber
- Milestones with negative slack are red

Apply Goldratt Theory of Constraints: identify the current constraint on the critical path -- the one milestone that, if delayed, delays the project completion date. All other work is subordinate to unblocking it.

## RACI for Key Decisions

For each decision point in the project:
| Decision | Responsible | Accountable | Consulted | Informed |
|----------|-------------|-------------|-----------|---------|

Accountable means one person. "The team" cannot be accountable. If the accountable field contains more than one name, there is no accountable party.

## Status Update Template

Weekly status updates require:

```
## [Project] Status -- Week of [date]

RAG: [Red / Amber / Green] -- [one-line reason with metric or milestone reference]

COMPLETED THIS WEEK
- [Milestone or deliverable, with completion evidence]

IN PROGRESS
- [Milestone], owner: [name], [X%] complete, on track for [date] / at risk because [reason]

BLOCKED
- [Item], blocked by: [named blocker], since: [date], unblock path: [specific action + owner + date]

NEXT WEEK
- [Specific deliverable, owner, due date]

RISKS
| Risk | Likelihood | Impact | Owner | Mitigation |
|------|-----------|--------|-------|-----------|
```

Do not use RAG Green to mean "nothing is on fire." Green means the project is on track against the agreed plan. Amber means at risk with active mitigation. Red means off track with no current recovery path.

## Change Request Process

Any change to scope, timeline, or budget after project kickoff requires:
- Description of the change
- Reason (client request / discovered dependency / scope error)
- Impact on timeline (working days)
- Impact on budget (hours or cost)
- Approval: named person with change authority

Do not absorb scope changes silently. Every unlogged change is a project that is running late for undocumented reasons.

## Frameworks Referenced

- Goldratt Theory of Constraints: the critical path constraint governs delivery. Surface it and subordinate everything else to resolving it.
- Linear project tracking discipline: status is one of four states -- not started, in progress (percent and owner), blocked (named blocker and path), complete.
- Felipe Castro OKRs: track project outcomes (what changed for the client) separately from project output (what we delivered). These are different reports for different audiences.
- Atlassian agile practices: sprint commitments are specific and bounded. Work that arrives after commitment belongs to a new sprint.

## Before / After

**1 -- Milestone without completion criteria**
Before: "Milestone 2: UX design complete."
After: "Milestone 2: UX design complete. Criteria: (1) all 12 screens in the scope have approved Figma designs, (2) client has provided written approval of the design direction, (3) design system component library is handed off to dev with documented component states. Owner: [designer]. Due: [date]. Depends on: Milestone 1 (discovery synthesis)."

**2 -- No dependency map**
Before: "Sprint 3 starts when Sprint 2 ends."
After: "Sprint 3 (dev build) depends on: (1) Milestone 2 completion (UX approval -- external dependency, client must act by [date]), (2) backend API spec signed off (internal, owner: [name], due: [date]). If Milestone 2 approval arrives late, Sprint 3 start slips by the same number of days with no recovery unless UX build is parallelized."

**3 -- RAG status without evidence**
Before: "Project status: Green. Things are progressing well."
After: "Project status: Amber. Three of 7 milestones are on track. Milestone 4 (data integration) is 4 days behind plan due to late API credentials from client (external dependency, unresolved since [date]). Downstream impact: Milestone 5 start delayed by minimum 4 working days. Recovery path: client escalation required by [date]. Owner: [PM name]."
