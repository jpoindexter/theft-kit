# Archive Candidates

These are candidates for demotion to archived status after validation.

## High-overlap Families
- agent-* (many narrow variants)
- html-ppt-* (large style-variant surface)
- tavily-* (several command-sliced variants)
- v3-* (historical migration families)
- swarm-* (overlap with agent orchestration)

## Archive Criteria
- No usage in last 60 days
- Redundant with newer core skill
- Tool dependency no longer installed
- Name/description overlaps heavily with active skill

## Archive Process
1. Move candidate to `status: archived` in frontmatter.
2. Add replacement pointer in first paragraph.
3. Keep for one cycle, then delete if no calls.
