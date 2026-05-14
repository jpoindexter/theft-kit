---
name: extract-component
description: Extract a component from fabrk-dev boilerplate into the framework monorepo
user-invocable: true
---

# Extract Component

Extract a component from the `fabrk-dev` boilerplate into `packages/components/`.

## Arguments

- `component`: The component name or path to extract (e.g., "button", "charts/funnel-chart")

## Workflow

### Step 1: Locate Source
Find the component in `../fabrk-dev/src/components/`. Check both flat files and directories.

### Step 2: Copy Files
Copy to the appropriate directory in `packages/components/src/`:
- UI primitives: `src/ui/`
- Charts: `src/charts/`
- Dashboard: `src/dashboard/`
- Admin: `src/admin/`
- AI: `src/ai/`
- Security: `src/security/`
- Org: `src/org/`

### Step 3: Transform Imports
Apply these transformations to every file:
```tsx
// FROM → TO
import { cn } from '@/lib/utils'        → import { cn } from '@fabrk/core'
import { mode } from '@/design-system'   → import { mode } from '@fabrk/design-system'
import { X } from '@/components/ui/x'    → import { X } from '../ui/x' (relative)
```

Remove any `@/` path aliases. Remove any application-specific business logic.

### Step 4: Add Barrel Export
Add the component to `packages/components/src/index.ts`:
```tsx
export * from './[category]/[component-name]'
```

### Step 5: Verify Design System Rules
Check the extracted component follows these rules:
- Full borders (`border`, `border-2`) have `mode.radius`
- Partial borders (`border-t`, `border-b`) do NOT have `mode.radius`
- No hardcoded colors (use design tokens)
- Terminal aesthetic (UPPERCASE labels/buttons/headlines)
- `"use client"` is NOT manually added (tsup banner handles it)

### Step 6: Build & Type Check
```bash
cd packages/components && pnpm build && pnpm type-check
```

### Step 7: Report
Output what was extracted, what imports were transformed, and whether build succeeded.
