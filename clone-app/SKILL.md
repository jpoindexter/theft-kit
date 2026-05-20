---
name: clone-app
description: "Reverse-engineer and rebuild mobile, desktop, or web apps as pixel-perfect Next.js clones from screenshots or a URL."
argument-hint: "<screenshots-folder | url> [ios|android|desktop|web]"
user-invocable: true
---

# Clone App

You are about to reverse-engineer **$ARGUMENTS** into a pixel-perfect Next.js clone.

App cloning differs fundamentally from website cloning: there is no live DOM to inspect. Extraction is vision-based — you analyze screenshots to infer design tokens, component structure, and interaction patterns. Be meticulous. A screenshot is the only ground truth you have.

## Argument Parsing

Parse `$ARGUMENTS` as:
1. A **folder path** containing app screenshots (e.g. `~/screenshots/myapp/`) — the primary input for native apps
2. A **URL** — treated as a web app; use browser MCP exactly like `/clone-website` but with mobile viewport emphasis
3. An optional **app type hint**: `ios`, `android`, `desktop`, or `web` (affects layout patterns to apply)

If screenshots are provided, read every image in the folder before doing anything else. If a URL is provided and no browser MCP is available, ask the user to provide screenshots instead.

## Scope Defaults

- **Fidelity level:** Pixel-perfect — exact match in colors, spacing, typography, visual patterns
- **Output:** Mobile-first Next.js (App Router, Tailwind v4, shadcn/ui)
- **In scope:** All screens provided, navigation patterns, component states visible in screenshots, inferred interactions
- **Out of scope:** Real backend, auth, push notifications, native device APIs, animations not evidenced in screenshots
- **Customization:** None — pure visual emulation

## Pre-Flight

1. If a folder path was given, list all image files in it. Read each screenshot. Note the total count and group them by apparent screen/flow.
2. If a URL was given, check for browser MCP. If available, navigate and take screenshots at 390px (mobile) and optionally 768px (tablet). If not available, ask the user for screenshots.
3. Verify the base project builds: `npm run build`.
4. Create output directories: `docs/research/`, `docs/research/screens/`, `docs/research/components/`, `docs/design-references/`.

## Guiding Principles

### 1. Screenshots Are the Contract

You have no `getComputedStyle()`. Every design decision — colors, spacing, typography, corner radius — must be inferred from the screenshots and committed to spec files. "It looks like 16px padding" is your extraction. Be precise about what you observe and what you're inferring vs. what you know exactly.

### 2. Identify the App Shell First

Before touching any individual screen, identify the persistent shell:
- **iOS pattern:** Status bar, navigation bar (back + title + actions), bottom tab bar
- **Android pattern:** Status bar, top app bar, bottom nav or navigation drawer
- **Desktop pattern:** Sidebar nav, top toolbar, content area
- **Web app pattern:** Header, sidebar, main content, optional footer

The shell is built once, shared everywhere. It goes first.

### 3. Screen Inventory Before Building

Map every screenshot to a named screen. Document the flow between them (what tap/click leads to what). This is your routing plan for the Next.js app.

### 4. Vision Extraction Is Your getComputedStyle

For each screen/component, analyze the screenshot and extract:
- **Colors** — background, text, accent, border, icon colors. Use eyedropper logic: what is the dominant bg? What color are the primary action elements? What is the text color?
- **Typography** — relative sizes (heading vs body vs caption). Note weight (bold vs regular). Estimate line height from visual density.
- **Spacing** — padding inside cards, gap between list items, margins from edges. Estimate in multiples of 4px.
- **Corner radius** — sharp (0), slight (4px), medium (8-12px), pill (full).
- **Shadows/elevation** — flat (no shadow), card (soft shadow), modal (stronger shadow).
- **Component patterns** — what is this element? A list row? A card? A chip? A bottom sheet?

Document every inferred value with a confidence note: `(exact)`, `(estimated)`, or `(inferred from context)`.

### 5. Interaction Model From Screenshot Evidence

You can infer interaction patterns even without live behavior:
- A **list of items** with a `>` chevron → tap to navigate to detail view
- A **bottom sheet** partially visible → swipe to expand/dismiss
- **Segmented control / tab bar** → tap to switch content panels
- **Search bar** at the top → tap to focus, keyboard appears
- **Pull-to-refresh** indicator → scroll gesture
- **Floating action button** → primary create action
- **Swipe-to-delete** hint (partially revealed red button) → swipe left gesture

Document each inferred interaction in the screen spec. Build the web equivalent: tap → click, swipe → keyboard shortcut or button, pull-to-refresh → refresh button.

## Phase 1: Screen Inventory

Read all screenshots. For each one:

1. **Name it** — give it a functional name: `HomeScreen`, `DetailScreen`, `ProfileScreen`, `SettingsScreen`, etc.
2. **Classify it** — is it a list, detail, form, modal/sheet, onboarding, empty state, loading state?
3. **Note the shell** — which persistent elements appear (tab bar, nav bar, header)?
4. **Note any state** — is this a logged-in state? Empty state? Error state? Loading?
5. **Note the flow context** — is there a back button suggesting this was navigated to? A "Done" button suggesting it's modal?

Save the full inventory to `docs/research/SCREEN_INVENTORY.md`:

```markdown
# Screen Inventory

## App Shell
- Navigation pattern: [ios-nav-bar | android-top-bar | bottom-tabs | sidebar | none]
- Tab bar screens: [list of tab names if visible]
- Persistent header: [description]

## Screens

### HomeScreen (screenshot: home.png)
- Type: list
- Shell: nav bar (title: "Home"), bottom tab bar (4 tabs: Home, Search, Inbox, Profile)
- State: logged-in, has content
- Flow: tapping a list item → DetailScreen

### DetailScreen (screenshot: detail.png)
- Type: detail
- Shell: nav bar (back button, title, share button)
- State: content loaded
- Flow: back → HomeScreen

[... etc for every screenshot ...]
```

## Phase 2: Design Token Extraction

From the full screenshot set, extract the app's design language. Save to `docs/research/DESIGN_TOKENS.md`.

### Color Extraction

Analyze the screenshots and document:
```markdown
## Colors
- Background (primary): #FFFFFF (estimated — main screen bg)
- Background (secondary): #F2F2F7 (estimated — grouped list bg, iOS gray)
- Text (primary): #000000 (estimated — main content text)
- Text (secondary): #8E8E93 (estimated — subtitles, captions)
- Accent (primary): #007AFF (estimated — buttons, links, active tabs)
- Destructive: #FF3B30 (estimated — delete, error states)
- Border/separator: #C6C6C8 (estimated — list row separators)
- Nav bar background: #F9F9F9 or blur (if frosted glass effect)
```

Map these to `globals.css` CSS custom properties and update `:root` and `.dark`.

### Typography

```markdown
## Typography
- Display/Hero: ~34px bold (title screens)
- Headline: ~28px bold (screen titles in nav bar)
- Title 1: ~22px bold (section headers)
- Title 2: ~20px semibold
- Body: ~17px regular (main content)
- Callout: ~16px regular
- Subheadline: ~15px regular
- Caption: ~13px regular (secondary info, timestamps)
- Footnote: ~12px regular (fine print)

Font family: [system-ui for iOS/Android clones unless a custom font is clearly visible]
```

### Spacing Scale

```markdown
## Spacing
- XS: 4px (icon-to-text gaps)
- SM: 8px (component padding)
- MD: 16px (standard content padding)
- LG: 24px (section gaps)
- XL: 32px (screen-level padding)

- List row height: ~44px (iOS standard) or ~56px (Material Design standard)
- Tab bar height: ~83px (iOS with home indicator) or ~56px (Android)
- Nav bar height: ~44px + status bar
- Card corner radius: [estimated from screenshots]
```

### Component Catalog

From all screenshots, list every distinct UI component you observe:
- Navigation components (nav bar, tab bar, drawer)
- Content components (list row, card, grid cell, section header)
- Control components (button, chip, toggle, segmented control, search bar)
- Feedback components (badge, loading indicator, empty state, error state)
- Overlay components (modal, bottom sheet, action sheet, tooltip)

## Phase 3: Component Specification & Dispatch

For each screen + each distinct component, write a spec and dispatch a builder.

### Next.js Routing Plan

Map each app screen to a Next.js route:
- `HomeScreen` → `src/app/page.tsx`
- `DetailScreen` → `src/app/[id]/page.tsx`
- `ProfileScreen` → `src/app/profile/page.tsx`
- `SettingsScreen` → `src/app/settings/page.tsx`
- **App shell** → `src/app/layout.tsx` (persistent nav, tab bar)
- Modal/sheet screens → either route-level (parallel routes) or component-level (`<Dialog>`)

### App Shell First

Before any screen, build the shell. This is always the first builder dispatch:
- For iOS/Android: bottom tab bar + nav bar + status bar area
- For desktop: sidebar + top toolbar
- Shell goes in `src/app/layout.tsx` or `src/components/AppShell.tsx`

The shell builder receives:
- All screenshots (to see the shell across multiple contexts)
- The full design token doc
- A mapping of tab names → routes

### Per-Screen Spec Template

```markdown
# [ScreenName] Specification

## Overview
- **Route:** `src/app/[route]/page.tsx`
- **Screenshot:** `docs/design-references/[filename]`
- **Type:** list | detail | form | modal | onboarding | settings
- **Shell:** uses AppShell (nav bar title: "[title]", back button: [yes/no], right actions: [list])

## Layout
[Describe the visual layout top to bottom: what's in the safe area, what's in the content area, how it scrolls]

## Components Used
[List every component visible: list rows, cards, section headers, buttons, etc.]

## Design Values (extracted from screenshot)
- Background: [color]
- Content padding: [estimated value]
- Item spacing: [estimated value]
- [any other values specific to this screen]

## Component Specs

### [ComponentName] (e.g., "User List Row")
- Height: ~44px (estimated)
- Layout: leading avatar (40px circle) | title + subtitle stack | trailing chevron
- Title: [font size] [weight] [color]
- Subtitle: [font size] [weight] [color]
- Separator: 1px solid [color], inset 72px from left
- Hover/tap state: background tint [color]
- Web equivalent: `<button>` or `<Link>` with full-width row layout

### [Another component...]

## Interaction Model
- Scroll: [vertical list scroll | no scroll | horizontal scroll]
- Tappable elements: [list with what each tap does]
- Navigation: [where taps navigate]
- Web equivalents: [how each native gesture maps to web]

## Content (verbatim from screenshots)
[All visible text, copy-pasted exactly as it appears]

## Mobile Responsive
- This screen is designed for mobile. Tablet (768px+): [how it should adapt — wider content area, potential 2-column list]
- Desktop (1024px+): [how it should adapt — sidebar for list, main for detail, etc.]
```

### Builder Dispatch

Each builder receives:
- The full screen spec inline (never "go read the spec file")
- The design tokens doc inline
- Instruction to use `cn()`, Tailwind v4 utilities, shadcn primitives
- Instruction to verify `npx tsc --noEmit` before finishing
- Mobile-first breakpoint guidance: build for 390px first, then add `md:` and `lg:` variants

Build order:
1. Design tokens → `globals.css` (do this yourself, no dispatch)
2. App shell → `src/app/layout.tsx` or `src/components/AppShell.tsx`
3. Shared components (list row, card, etc.) — parallel dispatch
4. Individual screens — parallel dispatch (depend only on shared components)

## Phase 4: Assembly

Wire up the Next.js app:
- Import all screen components into their routes
- Set up navigation: `<Link>` for tab bar navigation, `next/navigation` for programmatic nav
- Add mock data in `src/data/` to populate lists and detail views with realistic content from the screenshots
- Implement bottom tab bar highlighting based on `usePathname()`
- Implement nav bar back button using `useRouter().back()`
- Verify: `npm run build` passes clean

## Phase 5: Visual QA

Compare each screenshot against the rendered Next.js route:
- Open the route in a mobile viewport (390px wide, Chrome DevTools device mode)
- Screenshot and compare side by side
- Fix discrepancies: wrong color, wrong spacing, wrong component layout
- Check tablet and desktop adaptations make sense

Only after this QA pass is the clone complete.

## Platform-Specific Notes

### iOS Apps
- Use `font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif`
- Bottom tab bar: 5 tabs max, icons + labels, active tab uses accent color
- Navigation bar: large title style (collapsing on scroll) or standard fixed title
- List style: grouped (rounded sections on gray bg) or plain (full-width on white)
- Preferred corner radius: 10px (cards), 13px (large cards), pill (chips)
- Status bar height: `env(safe-area-inset-top)` — use `pt-safe` equivalent

### Android / Material Design Apps
- Use `font-family: 'Google Sans', Roboto, sans-serif`
- Bottom navigation: 3-5 destinations, icon only or icon + label
- Top app bar: centered or left-aligned title, navigation icon, action icons
- Cards: 12px corner radius, surface color with slight elevation shadow
- FAB: fixed bottom-right, 56px circle, accent color with shadow

### Desktop Apps
- Sidebar navigation: fixed left panel, icon + label rows, collapsible
- Content area: main scrollable region right of sidebar
- Toolbars: horizontal strip of actions at top
- Dense data: tables, smaller spacing than mobile equivalents

## Completion Report

When done, report:
- Total screens built
- Total routes created
- Total shared components
- Design tokens mapped to CSS custom properties
- Mock data files created
- Build status
- Visual QA results and any remaining gaps
