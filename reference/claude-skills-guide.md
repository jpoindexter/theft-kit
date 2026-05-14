# Claude Skills -- Complete Reference

Source: "The Complete Guide to Building Skills for Claude" (Anthropic, 2025). anthropic/skills on GitHub.

---

## Skill file structure

```
my-skill/
  SKILL.md          # Required. YAML frontmatter + instructions. Max 5,000 words.
  scripts/          # Optional. Validation scripts, helpers.
  references/       # Optional. Detailed docs loaded on demand -- keeps SKILL.md lean.
  assets/           # Optional. Templates, configs.
```

---

## YAML frontmatter

```yaml
---
name: my-skill-name          # kebab-case, required, no spaces, no capitals, no "claude"/"anthropic"
description: |               # Required, under 1024 chars. This is what triggers the skill.
  [What it does] + [When to use it] + [Key capabilities]
license: MIT
compatibility: "..."         # Optional
metadata:
  author: Company Name
  version: 1.0.0
  mcp-server: server-name
  category: productivity
  tags: [project-management, automation]
---
```

---

## Two design approaches

**Problem-first**: "I need to set up a workspace." User describes an outcome. Skill orchestrates tools in the right sequence.

**Tool-first**: "I have Notion MCP connected." User has access to something. Skill teaches Claude optimal workflows for that tool.

---

## 5 design patterns

**1. Sequential workflow orchestration**
Use when: multi-step processes that must run in a specific order.
Key: explicit step ordering, dependency validation at each stage, rollback path on failure.

**2. Multi-MCP coordination**
Use when: workflows span multiple services (Figma to Drive to Linear to Slack).
Key: clear phase separation, defined data passing between MCPs, validation before advancing, centralized error handling.

**3. Iterative refinement**
Use when: output quality improves with iteration (report generation, spec writing).
Key: explicit quality criteria, validation scripts, defined stop condition. Do not loop indefinitely.

**4. Context-aware tool selection**
Use when: same outcome, different tools depending on context (file storage routing, environment detection).
Key: clear decision criteria, documented fallback options, transparency about choices made.

**5. Domain-specific intelligence**
Use when: the skill adds specialized knowledge beyond tool access (financial compliance, legal review).
Key: domain rules embedded in logic, compliance checks before action, governance documented.

---

## Description field -- the trigger mechanism

The description is how Claude decides whether to invoke a skill. A poor description means the skill never triggers or triggers incorrectly.

- Too generic ("Helps with projects") = never triggers
- Must include phrases users would actually say
- Include relevant file types if the skill operates on files
- Add negative triggers to prevent false matches: "Do NOT use for [X]"

Debug method: ask Claude "When would you use [skill name]?" -- it quotes the description back. If the answer is wrong, the description is wrong.

---

## Context loading (progressive disclosure)

Three tiers. This prevents context bloat.

| Layer | When loaded | Contents |
|-------|------------|---------|
| YAML frontmatter | Always, on every skill match | Name, description, tool list |
| SKILL.md body | When skill is triggered | Instructions, patterns, examples |
| references/ files | On demand, when skill needs them | Detailed docs, large tables, templates |

Keep SKILL.md under 5,000 words. Move anything longer than a reference table to `references/`.

---

## Troubleshooting

**Skill will not upload**
- File must be named exactly `SKILL.md` (case-sensitive)
- YAML must use `---` delimiters. Check quoting on multi-line values.

**Skill does not trigger**
- Description too generic or missing the phrases users actually type
- Fix: add specific trigger phrases from real user requests

**Skill triggers too often**
- Add negative triggers: "Do NOT use for [X]"
- Narrow scope in the description

**Instructions not followed**
- Instructions too verbose: use numbered lists, not paragraphs
- Critical instructions buried: put them at the top with a `##` header
- Ambiguous language: be explicit ("CRITICAL: Before calling X, verify: [list]")
- For validations that must not be skipped: bundle a script instead of relying on language instructions

**MCP connection issues**
- Verify MCP server connected at Settings > Extensions
- Check auth: API keys, OAuth tokens, permission scopes
- Test MCP independently before testing with the skill
- Tool names are case-sensitive

---

## Testing methodology

Source: Anthropic skill development guidelines (anthropic/skills, 2025).

**Trigger tests**: should-trigger scenarios + should-NOT-trigger scenarios. Target: 90% accuracy before distributing.

**Functional tests**: Given / When / Then format for each capability.

**Performance comparison**: run the same task with-skill and without-skill. If the skill does not improve accuracy or reduce steps, the instructions need revision.

---

## Distribution

| Target | Method |
|--------|--------|
| Claude.ai (personal) | Settings > Capabilities > Skills > Upload folder |
| Claude Code | Skills in project directory |
| API | `/v1/skills` endpoint, `container.skills` parameter in Messages API |
| Org-level | Admin-deployed workspace-wide (available Dec 2025+) |

Do not enable more than 20-50 skills simultaneously -- context cost compounds.

---

## Quick checklist

Before upload:
- [ ] Folder name in kebab-case
- [ ] File named exactly `SKILL.md`
- [ ] YAML `---` delimiters present
- [ ] `name` is kebab-case, no capitals, no "claude"/"anthropic"
- [ ] Description includes WHAT and WHEN, specific enough to trigger correctly
- [ ] No XML tags (`< >`) anywhere in the file
- [ ] Critical validations at the top, not buried
- [ ] Error handling documented
- [ ] Tested triggering on obvious and paraphrased requests
- [ ] Tested NOT triggering on unrelated topics
- [ ] Compressed as .zip before upload to Claude.ai
