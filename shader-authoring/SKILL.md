---
name: shader-authoring
description: >-
  Author GLSL, WebGL, ShaderToy-style, and procedural shader visuals for
  websites, React/Next.js apps, HTML artifacts, iframe UI artifacts, and
  HyperFrames video compositions. Use when asked for shaders, WebGL, GLSL,
  ray marching, SDFs, procedural noise, particles, fluids, generative
  backgrounds, shader transitions, or deterministic canvas/WebGL effects in
  create_hyperframes_composition, create_html_artifact, create_iframe_ui_artifact,
  or frontend code.
metadata:
  version: 1.0.0
---

# Shader Authoring

Use this skill when the work needs a real shader or a shader-informed visual
system, not just a gradient. It is the bridge between general GLSL technique and
the surfaces this product actually ships: React/Next.js, self-contained HTML
artifacts, sandboxed iframe UI artifacts, and HyperFrames video.

This skill does not require MiniMax credentials or any video/image generation
API. It focuses only on shader technique and runtime authoring.

## Pick The Surface First

- **Website or app code**: build a client-only component. Use Three.js or
  react-three-fiber for 3D scenes, plain WebGL2 for full-screen fragment effects,
  and existing project shader libraries only after checking local usage.
- **`create_html_artifact`**: write one self-contained HTML file with inline
  CSS/JS and WebGL2. It can animate with `requestAnimationFrame`.
- **`create_iframe_ui_artifact`**: keep the shader tiny. The iframe has no
  network access, must include the CSP meta tag, and must fit the srcdoc size
  limit. Use no external textures, fonts, CDNs, workers, or nested frames.
- **HyperFrames video**: read the `hyperframes` skill too. Use timeline-driven
  canvas rendering, not `requestAnimationFrame`, so the capture engine can seek
  deterministically. Publish through `create_hyperframes_composition`.

## Workflow

1. Choose the output surface above.
2. Choose the technique family from
   [references/technique-router.md](references/technique-router.md).
3. Establish visual identity before writing code: palette, contrast, motion
   energy, and how the shader supports the content.
4. Adapt ShaderToy-style snippets using
   [references/webgl2-adaptation.md](references/webgl2-adaptation.md).
5. Use the right runtime pattern:
   - HyperFrames: [references/hyperframes-shader-canvas.md](references/hyperframes-shader-canvas.md)
   - HTML artifacts, iframe artifacts, and app code:
     [references/html-artifact-shaders.md](references/html-artifact-shaders.md)
6. Compile, render, and inspect a nonblank frame. For failures, use
   [references/debugging.md](references/debugging.md).

## HyperFrames Rules

- Use custom shader canvases for procedural backgrounds, particle fields,
  raymarched objects, noise layers, SDF ornaments, and audio-reactive visual
  layers.
- HyperFrames capture may fall back to screenshot mode. In that path, WebGL
  canvases can render blank in the MP4 while DOM overlays still render. For
  critical captured visuals that Canvas2D can express well, prefer deterministic
  Canvas2D over WebGL; otherwise verify actual MP4 frames, not just
  `render_status=ready`.
- Use `@hyperframes/shader-transitions` for scene-to-scene texture transitions
  when that package is available. Do not hand-roll scene texture capture unless
  the package is unavailable and the user explicitly needs a new transition.
- Do not mix custom live shader canvases inside scenes that are also captured by
  shader-transition textures unless you verify the rendered MP4. Prefer one
  shader system per composition.
- Every shader animation must be seekable from the GSAP timeline. No
  `requestAnimationFrame`, `Date.now()`, `performance.now()`, async setup, or
  infinite loops in HyperFrames compositions.
- Also expose `window.__hf = { duration, seek }` synchronously for screenshot
  mode and keep a matching `window.__timelines["<composition-id>"]` entry for
  the normal HyperFrames path.

## App And Artifact Rules

- Use WebGL2 first. Fall back to a static CSS background or Canvas 2D only when
  WebGL cannot compile or the output is intentionally simple.
- Keep mobile and video budgets in mind: avoid path tracing, deep ray marching,
  high particle counts, and multipass buffers unless the brief truly needs them.
- For React/Next.js, create client components with cleanup for contexts,
  animation frames, event listeners, and resize observers.
- For iframe UI artifacts, do not use remote textures or shader libraries. Use
  inline shader strings and data URIs only.

## Attribution

Technique routing and ShaderToy-to-WebGL2 adaptation guidance were informed by
MiniMax-AI's open-source `shader-dev` skill. This repo-specific skill rewrites
that guidance for GIC coding agents, HyperFrames, and artifact surfaces.
