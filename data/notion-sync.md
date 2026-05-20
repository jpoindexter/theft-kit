---
name: notion-sync
description: Invoke to mirror a Notion workspace into local SQLite and Markdown using notcrawl. Enables FTS search, SQL queries, and durable exports without hitting Notion's UI or rate limits.
tools: [Read, Write, Edit, Glob, Grep, Bash]
---

# Notion Sync

Mirrors Notion workspaces to local SQLite + normalized Markdown via `notcrawl`. Two ingestion paths: local Desktop cache (instant, no API key) and official Notion API (complete, rate-limit-aware). SQLite is the canonical store; Markdown is the agent-readable surface.

Tool: https://github.com/vincentkoc/notcrawl (65 stars)
Install: `brew tap vincentkoc/tap && brew install notcrawl`

## When to invoke

- Client shares a Notion workspace and you need to analyze its content without clicking through pages
- Research synthesis from a large Notion knowledge base
- Generating a durable local snapshot before a client offboarding
- Feeding Notion content into agent context (Markdown export is clean and LLM-friendly)
- Diffing workspace snapshots over time (Git share mode)

## When NOT to invoke

- The Notion content is small enough to copy-paste manually
- The MCP Notion integration is already registered and works for the task
- Write access to Notion is needed (notcrawl is read-only)

## Setup

```bash
# Option 1: Local Desktop cache (no API key, instant)
notcrawl init
notcrawl sync --source desktop

# Option 2: Official API
export NOTION_TOKEN="secret_..."        # from notion.com/my-integrations
notcrawl init
notcrawl sync --source api

# Verify
notcrawl doctor
notcrawl status
```

## Core commands

```bash
# Sync and export
notcrawl sync --source api             # full workspace sync
notcrawl sync --source desktop         # local cache import
notcrawl export-md --out ./notion-export/   # Markdown export (LLM-ready)
notcrawl search "launch plan"          # FTS5 search

# Databases
notcrawl databases                     # list all Notion databases
notcrawl export-csv --database <id> --out ./db.csv   # export database rows

# Archive management
notcrawl report                        # activity summary
notcrawl status                        # sync state

# Git share (publish snapshot for other agents to read without credentials)
notcrawl publish --repo git@github.com:org/notion-snapshot.git
notcrawl import --repo git@github.com:org/notion-snapshot.git
```

## SQL patterns (FTS5 + structured queries)

```sql
-- Full-text search across all pages
SELECT title, content_md, last_edited_time FROM pages
WHERE pages MATCH 'pricing strategy' ORDER BY last_edited_time DESC;

-- All pages in a specific database
SELECT title, properties, last_edited_time FROM pages
WHERE parent_database_id = '<db-id>' ORDER BY last_edited_time DESC;

-- Recently edited pages (research freshness check)
SELECT title, last_edited_by, last_edited_time FROM pages
ORDER BY last_edited_time DESC LIMIT 20;

-- Pages by property value (e.g. Status = "In Progress")
SELECT title, properties FROM pages
WHERE json_extract(properties, '$.Status.select.name') = 'In Progress';
```

## Markdown export structure

```
notion-export/
  {space-name}/
    {page-title}.md          # normalized, agent-readable
    {database-name}/
      {row-title}.md
```

Each Markdown file includes frontmatter with `id`, `parent`, `last_edited`, and `properties` — clean for LLM context injection.

## Hard rules

- Read-only. Never attempt write operations against Notion through this tool.
- Desktop cache ingestion reads local files only — no API calls, no token required.
- Do not publish Git snapshots of client workspaces without explicit client authorization.
- Scrub any PII from Markdown exports before including in shared deliverables.

## Handoffs

- **Research synthesis from exported Markdown** — route to `content/blog-writer` or `strategy/market-researcher`
- **Database analysis from CSV export** — route to `data/data-enrichment`
- **Client knowledge base audit** — route to `product/feedback-synthesizer`
- **Competitive intel from a competitor's public Notion** — route to `strategy/competitor-analyst`
