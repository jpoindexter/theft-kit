---
name: Next.js Patterns
description: Next.js 16.1.6 App Router patterns, Turbopack config, and performance best practices for Hashmark
tools: [Read, Write, Edit, Glob, Grep, Bash, LSP]
---

# Next.js 16 Patterns — Hashmark

## Project Context

Hashmark runs Next.js 16.1.6 with App Router, Turbopack, TypeScript, and Tailwind. The route structure uses two route groups:
- `(marketing)` — public landing, login
- `(dashboard)` — authenticated, protected by middleware

## Critical Next.js 16 Rules

### Dynamic Params — Always Await
```typescript
// CORRECT — Next.js 16 requires Promise<>
export default async function Page({ params }: { params: Promise<{ repoId: string }> }) {
  const { repoId } = await params;
}

// WRONG — breaks in Next.js 16
export default async function Page({ params }: { params: { repoId: string } }) {
  const { repoId } = params; // TS error + runtime warning
}
```

### Async Request APIs
```typescript
import { cookies, headers } from 'next/headers';

// All async in Next.js 16
const cookieStore = await cookies();
const headersList = await headers();
```

### Server vs Client Boundary
- Default to Server Components. Add `'use client'` only when you need:
  - `useState`, `useEffect`, `useReducer`
  - Event handlers (onClick, onChange)
  - Browser APIs (window, document, navigator)
  - Third-party client libraries
- Push `'use client'` as deep as possible — don't mark a whole page client when only one widget needs it

### Server Actions — Form Pattern
```typescript
'use server';

// Must return void | Promise<void> for form actions
export async function triggerScan(formData: FormData) {
  const session = await auth();
  if (!session?.user?.id) throw new Error('Unauthorized');
  const userId = session.user.id; // extract immediately for TS narrowing
  // ...
}
```

## Turbopack Config

Hashmark uses Turbopack (default in Next.js 16). Config lives at the top level in `next.config.ts`, NOT under `experimental`:

```typescript
// next.config.ts
const nextConfig = {
  turbopack: {
    // rules, aliases, etc.
  }
};
```

## Metadata Pattern

```typescript
// Always scope to session user to prevent IDOR via <title> tag
export async function generateMetadata({ params }: { params: Promise<{ repoId: string }> }) {
  const session = await auth();
  if (!session?.user?.id) return { title: 'Hashmark' };
  const { repoId } = await params;
  const repo = await db.repository.findUnique({
    where: { id: repoId, userId: session.user.id }, // scope to owner!
    select: { fullName: true },
  });
  return { title: `${repo?.fullName ?? 'Repo'} — Hashmark` };
}
```

## Caching Patterns

### Revalidation after mutations
```typescript
import { revalidatePath } from 'next/cache';

// After server action mutation
revalidatePath('/dashboard/repos');
revalidatePath(`/dashboard/${repoId}`);
```

### No caching for authenticated pages
Dashboard pages are dynamic by default — don't add `revalidate` exports to authenticated routes.

## Loading and Error States

Every dashboard page segment needs:
- `loading.tsx` — skeleton UI with proper grid spacing
- `error.tsx` — error boundary with recovery action

```typescript
// loading.tsx
export default function Loading() {
  return (
    <div className="px-[var(--grid-6)] py-[var(--grid-8)]">
      <div className="h-8 w-48 bg-muted animate-pulse mb-[var(--grid-4)]" />
      <div className="h-64 bg-muted animate-pulse" />
    </div>
  );
}
```

## Image Configuration

GitHub avatars are whitelisted in `next.config.ts`:
```typescript
images: {
  remotePatterns: [
    { protocol: 'https', hostname: 'avatars.githubusercontent.com' }
  ]
}
```

Always use `next/image` for GitHub avatars, never `<img>`.

## Key Files

- `src/middleware.ts` — session cookie check, protects `/dashboard/*` and `/api/*`
- `src/lib/auth.ts` — NextAuth v5 config (24 dependents — edit carefully)
- `src/app/(dashboard)/layout.tsx` — dashboard shell, fetches session
- `src/app/(marketing)/layout.tsx` — public layout
- `next.config.ts` — image domains, turbopack config
