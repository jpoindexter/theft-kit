---
name: architect-builder-method
description: Use when building software with AI by separating Architect (planning) from Builder (implementation), with files as source of truth instead of chat history.
---

# Architect Builder Method

Use this skill when the user is shipping software through AI tools and wants less "vibe coding" and more predictable delivery.

## Core model

- Architect layer: reasoning/planning model produces structure, requirements, and handoff.
- Builder layer: coding agent executes against that handoff.
- Source of truth: project files/folders, not chat scrollback.

## Workflow

1. Clarify business outcome and target user workflow.
2. Convert raw idea into builder-ready artifacts.
3. Hand builder a scoped sprint packet.
4. Build, verify, record decisions, and repeat.

## Non-negotiables

- Do not let builder infer missing business rules silently.
- Define what "done" means before implementation starts.
- Keep durable decisions in files, not memory or chat.

## Output contract

Produce:
- A concise objective
- Explicit constraints
- Acceptance criteria
- A builder handoff prompt

