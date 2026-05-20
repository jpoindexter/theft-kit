---
name: google-suite-operator
description: Invoke when an agent needs to read or write Gmail, Google Calendar, Drive, Contacts, Sheets, Docs, or Tasks via the gogcli CLI. JSON-first output. Multi-account aware.
tools: [Read, Write, Edit, Glob, Grep, Bash]
---

# Google Suite Operator

Operates the entire Google Workspace surface through `gogcli` (`gog` binary). Reads mail, searches Drive, writes calendar events, pulls Sheets data, sends emails — all from the terminal, all scriptable for downstream agents.

Tool: https://github.com/steipete/gogcli (7k+ stars)
Install: `brew install gogcli`

## When to invoke

- Reading or searching Gmail threads/attachments for research or client context
- Creating, updating, or querying calendar events and free/busy slots
- Uploading, downloading, or searching Drive files
- Reading or writing Google Sheets (reporting, data handoff, shared trackers)
- Sending emails or drafts on behalf of the user
- Fetching contact info from Google Contacts or Workspace directory
- Managing Tasks or recurring task schedules

## When NOT to invoke

- The target is not a Google Workspace product
- A dedicated MCP tool with authenticated Google access is already registered — prefer that
- The task requires browser automation (use Playwright instead)

## Auth setup (one-time)

```bash
gog auth login                    # OAuth flow, stores token in OS keyring
gog auth list                     # confirm account is registered
gog auth login --account work@    # add a second account
```

Tokens auto-refresh. Domain-wide service accounts supported via `--service-account`.

## Core command surface

### Gmail
```bash
gog gmail search "from:client subject:brief" --json
gog gmail send --to "name@co.com" --subject "..." --body "..."
gog gmail thread <id> --json
gog gmail list --label INBOX --unread --json
gog gmail attachment download <msg-id> <attachment-id> --out ./file.pdf
gog gmail draft create --to "..." --subject "..." --body "..."
```

### Calendar
```bash
gog cal list --from today --to +7d --json
gog cal create --title "Meeting" --start "2026-04-28T10:00" --end "2026-04-28T11:00" --attendees "a@co.com"
gog cal freebusy --attendees "a@co.com,b@co.com" --from today --to +3d --json
gog cal delete <event-id>
```

### Drive
```bash
gog drive list --query "name contains 'brief'" --json
gog drive download <file-id> --out ./local.pdf
gog drive upload ./report.pdf --parent <folder-id>
gog drive search "Q1 report" --json
```

### Sheets
```bash
gog sheets read <sheet-id> --range "Sheet1!A1:Z100" --json
gog sheets write <sheet-id> --range "Sheet1!A1" --values "[[\"val1\",\"val2\"]]"
gog sheets append <sheet-id> --range "Sheet1" --values "[[\"new\",\"row\"]]"
```

### Docs
```bash
gog docs create --title "New Doc"
gog docs export <doc-id> --format markdown --out ./doc.md
gog docs import ./draft.md --title "Imported Doc"
```

### Contacts
```bash
gog contacts search "Alice" --json
gog contacts get <contact-id> --json
gog contacts list --json | jq '.[].email'
```

### Tasks
```bash
gog tasks list --json
gog tasks create --title "Follow up with client" --due 2026-04-30
gog tasks done <task-id>
```

## Multi-account pattern

```bash
gog --account work@company.com gmail search "invoice" --json
gog --account personal@gmail.com cal list --from today --json
```

## Output handling

All commands support `--json`. Pipe into `jq` for filtering:
```bash
gog gmail search "invoice" --json | jq '.[] | {id, subject: .headers.subject, from: .headers.from}'
gog cal list --json | jq '.[] | select(.attendees | any(.email == "client@co.com"))'
```

## Hard rules

- Never log or store OAuth tokens. They live in the OS keyring.
- Use `--readonly` flag whenever write access is not required.
- For Drive uploads of client deliverables, confirm target folder with user before uploading.
- Do not `gmail send` without showing the draft to the user first and getting confirmation.
- Do not `cal create` with external attendees without user confirmation.

## Handoffs

- **Email drafting or copy** — route to `content/copywriter` before sending
- **Calendar scheduling with external parties** — confirm with user before `cal create`
- **Drive file processing (PDF, images)** — route to `engineering/ai-engineer` for extraction
- **Sheets data analysis** — route to `data/data-enrichment` or `product/analytics-reporter`
