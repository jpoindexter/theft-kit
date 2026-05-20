# HyperFrames Shader Canvas Pattern

Use this pattern for procedural shader backgrounds, generated texture layers,
raymarched objects, particle fields, and shader-driven accents inside a
HyperFrames composition.

The key rule: HyperFrames capture seeks the GSAP timeline. The shader must
render from timeline time, not wall-clock time.

## Runtime Rules

- No `requestAnimationFrame`.
- No `Date.now()`, `performance.now()`, `Math.random()`, `setTimeout`, async
  shader setup, or infinite repeats.
- Compile shaders synchronously during page load.
- Render once at time 0, then render from a GSAP timeline `onUpdate`.
- Register the timeline in `window.__timelines["composition-id"]`.
- Also expose `window.__hf = { duration, seek }` synchronously. Screenshot-mode
  capture polls this adapter and can fail with `window.__hf not ready` even when
  the GSAP timeline exists.
- Size the canvas to the composition dimensions, not the browser viewport.
- If logs show `HeadlessExperimental.beginFrame unavailable`, inspect the
  rendered MP4 itself. WebGL canvases can be blank in screenshot-mode capture
  while DOM overlays still paint; prefer Canvas2D for critical procedural
  visuals when it can deliver the look.

## Skeleton

Use this as the starting shape, then replace the fragment shader and scene
content. Keep the composition id, `data-duration`, and timeline duration in
sync.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <script src="https://cdn.jsdelivr.net/npm/gsap@3.14.2/dist/gsap.min.js"></script>
    <style>
      body {
        margin: 0;
        width: 1920px;
        height: 1080px;
        overflow: hidden;
        background: #05070d;
      }
      [data-composition-id="shader-scene"] {
        position: relative;
        width: 1920px;
        height: 1080px;
        overflow: hidden;
        background: #05070d;
        font-family: "YOUR DISPLAY FONT", sans-serif;
      }
      #shader-canvas {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        display: block;
      }
      .copy {
        position: relative;
        z-index: 2;
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        padding: 120px 150px;
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
        color: white;
      }
    </style>
  </head>
  <body>
    <div
      data-composition-id="shader-scene"
      data-start="0"
      data-duration="8"
      data-width="1920"
      data-height="1080"
    >
      <canvas id="shader-canvas"></canvas>
      <div class="copy">
        <h1>Shader-led motion</h1>
      </div>

      <script id="shader-vs" type="x-shader/x-vertex">
        #version 300 es
        in vec2 position;
        void main() {
          gl_Position = vec4(position, 0.0, 1.0);
        }
      </script>

      <script id="shader-fs" type="x-shader/x-fragment">
        #version 300 es
        precision highp float;

        out vec4 fragColor;
        uniform vec3 iResolution;
        uniform float iTime;

        float hash(vec2 p) {
          vec3 p3 = fract(vec3(p.xyx) * 0.1031);
          p3 += dot(p3, p3.yzx + 33.33);
          return fract((p3.x + p3.y) * p3.z);
        }

        float noise(vec2 p) {
          vec2 i = floor(p);
          vec2 f = fract(p);
          f = f * f * (3.0 - 2.0 * f);
          float a = hash(i);
          float b = hash(i + vec2(1.0, 0.0));
          float c = hash(i + vec2(0.0, 1.0));
          float d = hash(i + vec2(1.0, 1.0));
          return mix(mix(a, b, f.x), mix(c, d, f.x), f.y);
        }

        void mainImage(out vec4 color, in vec2 fragCoord) {
          vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
          float n = noise(uv * 3.0 + vec2(iTime * 0.15, -iTime * 0.08));
          float glow = smoothstep(0.15, 1.1, 1.0 - length(uv * vec2(0.8, 1.2)));
          vec3 ink = mix(vec3(0.02, 0.03, 0.06), vec3(0.07, 0.42, 0.55), n);
          vec3 accent = vec3(0.95, 0.55, 0.22) * glow * 0.65;
          color = vec4(ink + accent, 1.0);
        }

        void main() {
          mainImage(fragColor, gl_FragCoord.xy);
        }
      </script>

      <script>
        (() => {
          const DURATION = 8;
          const canvas = document.getElementById("shader-canvas");
          const gl = canvas.getContext("webgl2", {
            antialias: false,
            depth: false,
            stencil: false,
            preserveDrawingBuffer: true,
          });
          if (!gl) throw new Error("WebGL2 is unavailable");

          function source(id) {
            return document.getElementById(id).textContent.trim();
          }

          function compile(type, shaderSource) {
            const shader = gl.createShader(type);
            gl.shaderSource(shader, shaderSource);
            gl.compileShader(shader);
            if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
              throw new Error(gl.getShaderInfoLog(shader) || "Shader compile failed");
            }
            return shader;
          }

          const program = gl.createProgram();
          gl.attachShader(program, compile(gl.VERTEX_SHADER, source("shader-vs")));
          gl.attachShader(program, compile(gl.FRAGMENT_SHADER, source("shader-fs")));
          gl.linkProgram(program);
          if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
            throw new Error(gl.getProgramInfoLog(program) || "Shader link failed");
          }

          const position = gl.getAttribLocation(program, "position");
          const buffer = gl.createBuffer();
          gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
          gl.bufferData(
            gl.ARRAY_BUFFER,
            new Float32Array([-1, -1, 3, -1, -1, 3]),
            gl.STATIC_DRAW
          );
          gl.enableVertexAttribArray(position);
          gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);

          const resolution = gl.getUniformLocation(program, "iResolution");
          const time = gl.getUniformLocation(program, "iTime");

          function resize() {
            const root = canvas.closest("[data-composition-id]");
            const width = Number(root.dataset.width) || 1920;
            const height = Number(root.dataset.height) || 1080;
            if (canvas.width !== width || canvas.height !== height) {
              canvas.width = width;
              canvas.height = height;
            }
            gl.viewport(0, 0, width, height);
            return { width, height };
          }

          function render(seconds) {
            const size = resize();
            gl.useProgram(program);
            gl.uniform3f(resolution, size.width, size.height, 1);
            gl.uniform1f(time, seconds);
            gl.drawArrays(gl.TRIANGLES, 0, 3);
          }

          render(0);

          window.__timelines = window.__timelines || {};
          const state = { t: 0 };
          const tl = gsap.timeline({ paused: true });
          tl.to(
            state,
            {
              t: DURATION,
              duration: DURATION,
              ease: "none",
              onUpdate: () => render(state.t),
            },
            0
          );
          tl.from(".copy", { y: 48, opacity: 0, duration: 0.7, ease: "power3.out" }, 0.35);
          window.__timelines["shader-scene"] = tl;

          window.__hf = {
            duration: DURATION,
            seek(time) {
              const seconds = Math.max(0, Math.min(DURATION, Number(time) || 0));
              tl.seek(seconds, true);
              render(seconds);
            },
          };
        })();
      </script>
    </div>
  </body>
</html>
```

## Review Checklist

- Scrub the composition: the shader changes with timeline time.
- Render at least one frame where text overlays the shader; text must stay
  legible.
- Check the MP4 for black frames. A successful render can still be visually
  blank if WebGL compiled but uniforms or canvas sizing are wrong.
- If the MP4 has blank WebGL canvas frames under screenshot-mode capture,
  rebuild the effect as a deterministic Canvas2D `frame(t)` renderer unless the
  brief truly requires GLSL.
- If using shader transitions too, read `hyperframes/references/transitions.md`
  and keep scene backgrounds explicit.
