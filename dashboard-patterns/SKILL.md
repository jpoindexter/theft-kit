---
name: dashboard-patterns
description: Create dashboard pages and features following established patterns
license: MIT
compatibility: Claude Code
metadata:
  author: indx.sh
  version: "1.0"
  project: indx-web
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
---

# Dashboard Patterns Skill

## When to use this skill
Use when creating or modifying dashboard pages. Invoke this skill when:
- Creating new dashboard views (saved, submissions, history, analytics)
- Adding KPI cards or metrics displays
- Building data tables with filters and actions
- Implementing pagination and bulk actions

## Core Pattern

All dashboard pages MUST use the `UserDashboardTemplate`:

```tsx
import { UserDashboardTemplate } from '@/components/templates/user-dashboard-template';

export default function DashboardSavedPage() {
  return (
    <UserDashboardTemplate
      title="Saved Items"
      description="Your bookmarked rules and MCP servers"
      user={user}
      headerActions={<Button>> MANAGE</Button>}
    >
      {/* Dashboard content */}
    </UserDashboardTemplate>
  );
}
```

## Required Components

### 1. KPI Cards Row

Display metrics at the top of dashboard:

```tsx
import { KPICard } from '@/components/ui/kpi-card';

<div className="grid grid-cols-2 gap-4 md:grid-cols-4">
  <KPICard
    title="Total Views"
    value={stats.views.toLocaleString()}
    trend={{ value: 12, direction: 'up' }}
  />
  <KPICard
    title="Total Copies"
    value={stats.copies.toLocaleString()}
    trend={{ value: 5, direction: 'up' }}
  />
  <KPICard
    title="Rules"
    value={stats.rules}
  />
  <KPICard
    title="MCP Servers"
    value={stats.mcps}
  />
</div>
```

### 2. Data Table with Toolbar

Use DataTable component with toolbar for lists:

```tsx
import { DataTable } from '@/components/ui/data-table/data-table';

<DataTable
  columns={columns}
  data={items}
  searchKey="title"
  searchPlaceholder="Search saved items..."
  filterColumn="type"
  filterOptions={[
    { label: 'All', value: 'all' },
    { label: 'Rules', value: 'rule' },
    { label: 'MCP Servers', value: 'mcp' },
  ]}
  pageSize={10}
/>
```

### 3. Empty States with CTAs

Always provide helpful empty states:

```tsx
import { EmptyState } from '@/components/ui/empty-state';

{items.length === 0 ? (
  <EmptyState
    icon={<Bookmark className="h-12 w-12" />}
    title="No Saved Items"
    description="Items you save will appear here for quick access"
    action={
      <Button asChild>
        <Link href="/rules">> BROWSE RULES</Link>
      </Button>
    }
  />
) : (
  <DataTable ... />
)}
```

### 4. Bulk Actions Toolbar

For tables with selection:

```tsx
import { DataTableBulkActions } from '@/components/ui/data-table/data-table-bulk-actions';

<DataTableBulkActions
  selectedCount={selectedRows.length}
  onUnsave={() => handleBulkUnsave(selectedRows)}
  onExport={() => handleExport(selectedRows)}
  onDelete={() => handleBulkDelete(selectedRows)}
/>
```

## Data Fetching Patterns

### Client-side with useEffect

```tsx
'use client';

export default function DashboardSavedPage() {
  const [items, setItems] = React.useState<SavedItem[]>([]);
  const [isLoading, setIsLoading] = React.useState(true);

  React.useEffect(() => {
    async function fetchSaved() {
      try {
        const response = await fetch('/api/user/bookmarks');
        const data = await response.json();
        setItems(data.bookmarks);
      } catch (error) {
        console.error('Failed to fetch:', error);
      } finally {
        setIsLoading(false);
      }
    }
    fetchSaved();
  }, []);

  if (isLoading) {
    return <DashboardSkeleton />;
  }

  return (/* ... */);
}
```

### Server-side (preferred for SEO/initial load)

```tsx
// page.tsx (Server Component)
import { auth } from '@/lib/auth';
import { prisma } from '@/lib/prisma';
import { DashboardSavedClient } from './client';

export default async function DashboardSavedPage() {
  const session = await auth();
  if (!session?.user) redirect('/login');

  const bookmarks = await prisma.bookmark.findMany({
    where: { userId: session.user.id },
    include: { rule: true, mcpServer: true },
    orderBy: { createdAt: 'desc' },
  });

  return <DashboardSavedClient items={bookmarks} />;
}
```

## Loading States

Always show loading skeletons:

```tsx
import { Skeleton } from '@/components/ui/skeleton';

function DashboardSkeleton() {
  return (
    <div className="space-y-6">
      {/* KPI Skeleton */}
      <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
        {[...Array(4)].map((_, i) => (
          <Skeleton key={i} className="h-24" />
        ))}
      </div>

      {/* Table Skeleton */}
      <div className="space-y-2">
        <Skeleton className="h-10" />
        {[...Array(5)].map((_, i) => (
          <Skeleton key={i} className="h-16" />
        ))}
      </div>
    </div>
  );
}
```

## Design System Compliance

### Spacing (8-point grid)
- Section spacing: `space-y-6`
- Card grid: `gap-4`
- Table rows: `space-y-2`
- Never use p-3, p-5, gap-3, gap-5

### Colors (design tokens only)
```tsx
// KPI cards
mode.color.bg.surface
mode.color.border.default

// Trends
mode.color.text.success  // Positive trend
mode.color.text.danger   // Negative trend

// Table
mode.color.bg.muted      // Zebra striping
```

### Typography
```tsx
// Dashboard title
mode.typography.display.lg

// KPI values
mode.typography.headline.lg

// KPI labels
mode.typography.label.md
```

## Checklist for Dashboard Pages

- [ ] Uses UserDashboardTemplate
- [ ] Has KPI cards for key metrics (if applicable)
- [ ] Data loads from API or server
- [ ] Shows loading skeleton during fetch
- [ ] Has proper empty state with CTA
- [ ] Tables use DataTable component
- [ ] Pagination implemented for large datasets
- [ ] Bulk actions available (if applicable)
- [ ] All spacing follows 8-point grid
- [ ] All colors use design tokens

## File Organization

```
src/app/(platform)/dashboard/
  ├── page.tsx              # Main dashboard (KPIs, recent activity)
  ├── saved/
  │   └── page.tsx          # Bookmarked items
  ├── submissions/
  │   └── page.tsx          # User's submitted content
  ├── history/
  │   └── page.tsx          # Recently viewed
  └── analytics/
      └── page.tsx          # Detailed analytics/charts
```

## Common API Routes

```
src/app/api/user/
  ├── bookmarks/route.ts    # GET, POST, DELETE
  ├── submissions/route.ts  # GET
  ├── history/route.ts      # GET
  └── analytics/route.ts    # GET (stats, charts data)
```

## Chart Integration

For analytics pages, use chart components:

```tsx
import { LineChart } from '@/components/ui/line-chart';
import { BarChart } from '@/components/ui/bar-chart';

<LineChart
  data={viewsData}
  categories={['Views', 'Copies']}
  index="date"
  title="Activity Over Time"
/>
```
