# ShaderToy To WebGL2 Adaptation

Use this when adapting GLSL snippets from ShaderToy-style references into a
browser, HTML artifact, iframe artifact, or HyperFrames composition.

## Required WebGL2 Changes

- Request `canvas.getContext("webgl2")`.
- Put `#version 300 es` as the first character of each shader source.
- Add `precision highp float;` in fragment shaders.
- Declare `out vec4 fragColor;` instead of writing `gl_FragColor`.
- Use `in` / `out` instead of `attribute` / `varying`.
- Use `texture(...)` instead of `texture2D(...)`.
- Use `gl_FragCoord.xy` in the WebGL2 entry point.
- If the source has `mainImage(out vec4 color, in vec2 fragCoord)`, wrap it:

```glsl
void main() {
  mainImage(fragColor, gl_FragCoord.xy);
}
```

## Uniform Contract

Use ShaderToy-compatible names unless the host already has a convention:

```glsl
uniform vec3 iResolution; // width, height, pixel ratio or 1.0
uniform float iTime;      // seconds
uniform vec4 iMouse;      // optional: x, y, pressed-x, pressed-y
uniform int iFrame;       // optional
```

Always use declared uniforms. Browsers can optimize unused uniforms away, which
makes `gl.getUniformLocation(...)` return `null`.

## Common Compile Fixes

- Reorder functions so callees are declared before callers.
- Replace macros that call functions with constants or ordinary functions.
- Extract the right vector components. If a helper expects `vec2`, pass
  `pos.xz`, not a `vec3`.
- Keep loop bounds simple integer literals when possible.
- When shader source lives in `<script>` tags, read it with
  `.textContent.trim()` so `#version` stays first.

## Minimal Fragment Pattern

```glsl
#version 300 es
precision highp float;

out vec4 fragColor;
uniform vec3 iResolution;
uniform float iTime;

void mainImage(out vec4 color, in vec2 fragCoord) {
  vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
  float ring = 0.5 + 0.5 * sin(8.0 * length(uv) - iTime * 2.0);
  vec3 base = mix(vec3(0.02, 0.03, 0.06), vec3(0.4, 0.8, 1.0), ring);
  color = vec4(base, 1.0);
}

void main() {
  mainImage(fragColor, gl_FragCoord.xy);
}
```

## Visual Quality

- Work in linear-ish color, then apply simple tone mapping or gamma correction
  if highlights clip.
- Use analytic anti-aliasing (`smoothstep` around edges) for SDF shapes.
- Avoid one-note hue shifts. Tie palette values to the product or DESIGN.md.
- For text-heavy layouts, keep shader contrast lower behind copy and reserve
  high-frequency detail for edges or scene transitions.
