---
name: grid-spacing
description: Enforce Swiss-Industrial 4px grid spacing system - ALL spacing must use --grid-* CSS variables
license: MIT
compatibility: Claude Code
metadata:
  version: "1.0"
  priority: critical
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - Bash
---

# Grid Spacing System - MANDATORY ENFORCEMENT

**ALL spacing MUST use `--grid-*` CSS variables. NO hardcoded Tailwind spacing.**

Based on IBM Carbon's 4px Mini-Unit system for Swiss-Industrial design.

---

## Grid Spacing Scale

| Token | Value | Grid Units | Use Case |
|-------|-------|------------|----------|
| `--grid-0` | 0 | 0 | None |
| `--grid-1` | 4px | 1 | Micro: icon padding, tight gaps |
| `--grid-2` | 8px | 2 | Small: between related items |
| `--grid-3` | 12px | 3 | Medium-small: toolbar gaps |
| `--grid-4` | 16px | 4 | Standard: 1 line-height |
| `--grid-6` | 24px | 6 | Medium: section headers |
| `--grid-8` | 32px | 8 | Large: between sections |
| `--grid-12` | 48px | 12 | XL: page sections |
| `--grid-16` | 64px | 16 | XXL: major divisions |

---

## Usage Examples

```tsx
// ✅ CORRECT - Grid variables
<div className="pt-[var(--grid-4)] mb-[var(--grid-8)]">
<div className="gap-[var(--grid-2)] space-y-[var(--grid-4)]">
<header className="pb-[var(--grid-4)] mb-[var(--grid-8)]">
<section className="mt-[var(--grid-12)]">
<main className="space-y-[var(--grid-16)]">

// ❌ BANNED - Hardcoded Tailwind spacing
<div className="pt-4 mb-8">        // NEVER
<div className="gap-2 space-y-4">  // NEVER
<div className="p-3 mt-5">         // NEVER
<section className="mt-12">        // NEVER
```

---

## Swiss Layout Hierarchy

Standard vertical spacing between page elements:

```
┌─────────────────────────────────────────────────────────┐
│ GLOBAL BAR (Site Header)                                │
├─────────────────────────────────────────────────────────┤
│                    ↓ --grid-4 (16px)                    │
├─────────────────────────────────────────────────────────┤
│ PAGE HEADER (Title, Description, Breadcrumbs)           │
├─────────────────────────────────────────────────────────┤
│                    ↓ --grid-6 (24px)                    │
├─────────────────────────────────────────────────────────┤
│ TOOLBAR / FILTERS                                       │
├─────────────────────────────────────────────────────────┤
│                    ↓ --grid-4 (16px)                    │
├─────────────────────────────────────────────────────────┤
│ MAIN CONTENT                                            │
├─────────────────────────────────────────────────────────┤
│                    ↓ --grid-8 (32px)                    │
├─────────────────────────────────────────────────────────┤
│ SECONDARY SECTION                                       │
├─────────────────────────────────────────────────────────┤
│                    ↓ --grid-12 (48px)                   │
├─────────────────────────────────────────────────────────┤
│ FOOTER                                                  │
└─────────────────────────────────────────────────────────┘
```

| Transition | Spacing | Token |
|------------|---------|-------|
| Global Bar → Page Header | 16px | `pt-[var(--grid-4)]` |
| Page Header → Toolbar | 24px | `pb-[var(--grid-6)]` |
| Toolbar → Content | 16px | `mt-[var(--grid-4)]` |
| Section → Section | 32px | `space-y-[var(--grid-8)]` |
| Content → Footer | 48px | `mt-[var(--grid-12)]` |
| Major divisions | 64px | `space-y-[var(--grid-16)]` |

---

## Page-Type Standards

### Table/Listing Pages

| Element | Spacing | Token |
|---------|---------|-------|
| Page header top padding | 16px | `pt-[var(--grid-4)]` |
| Page header bottom padding | 16px | `pb-[var(--grid-4)]` |
| Toolbar item gap | 8px | `gap-[var(--grid-2)]` |
| Table row height (compact) | 16px | `h-[var(--grid-4)]` |
| Table row height (comfortable) | 24px | `h-[var(--grid-6)]` |
| Table row height (spacious) | 32px | `h-[var(--grid-8)]` |

### Detail Pages

| Element | Spacing | Token |
|---------|---------|-------|
| Header bottom padding | 16px | `pb-[var(--grid-4)]` |
| Header bottom margin | 32px | `mb-[var(--grid-8)]` |
| Badge gap | 8px | `gap-[var(--grid-2)]` |
| Title to description | 16px | `mt-[var(--grid-4)]` |
| Content section gap | 64px | `space-y-[var(--grid-16)]` |
| Back link top margin | 64px | `mt-[var(--grid-16)]` |
| Back link top padding | 32px | `pt-[var(--grid-8)]` |

### Right Rail Sections

| Element | Spacing | Token |
|---------|---------|-------|
| Section divider padding | 16px | `pt-[var(--grid-4)]` |
| Section header margin | 8px | `mb-[var(--grid-2)]` |
| Property row gap | 4px | `space-y-[var(--grid-1)]` |
| Button gap | 8px | `gap-[var(--grid-2)]` |
| Tag/chip gap | 4px | `gap-[var(--grid-1)]` |

### Sidebar Navigation

| Element | Spacing | Token |
|---------|---------|-------|
| Section header margin-top | 16px | `mt-[var(--grid-4)]` |
| Section header margin-bottom | 4px | `mb-[var(--grid-1)]` |
| Nav item height | 24px | `h-[var(--grid-6)]` |
| Nav item gap | 4px | `space-y-[var(--grid-1)]` |
| Nav item padding-x | 2ch | `px-[2ch]` |
| Icon to label gap | 8px | `gap-[var(--grid-2)]` |

### Content Pages (About, Legal, Docs)

| Element | Spacing | Token |
|---------|---------|-------|
| Hero bottom margin | 48px | `mb-[var(--grid-12)]` |
| Section bottom margin | 32px | `mb-[var(--grid-8)]` |
| Paragraph margin | 32px | `mb-[var(--grid-8)]` |
| Card padding | 32px | `p-[var(--grid-8)]` |
| List item gap | 8px | `space-y-[var(--grid-2)]` |

---

## Horizontal Spacing

Use character widths (`ch`) for horizontal padding in monospace layouts:

```tsx
px-[1ch]   // 1 character width (~16px)
px-[2ch]   // 2 character widths (~32px)
px-[3ch]   // 3 character widths (~48px)
```

---

## BANNED Patterns

**ALL of these are BANNED - use grid variables instead:**

```tsx
// Padding
p-1, p-2, p-3, p-4, p-5, p-6, p-8, p-10, p-12, p-16
px-1, px-2, px-3, px-4, py-1, py-2, py-3, py-4
pt-1, pt-2, pt-4, pt-6, pt-8, pb-1, pb-2, pb-4, pb-6, pb-8

// Margin
m-1, m-2, m-3, m-4, m-5, m-6, m-8, m-10, m-12, m-16
mx-1, mx-2, mx-4, my-1, my-2, my-4
mt-1, mt-2, mt-4, mt-8, mt-12, mt-16
mb-1, mb-2, mb-4, mb-8, mb-12, mb-16

// Gap
gap-1, gap-2, gap-3, gap-4, gap-5, gap-6, gap-8, gap-10, gap-12

// Space
space-y-1, space-y-2, space-y-4, space-y-6, space-y-8, space-y-12, space-y-16
space-x-1, space-x-2, space-x-4, space-x-6, space-x-8
```

---

## Audit Commands

Find hardcoded spacing violations:

```bash
# Find all padding/margin violations
grep -rn "\s[pm][tblrxy]?-[0-9]" src/components src/app | grep -v "var(--" | head -50

# Find gap violations
grep -rn "gap-[0-9]" src/components src/app | grep -v "var(--" | head -50

# Find space-y/x violations
grep -rn "space-[xy]-[0-9]" src/components src/app | grep -v "var(--" | head -50

# Count total violations
grep -rn "\s[pm][tblrxy]?-[0-9]\|gap-[0-9]\|space-[xy]-[0-9]" src/ | grep -v "var(--" | wc -l
```

---

## Migration Guide

When fixing hardcoded spacing:

| Old (Tailwind) | New (Grid Variable) |
|----------------|---------------------|
| `p-1` | `p-[var(--grid-1)]` |
| `p-2`, `gap-2` | `p-[var(--grid-2)]`, `gap-[var(--grid-2)]` |
| `p-4`, `gap-4`, `mt-4` | Use `--grid-4` |
| `p-6`, `pt-6` | Use `--grid-6` |
| `p-8`, `gap-8`, `mt-8` | Use `--grid-8` |
| `mt-12`, `space-y-12` | Use `--grid-12` |
| `mt-16`, `space-y-16` | Use `--grid-16` |

---

## Checklist

Before submitting code:

- [ ] ALL spacing uses `--grid-*` CSS variables
- [ ] NO hardcoded Tailwind spacing (p-4, gap-2, mt-8, etc.)
- [ ] Swiss Layout Hierarchy followed
- [ ] Horizontal padding uses `ch` units where appropriate
- [ ] Heights use grid values (--grid-4, --grid-6, --grid-8)
