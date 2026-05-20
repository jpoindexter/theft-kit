---
name: clean-context-engineering
description: "Reduce AI build errors by enforcing clean context discipline: curated inputs, minimal file payloads, and explicit update loops."
---

# Clean Context Engineering

Use this skill when model outputs drift, loop, or hallucinate due to noisy context.

## Rules

- Prefer minimal, relevant file excerpts over dumping large folders.
- Never rely on chat-only memory for project-critical facts.
- Keep one canonical location for each class of information.

## Context loop

1. Identify exact task.
2. Load only required files.
3. Execute.
4. Write decisions/state changes back to project brain.
5. Start next task from updated files.

## Anti-patterns

- "Build me an app" with no constraints.
- Re-sending stale context repeatedly.
- Letting acceptance criteria remain implicit.

