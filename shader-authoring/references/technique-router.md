# Shader Technique Router

Pick the smallest technique family that creates the desired effect. Most
marketing, product, and artifact work should use 2D procedural shaders before
reaching for expensive 3D ray marching.

| User wants | Start with | Combine with |
|---|---|---|
| Brand aura, animated hero background, soft energy field | Procedural noise | Domain warping, color palette |
| Liquid metal, ink, plasma, smoke-like gradients | Domain warping | FBM noise, post-processing |
| Stars, sparks, trails, snowfall, fireflies | Particle field | Noise, additive blending |
| 3D object made from math | Ray marching | SDF primitives, lighting, normals |
| Abstract 3D tunnel, repeated forms, impossible geometry | Ray marching | Domain repetition, camera transforms |
| UI ornaments, blobs, rings, pills, glyph-like shapes | 2D SDF | Anti-aliasing, palette mapping |
| Voronoi cells, cracked glass, crystal facets | Cellular noise | Palette mapping, edge detection |
| Terrain, mountains, topographic surfaces | Height field | FBM, atmospheric fog |
| Ocean, waves, caustic shimmer | Wave field | Noise, lighting, refraction hints |
| Clouds, fog, glow, volumetric beams | Volume march | Noise, atmospheric scattering |
| Glitch, chromatic split, bloom, vignette | Post-processing | Texture sampling |
| Scene-to-scene video transition in HyperFrames | Shader transition package | HyperFrames transition CSS rules |
| Audio-reactive spectrum or beat visuals | Shader canvas | HyperFrames audio-reactive skill |

## Technique Choice

- For **landing pages and HTML artifacts**, prefer full-screen fragment shaders
  with 2D coordinates, noise, domain warping, and palette control. They are
  cheaper, more legible, and easier to tune.
- For **HyperFrames**, prefer deterministic shader canvases for ambient layers
  and product mood. Use ray marching only when the 3D form is the point of the
  scene.
- For **data or product storytelling**, shaders should support hierarchy. Use
  them as background energy, reveal masks, proof visualizations, or texture
  systems, not as noise that competes with the copy.
- For **iframe UI artifacts**, keep to one pass and a short shader. Avoid
  multipass buffers, external textures, and high-DPI overdraw.

## Cost Guardrails

- Ray marching: cap steps, cap distance, and avoid nested loops.
- Particles: stateless particle formulas are easier to render and seek than
  stateful simulation.
- Fluids and reaction-diffusion: use approximations unless the output surface
  supports multipass buffers and the user asked for simulation specifically.
- Path tracing: avoid for product UI, artifacts, and HyperFrames unless the
  user explicitly asks for a slow photorealistic shader.
