# theft-kit

A centralized library of reusable `SKILL.md` modules for Claude Code and other AI coding agents. Skills are structured prompt packets that give agents specialized knowledge and workflows — loaded on demand via the Claude Code plugin system or symlinked globally.

## What is a skill?

Each top-level directory is one skill. Every skill contains a `SKILL.md` with YAML frontmatter and a Markdown body:

```
dev-workflow/
  SKILL.md        # name, description, instructions
  overview.md     # optional supporting docs
```

Skills are invoked inside Claude Code sessions via `/skill <name>` or referenced in agent configurations.

## Key docs

| File | Purpose |
|------|---------|
| `CORE_SKILLS.md` | Curated daily-use skill set |
| `SKILL_PACKS.md` | Grouped packs by workflow type |
| `SKILLS_INDEX.md` | Full inventory |
| `IMPROVEMENTS.md` | Consolidation and governance roadmap |
| `ARCHIVE_CANDIDATES.md` | Staged cleanup targets |

## Skill packs (quick reference)

- **Core Delivery** — `architect-builder-method`, `sprint-architect-pack`, `clean-context-engineering`, `local-release-gate`, `model-switch-router`
- **Product Flow Quality** — `onboarding-state-machine`, `happy-path-route-audit`, `integration-contract-test`, `postmortem-rca`
- **Research & Strategy** — `research-brief-builder`, `source-truth-check`, `user-interview-synth`, `competitor-teardown`
- **UX** — `nng-ux-research-orchestrator`, `nng-web-usability`, `nng-information-architecture`, `nng-navigation-audit`
- **Design & Engineering** — `design-system-audit`, `frontend-design`, `code-review`, `security-scan`

## Setup

Clone the repo and symlink to wherever your agent runtime resolves skills from:

```sh
git clone https://github.com/jpoindexter/theft-kit.git
```

Then point your Claude Code or Codex config at the cloned path.

## Quality rules

- Every skill requires valid YAML frontmatter between `---` delimiters.
- Extend existing skills before creating new ones.
- Skill instructions should be concise and workflow-specific — not general documentation.

## Validation

```sh
./scripts/validate_skills.sh        # YAML frontmatter check on all SKILL.md files
./scripts/streamline_inventory.py   # overlap report and grouping CSV
```

## License

MIT
