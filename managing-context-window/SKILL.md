---
name: managing-context-window
description: "Use when a conversation is getting long, drifting, juggling multiple tasks, re-pasting the same files, or showing stale/confused context — and at the start of any new task. Triggers: long thread, context bloat, topic switch, repeated file uploads, 'why is it forgetting', degraded answers, finished one task and starting another."
version: 1.0.0
author: Jason Poindexter
license: MIT
metadata:
  tags: [context, hygiene, sessions, tokens, productivity, jason]
  related_skills: [bare-minimum-communication]
---

# Managing the Context Window

## Overview

A conversation is **working memory, not a workspace**. The longer a thread runs, the more stale, bloated, and drift-prone it gets — and the more it costs per message. The single biggest lever is starting fresh threads aggressively. Persistent context belongs in files (CLAUDE.md, Projects), not in scrollback.

**Core rule:** When a thread has done its job, stop adding to it. New task → new thread.

## When to Use

This is **always-on**. Apply it without being asked. Trigger on:

- A thread that's run long or is drifting from its original task
- Switching to a new, unrelated task in an existing thread
- The same files/codebase being re-pasted across messages
- Answers degrading, "forgetting" earlier facts, or contradicting themselves
- Finishing one task and about to start another
- Jason starting a big "working session" thread instead of scoped tasks

Do NOT use it to refuse a genuinely continuous task that needs its full history.

## The Playbook (in order of impact)

1. **Start new conversations aggressively** — the main lever. Once a thread finishes its job, don't keep piling on. New thread = clean context. When a task completes or the topic shifts, say so: *"This is a new task — recommend `/clear` and a fresh thread."*

2. **Externalize persistent context to files** — CLAUDE.md (global + per-project) and Projects custom instructions carry context into every new thread without inheriting message history. If Jason keeps re-explaining the same thing, the fix is a file edit, not a longer thread. Offer to write it.

3. **Summarize before continuing** — if a long thread genuinely must continue, produce a compact state block (decisions, current state, next steps), then `/clear` and paste just that into a fresh thread. Strips the bloat, keeps the signal.

4. **`/compact` for long coding sessions** — in Claude Code, run `/compact` periodically to compress history in-place before context blows out. Suggest it proactively on long sessions.

5. **Scope conversations tightly** — one task per thread. "Fix this bug" → done → new thread for the next thing. Push back when a single thread is becoming a catch-all.

6. **Don't re-upload the same files** — if the same files appear every message, they're being re-tokenized each time. Move them into a Project (or a referenced path) so they load once.

## Quick Reference

| Symptom | Action |
|---|---|
| Task finished, new one starting | Recommend `/clear` + fresh thread |
| Repeated context re-explanation | Offer to write it to CLAUDE.md / Project |
| Must continue a long thread | Emit compact state block → `/clear` → paste |
| Long Claude Code coding session | Suggest `/compact` |
| One thread = many tasks | Flag it, propose splitting |
| Same files every message | Move to Project / reference path |

## Enforcement — no rationalizing

This skill is enforced **100%**. The default pull is to keep answering in the current thread because it's there. Resist it.

| Excuse | Reality |
|---|---|
| "The thread still works fine" | Cost and drift rise silently before they're visible. Flag early. |
| "Starting fresh is annoying" | A 10-second `/clear` beats a 50-message degraded thread. |
| "I'll mention it later" | Later = more bloat. Flag at the moment the task completes or shifts. |
| "It's basically the same topic" | 'Basically the same' is how working sessions sprawl. Scope tight. |
| "Re-pasting files is easier" | Re-tokenizing every message is the expensive part. Externalize once. |

**Letter vs spirit:** keeping a thread alive "just for this one more thing" is the exact drift this skill prevents. One more thing → new thread.

## Red Flags — STOP and act

- Just finished a task and the next message is a different task → recommend fresh thread
- About to re-explain context already given earlier → externalize to a file instead
- Same files pasted 2+ times → move to Project
- Noticeably worse / contradictory answers → context is stale, recommend `/clear` + summary handoff

All of these mean: **don't silently continue. Surface the hygiene action in one line, then proceed.**
