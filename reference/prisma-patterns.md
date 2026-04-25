---
name: Prisma Patterns
description: Prisma + Neon Postgres patterns, ownership scoping, transactions, and migration workflow for Hashmark
tools: [Read, Write, Edit, Glob, Grep, Bash, LSP]
---

# Prisma + Neon Patterns — Hashmark

## Project Context

Hashmark uses Prisma ORM with Neon serverless Postgres. The Prisma client is a singleton at `src/lib/db.ts`. Schema lives at `prisma/schema.prisma` with 9 models.

## Critical Rules

### Always Scope Queries to Owner
Every query that touches user data MUST include `userId` in the where clause. This prevents IDOR (Insecure Direct Object Reference) vulnerabilities.

```typescript
// CORRECT — scoped to session user
const repo = await db.repository.findUnique({
  where: { id: repoId, userId: session.user.id },
});

// WRONG — any authenticated user can access any repo
const repo = await db.repository.findUnique({
  where: { id: repoId },
});
```

Same pattern applies to scans — always join through the repo to verify ownership:
```typescript
const scan = await db.scan.findUnique({
  where: { id: scanId },
  include: { repository: { select: { userId: true } } },
});
if (scan?.repository.userId !== session.user.id) throw new Error('Not found');
```

## Models Overview

```
User (id, email, plan: FREE|PRO, trialEndsAt, stripeCustomerId, ...)
  └── Repository (id, userId, githubRepoId, fullName, defaultBranch, scanRoot, ...)
        └── Scan (id, repositoryId, status: PENDING|SCANNING|COMPLETE|FAILED, results, ...)
              ├── GeneratedFile (id, scanId, fileName, content, tokenCount, ...)
              └── SearchChunk (id, scanId, repositoryId, sectionHeading, content, ...)
  └── CustomRule (id, userId, name, description, rule, ...)
Account / Session / VerificationToken (NextAuth v5)
WebhookEvent (id, processedAt) — Stripe webhook dedup
```

## Migration Workflow

```bash
# After schema changes:
npm run db:push        # push schema to Neon dev DB + regenerate client
npm run db:generate    # regenerate Prisma client only (no DB change)
npm run db:studio      # open Prisma Studio for inspection
```

### dotenv-cli Requirement
Prisma CLI can't read `.env.local` natively. All `db:*` scripts use `dotenv -e .env.local --` prefix. **Never run `prisma` directly** — always via `npm run db:*`.

## Neon Connection Gotcha

Neon provides two connection strings:
- **Pooled** (`-pooler` in hostname): for app runtime, required for serverless
- **Direct** (no `-pooler`): required for Prisma migrations and DDL

If `db:push` fails with connection errors, check that `DATABASE_URL` uses the direct connection for migrations. The app at runtime should use pooled.

## Transactions

Use `$transaction` for any multi-step operation where consistency matters:

```typescript
// Plan gate + upsert — prevents TOCTOU race
await db.$transaction(async (tx) => {
  const count = await tx.repository.count({ where: { userId } });
  if (count >= 1) throw new Error('Limit reached');
  await tx.repository.upsert({ ... });
});
```

## Scan Status Pattern

Scans go through: `PENDING` → `SCANNING` → `COMPLETE` | `FAILED`

```typescript
// Create scan record first, then fire-and-forget worker
const scan = await db.scan.create({
  data: { repositoryId: repoId, status: 'PENDING' },
});
runScan(scan.id, repo.fullName, token, ...).catch(console.error);
```

Never await `runScan` in a server action — it would block the request. The scan worker updates status via periodic `db.scan.update` calls.

## Orphaned Scan Recovery

`src/lib/scan-error.ts` exports `recoverOrphanedScans()` — debounced (60s) recovery that marks PENDING/SCANNING scans older than 10 minutes as FAILED. Called from the polling endpoint.

## Webhook Dedup

`WebhookEvent` model provides durable idempotency for Stripe webhooks:
```typescript
try {
  await db.webhookEvent.create({ data: { id: stripeEventId } });
} catch {
  // Unique constraint violation = already processed
  return NextResponse.json({ received: true, action: 'duplicate' }, { status: 202 });
}
```

## Select Only What You Need

```typescript
// Good — minimal select
const user = await db.user.findUnique({
  where: { id: userId },
  select: { plan: true },
});

// Avoid — fetches all columns including tokens
const user = await db.user.findUnique({ where: { id: userId } });
```

## High-Impact Files

- `prisma/schema.prisma` — schema changes cascade to all 24 db dependents
- `src/lib/db.ts` — Prisma singleton (24 dependents — do not restructure)
- `src/lib/scan-error.ts` — orphan recovery + error formatting
- `src/lib/scan-worker.ts` — background scan with progress updates
