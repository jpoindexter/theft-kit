# HTML Artifact And Website Shader Patterns

Use this for `create_html_artifact`, `create_iframe_ui_artifact`, and normal
frontend implementation.

## Self-Contained HTML Artifacts

`create_html_artifact` is good for interactive shader demos, branded one-pagers,
HTML-first pitch decks, and visual explainers. It can use inline JavaScript and
`requestAnimationFrame`.

Rules:

- Keep all code in one HTML file.
- Prefer WebGL2 full-screen fragment shaders for backgrounds and visual systems.
- Use CSS and DOM for text, buttons, charts, and layout. Do not render body copy
  inside the shader.
- Resize the canvas with a `ResizeObserver` or window resize handler.
- Pause the loop when the page is hidden.
- Provide a static fallback color or gradient behind the canvas.

## Sandboxed Iframe UI Artifacts

`create_iframe_ui_artifact` is for small preview cards. It runs in a null-origin
iframe with no network access.

Rules:

- Include the required CSP meta tag.
- Inline everything. No `http://`, `https://`, protocol-relative URLs, web
  fonts, CDNs, workers, or nested frames.
- Keep shader code short and one-pass.
- Use data URIs only when an image is genuinely necessary.
- The iframe artifact should be a design mockup, not a full application.

## React And Next.js

- Mark shader components as client components.
- Do WebGL setup in `useEffect`, not during render.
- Clean up animation frames, listeners, observers, and GPU resources.
- Use `dynamic(..., { ssr: false })` or a client boundary for browser-only
  shader libraries.
- Use Three.js or react-three-fiber for 3D scenes and cameras. Use plain WebGL2
  for single-pass fragment backgrounds.
- Do not introduce a new shader dependency until local patterns and package
  availability have been checked with `rg`.

## Interaction Patterns

- Pointer-driven shaders: map pointer coordinates to `iMouse`, but provide a
  calm default when no pointer is present.
- Scroll-driven shaders: map scroll progress to a uniform. Throttle updates and
  keep layout independent of WebGL.
- Reduced motion: freeze time, lower intensity, or replace with a static frame
  when `prefers-reduced-motion` is set.
- Text overlays: reserve a quiet region behind text by lowering shader contrast
  or adding a subtle solid scrim.

## Validation

- Confirm the browser console has no shader compile errors.
- Confirm the first frame is nonblank before adding interaction.
- Test one narrow mobile viewport if the shader sits behind responsive content.
- For artifacts, open the rendered artifact surface rather than only viewing the
  raw HTML string.
