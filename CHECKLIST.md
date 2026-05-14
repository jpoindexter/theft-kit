# theft-kit draft review checklist

Run this checklist before committing a new agent or a substantial rewrite. Fix every failing item. Adapted from Paperclip's draft-review checklist, scoped to Claude Code subagent format (no Paperclip runtime fields).

---

## A. Identity and framing

- [ ] YAML frontmatter has `name` (kebab-case), `description` (one sentence), and optional `tools`
- [ ] `description` states a concrete trigger: when Claude should invoke this agent. Not "assists with X."
- [ ] First sentence of the body names the role and the standard it holds (e.g., "a senior staff engineer at Stripe").

## B. Role clarity

- [ ] A stranger reading the description plus the first paragraph can tell in 30 seconds what this agent is for.
- [ ] "When to invoke" lists concrete triggers, not vague situations.
- [ ] "When NOT to invoke" names anti-patterns the agent should refuse or hand off.
- [ ] The role is something a single human specialist could do. If it spans 3 disciplines, split it.

## C. Authority and grounding

- [ ] "Authoritative references" cites named frameworks, books, specs, or people specific to the domain. Not generic "best practices."
- [ ] References are real and load-bearing: the agent's process draws from them, not just name-drops.

## D. Domain lenses (expert agents only)

- [ ] Expert / architect / strategist roles have 5-15 named lenses, each with a one-line explanation.
- [ ] Lenses are role-specific perspectives, not generic productivity advice.
- [ ] Operational / single-purpose agents do NOT carry copy-pasted lenses.

## E. Process

- [ ] Numbered, concrete steps. Each step has a verifiable output.
- [ ] No vague verbs ("ensure quality", "make it good"). Replace with measurable actions.
- [ ] Where commands or paths matter, they are spelled out (not "run the test suite").

## F. Output / review bar

- [ ] "Output format" describes the exact deliverable structure.
- [ ] "Quality bar" lists measurable success criteria.
- [ ] Negative examples included where useful ("a flow that compiles but isn't styled is not done").
- [ ] Evidence expectations are concrete: tests, screenshots, repro steps, percentile numbers, spec sections.

## G. Collaboration / handoffs (expert agents only)

- [ ] Cross-role handoffs name specific other agents in the kit (e.g., "route to security/security-reviewer for auth changes").
- [ ] Handoffs are listed only when the role actually touches that domain. No generic "consults with the team."

## H. Anti-patterns to refuse

- [ ] The agent has an explicit refusal posture for the most common abuses of the role.
  - Backtester refuses results without bootstrap CIs and walk-forward.
  - Code-reviewer refuses scope expansion beyond the diff.
  - Copywriter refuses "make it pop" without a brief.
- [ ] Refusals are domain-specific, not generic ("avoid bad code").

## I. Safety and least privilege

- [ ] If `tools` is set, it grants only what the role needs. No "just in case" Bash on a research agent.
- [ ] No secrets, API keys, or implementation details embedded in the agent body.
- [ ] Anything destructive (writes, network, sub-process spawning) is justified by the role.
- [ ] Project-coupled agents (specialized/) carry the project-coupled banner and name their target stack.

## J. Voice

- [ ] No em-dashes anywhere.
- [ ] No emojis.
- [ ] No marketing fluff ("delight users", "10x", "ship fast and break things").
- [ ] Voice matches the agent's domain target:
  - Engineering: Stripe Eng / Linear Eng / Anthropic safety
  - Security: OWASP / Google Project Zero
  - Trading: Two Sigma / Jane Street / Lopez de Prado rigor
  - Agency content/creative/strategy: Pentagram / Stripe Press / Linear marketing
  - Testing: Google Testing Blog / Test Pyramid

## K. Length

- [ ] 80-180 lines for most roles. Sub-80 only if the role is genuinely narrow. Over-180 only for lens-heavy expert roles.
- [ ] No padding. If a section adds nothing, drop it.

---

## Failure modes to grep for

- **Boilerplate pass-through.** If the body could apply to any role, the charter and lenses are too generic. Rewrite.
- **Capability expansion.** Wide tool grants, browser/filesystem reach, or external-system access without justification.
- **Marketing fluff.** "Empower", "delight", "seamless", "best-in-class" — strip them.
- **Process theatre.** Steps that name verbs without verifiable output ("review thoroughly", "iterate as needed").
- **Reference name-dropping.** Citing OWASP / 12-factor / Lopez de Prado without the agent's process actually drawing from them.
- **Cross-folder confusion.** Agent in `engineering/` doing work that belongs in `security/` or `testing/`. Move it.
- **Project-coupled in generic folder.** If the agent assumes a specific stack, it goes in `specialized/`.
