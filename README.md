# agents

Personal library of Claude Code agents. Subagent definitions organized by domain.

## Install

Clone into any location, then symlink or copy to `~/.claude/agents/`:

```bash
git clone git@github.com:jpoindexter/agents.git ~/code/agents
# Option A: symlink (keeps repo as source of truth)
ln -s ~/code/agents ~/.claude/agents
# Option B: copy
cp -r ~/code/agents/ ~/.claude/agents/
```

## Categories

Each folder is a domain with related subagents. All agents are Markdown files with frontmatter — Claude Code auto-discovers them.

| Folder | Purpose | Notes |
|--------|---------|-------|
| `content/` | Long-form writing — blogs, newsletters, content calendars, SEO | Useful for product launches and blog posts |
| `creative/` | Copy, ads, voice, video, visuals | `copywriter.md` is the main one I actually use |
| `data/` | Data quality, pipeline health, scrapers | Gripe and scraping work |
| `design/` | Accessibility, design critique, lint, QA, Figma, UX research, design-dev bridge | Most-used category |
| `engineering/` | Code review, security, bundle health, design-system compliance, documentation, rapid prototyping | Core dev workflow |
| `influencer/` | Creator scouting, campaign tracking, outreach, performance | Not actively used |
| `meta/` | Reality checks and self-audits for agent outputs | Use when stuck |
| `ops/` | Project tracking, deadlines, client reporting, invoicing, analytics | Project management |
| `pr/` | Press releases, journalist outreach, coverage tracking, crisis | Not actively used |
| `product/` | Spec validation | |
| `reference/` | Static guides (Claude skills, handoff patterns, identity/trust) | Docs, not agents per se |
| `sales/` | Follow-up, lead qualification, pipeline, pitch decks, proposals | Not actively used |
| `strategy/` | Competitor analysis, positioning, trend scouting, market research, campaigns | Most-used for Gripe + THEFT positioning |
| `testing/` | Test result analysis | |

## Philosophy

- Keep agent frontmatter descriptions specific — overlapping descriptions cause router confusion when Claude picks which agent to use.
- Every agent should have a name-when-NOT-to-use criterion in its description.
- Reference docs live in `reference/` because they're not invocable agents.
- Empty categories were deleted; re-add when populated.

## License

Personal use. Not published to any marketplace.
