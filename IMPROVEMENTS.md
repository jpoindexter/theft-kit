# theft-kit Improvement Plan

## Completed
- Centralized skills into theft-kit root
- Added global symlink usage through ~/.codex/skills
- Repaired invalid SKILL.md frontmatter issues
- Added missing operational skills:
  - happy-path-route-audit
  - paperclip-parity-delta
  - onboarding-state-machine
  - local-release-gate
  - model-switch-router
- Published standalone repo: https://github.com/jpoindexter/theft-kit

## Next Consolidation Actions
1. Collapse `agent-*` family into 3 umbrellas:
   - agent-orchestration
   - agent-implementation
   - agent-review
2. Collapse `html-ppt*` variants into one framework skill + style presets.
3. Mark every skill with status metadata: active / experimental / archived.
4. Introduce quarterly cleanup: remove stale duplicates and dead integrations.

## Operational Policy
- New skill must solve a repeated workflow or close a known gap.
- Prefer extending an existing skill over creating a near-duplicate.
- All skills require valid YAML frontmatter and explicit trigger description.
