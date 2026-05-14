---
name: vertical-grid
description: Enforce Swiss-Industrial 4px baseline vertical grid for consistent rhythm
license: MIT
compatibility: Claude Code
metadata:
  version: "1.0"
allowed-tools:
  - Read
  - Grep
  - Glob
  - Edit
---

# Vertical Grid Skill (4px Baseline)

## When to use this skill
Use when creating or auditing UI components to ensure they follow the Swiss-Industrial 4px baseline grid. Invoke this skill when:
- Creating new components with spacing
- Auditing existing components for vertical rhythm
- Setting line-heights, margins, padding, or heights
- Ensuring mathematical consistency across the page

## Core Philosophy

The **4px Baseline Grid** creates "Vertical Rhythm" - every vertical measurement must be a multiple of **4px**. This ensures the page feels mathematically "locked in" rather than floating.

```
┌────────────────────────────────────────┐
│  4px │ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │  │
│      ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼ ▼  │
│ ┌──────────────────────────────────┐   │
│ │  HEADLINE (32px height)          │   │
│ └──────────────────────────────────┘   │
│ ▼ 16px gap                             │
│ ┌──────────────────────────────────┐   │
│ │  Body text (24px line-height)    │   │
│ │  Body text (24px line-height)    │   │
│ └──────────────────────────────────┘   │
│ ▼ 32px section gap                     │
│ ┌──────────────────────────────────┐   │
│ │  Button (32px height)            │   │
│ └──────────────────────────────────┘   │
└────────────────────────────────────────┘
```

## The Vertical Spacing Scale

**ONLY use `--grid-*` CSS variables. All must be multiples of 4.**

| Token | Value | Use Case |
|-------|-------|----------|
| `--grid-0` | 0 | None |
| `--grid-1` | 4px | Micro-spacing (Icon to Text) |
| `--grid-2` | 8px | Tight grouping (Label to Input) |
| `--grid-3` | 12px | ⚠️ Use sparingly |
| `--grid-4` | 16px | Standard grouping (Title to Subtitle) |
| `--grid-6` | 24px | Medium spacing |
| `--grid-8` | 32px | Component spacing (Between input rows) |
| `--grid-12` | 48px | Large section spacing |
| `--grid-16` | 64px | Section spacing (Between major sections) |

### ⚠️ Banned Patterns

**ALL hardcoded Tailwind spacing is BANNED:**

```tsx
// ❌ BANNED - Hardcoded Tailwind spacing
p-1, p-2, p-3, p-4, p-5, p-6, p-8, p-10, p-12, p-16
gap-1, gap-2, gap-3, gap-4, gap-5, gap-6, gap-8
m-1, m-2, m-3, m-4, m-5, m-6, m-8, m-10, m-12
pt-4, pb-4, mt-8, mb-8, space-y-4, space-y-8

// ✅ GOOD - Grid variables
p-[var(--grid-1)], p-[var(--grid-2)], p-[var(--grid-4)]
gap-[var(--grid-2)], gap-[var(--grid-4)], gap-[var(--grid-8)]
mt-[var(--grid-4)], mb-[var(--grid-8)], space-y-[var(--grid-4)]
```

## Line-Height (Leading) - The Grid Master

Line heights MUST align to the 4px scale. This is the most common mistake.

| Element | Line-Height | Tailwind | Notes |
|---------|-------------|----------|-------|
| **Headlines** | 1.0-1.1 | `leading-none` or `leading-tight` | Dense, machined feel |
| **Body Prose** | 24px | `leading-6` | Every line sits on baseline |
| **Body Alt** | 28px | `leading-7` | Looser reading |
| **Labels/Mono** | 16px | `leading-4` | Compact labels |
| **Captions** | 20px | `leading-5` | Small text readability |

```tsx
// ❌ BAD - Default line-heights break the grid
<p className="text-base">  // Uses browser default ~22px

// ✅ GOOD - Explicit line-height on the grid
<p className="text-base leading-6">  // 24px line-height
```

## Component Heights - The 32px Rule

**32px (`--grid-8`)** is the Swiss-Industrial standard height for interactive elements.

Why 32px?
- It's a multiple of 4 and 8
- Stacking 3 × 32px rows with 16px gaps = 128px (perfectly aligned)
- Touch-friendly but not bloated

| Component | Height | Token |
|-----------|--------|-------|
| Buttons (default) | 32px | `h-[var(--grid-8)]` |
| Inputs (default) | 32px | `h-[var(--grid-8)]` |
| Dropdown triggers | 32px | `h-[var(--grid-8)]` |
| Table rows (compact) | 24px | `h-[var(--grid-6)]` |
| Table rows (comfortable) | 32px | `h-[var(--grid-8)]` |
| Nav items | 24px | `h-[var(--grid-6)]` |

```tsx
// ❌ BAD - Arbitrary heights or hardcoded Tailwind
<button className="py-2.5">  // Results in ~36px
<input className="h-9">      // 36px - not on scale
<button className="h-8">     // Hardcoded - use grid variable

// ✅ GOOD - Grid variables
<button className="h-[var(--grid-8)]">     // 32px
<input className="h-[var(--grid-8)]">      // 32px
<nav className="h-[var(--grid-6)]">        // 24px nav items
```

## The Asymmetric Break Rule

In Swiss design, space **above** a heading is LARGER than space **below**. This "anchors" the heading to its content.

```tsx
// ❌ BAD - Equal spacing or hardcoded values
<section className="mt-8 mb-8">
  <h2>Heading</h2>
</section>

// ✅ GOOD - Asymmetric with grid tokens
<section className="mt-[var(--grid-16)] mb-[var(--grid-6)]">
  <h2>Heading</h2>
</section>

// Page sections
<section className="mt-[var(--grid-16)] mb-[var(--grid-4)]">
  <h2>Heading</h2>
  <p>Content anchored to heading</p>
</section>
```

### Standard Asymmetric Ratios

| Element | Top Margin | Bottom Margin |
|---------|------------|---------------|
| Page sections | `mt-[var(--grid-16)]` (64px) | `mb-[var(--grid-6)]` (24px) |
| Card headers | `mt-0` | `mb-[var(--grid-4)]` (16px) |
| Subsections | `mt-[var(--grid-12)]` (48px) | `mb-[var(--grid-4)]` (16px) |
| Paragraphs | `mt-0` | `mb-[var(--grid-4)]` (16px) |

## Audit Checklist

When auditing a component for vertical grid compliance:

- [ ] ALL spacing uses `--grid-*` CSS variables
- [ ] NO hardcoded Tailwind spacing (p-4, gap-2, mt-8, etc.)
- [ ] Heights use grid variables (h-[var(--grid-6)], h-[var(--grid-8)])
- [ ] Headings use asymmetric spacing (more above, less below)
- [ ] No arbitrary pixel values like `h-[33px]` or `p-[13px]`
- [ ] Swiss Layout Hierarchy followed

## Grep Patterns for Violations

```bash
# Find ALL hardcoded spacing violations
grep -rn "\s[pm][tblrxy]?-[0-9]" src/components src/app | grep -v "var(--" | head -50

# Find gap violations
grep -rn "gap-[0-9]" src/components src/app | grep -v "var(--" | head -50

# Find space-y/x violations
grep -rn "space-[xy]-[0-9]" src/components src/app | grep -v "var(--" | head -50

# Find off-grid arbitrary values
grep -rn "h-\[.*[13579]px\]" src/
grep -rn "p-\[.*[13579]px\]" src/
grep -rn "m-\[.*[13579]px\]" src/
grep -rn "gap-\[.*[13579]px\]" src/

# Count total violations
grep -rn "\s[pm][tblrxy]?-[0-9]\|gap-[0-9]\|space-[xy]-[0-9]" src/ | grep -v "var(--" | wc -l
```

## Output Format

When reporting violations:

```
VERTICAL GRID AUDIT RESULTS
============================

VIOLATIONS FOUND: X

[SPACING] src/components/ui/button.tsx:45
  Found: p-3 (12px)
  Fix: Use p-2 (8px) or p-4 (16px)

[GAP] src/components/layout/header.tsx:23
  Found: gap-5 (20px)
  Fix: Use gap-4 (16px) or gap-6 (24px)

[HEIGHT] src/components/ui/input.tsx:67
  Found: h-9 (36px)
  Fix: Use h-8 (32px) or h-10 (40px)

[LINE-HEIGHT] src/components/card.tsx:89
  Found: text-base without leading-*
  Fix: Add leading-6 for 24px line-height

[ASYMMETRY] src/components/section.tsx:12
  Found: mt-8 mb-8 (symmetric spacing)
  Fix: Use mt-16 mb-6 (asymmetric, anchors content)
```

## Quick Reference Card

```
┌─────────────────────────────────────────────────────────────────┐
│  SWISS-INDUSTRIAL 4px GRID - QUICK REFERENCE                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  SPACING SCALE (use --grid-* CSS variables)                     │
│  ───────────────────────────────────────────                    │
│  --grid-1  →  4px   Micro: icon padding                        │
│  --grid-2  →  8px   Small: related items                       │
│  --grid-4  →  16px  Standard: 1 line-height ★                  │
│  --grid-6  →  24px  Medium: section headers                    │
│  --grid-8  →  32px  Large: between sections ★                  │
│  --grid-12 →  48px  XL: page sections                          │
│  --grid-16 →  64px  XXL: major divisions ★                     │
│                                                                 │
│  USAGE:                                                         │
│  p-[var(--grid-4)], gap-[var(--grid-2)], mt-[var(--grid-8)]    │
│                                                                 │
│  BANNED: p-4, gap-2, mt-8, space-y-4 (ALL hardcoded spacing)   │
│                                                                 │
│  COMPONENT HEIGHTS                                              │
│  ─────────────────                                              │
│  Nav items       →  h-[var(--grid-6)]  (24px)                  │
│  Buttons/Inputs  →  h-[var(--grid-8)]  (32px)                  │
│                                                                 │
│  ASYMMETRIC SPACING (headings)                                  │
│  ─────────────────────────────                                  │
│  Sections    →  mt-[var(--grid-16)] mb-[var(--grid-6)]         │
│  Subsections →  mt-[var(--grid-12)] mb-[var(--grid-4)]         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## CSS Debug Mode

Add this to your browser console to visualize the 4px grid:

```javascript
// Paste in browser console to see 4px grid overlay
const style = document.createElement('style');
style.textContent = `
  body {
    background-image: linear-gradient(
      to bottom,
      rgba(255, 0, 0, 0.1) 1px,
      transparent 1px
    );
    background-size: 100% 4px;
  }
`;
document.head.appendChild(style);
```

Or create a React component:

```tsx
// components/debug/grid-overlay.tsx
export function GridOverlay() {
  if (process.env.NODE_ENV !== 'development') return null;

  return (
    <div
      className="fixed inset-0 pointer-events-none z-[9999]"
      style={{
        backgroundImage: 'linear-gradient(to bottom, rgba(255,0,0,0.1) 1px, transparent 1px)',
        backgroundSize: '100% 4px',
      }}
    />
  );
}
```
