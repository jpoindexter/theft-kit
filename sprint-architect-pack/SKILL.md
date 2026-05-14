---
name: sprint-architect-pack
description: "Generate a four-document architect pack per sprint: requirements, blueprint, acceptance criteria, and builder handoff prompt."
---

# Sprint Architect Pack

Use this skill to prepare one sprint before any code changes start.

## Inputs

- Current project `state.md`
- Relevant docs in `docs/`
- Open risks/questions

## Produce exactly 4 artifacts per sprint

1. `requirements.md`: what we are building and why.
2. `blueprint.md`: how we will build it (components, flows, data, interfaces).
3. `acceptance-criteria.md`: objective pass/fail checks for done.
4. `handoff-prompt.md`: direct prompt for builder with scope, constraints, and verification steps.

## Rules

- Keep sprint scope finite.
- Tie each requirement to at least one acceptance criterion.
- Include explicit out-of-scope notes.
- Include rollback notes for risky changes.

## Builder kickoff template

"Implement only what is specified in this sprint pack. If required information is missing, stop and request clarification before coding."

