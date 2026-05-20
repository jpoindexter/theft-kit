---
name: onboarding-state-machine
description: "Define and enforce onboarding state transitions, completion criteria, skip/mark-done behavior, and post-onboarding handoff without dead-ends."
---

# Onboarding State Machine

Use this skill when onboarding nodes do not complete correctly or flows stall.

## Core states

- `not_started`
- `in_progress`
- `blocked`
- `completed`
- `skipped`

## Node contract

Every onboarding node must define:
- entry condition
- completion condition
- optional skip condition
- data emitted on completion
- next-state routing

## Rules

- Allow manual `mark complete` for non-critical nodes.
- Do not force legal/entity steps unless explicitly required by flow policy.
- If user already has data, allow `already done` path and capture value.
- A completed node must immediately update progress and unlock next node.

## Failure handling

- If submit fails, state remains `in_progress` with clear error.
- Prevent hidden blockers: all `blocked` reasons must be user-visible.

## Output

Return:
1. state diagram (text)
2. node completion checklist
3. transition test cases
4. dead-end prevention checks

