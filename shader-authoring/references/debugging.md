# Shader Debugging

Most shader failures are wiring failures, not math failures. Check in this
order.

## If The Canvas Is Black

1. Confirm WebGL2 exists: `canvas.getContext("webgl2")` is not `null`.
2. Print shader compile and program link logs.
3. Confirm `#version 300 es` is the first character in the shader source.
4. Confirm the canvas has nonzero `width` and `height` attributes, not just CSS
   dimensions.
5. Confirm `gl.viewport(0, 0, width, height)` runs after sizing.
6. Confirm `gl.drawArrays(...)` runs after `gl.useProgram(...)`.
7. Temporarily replace the fragment shader with a solid color.

## Common Errors

| Error | Likely cause | Fix |
|---|---|---|
| `fragCoord` undeclared | ShaderToy variable used outside `mainImage` | Use `gl_FragCoord.xy` or pass `fragCoord` from `mainImage` |
| Missing `main()` | ShaderToy snippet only defines `mainImage` | Add `void main() { mainImage(fragColor, gl_FragCoord.xy); }` |
| `#version` not first | Leading whitespace from script tag | Read source with `.textContent.trim()` |
| Uniform location is `null` | Uniform optimized away or misspelled | Use it in shader and check exact name |
| Function overload mismatch | Wrong vector type or declaration order | Reorder helpers and pass matching `vec2` / `vec3` |
| Works live, fails in HyperFrames | Uses wall-clock time or RAF | Drive render from GSAP timeline time |

## HyperFrames Checks

- Ensure `window.__timelines["id"]` exists and the id matches
  `data-composition-id`.
- Ensure shader setup is synchronous.
- Render at `0` before registering the timeline.
- Use `preserveDrawingBuffer: true` when capture needs stable canvas pixels.
- Avoid hidden templates in standalone compositions.
- If a render returns `ready` but the video looks wrong, fix the composition and
  call `create_hyperframes_composition` again.

## Performance Checks

- Lower ray-march steps before lowering resolution.
- Replace nested noise calls with fewer octaves.
- Remove unused post-processing.
- Prefer one full-screen triangle over a quad.
- Avoid high-DPI render targets behind text-heavy layouts.
