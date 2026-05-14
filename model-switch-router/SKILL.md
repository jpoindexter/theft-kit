---
name: model-switch-router
description: "Decide when to switch AI models by task type, risk, loop length, and cost-quality tradeoff. Use for routing architecture, implementation, debugging, refactors, cleanup, and batch agent workflows."
---

# Model Switch Router

Use this skill when selecting or switching models during software delivery.

## Goal

Pick the cheapest model that reliably meets quality for the current task.

## Fast Decision Tree

1. Is failure expensive (security, architecture, concurrency, irreversible migrations)?
- Use premium reasoning model.

2. Is this normal implementation/debugging/refactor work?
- Use workhorse model.

3. Is this trivial cleanup (lint, format, typo, rename, boilerplate)?
- Use utility/local model.

4. Is this a long multi-step loop?
- Keep planning on premium, run loop execution on workhorse.

## Routing Matrix

- Architecture / system design / critical reviews:
  - Primary: premium model
  - Fallback: secondary premium model

- Feature implementation / code review / bug fixing:
  - Primary: workhorse model
  - Escalate to premium if two retries fail or ambiguity remains

- Long agentic loops (10+ iterations):
  - Plan once on premium
  - Execute iterations on workhorse
  - Return to premium only for final risk review

- Cleanup tasks (lint/format/rename/low-risk edits):
  - Primary: utility model

- Boilerplate/autocomplete/stubs:
  - Primary: local model

## Escalation Rules

Escalate to a stronger model when any condition is true:
- Two failed attempts on same task
- Contradictory outputs between runs
- Security-sensitive or compliance-sensitive code path
- Cross-module changes with unclear side effects
- Unknown production incident with user impact

## Downgrade Rules

Downgrade to cheaper model when all are true:
- Task is deterministic and low-risk
- Expected output is short and mechanical
- Existing tests constrain behavior
- No architecture decision required

## Session Policy

- Re-evaluate model choice at each phase boundary:
  - discovery
  - planning
  - implementation
  - verification
- Log model choice and reason in the task note.
- Prefer batching small asks to reduce repeated prefix/context cost.
- Keep stable context cached; avoid re-sending large unchanged files.

## Output Format

When invoked, return:
1. Recommended model tier now
2. Why this tier (risk + complexity + cost)
3. Escalation trigger for next switch
4. Downgrade trigger after stabilization
