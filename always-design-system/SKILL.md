---
name: always-design-system
description: MANDATORY rules - use design system tokens, existing components, and INDX Swiss patterns
license: MIT
compatibility: Claude Code
metadata:
  author: indx.sh
  version: "4.1"
  project: indx-web
  priority: critical
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - Bash
---

# DESIGN SYSTEM - MANDATORY RULES

**Every line of code MUST comply. NO EXCEPTIONS.**

---

## SOURCE OF TRUTH

**Before implementing ANY feature, check the specs:**

```bash
# Comprehensive specs for everything
ls docs/specs/

# 94+ specification files:
# - 01-PAGES/      → 48 page specs
# - 02-FEATURES/   → 14 feature specs
# - 03-USER-FLOWS/ → 6 user journey specs
# - 04-ADMIN-FLOWS/→ 4 admin operation specs
# - 05-API/        → 5 API endpoint specs
# - 06-DATA-MODELS/→ 9 database schema specs
# - 07-COMPONENTS/ → 5 component pattern specs
# - 08-HOOKS.md    → ALL 33 hooks with TypeScript signatures
```

**Read the relevant spec BEFORE implementing.**

---

## RULE 0: COMPONENT SOURCE HIERARCHY

**Before creating ANY component:**

### Step 1: Check Local

```bash
ls src/components/ui/
```

### Step 2: Check Fabrk Boilerplate

```bash
ls ../fabrk-dev/src/components/ui/
```

### Step 3: Decision

| Situation | Action |
|-----------|--------|
| Exists in web | Use it |
| Exists in fabrk-dev only | Copy to web, then use |
| Doesn't exist in EITHER | ONLY THEN build custom |

**fabrk-dev is READ-ONLY. Never modify. Only copy FROM it.**

---

## RULE 1: USE EXISTING COMPONENTS

We have **98 UI components**. NEVER build custom UI.

### Core Form Components

| Need | Component | Import |
|------|-----------|--------|
| Button | `Button` | `@/components/ui/button` |
| Input | `Input` | `@/components/ui/input` |
| InputSearch | `InputSearch` | `@/components/ui/input-search` |
| InputPassword | `InputPassword` | `@/components/ui/input-password` |
| InputOTP | `InputOTP` | `@/components/ui/input-otp` |
| InputNumber | `InputNumber` | `@/components/ui/input-number` |
| InputGroup | `InputGroup` | `@/components/ui/input-group` |
| Textarea | `Textarea` | `@/components/ui/textarea` |
| Select | `Select, SelectTrigger, SelectContent, SelectItem` | `@/components/ui/select` |
| Checkbox | `Checkbox` | `@/components/ui/checkbox` |
| Switch | `Switch` | `@/components/ui/switch` |
| RadioGroup | `RadioGroup, RadioGroupItem` | `@/components/ui/radio-group` |
| Slider | `Slider` | `@/components/ui/slider` |
| Label | `Label` | `@/components/ui/label` |
| Form | `Form, FormField, FormItem, FormLabel, FormMessage` | `@/components/ui/form` |
| FormError | `FormError` | `@/components/ui/form-error` |
| TagInput | `TagInput` | `@/components/ui/tag-input` |
| FileUpload | `FileUpload` | `@/components/ui/file-upload` |
| DatePicker | `DatePicker` | `@/components/ui/date-picker` |
| Calendar | `Calendar` | `@/components/ui/calendar` |

### Layout Components

| Need | Component | Import |
|------|-----------|--------|
| Card | `Card, CardHeader, CardContent, CardFooter` | `@/components/ui/card` |
| Container | `Container` | `@/components/ui/container` |
| Tabs | `Tabs, TabsList, TabsTrigger, TabsContent` | `@/components/ui/tabs` |
| StyledTabs | `StyledTabs` | `@/components/ui/styled-tabs` |
| Accordion | `Accordion, AccordionItem, AccordionTrigger, AccordionContent` | `@/components/ui/accordion` |
| Sidebar | `Sidebar, SidebarContent, SidebarGroup` | `@/components/ui/sidebar` |
| ScrollArea | `ScrollArea` | `@/components/ui/scroll-area` |
| Separator | `Separator` | `@/components/ui/separator` |
| Resizable | `ResizablePanelGroup, ResizablePanel, ResizableHandle` | `@/components/ui/resizable` |

### Feedback & Overlay

| Need | Component | Import |
|------|-----------|--------|
| Dialog | `Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle` | `@/components/ui/dialog` |
| Sheet | `Sheet, SheetTrigger, SheetContent, SheetHeader` | `@/components/ui/sheet` |
| Alert | `Alert, AlertTitle, AlertDescription` | `@/components/ui/alert` |
| AlertDialog | `AlertDialog, AlertDialogTrigger, AlertDialogContent, AlertDialogAction` | `@/components/ui/alert-dialog` |
| Toast | `toast()` | `@/components/ui/toast` |
| Toaster | `Toaster` | `@/components/ui/toaster` |
| Tooltip | `Tooltip, TooltipTrigger, TooltipContent` | `@/components/ui/tooltip` |
| InfoTooltip | `InfoTooltip` | `@/components/ui/info-tooltip` |
| Popover | `Popover, PopoverTrigger, PopoverContent` | `@/components/ui/popover` |

### Navigation

| Need | Component | Import |
|------|-----------|--------|
| Breadcrumb | `Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink` | `@/components/ui/breadcrumb` |
| Pagination | `Pagination` | `@/components/ui/pagination` |
| ScrollToTop | `ScrollToTop` | `@/components/ui/scroll-to-top` |

### Data Display

| Need | Component | Import |
|------|-----------|--------|
| Table | `Table, TableHeader, TableBody, TableRow, TableHead, TableCell` | `@/components/ui/table` |
| TableSkeleton | `TableSkeleton` | `@/components/ui/table-skeleton` |
| DataTable | `DataTable` | `@/components/ui/data-table` |
| Badge | `Badge` | `@/components/ui/badge` |
| Avatar | `Avatar, AvatarImage, AvatarFallback` | `@/components/ui/avatar` |
| Progress | `Progress` | `@/components/ui/progress` |
| Skeleton | `Skeleton` | `@/components/ui/skeleton` |
| EmptyState | `EmptyState` | `@/components/ui/empty-state` |
| CodeBlock | `CodeBlock` | `@/components/ui/code-block` |
| KPICard | `KPICard` | `@/components/ui/kpi-card` |
| StatCard | `StatCard` | `@/components/ui/stat-card` |
| PricingCard | `PricingCard` | `@/components/ui/pricing-card` |
| BillingSummaryCard | `BillingSummaryCard` | `@/components/ui/billing-summary-card` |
| MemberCard | `MemberCard` | `@/components/ui/member-card` |

### Menus

| Need | Component | Import |
|------|-----------|--------|
| DropdownMenu | `DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem` | `@/components/ui/dropdown-menu` |
| ContextMenu | `ContextMenu, ContextMenuTrigger, ContextMenuContent` | `@/components/ui/context-menu` |
| Command | `Command, CommandInput, CommandList, CommandItem` | `@/components/ui/command` |

### Charts

| Need | Component | Import |
|------|-----------|--------|
| AreaChart | `AreaChart` | `@/components/ui/area-chart` |
| BarChart | `BarChart` | `@/components/ui/bar-chart` |
| LineChart | `LineChart` | `@/components/ui/line-chart` |
| PieChart | `PieChart` | `@/components/ui/pie-chart` |
| DonutChart | `DonutChart` | `@/components/ui/donut-chart` |
| FunnelChart | `FunnelChart` | `@/components/ui/funnel-chart` |
| Heatmap | `Heatmap` | `@/components/ui/heatmap` |
| Sparkline | `Sparkline` | `@/components/ui/sparkline` |
| Gauge | `Gauge` | `@/components/ui/gauge` |

### Special Components

| Need | Component | Import |
|------|-----------|--------|
| Loading | `Loading` | `@/components/ui/loading` |
| TerminalSpinner | `TerminalSpinner` | `@/components/ui/terminal-spinner` |
| Marquee | `Marquee` | `@/components/ui/marquee` |
| Typewriter | `Typewriter` | `@/components/ui/typewriter` |
| Icon | `Icon` | `@/components/ui/icon` |
| SimpleIcon | `SimpleIcon` | `@/components/ui/simple-icon` |
| CookieConsent | `CookieConsent` | `@/components/ui/cookie-consent` |
| KeyboardHelpModal | `KeyboardHelpModal` | `@/components/ui/keyboard-help-modal` |
| NotificationBadge | `NotificationBadge` | `@/components/ui/notification-badge` |
| NotificationCenter | `NotificationCenter` | `@/components/ui/notification-center` |
| NotificationList | `NotificationList` | `@/components/ui/notification-list` |
| OnboardingChecklist | `OnboardingChecklist` | `@/components/ui/onboarding-checklist` |
| ApiKeyManager | `ApiKeyManager` | `@/components/ui/api-key-manager` |
| AuditLog | `AuditLog` | `@/components/ui/audit-log` |
| WebhookLog | `WebhookLog` | `@/components/ui/webhook-log` |
| SignInForm | `SignInForm` | `@/components/ui/sign-in-form` |

### Templates (USE FOR PAGES)

| Need | Template | Import |
|------|----------|--------|
| Form pages | `FormPageTemplate` | `@/components/templates/form-page-template` |
| Directory listings | `DirectoryListingTemplate` | `@/components/templates/directory-listing-template` |
| Directory detail | `DirectoryDetailTemplate` | `@/components/templates/directory-detail-template` |
| Directory table | `DirectoryTableTemplate` | `@/components/templates/directory-table-template` |
| Admin pages | `AdminPageTemplate` | `@/components/templates/admin-page-template` |
| Auth pages | `AuthPageTemplate` | `@/components/templates/auth-page-template` |
| Auth form field | `AuthFormField` | `@/components/templates/auth-form-field` |
| Settings pages | `SettingsPageTemplate` | `@/components/templates/settings-page-template` |
| User dashboard | `UserDashboardTemplate` | `@/components/templates/user-dashboard-template` |
| Marketing pages | `MarketingPageTemplate` | `@/components/templates/marketing-page-template` |
| Landing pages | `CommunityLandingTemplate` | `@/components/templates/community-landing-template` |
| Docs hub | `DocsHubTemplate` | `@/components/templates/docs-hub-template` |
| Docs article | `DocsArticleTemplate` | `@/components/templates/docs-article-template` |
| Learn course | `LearnCourseTemplate` | `@/components/templates/learn-course-template` |
| Legal pages | `LegalPageTemplate` | `@/components/templates/legal-page-template` |
| Error pages | `ErrorPageTemplate` | `@/components/templates/error-page-template` |
| Success pages | `SuccessPageTemplate` | `@/components/templates/success-page-template` |
| TE Directory | `TEDirectoryTemplate` | `@/components/templates/te-directory-template` |

---

## RULE 2: USE HOOKS

**See `docs/specs/08-HOOKS.md` for complete signatures.**

### Data Fetching Hooks

```tsx
import {
  useRulesData,        // Fetch rules with filters
  useMcpData,          // Fetch MCP servers
  useToolsData,        // Fetch tools
  useSkillsData,       // Fetch skills
  useJobsData,         // Fetch job listings
  useFeaturedItems,    // Fetch homepage featured content
  useDirectoryStats,   // Fetch content counts
  useSidebarStats,     // Formatted sidebar stats
  useRegistryData,     // Registry data
} from '@/hooks';
```

### Form Hooks

```tsx
import {
  useRuleSubmitForm,   // Rule submission
  useRuleEditForm,     // Rule editing
  useMcpSubmitForm,    // MCP submission
  useMcpEditForm,      // MCP editing
  useToolSubmitForm,   // Tool submission
  useToolEditForm,     // Tool editing
  useSkillSubmitForm,  // Skill submission
  useSkillEditForm,    // Skill editing
  useJobPostForm,      // Job posting with payment
  useJobEditForm,      // Job editing
  useInlineEdit,       // Inline field editing
} from '@/hooks';
```

### UI State Hooks

```tsx
import {
  useToast,            // Toast notifications
  useAuthModal,        // Auth modal state
  usePalette,          // Theme palette
  useMediaQuery,       // Responsive breakpoints
  useDataTableState,   // TanStack Table state
  useColumnResize,     // Table column resizing
  useKeyboardShortcuts,// Global keyboard shortcuts
  useListKeyboardNav,  // List keyboard navigation
} from '@/hooks';
```

### Action Hooks

```tsx
import {
  useBookmark,         // Single item bookmark
  useBulkBookmarks,    // Bulk bookmark operations
  useCheckout,         // Polar payment checkout
  useModeration,       // Admin moderation actions
  useCsrfToken,        // CSRF token for mutations
  useAnalytics,        // Event tracking
  useNotifications,    // Real-time notifications
  usePusher,           // Real-time events
  useSearch,           // Global search
  useRecentSearches,   // Search history
  useAutocomplete,     // Search suggestions
} from '@/hooks';
```

### Usage Examples

```tsx
// Toast
const { toast } = useToast();
toast({ title: 'SUCCESS', description: 'Saved.' });
toast({ title: 'ERROR', description: 'Failed.', variant: 'destructive' });

// CSRF Fetch
const csrfFetch = useCsrfToken();
await csrfFetch('/api/endpoint', {
  method: 'POST',
  body: JSON.stringify(data),
});

// Moderation
const { approve, reject } = useModeration();
await approve('rule', ruleId);
```

---

## RULE 3: DESIGN TOKENS

### Required Imports

```tsx
import { mode } from '@/design-system';
import { cn } from '@/lib/utils';
```

### Color Tokens

```tsx
// Backgrounds
mode.color.bg.base            // Page background
mode.color.bg.surface         // Card/panel
mode.color.bg.surfaceRaised   // Raised surface
mode.color.bg.elevated        // Popover
mode.color.bg.muted           // Subtle
mode.color.bg.mutedSubtle     // 20% opacity
mode.color.bg.mutedLight      // 30% opacity
mode.color.bg.mutedMedium     // 50% opacity
mode.color.bg.secondary       // Secondary
mode.color.bg.accent          // Accent
mode.color.bg.accentMuted     // Accent 10%
mode.color.bg.accentHover     // Accent 90%
mode.color.bg.primarySubtle   // Primary 5%
mode.color.bg.primaryLight    // Primary 10%
mode.color.bg.danger          // Error solid
mode.color.bg.dangerMuted     // Error 10%
mode.color.bg.success         // Success solid
mode.color.bg.successMuted    // Success 10%
mode.color.bg.successSubtle   // Success 20%
mode.color.bg.warning         // Warning solid
mode.color.bg.warningMuted    // Warning 10%
mode.color.bg.info            // Info solid
mode.color.bg.infoMuted       // Info 10%

// Text
mode.color.text.primary       // Main text
mode.color.text.secondary     // Secondary text
mode.color.text.muted         // Muted text
mode.color.text.inverse       // On dark backgrounds
mode.color.text.disabled      // Disabled (38% opacity)
mode.color.text.accent        // Accent color
mode.color.text.danger        // Error text
mode.color.text.dangerOnColor // White on danger bg
mode.color.text.success       // Success text
mode.color.text.successOnColor // White on success bg
mode.color.text.warning       // Warning text
mode.color.text.warningOnColor // White on warning bg
mode.color.text.info          // Info text
mode.color.text.infoOnColor   // White on info bg

// Borders
mode.color.border.default     // Default border
mode.color.border.divider     // Divider line
mode.color.border.focus       // Focus ring
mode.color.border.active      // Active state
mode.color.border.accent      // Accent border
mode.color.border.accentSubtle // Accent 30%
mode.color.border.mutedSubtle // Muted 30%
mode.color.border.danger      // Error border
mode.color.border.success     // Success border
mode.color.border.warning     // Warning border
mode.color.border.info        // Info border

// Icons
mode.color.icon.primary       // Primary icon
mode.color.icon.secondary     // Secondary icon
mode.color.icon.muted         // Muted icon
mode.color.icon.accent        // Accent icon
mode.color.icon.danger        // Error icon
mode.color.icon.success       // Success icon
mode.color.icon.warning       // Warning icon
mode.color.icon.info          // Info icon

// Badge (compound tokens)
mode.color.badge.success.border
mode.color.badge.success.bg
mode.color.badge.success.text
mode.color.badge.warning.border
mode.color.badge.warning.bg
mode.color.badge.warning.text
mode.color.badge.error.border
mode.color.badge.error.bg
mode.color.badge.error.text
mode.color.badge.info.border
mode.color.badge.info.bg
mode.color.badge.info.text
mode.color.badge.neutral.border
mode.color.badge.neutral.bg
mode.color.badge.neutral.text
mode.color.badge.accent.border
mode.color.badge.accent.bg
mode.color.badge.accent.text

// Terminal (syntax highlighting)
mode.color.terminal.magenta        // text-[oklch(var(--terminal-magenta))]
mode.color.terminal.magentaBright  // text-[oklch(var(--terminal-magenta-bright))]
mode.color.terminal.cyan           // text-[oklch(var(--terminal-cyan))]
mode.color.terminal.cyanBright     // text-[oklch(var(--terminal-cyan-bright))]
mode.color.terminal.red            // text-[oklch(var(--terminal-red))]
mode.color.terminal.orange         // text-[oklch(var(--terminal-orange))]
mode.color.terminal.green          // text-success
mode.color.terminal.yellow         // text-warning
mode.color.terminal.blue           // text-info
```

### Typography Tokens

```tsx
// Display - Hero text (57px, 45px, 36px)
mode.typography.display.lg    // Hero headlines
mode.typography.display.md    // Section heroes
mode.typography.display.sm    // Feature headings

// Headline - Titles (32px, 28px, 24px)
mode.typography.headline.lg   // Page titles
mode.typography.headline.md   // Sub-page titles
mode.typography.headline.sm   // Card titles

// Title - Item titles (22px, 16px, 14px)
mode.typography.title.lg      // List headers
mode.typography.title.md      // List item titles
mode.typography.title.sm      // Small card titles

// Body - Content text
mode.typography.body.lg       // Long-form (~16px)
mode.typography.body.md       // Default (~14px)
mode.typography.body.sm       // Captions (~12px)

// Label - UI labels (auto-uppercase)
mode.typography.label.lg      // Form labels (~14px)
mode.typography.label.md      // Tags, badges (~12px)
mode.typography.label.sm      // Small labels (~11px)

// Special
mode.typography.code          // Code/monospace
mode.typography.caps          // Uppercase tracking
mode.typography.button        // Button text
mode.typography.caption       // Caption text
mode.typography.input         // Input text

// Sidebar-specific
mode.typography.sidebar.item   // Sidebar item text
mode.typography.sidebar.header // Sidebar header text

// Legacy aliases (use display/headline/title instead)
mode.typography.heading.h1    // = display.lg
mode.typography.heading.h2    // = display.md
mode.typography.heading.h3    // = display.sm
mode.typography.heading.h4    // = headline.lg
mode.typography.heading.h5    // = headline.md
mode.typography.heading.h6    // = headline.sm
```

### State Tokens (ACTUAL from state-config.ts)

```tsx
// Hover states
mode.state.hover.bg           // hover:bg-foreground/10
mode.state.hover.text         // hover:text-foreground
mode.state.hover.card         // hover:bg-foreground/10
mode.state.hover.cardSubtle   // hover:bg-foreground/5
mode.state.hover.link         // hover:text-primary
mode.state.hover.linkOpacity  // hover:opacity-80
mode.state.hover.listItem     // hover:bg-foreground/10
mode.state.hover.opacity      // hover:brightness-110
mode.state.hover.border       // hover:border-foreground
mode.state.hover.borderWarning // hover:border-warning
mode.state.hover.textWarning  // hover:text-warning
mode.state.hover.borderAccent // hover:border-accent
mode.state.hover.textAccent   // hover:text-accent

// Focus states
mode.state.focus.ring         // focus-visible:outline-2 focus-visible:outline-ring
mode.state.focus.layer        // focus:bg-foreground/[0.12]

// Pressed states
mode.state.pressed.layer      // active:bg-foreground/[0.12]

// Dragged states
mode.state.dragged.layer      // bg-foreground/[0.16]

// Disabled states
mode.state.disabled.opacity   // disabled:opacity-[0.38]
mode.state.disabled.cursor    // disabled:cursor-not-allowed
mode.state.disabled.layer     // disabled:bg-foreground/[0.12]

// Selected states
mode.state.selected.layer     // bg-primary/[0.08]
mode.state.selected.layerHover // hover:bg-primary/[0.12]

// Visual states
mode.state.completed.opacity  // opacity-60
mode.state.muted.opacity      // opacity-50
mode.state.subtle.opacity     // opacity-40
mode.state.secondary.opacity  // opacity-70
```

### Mode Configuration Tokens

```tsx
mode.font            // font-mono
mode.radius          // border radius
mode.shadow          // shadow-sm
mode.borderWidth     // border
mode.buttonPrefix    // "> "
mode.textTransform   // uppercase
mode.inputStyle      // Input styling classes
mode.labelFormat     // 'brackets' | 'plain'
mode.cardHeader      // 'bracketed' | 'simple' | 'minimal'
```

### Font Role Tokens (Swiss Typography)

```tsx
mode.fontRole.ui        // UI elements (buttons, labels)
mode.fontRole.data      // Data display (tables, stats)
mode.fontRole.body      // Body text
mode.fontRole.headline  // Headlines
```

### Icon Sizing

```tsx
mode.icon['2xs']  // 10px (h-2.5 w-2.5)
mode.icon.xs      // 12px (h-3 w-3)
mode.icon.sm      // 14px (h-3.5 w-3.5)
mode.icon.md      // 16px (h-4 w-4) - DEFAULT
mode.icon.lg      // 20px (h-5 w-5)
mode.icon.xl      // 24px (h-6 w-6)
```

### Spacing Tokens

```tsx
// Button (includes flex alignment)
mode.spacing.button.sm   // h-6 px-2 (24px)
mode.spacing.button.md   // h-7 px-2 (28px)
mode.spacing.button.lg   // h-8 px-4 (32px)

// Other
mode.spacing.input       // px-2 py-1
mode.spacing.card        // p-4
mode.spacing.badge.sm    // px-1 py-0
mode.spacing.badge.md    // px-2 py-0.5

// Sidebar
mode.spacing.sidebar.container  // p-4
mode.spacing.sidebar.item       // px-2 py-1
mode.spacing.sidebar.header     // px-2 py-0.5
mode.spacing.sidebar.gap        // gap-1.5
```

### Sizing Tokens

```tsx
mode.sizing.panel              // h-panel
mode.sizing.panelSm            // h-panel-sm
mode.sizing.sidebar            // w-sidebar
mode.sizing.auth               // max-w-auth
mode.sizing.dropdown           // min-w-dropdown
mode.sizing.select             // min-w-select
mode.sizing.dropdownHeight     // max-h-dropdown
mode.sizing.textareaHeight     // max-h-textarea
mode.sizing.touch              // min-h-touch min-w-touch
mode.sizing.buttonHeight.sm    // h-6
mode.sizing.buttonHeight.md    // h-7
mode.sizing.buttonHeight.lg    // h-8
```

### Line Height Tokens

```tsx
mode.lineHeight.display   // leading-[0.85] - Hero headlines (super tight)
mode.lineHeight.compact   // leading-[0.9] - Display text (very tight)
mode.lineHeight.tight     // leading-tight (1.25)
mode.lineHeight.snug      // leading-snug (1.375)
mode.lineHeight.normal    // leading-normal (1.5) - default
mode.lineHeight.relaxed   // leading-relaxed (1.625)
mode.lineHeight.loose     // leading-loose (2.0)
```

### Z-Index Tokens

```tsx
mode.zIndex.banner   // z-banner
mode.zIndex.modal    // z-modal
```

### M3 Tokens (Material Design 3)

For advanced layouts, see `src/design-system/mode/m3-config.ts`:

```tsx
mode.layout          // M3 layout tokens
mode.space           // M3 spacing scale
mode.shape           // M3 shape tokens
mode.elevation       // M3 elevation/shadow
mode.motion          // M3 animation tokens
mode.density         // M3 density variants
mode.iconSpec        // M3 icon specifications
mode.stateLayer      // M3 state layer opacity
mode.accessibility   // M3 a11y tokens
mode.canonicalLayout // M3 canonical layouts
mode.contentDensity  // M3 content density
```

---

## RULE 4: GRID SPACING SYSTEM (MANDATORY)

**ALL spacing MUST use `--grid-*` CSS variables. NO hardcoded Tailwind spacing.**

Based on IBM Carbon's 4px Mini-Unit system for Swiss-Industrial design.

### Grid Spacing Scale

| Token | Value | Use Case |
|-------|-------|----------|
| `--grid-1` | 4px | Micro: icon padding, tight gaps |
| `--grid-2` | 8px | Small: between related items |
| `--grid-3` | 12px | Medium-small: toolbar gaps |
| `--grid-4` | 16px | Standard: 1 line-height |
| `--grid-6` | 24px | Medium: section headers |
| `--grid-8` | 32px | Large: between sections |
| `--grid-12` | 48px | XL: page sections |
| `--grid-16` | 64px | XXL: major divisions |

### Usage

```tsx
// ✅ CORRECT - Grid variables
<div className="pt-[var(--grid-4)] mb-[var(--grid-8)]">
<div className="gap-[var(--grid-2)] space-y-[var(--grid-4)]">
<header className="pb-[var(--grid-4)] mb-[var(--grid-8)]">
<section className="mt-[var(--grid-12)]">

// ❌ BANNED - Hardcoded Tailwind spacing
<div className="pt-4 mb-8">        // NEVER
<div className="gap-2 space-y-4">  // NEVER
<div className="p-3 mt-5">         // NEVER
<section className="mt-12">        // NEVER
```

### Swiss Layout Hierarchy

Standard vertical spacing between page elements:

| Transition | Spacing | Token |
|------------|---------|-------|
| Global Bar → Page Header | 16px | `pt-[var(--grid-4)]` |
| Page Header → Toolbar | 24px | `pb-[var(--grid-6)]` |
| Toolbar → Content | 16px | `mt-[var(--grid-4)]` |
| Section → Section | 32px | `space-y-[var(--grid-8)]` |
| Content → Footer | 48px | `mt-[var(--grid-12)]` |
| Major divisions | 64px | `space-y-[var(--grid-16)]` |

### Horizontal Spacing

Use character widths (`ch`) for horizontal padding:

```tsx
px-[1ch]   // 1 character width (~16px)
px-[2ch]   // 2 character widths (~32px)
px-[3ch]   // 3 character widths (~48px)
```

### Heights

```tsx
h-[var(--grid-4)]   // 16px
h-[var(--grid-6)]   // 24px (input height)
h-[var(--grid-8)]   // 32px (button height)
```

### Asymmetric Spacing Rule

Space ABOVE headings > Space BELOW (anchors heading to content):

```tsx
// ❌ BAD - Equal spacing with hardcoded values
<section className="mt-8 mb-8">

// ✅ GOOD - Asymmetric with grid tokens
<section className="mt-[var(--grid-16)] mb-[var(--grid-6)]">   // Sections
<section className="mt-[var(--grid-12)] mb-[var(--grid-4)]">   // Subsections
```

### BANNED Patterns

```tsx
// ALL of these are BANNED:
p-1, p-2, p-3, p-4, p-5, p-6, p-8, p-10, p-12
m-1, m-2, m-3, m-4, m-5, m-6, m-8, m-10, m-12
gap-1, gap-2, gap-3, gap-4, gap-5, gap-6, gap-8
space-y-1, space-y-2, space-y-4, space-y-8, space-y-12
pt-4, pb-4, mt-8, mb-8  // Any hardcoded spacing
```

**See `grid-spacing` skill for complete reference.**

---

## RULE 5: BORDER RADIUS

```tsx
// Full border (all 4 sides) = MUST use mode.radius
<div className={cn('border', mode.radius)}>

// Partial border = NO radius
<div className="border-b">
<div className="border-t border-b">

// Tables = NEVER radius
<th className="border">
<td className="border">

// BANNED - never hardcode radius
// rounded-sm, rounded-md, rounded-lg, rounded-xl, rounded-2xl
```

---

## RULE 6: THEME SWITCHING

The app supports light and dark mode via `data-theme-mode` attribute.

```tsx
// Theme is managed by ThemeProvider
// Components automatically respond to theme changes
// All mode.color.* tokens are theme-aware

// To check current theme:
import { useTheme } from 'next-themes';
const { theme, setTheme } = useTheme();
```

---

## RULE 7: RESPONSIVE PATTERNS

### Breakpoints

```tsx
// Mobile first - no prefix = mobile
// sm: = 640px+
// md: = 768px+
// lg: = 1024px+
// xl: = 1280px+
// 2xl: = 1536px+

<div className="p-4 md:p-6 lg:p-8">
<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
```

### Common Patterns

```tsx
// Stack on mobile, row on desktop
<div className="flex flex-col md:flex-row gap-4">

// Full width mobile, constrained desktop
<div className="w-full max-w-2xl mx-auto">

// Hide on mobile
<div className="hidden sm:block">

// Show only on mobile
<div className="block sm:hidden">
```

---

## RULE 8: ACCESSIBILITY

### Focus States (REQUIRED)

```tsx
<button className={cn(
  mode.state.focus.ring,
  // ... other styles
)}>

// Manual focus ring:
<button className="focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring">
```

### Screen Reader Text

```tsx
<span className="sr-only">Description for screen readers</span>

// Icon-only buttons need labels
<button aria-label="Close dialog">
  <X className={mode.icon.md} />
</button>
```

---

## RULE 9: FILE SIZE LIMITS

| File Type | Max Lines |
|-----------|-----------|
| Components | 150 |
| Pages | 200 |
| Hooks | 100 |
| API Routes | 150 |

**If over limit: Split into smaller files.**

---

## RULE 10: ADDING NEW TOKENS

**If a token doesn't exist, ADD IT to the design system first.**

### Step 1: Add to config.ts

```tsx
// src/design-system/mode/config.ts
color: {
  bg: {
    newToken: 'bg-[oklch(var(--your-css-var))]',
  }
}
```

### Step 2: Add CSS variable (if needed)

```css
/* src/app/globals.css */
:root {
  --your-variable: 65% 0.15 240; /* OKLCH: lightness chroma hue */
}

[data-theme-mode="dark"] {
  --your-variable: 35% 0.15 240;
}
```

### Swiss Color Rules

- 90% grayscale (0 chroma) - structure, text
- 8% accent (0.15-0.22 chroma) - interactive
- 2% semantic (0.15-0.18 chroma) - status

---

## RULE 11: COMPLETE EXAMPLES

### Basic Component

```tsx
'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';
import { mode } from '@/design-system';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

interface ExampleProps {
  title: string;
  description?: string;
}

export function Example({ title, description }: ExampleProps) {
  return (
    <Card className={cn('border', mode.radius, mode.color.border.default)}>
      <CardHeader className="p-4">
        <h2 className={cn(
          mode.typography.headline.sm,
          mode.font,
          mode.color.text.primary
        )}>
          {title}
        </h2>
      </CardHeader>
      <CardContent className="p-4 space-y-4">
        {description && (
          <p className={cn(
            mode.typography.body.md,
            mode.font,
            mode.color.text.muted
          )}>
            {description}
          </p>
        )}
        <div className="flex gap-4">
          <Input placeholder="Enter text" />
          <Button>{mode.buttonPrefix}SUBMIT</Button>
        </div>
      </CardContent>
    </Card>
  );
}
```

### Form with Validation

```tsx
'use client';

import * as React from 'react';
import { useState } from 'react';
import { cn } from '@/lib/utils';
import { mode } from '@/design-system';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useToast } from '@/hooks';

export function ExampleForm() {
  const { toast } = useToast();
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      // ... submit logic
      toast({ title: 'SUCCESS', description: 'Form submitted.' });
    } catch {
      setError('Failed to submit');
      toast({ title: 'ERROR', description: 'Submission failed.', variant: 'destructive' });
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="space-y-2">
        <Label className={cn(mode.typography.label.md, mode.font)}>
          EMAIL
        </Label>
        <Input type="email" required />
      </div>

      {error && (
        <div className={cn(
          'p-4 border',
          mode.radius,
          mode.color.border.danger,
          mode.color.bg.dangerMuted
        )}>
          <p className={cn(mode.typography.body.sm, mode.color.text.danger)}>
            {error}
          </p>
        </div>
      )}

      <Button type="submit" className={mode.state.focus.ring}>
        {mode.buttonPrefix}SUBMIT
      </Button>
    </form>
  );
}
```

---

## BANNED PATTERNS

```tsx
// BANNED: Custom button
<button className="px-4 py-2 bg-blue-500 rounded">

// BANNED: Custom card
<div className="border rounded-lg p-4 shadow">

// BANNED: Custom modal
<div className="fixed inset-0 bg-black/50 z-50">

// BANNED: Custom input
<input className="border rounded px-2 py-1" />

// BANNED: Hardcoded colors
className="bg-gray-100"
className="text-blue-500"
className="border-slate-200"

// BANNED: Hardcoded typography
className="text-sm"
className="text-xl"
className="font-bold"

// BANNED: ALL hardcoded Tailwind spacing (use --grid-* variables)
className="p-4"          // Use p-[var(--grid-4)]
className="gap-2"        // Use gap-[var(--grid-2)]
className="mt-8"         // Use mt-[var(--grid-8)]
className="space-y-4"    // Use space-y-[var(--grid-4)]
className="mb-12"        // Use mb-[var(--grid-12)]
className="pt-6"         // Use pt-[var(--grid-6)]

// BANNED: Hardcoded radius
className="rounded-lg"
className="rounded-xl"

// BANNED: Inline styles
style={{ color: '#333', backgroundColor: '#f5f5f5' }}
```

---

## CHECKLIST

Before submitting code:

- [ ] Read relevant spec from `docs/specs/`
- [ ] Checked `src/components/ui/` for existing component
- [ ] Checked fabrk-dev for existing component
- [ ] Using design tokens (`mode.*`) not hardcoded values
- [ ] Using `--grid-*` CSS variables (NO hardcoded Tailwind spacing like p-4, gap-2, mt-8)
- [ ] Using `mode.radius` for full borders
- [ ] Imports include `mode` from `@/design-system`
- [ ] Imports include `cn` from `@/lib/utils`
- [ ] Interactive elements have focus states
- [ ] File under size limit
- [ ] Any new styles added to design system first
- [ ] Swiss Layout Hierarchy followed (16px → 24px → 32px → 48px → 64px)
