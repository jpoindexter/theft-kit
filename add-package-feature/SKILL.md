---
name: add-package-feature
description: Add a new feature to an existing @fabrk/* package following monorepo conventions
user-invocable: true
---

# Add Package Feature

Add a new feature to an existing `@fabrk/*` package following the monorepo's conventions.

## Arguments

- `package`: The package name (e.g., "ai", "auth", "core")
- `feature`: Description of the feature to add

## Workflow

### Step 1: Understand the Package
Read `packages/{package}/src/index.ts` to understand existing exports and patterns.
Read `packages/{package}/package.json` for dependencies.

### Step 2: Check for Existing Patterns
Look at similar features in the same package. Follow the established patterns:
- **Adapter pattern**: External services behind interfaces from `@fabrk/core`
- **Store pattern**: Injectable stores with in-memory defaults
- **Web Crypto API**: No Node.js `crypto` — use Web Crypto for edge compatibility
- **Callback props**: Components accept callbacks, not direct API calls
- **Zod validation**: Use `z.input<>` for params, `z.infer<>` for internal types

### Step 3: Write Tests First (TDD)
Create test file at `packages/{package}/src/{feature}.test.ts`.
Write failing tests that define the expected behavior.
Run: `cd packages/{package} && npx vitest run {feature}.test.ts`

### Step 4: Implement
Create feature file at `packages/{package}/src/{feature}.ts`.
Implement the minimum code to pass all tests.

### Step 5: Export
Add to `packages/{package}/src/index.ts`:
```tsx
export * from './{feature}'
```

### Step 6: Build & Verify
```bash
pnpm build          # All 20 packages must build
pnpm type-check     # All 25 must pass
pnpm test           # All tests must pass
```

### Step 7: Cross-Package Impact
If the feature is used by other packages, check dependents still build:
```bash
pnpm build --filter="...@fabrk/{package}"
```

### Step 8: Changeset
```bash
pnpm changeset
```
Select the package, choose patch/minor/major, write description.
