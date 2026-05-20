---
name: ops-to-builder-instructions
description: Translate operator or stakeholder knowledge into builder-ready instructions with workflows, business rules, edge cases, approvals, and success conditions.
---

# Ops to Builder Instructions

Use this skill when the user knows the business process but not implementation details.

## Translation frame

Capture:
- Inputs and sources
- Process steps and decision points
- Business rules and exceptions
- Approvals/roles
- Outputs/reports
- Failure modes and edge cases

## Deliverable format

Return a "builder-ready instruction pack" with:

1. Workflow map (step-by-step)
2. Rule table (if/then)
3. Edge-case list
4. Role/permission notes
5. Testable success criteria

## Quality bar

If a builder could still guess major rules, the pack is incomplete.

