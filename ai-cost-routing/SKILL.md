---
name: ai-cost-routing
description: "Use when the user wants to reduce AI coding spend without reducing shipping speed. Covers model routing, prompt caching, context discipline, loop profiling, and rollout plans for Cursor, Claude Code, Aider, and similar coding-agent stacks."
metadata:
  version: 1.0.0
---

# AI Cost Routing

This skill turns expensive coding-agent usage into a controlled routing system:
- premium models only for high-consequence decisions
- low-cost workhorse for most implementation loops
- utility/local models for trivial edits and boilerplate
- strict context discipline so you stop paying to resend unchanged files

## Trigger

Use this skill when the user asks to:
- cut AI token/model costs
- set up model routing
- profile coding-agent token leaks
- optimize Cursor / Claude Code / Aider economics
- keep shipping speed while lowering monthly spend

## Core Outcomes

Produce these outputs in order:

1. **Current leak diagnosis**  
   Quantify where cost is being wasted right now (context bloat, loop retries, premium misuse).

2. **Router policy**  
   A concrete model routing table by task type.

3. **Execution settings**  
   Prompt caching, context limits, summarization cadence, and grep-first rules.

4. **30-day rollout plan**  
   Week-by-week adoption with measurable checkpoints.

5. **Guardrails**  
   Explicit rules for when premium models are mandatory.

## Routing Baseline

Use this as default policy unless the user overrides:

- **Planning / architecture / security-critical review**: premium tier (`claude-opus-*` or `gpt-5*`)
- **Implementation / debugging / refactor / code review**: workhorse tier (`kimi-*` or equivalent low-cost high-quality model)
- **Lint / format / rename / trivial edits**: utility tier (`haiku` / mini-tier)
- **Boilerplate / autocomplete / stubs**: local model tier (`ollama:qwen*` / local)

## 5 Leak Checks (Mandatory)

For every audit, check and report:

1. **Repo re-send leak**  
   Repeated large file prefixes in consecutive turns.
2. **Tool-loop spiral leak**  
   Multiple tool calls rehydrating the same context each iteration.
3. **Premium misuse leak**  
   Premium model used for trivial operations.
4. **Session bloat leak**  
   Long chats with no periodic summarization reset.
5. **Dead-action leak**  
   Agent runs that end in retries/stalls because of non-functional actions/CTAs.

## Execution Rules

- Use `rg`/`rg --files` before including files in prompts.
- Prefer targeted snippets over full-file context unless required.
- Batch related tasks/questions when context prefix is stable.
- Summarize and reset context every 10-15 turns in long sessions.
- Cache stable system/context prefixes whenever tooling supports it.
- For repeated workflows, encode process into reusable skill docs (`SKILL.md`) to reduce rediscovery tokens.

## Deliverable Template

Return:

1. **Spend diagnosis** (top 3-5 leaks by impact)
2. **Router config** (copy-paste YAML/JSON)
3. **Tool settings changes** (exact toggles/flags)
4. **Weekly rollout plan** (Week 1-4)
5. **Expected savings range** and risks

## Copy-Paste Router Starter

```yaml
default: kimi-2.6-instruct

routes:
  planning:
    model: claude-opus-4-6
    fallback: gpt-5
    triggers: ["plan", "architect", "security review", "system design"]

  implementation:
    model: kimi-2.6-instruct
    triggers: ["implement", "debug", "refactor", "review", "build feature"]

  cleanup:
    model: claude-haiku-4-5
    triggers: ["lint", "format", "fix typo", "rename variable"]

  boilerplate:
    model: ollama:qwen3:7b
    triggers: ["autocomplete", "stub", "generate boilerplate"]

caching:
  enabled: true
  prefix_cache: true

context:
  max_tokens: 50000
  auto_summarize_after: 15
  use_grep_first: true
```

## Guardrail: When Not To Optimize Cost

Force premium tier when error cost is high:
- security-sensitive logic
- concurrency/race-condition debugging
- architectural choices with cross-module blast radius
- irreversible migration/infra decisions

If failure cost is significantly higher than model delta, spend on quality.
