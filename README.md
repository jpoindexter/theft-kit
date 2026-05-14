# theft-kit

Centralized skill library for THEFT AI and agentic software delivery workflows.

## Repo purpose
- Maintain one source of truth for reusable `SKILL.md` modules.
- Keep global skill availability stable through symlinks.
- Enforce predictable build quality via core process skills.

## Core docs
- `CORE_SKILLS.md` — daily-use curated skill set
- `IMPROVEMENTS.md` — consolidation and governance roadmap
- `ARCHIVE_CANDIDATES.md` — staged cleanup targets
- `SKILLS_INDEX.md` — full inventory

## Global setup
- Source: `/Users/jasonpoindexter/Documents/GitHub/theft-kit`
- Global: `/Users/jasonpoindexter/.codex/skills`

## High-priority core skills
- architect-builder-method
- project-brain-folder-system
- sprint-architect-pack
- clean-context-engineering
- model-switch-router
- onboarding-state-machine
- happy-path-route-audit
- local-release-gate
- paperclip-parity-delta

## Quality rules
- Every skill must include valid YAML frontmatter delimited by `---`.
- Avoid duplicate-purpose skills; extend existing skills first.
- Keep skill instructions concise and workflow-specific.

## Streamline Tools

- `SKILL_PACKS.md` — curated packs for day-to-day use.
- `scripts/validate_skills.sh` — validates SKILL.md YAML/frontmatter.
- `scripts/streamline_inventory.py` — generates overlap report and grouping CSV.
