---
name: swiss-design
description: Enforce Swiss design principles - reduce visual noise, remove harsh borders
license: MIT
compatibility: Claude Code
metadata:
  version: "1.0"
allowed-tools:
  - Read
  - Grep
  - Glob
---

# Swiss Design Skill

## When to use this skill
Use when creating or auditing UI components to ensure they follow Swiss design principles. Invoke this skill when:
- Creating new components with visual structure
- Auditing existing components for visual noise
- Removing harsh borders that create compartmentalized aesthetics

## Core Philosophy

Swiss design (International Typographic Style) emphasizes:
- **Cleanliness**: Minimal visual noise
- **Hierarchy**: Clear information structure through whitespace, not borders
- **Restraint**: Maximum 1 level of border nesting

## Key Rules

### 1. Border Reduction Hierarchy

**Container-Only Borders**
Borders should only appear on outermost containers. Internal elements use background colors or spacing for separation.

```tsx
// BAD - Multiple border levels
<Card className="border">
  <CardHeader className="border-b">  // ❌ Inner border
    <h3>Title</h3>
  </CardHeader>
  <CardContent>...</CardContent>
</Card>

// GOOD - Container border only, internal separation via background
<Card className="border">
  <CardHeader className="bg-muted/10">  // ✅ Subtle background
    <h3>Title</h3>
  </CardHeader>
  <CardContent>...</CardContent>
</Card>
```

### 2. Visual Separation Alternatives

Instead of `border-b` or `border-t`, use these alternatives:

| Instead of | Use |
|------------|-----|
| `border-b` | `bg-muted/5` or `bg-muted/10` |
| `border-t` | `pt-4` with `space-y-4` parent |
| Row dividers | `space-y-1` or `gap-1` |
| Zebra borders | `even:bg-muted/5` (zebra striping) |

### 3. Table Styling

```tsx
// BAD - Row borders create harsh lines
<TableRow className="border-b">
  <TableCell>...</TableCell>
</TableRow>

// GOOD - Zebra striping is softer
<TableRow className="even:bg-muted/5">
  <TableCell>...</TableCell>
</TableRow>
```

### 4. Section Separation

```tsx
// BAD - Borders between sections
<section className="border-b pb-4">
  <h2>Section 1</h2>
</section>
<section className="border-b pb-4">
  <h2>Section 2</h2>
</section>

// GOOD - Whitespace and optional subtle backgrounds
<div className="space-y-8">
  <section>
    <h2>Section 1</h2>
  </section>
  <section className="pt-8">
    <h2>Section 2</h2>
  </section>
</div>
```

### 5. Sidebar and Navigation

```tsx
// BAD - Hard border creates harsh line
<aside className="border-r w-64">

// GOOD - Subtle background difference
<aside className="bg-muted/5 w-64">
```

## Audit Checklist

When auditing a component for Swiss design compliance:

- [ ] Only outer container has full border
- [ ] No `border-b` inside cards/containers (use `bg-muted/*` instead)
- [ ] No `border-t` for footers (use `pt-4` spacing instead)
- [ ] Tables use zebra striping not row borders
- [ ] Sidebars use background not border-r
- [ ] Maximum 1 level of border nesting

## Grep Patterns for Violations

```bash
# Find internal borders in cards
grep -rn "CardHeader.*border-b\|CardFooter.*border-t" src/

# Find table row borders
grep -rn "TableRow.*border-b" src/

# Find sidebar borders
grep -rn "sidebar.*border-r\|border-r.*sidebar" src/

# Find excessive borders
grep -rn "border-b.*border-t\|border-t.*border-b" src/
```

## Output Format

When reporting violations:

```
SWISS DESIGN AUDIT RESULTS
===========================

VIOLATIONS FOUND: X

[INNER BORDER] src/components/ui/card.tsx:96
  Found: CardHeader with border-b
  Fix: Replace border-b with bg-muted/10

[TABLE BORDER] src/components/ui/table.tsx:133
  Found: TableRow with border-b
  Fix: Replace border-b with even:bg-muted/5

[SIDEBAR BORDER] src/components/layout/sidebar.tsx:45
  Found: aside with border-r
  Fix: Replace border-r with bg-muted/5
```

## Design Token Usage

Always use semantic tokens for backgrounds:

```tsx
import { mode } from '@/design-system';

// Subtle header separation
className={cn(mode.color.bg.muted, 'bg-opacity-10')}

// Even softer footer area
className={cn(mode.color.bg.muted, 'bg-opacity-5')}

// Container border (only one level)
className={cn('border', mode.color.border.default, mode.radius)}
```
