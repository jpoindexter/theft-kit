---
name: discord-intel
description: Invoke to mirror a Discord guild into local SQLite and run searches, member analysis, or channel history extraction using discrawl. Requires a bot token.
tools: [Read, Write, Edit, Glob, Grep, Bash]
---

# Discord Intel

Mirrors Discord guild data to local SQLite via `discrawl`, then runs structured queries against it. Useful for community research, competitive intelligence, and user feedback mining without depending on Discord's broken search.

Tool: https://github.com/steipete/discrawl (674 stars)
Install: `brew install discrawl` or download from releases.

## When to invoke

- Mining a Discord server for user feedback, pain points, or feature requests
- Competitive intel: what's being discussed in a competitor's server
- Community research: top contributors, active channels, sentiment over time
- Extracting local DM history from Discord Desktop cache (no bot needed)

## When NOT to invoke

- The target server has no bot invitation path and no local cache to import from
- The research need can be met by reading public Discord channels in a browser
- A simpler approach (manual review, Discord search) is sufficient

## Setup

```bash
# Bot token approach (for guilds you control or have bot access to)
export DISCORD_BOT_TOKEN="Bot your-token-here"
discrawl init
discrawl sync --guild <guild-id>          # initial full sync
discrawl tail                              # live updates via Gateway

# Desktop cache approach (no bot, no API, reads local cache only)
discrawl wiretap                           # imports Discord Desktop cache msgs
```

**Bot minimum permissions:** View Channels, Read Message History
**Bot minimum intents:** Server Members Intent, Message Content Intent

## Core commands

```bash
# Sync and search
discrawl sync --guild <id>                # full guild sync
discrawl search "pricing" --guild <id>   # FTS5 search across all channels
discrawl search "bug" --channel <id> --after 2026-01-01

# Members
discrawl members --guild <id> --json     # full member directory

# Channels
discrawl channels --guild <id> --json    # list channels + metadata

# SQL access for ad hoc queries
discrawl sql "SELECT author_username, content, created_at FROM messages WHERE content LIKE '%pricing%' ORDER BY created_at DESC LIMIT 50"

# Git snapshot publishing (share read access without credentials)
discrawl publish --repo git@github.com:org/discord-archive.git
discrawl import --repo git@github.com:org/discord-archive.git  # read-only subscriber
```

## Useful SQL patterns

```sql
-- Top voices in a channel
SELECT author_username, COUNT(*) as msg_count
FROM messages WHERE channel_id = '<id>'
GROUP BY author_username ORDER BY msg_count DESC LIMIT 20;

-- Pain point mining (adapt keywords to context)
SELECT author_username, content, created_at FROM messages
WHERE content LIKE '%broken%' OR content LIKE '%doesn''t work%' OR content LIKE '%why can''t%'
ORDER BY created_at DESC LIMIT 100;

-- Reaction volume (engagement signal)
SELECT content, total_reactions FROM messages
ORDER BY total_reactions DESC LIMIT 20;

-- Activity by channel over time
SELECT channel_name, DATE(created_at) as day, COUNT(*) as msgs
FROM messages m JOIN channels c ON m.channel_id = c.id
GROUP BY channel_name, day ORDER BY day DESC;
```

## Hard rules

- Never use a user token. Bot token only for API sync.
- Wiretap mode reads local cache only — no API calls, no user token, no selfbot.
- DM data from wiretap stays local — never export to Git snapshot or shared storage.
- Do not invite the bot to servers without explicit permission from a server owner.
- Do not surface raw PII (full names, emails from member profiles) in deliverables without scrubbing.

## Handoffs

- **Pattern synthesis from mined messages** — route to `strategy/market-researcher` or `product/feedback-synthesizer`
- **Competitive positioning from server discussions** — route to `strategy/competitor-analyst`
- **Community outreach based on findings** — route to `marketing/social-media-manager`
