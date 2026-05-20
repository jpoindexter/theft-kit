---
name: settings-page-create
description: Create settings pages following established patterns and design system
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

# Settings Page Create Skill

## When to use this skill
Use when creating new settings pages or adding new settings sections. Invoke this skill when:
- Creating a new settings page (e.g., /settings/password, /settings/sessions)
- Adding new sections to existing settings pages
- Building settings forms with save/reset functionality

## Core Pattern

All settings pages MUST use the `SettingsPageTemplate` component:

```tsx
import { SettingsPageTemplate, SettingsSectionCard } from '@/components/templates/settings-page-template';

export default function SettingsPasswordPage() {
  return (
    <SettingsPageTemplate
      title="Password Settings"
      description="Manage your account password"
      sections={sections}
      activeSection={activeSection}
      onSectionChange={setActiveSection}
      onSave={handleSave}
      saving={isSaving}
    >
      {/* Section content */}
    </SettingsPageTemplate>
  );
}
```

## Required Elements

### 1. Page Structure

```tsx
// File: src/app/(platform)/settings/[page]/page.tsx
'use client';

import * as React from 'react';
import { useToast } from '@/hooks';
import { SettingsPageTemplate, SettingsSectionCard } from '@/components/templates/settings-page-template';
import { mode } from '@/design-system';
import { cn } from '@/lib/utils';

const sections = [
  { id: 'general', label: 'General', icon: <Settings className="h-4 w-4" /> },
  { id: 'security', label: 'Security', icon: <Lock className="h-4 w-4" /> },
  { id: 'danger', label: 'Danger Zone', isDanger: true },
];
```

### 2. Section Cards

Use `SettingsSectionCard` for each settings group:

```tsx
<SettingsSectionCard
  title="Change Password"
  description="Update your account password"
>
  <div className="space-y-4">
    <Input label="Current Password" type="password" />
    <Input label="New Password" type="password" />
    <Input label="Confirm Password" type="password" />
  </div>
</SettingsSectionCard>

{/* Danger zone sections use isDanger prop */}
<SettingsSectionCard
  title="Delete Account"
  description="Permanently delete your account and all data"
  isDanger
>
  <Button variant="destructive">> DELETE ACCOUNT</Button>
</SettingsSectionCard>
```

### 3. Form State Management

```tsx
const [formData, setFormData] = React.useState({
  currentPassword: '',
  newPassword: '',
  confirmPassword: '',
});
const [isSaving, setIsSaving] = React.useState(false);
const { success, error } = useToast();

async function handleSave() {
  setIsSaving(true);
  try {
    const response = await fetch('/api/user/password', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(formData),
    });

    if (!response.ok) {
      const data = await response.json();
      throw new Error(data.error || 'Failed to save');
    }

    success('Password Updated', 'Your password has been changed');
    setFormData({ currentPassword: '', newPassword: '', confirmPassword: '' });
  } catch (err) {
    error('Error', err instanceof Error ? err.message : 'Failed to save');
  } finally {
    setIsSaving(false);
  }
}
```

### 4. API Error Handling

Always handle API errors gracefully:

```tsx
// Display inline validation errors
{errors.newPassword && (
  <p className={cn(mode.typography.body.md, mode.color.text.danger)}>
    {errors.newPassword}
  </p>
)}

// Use toast for server errors
catch (err) {
  error('Error', err instanceof Error ? err.message : 'An error occurred');
}
```

## Design System Compliance

### Spacing (8-point grid)
- Section cards: `space-y-6` between cards
- Form fields: `space-y-4` between inputs
- Never use p-3, p-5, gap-3, gap-5

### Colors (design tokens only)
```tsx
// Text
mode.color.text.primary   // Main text
mode.color.text.muted     // Descriptions
mode.color.text.danger    // Error text

// Backgrounds
mode.color.bg.surface     // Card backgrounds
mode.color.bg.muted       // Input backgrounds

// Borders
mode.color.border.default // Normal borders
mode.color.border.danger  // Danger zone borders
```

### Typography
```tsx
// Section titles
mode.typography.body.md

// Descriptions
cn(mode.typography.body.md, mode.color.text.muted)

// Form labels
mode.typography.label.md
```

## Checklist for New Settings Pages

- [ ] Uses SettingsPageTemplate
- [ ] Has sections array with icons
- [ ] Uses SettingsSectionCard for groups
- [ ] Includes loading state (saving prop)
- [ ] Has toast notifications for success/error
- [ ] Form validation with error display
- [ ] API route exists at /api/user/[action]
- [ ] Danger zone sections use isDanger prop
- [ ] All spacing follows 8-point grid
- [ ] All colors use design tokens

## File Organization

```
src/app/(platform)/settings/
  ├── page.tsx              # Main settings page
  ├── profile/
  │   └── page.tsx          # Profile settings
  ├── password/
  │   └── page.tsx          # Password settings
  ├── sessions/
  │   └── page.tsx          # Active sessions
  ├── email/
  │   └── page.tsx          # Email preferences
  └── data/
      └── page.tsx          # Data export/delete
```

## Common API Routes

Settings pages typically need corresponding API routes:

```
src/app/api/user/
  ├── password/route.ts     # PUT /api/user/password
  ├── email/route.ts        # PUT /api/user/email
  ├── sessions/route.ts     # GET, DELETE /api/user/sessions
  └── data/route.ts         # GET (export), DELETE /api/user/data
```
