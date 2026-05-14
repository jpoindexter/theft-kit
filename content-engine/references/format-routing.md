# Content Engine — Format Routing & Tool Briefing

Reference for picking the right production tool and briefing it well. Paired with [../SKILL.md](../SKILL.md).

---

## Decision Table

| Brief shape | Format | Tool | Why |
|-------------|--------|------|-----|
| Trend-jack, talking-head, first-person take | UGC video | `agent-media-create-ugc-video` | Creator-style pacing, authentic feel, fast |
| Cinematic scene, abstract concept, product-in-world, continuity-sensitive short clip | AI video | `openrouter-video-create` (Seedance) | Photoreal motion, strong lighting, native audio, compact prompt-driven continuity |
| Structured promo with text + graphics | Structured promo video | `create_hyperframes_composition` | Deterministic, editable, reusable motion system |
| Bespoke branded motion graphics | Custom video | `create_hyperframes_composition` | Full HTML/CSS/GSAP control, brand-perfect; pair with `shader-authoring` for GLSL/WebGL layers |
| Trailer or teaser with both realistic scenes and designed title beats | Hybrid video plan | Seedance + HyperFrames | Realistic footage plus deterministic overlays/end cards |
| Jingle, sonic logo, intro/outro cue, background bed | Original music | `create_lyria_music_audio_artifact` | Useful when audio identity matters as much as visuals |
| Static image + caption (feed post, carousel cover) | Image | `generate_image` | Fast, cheap, captioned separately |
| Text-only post (thread, hot take) | Text | (no render tool — pure caption tournament) | Caption-only formats skip render |

---

## When to Use Each Video Tool

### agent-media — UGC / creator-style

**Pick when:** the brief calls for a human voice and face, trend-jack energy, or anything that would feel fake as a cinematic AI render. Best for reactive content and first-person takes.

**Script pacing (hard caps):**
- 5s → ≤12 words
- 10s → ≤25 words
- 15s → ≤37 words

Pick the shortest duration that fits the hook + CTA. Longer isn't better on short-form.

**Inputs to provide:**
- `script` (preferred over `prompt` — gives you deterministic control over words)
- `aspect_ratio` — default to `9:16` for social
- `tone` — energetic / conversational / deadpan — match the angle
- `cta` — one clear action

**Gotchas:**
- Polling can take minutes. Start the job early and do other work while waiting.
- Actor slug matters — a mismatched face kills the vibe. Check available actors before committing.
- Do not use it for typography-led motion promos, deterministic title timing, or
  reusable branded compositions.

---

### Seedance — cinematic AI video

**Pick when:** the concept is scene-heavy, abstract, or product-in-world rather than person-to-camera. Great for product shots in motion, atmospheric b-roll, and concept work.

**Brief structure:**
Use this frame in order. A weak Seedance prompt is vague; a strong one reads like a compact shot plan with continuity constraints.

```
FORMAT: [3-10s, shot count, pacing, aspect ratio]
SUBJECT: [specific person/product described in prompt]
ENVIRONMENT: [where it happens, practical light, background feel]
MOOD: [emotional tone in a phrase, not a single adjective]
STYLE: [realism level, camera feel, color treatment]
LOGIC RULE: [continuity instruction: same face/product/wardrobe/jewelry/color across shots]
```

**Prompt shape:**
- **Direct video** — for prompt-only text-to-video, describe the scene and continuity constraints explicitly in text.
- **Image-to-video** — if the user says "make a video out of this image" or gives you a still to animate, pass it to `openrouter-video-create` as `conditioning_image_url` so Seedance uses it as the first frame. If the image lives in the workspace or came in as an upload, call `get_workspace_file_signed_url` first.
- **Video continuation** — if the user asks for a second Seedance clip after an existing video, pass the prior clip's playable signed URL as `continuation_source_video_url`; the tool extracts the prior video's last frame and uses it as the next clip's first-frame image. After the new clip lands, stitch the original and continuation in Canvas Video Studio when the user wants one combined file.

**Continuity rule of thumb:**
If the prompt spans multiple beats, always state what must not drift. Examples: `same face across all shots`, `same product bag color and label`, `same earrings and jacket`, `consistent smoothie color during tilt`.

**Constraints:**
- GIC's `openrouter-video-create` tool supports 3-10 seconds
- Aspect ratio must be `16:9`, `9:16`, or `1:1`
- Resolution must be `480p`, `720p`, or `1080p`
- Native audio is opt-in; keep `generate_audio=false` unless the user explicitly asks for native generated audio

**Gotchas:**
- Prompt drift from desired aspect ratio — state aspect explicitly
- Conditioning image or continuation extraction helps lock the opening look, but later beats can still drift; describe the persistent traits explicitly and keep the scene count compact when brand accuracy matters
- Do not confuse distribution surfaces with model guidance: Higgsfield can be a useful creation surface, but for GIC the routing decision is still about Seedance via `openrouter-video-create`
- Do not hard-code vendor pricing comparisons into reusable prompts or skills; those drift faster than model behavior
- Do not use Seedance when the asset is primarily text, charts, proof overlays,
  or motion-graphics timing that must be rerenderable and easy to tweak.

---

### HyperFrames — structured promo video

**Pick when:** you want deterministic, editable, rerenderable video with text and graphics. Best for promo cuts, feature launches, data-driven content, and anything you'll want to produce variations of.

**Briefing rule — describe in beats first:**
```
0:00–0:02  Hook: [copy + visual]
0:02–0:05  Proof: [copy + overlay/stat]
0:05–0:10  Product visual: [screenshot / product shot / animation]
0:10–0:15  CTA: [action + URL]
```

Then turn those beats into a self-contained HTML/CSS/GSAP composition with a stable `data-composition-id`, explicit stage timing, and any supporting media layers (logo, product screenshots, overlay graphics, audio cues) the placement needs.

**Brand rule:** Before authoring the HTML/CSS, read `/workspace/library/design/DESIGN.md` when present and treat it as the source of truth for colors, fonts/typefaces, spacing rhythm, component treatment, imagery, motion tone, and do/don't constraints. If it is absent or incomplete, use `.agents/brand-page-context.md`, `visual-style.md`, or explicit user direction before defaulting to generic house palettes.

**Source-media shortcut:**
- If the user already has a still image or source clip and wants a longer stitched edit, title sequence, proof overlay stack, or end card built around it, HyperFrames is a strong default. Reference durable local media paths from the composition HTML instead of signed URLs: use `/workspace/work/uploaded_files/...` paths for uploads, and generated or copied assets should live under `/workspace/work/artifacts/...` and be referenced either by that absolute sandbox path or by a composition-relative path like `../images/shot.png` from `artifacts/compositions/*.html` so later previews and rerenders keep working.

**Gotchas:**
- Keep the motion system simple enough to rerender and tweak quickly; overbuilt compositions slow iteration.
- Make timing explicit in the composition so hook, proof, and CTA beats stay easy to adjust.
- Do not use HyperFrames when the user explicitly wants filmed-feeling realism,
  live-action atmosphere, or camera-driven scenes more than designed motion.

---

### Hyperframes — bespoke motion graphics

**Pick when:** you need full control over motion, brand, and timing that templates can't express. Kinetic typography, data-in-motion, abstract branded visuals, shader-led atmospheres, product explainers with custom choreography.

**Composition structure:**
- Full self-contained HTML/CSS/GSAP
- `#stage` root with `data-composition-id`
- Timing attributes on timeline elements
- Path under `artifacts/compositions/*.html`

**Brief as code, not prose.** Hyperframes works best when you write the composition directly and let the renderer do its job.

**Gotchas:**
- Render timeout 360s — stuck renders fail silently. Check status.
- Iteration usually means editing the composition directly rather than swapping a tiny prop bag, so keep the structure clean and the choreography deliberate.

---

## Hybrid Rule: Seedance + HyperFrames

Use a hybrid route when the job needs both:
- realism-first footage or product-in-world scenes
- designed title cards, proof overlays, CTA frames, or deterministic end cards

Plan the split before rendering:
1. Seedance owns the realistic scene beats.
2. HyperFrames owns the overlays, supers, title cards, and end-card timing.
3. `create_lyria_music_audio_artifact` owns original jingles, sonic logos, and music
   beds when the campaign needs a reusable audio identity.
4. Save the route choice and one-line reason in the artifact so later runs do
   not reroute the concept from scratch.

---

## Music: Lyria

Use `create_lyria_music_audio_artifact` when the user asks for original campaign
music, a sonic logo, a jingle, an intro/outro cue, a loopable background bed, or
an audio identity to pair with a launch/video/social asset.

Prompt with genre, energy, tempo/BPM if known, instrumentation, audience,
placement, duration, vocal/lyric constraints, and whether the cue should loop or
resolve. Do not ask for a living artist, copyrighted song, or soundalike; adapt
the musical role instead of naming the protected reference.

---

## When to Reach for generate_image

For static posts, carousels, thumbnails, and reference images for tools that actually accept them.

**Best for:**
- Feed posts that work as image + caption
- Hero images used as input to Hyperframes or as standalone social visuals
- Carousel covers and interior slides
- Non-UI visuals (use Stitch for UI mockups)

**Size mapping:**
| Placement | Size | Ratio |
|-----------|------|-------|
| Instagram feed, LinkedIn feed | SQUARE | 1:1 |
| Stories, Reels cover, TikTok | PORTRAIT | 9:16 |
| Twitter/X feed, YouTube thumbnail | LANDSCAPE | 16:9 |

**Always provide reference images when the target tool supports them:**
- Brand consistency matters (product shots, spokesperson)
- Iterating on an existing image
- Matching a specific composition

---

## Scoring Gate Protocol

Every rendered asset goes through a gate. The gates are defined in SKILL.md — this section is the briefing rule for *how to regenerate* when a gate fails.

### When text fails (Step 3 gate)

If the tournament winner doesn't beat median by 5+:
- Problem: not enough angle variance
- Fix: regenerate with explicit angle assignment per variant ("variant 1 = contrarian, variant 2 = social proof, variant 3 = curiosity, variant 4 = outcome, variant 5 = identity")

If the winner scores below 40 absolute:
- Problem: the concept itself is weak
- Fix: reframe the brief. Ask for a sharper angle or a different hook pattern from the signal set.

### When hero image fails (Step 4 gate)

Read the subscores:
- Low **attention** → reduce background clutter, increase foreground contrast, tighten composition on the subject
- Low **emotion** → change the subject's expression, add human element, shift color temperature
- Low **visual** → improve lighting, add depth, fix framing

Regenerate with the specific critique.

### When video fails (Step 6 gate)

Read `hook_score` and `peak_moments`:
- Low **hook_score** → regenerate the first segment only (don't rerender the whole thing). Pattern-interrupt in the first 0.8s, front-load the visual payoff.
- Backloaded peaks (all in last 40%) → tighten pacing. Pull the visual climax earlier. Remove setup that delays the payoff.
- Low overall with good hook → middle sag. Remove filler, cut a beat.

### When static image final fails (Step 6 gate)

Same as hero image critique above. If it still fails after one retry, the concept isn't carrying — return to the user.

---

## Cost Awareness (Rough)

Not a hard rule, but useful context for format choice:

| Format | Cost per asset | Iteration speed |
|--------|----------------|-----------------|
| generate_image | $0.02–0.10 | Seconds |
| agent-media (15s) | $1–3 | 2–5 min |
| Seedance (10s) | $2–6 | 1–3 min |
| Hyperframes | $0 (self-hosted) | 1–5 min |

Variant tournaments on text are effectively free — that's why the pre-render gate catches 70% of failures before anything expensive runs.

---

## Related

- `ad-creative/references/generative-tools.md` — Deeper tool-by-tool reference including voice, comparison tables, and paid-creative-specific workflows
- `../SKILL.md` — The pipeline this reference serves
