---
name: favicon-pack-maker
description: "Use when the user asks to create, redesign, preview, or install a favicon, browser tab icon, app icon set, or favicon pack for a website or generated app, including prompt-only requests with no reference image such as 'make me a pack of favicons'."
metadata:
  version: 1.0.0
---

# Favicon Pack Maker

Create small-icon systems that work as real site assets, not just a pretty
mockup. The default path is:

1. infer or collect brand direction
2. create a few simple SVG concepts
3. preview them in HTML at real favicon sizes
4. generate the final asset pack
5. install repository assets with direct coding tools when code changes are needed

Natural language is enough. Do not require the user to use `$favicon-pack-maker`
or any other special notation.

## When To Use

Use this skill for requests like:

- "make me a pack of favicons"
- "generate favicons for this app"
- "turn this logo/image into a browser icon"
- "make an app icon set and preview it"
- "install favicon assets in the site"
- "fix the old tab icon / manifest icons"

If the request is mostly page design, use the brand/design guidance in
`gic-skills` first. If the request requires files in a repository, use direct
repository coding tools with the exact target app and asset paths.

## Approach Menu

Consider these approaches, then choose the smallest one that satisfies the
task:

1. Existing-logo adaptation: simplify a supplied logo or current favicon.
2. Prompt-only SVG concepts: infer icon ideas from the product, name, initials,
   category, and brand tone.
3. Brand-system extraction: use an existing site, `DESIGN.md`, or
   `.agents/brand-page-context.md` to match tokens and visual language.
4. Raster-first concepting: use native image generation only for exploratory
   icon ideas, then redraw the chosen direction as a clean vector source.
5. Repo installation: generate assets and update metadata, manifest links, and
   static files in the target codebase.

Default to prompt-only SVG concepts plus an HTML preview when the user gives no
reference image. If the user says "make the pack" rather than "show options",
choose the strongest concept and continue to a final pack while keeping the
preview reviewable.

## Workflow

### 1. Establish Inputs

Identify:

- target product, site, app, or repo
- brand name and likely initials
- existing brand colors, typography, and visual motifs
- whether there is a source logo/image, an existing favicon, or no reference
- whether the user wants preview-only, final assets, or installed code changes

If brand context is missing, proceed with explicit assumptions instead of
blocking. For example, infer from the app name, category, current site, or user
description.

### 2. Design Concepts

Create 3-6 candidate SVG concepts unless the user already chose a direction.
Each concept should be favicon-native:

- one dominant silhouette
- strong contrast at 16x16 and 32x32
- no tiny paragraphs or detailed lettering
- limited colors, usually 1-3 plus transparency
- clear dark-mode and light-mode behavior
- enough safe area for maskable icons
- no copied third-party marks or trademark lookalikes

For prompt-only work, useful concept families include initials, monograms,
abstract product metaphors, simple geometric marks, and category symbols
adapted to the user's brand.

### 3. Preview In HTML

Create an HTML preview whenever the user asks for options, review, or a pack.
The preview should show every concept at real sizes:

- 16x16, 32x32, 48x48
- 180x180 for Apple touch
- 192x192 and 512x512 for manifest icons
- light background, dark background, and maskable safe-area checks

Label each concept with its name, palette, and recommended use. The preview is
the place to compare clarity; do not rely on a large hero rendering alone.

### 4. Generate The Pack

A complete pack should include, when supported by the chosen source:

- `favicon.svg` for vector-capable browsers
- `favicon.ico`
- `favicon-16x16.png`
- `favicon-32x32.png`
- `favicon-48x48.png`
- `apple-touch-icon.png`
- `icon-192.png`
- `icon-512.png`
- `maskable-icon-192.png`
- `maskable-icon-512.png`
- `site.webmanifest`
- `favicon-preview.html`

If the source is PNG-only and no SVG source is emitted, do not reference
`favicon.svg` in metadata or HTML. Remove stale SVG favicon references instead
of leaving browsers pointed at an old logo.

Prefer existing repo tooling. For Node projects, `sharp` plus `png-to-ico` is a
good implementation choice when available. For Python or static projects, use
the image tooling already present in the repo. Avoid external favicon-generator
websites unless the user explicitly asks for them.

### 5. Install In A Site

For a Next.js app, put generated assets in the app's `public/` directory and
update metadata in the existing layout or metadata helper. Include the SVG icon
only when `favicon.svg` exists.

Vector-capable example:

```ts
icons: {
  icon: [
    { url: "/favicon.ico", sizes: "any" },
    { url: "/favicon.svg", type: "image/svg+xml" },
    { url: "/favicon-32x32.png", sizes: "32x32", type: "image/png" },
  ],
  apple: [{ url: "/apple-touch-icon.png", sizes: "180x180" }],
}
```

PNG-only example:

```ts
icons: {
  icon: [
    { url: "/favicon.ico", sizes: "any" },
    { url: "/favicon-32x32.png", sizes: "32x32", type: "image/png" },
  ],
  apple: [{ url: "/apple-touch-icon.png", sizes: "180x180" }],
}
```

For `gic_admin_frontend`, do not install `site.webmanifest` unless the admin
middleware explicitly allows unauthenticated `.webmanifest` requests. Otherwise
browsers may receive a login redirect instead of the manifest. If the
middleware is not updated, install the favicon and touch icons only.

For static HTML, add standard `<link rel="icon">`, `<link rel="apple-touch-icon">`,
and `<link rel="manifest">` tags, again omitting the manifest when the server
cannot serve it publicly.

### 6. Repository Installation

When repository changes are needed, keep this brief explicit:

- target app or package
- source concept or selected SVG
- desired output filenames
- metadata files to inspect
- middleware caveat for admin apps
- verification expectations

Ask it to read existing metadata/icon patterns before editing, generate the
assets, install them, run relevant lints/type checks/tests, and verify the
preview or served files in the browser when possible.

### 7. Verification

Verify:

- assets exist at the expected paths
- metadata and manifest only reference emitted files
- `site.webmanifest` is publicly fetchable when installed
- the HTML preview renders every candidate and final icon
- the icon is readable at 16x16 and 32x32
- relevant lint, type check, and targeted tests pass

Do not claim the pack is installed if only preview assets were created.
