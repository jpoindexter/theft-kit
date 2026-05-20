---
name: local-release-gate
description: "Run a local pre-push release gate: build, route smoke tests, happy-path flows, and regression checks before committing or pushing."
---

# Local Release Gate

Use this skill before push/merge on active product branches.

## Gate stages

1. Static checks (type/lint where configured)
2. Build verification
3. Route smoke checks
4. Core happy-path flow checks
5. Known-regression checks

## Mandatory outputs

- Build result: pass/fail
- Route smoke matrix: pass/fail
- Happy-path outcomes
- Blocking issues and reproduction steps

## Fail policy

If any blocker fails, do not mark release-ready.

## Suggested route set

- `/`
- `/:org`
- `/:org/onboarding`
- `/:org/tasks`
- `/:org/agents`
- `/:org/settings`

