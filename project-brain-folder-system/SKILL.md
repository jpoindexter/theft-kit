---
name: project-brain-folder-system
description: Create and maintain a file-based project brain for AI software delivery, including AGENTS router, docs, planning state, risks, questions, and sprint folders.
---

# Project Brain Folder System

Use this skill when setting up or repairing project structure so agents have clean context.

## Principle

Do not add more context; provide cleaner context in stable files.

## Required root files

- `AGENTS.md`: main router for all agents/builders.
- `README.md`: human overview and where to look next.
- Tool adapters (`CLAUDE.md`, `CODEX.md`) are thin, not the project brain.

## Required directories

- `docs/`: durable technical docs
  - `architecture.md`
  - `data-model.md`
  - `api-contracts.md`
  - `permissions.md`
  - `validation.md`
- `planning/`: current state and control docs
  - `state.md` (now/next/blocked)
  - `decisions.md`
  - `domain.md`
  - `risks.md`
  - `questions.md`
  - `file-inventory.md`
  - `meetings/`
  - `sprints/`

## Operating rule

Every significant decision or risk discovered during build must be written back to planning files in the same session.

