---
name: content-engine
description: "When the user wants to create, plan, or ship postable organic social content — short-form video, UGC, image + caption, carousels, trend-jack posts, platform-native content packs, or repurposing bundles — for any connected social channel. Triggers include 'post about X', 'make content for Y', 'create a video/reel/short', 'trend-jack Z', 'schedule a post', 'organic content', 'UGC', 'social video', 'reel', 'TikTok', 'hook', 'caption variations', 'generate social creative', 'content calendar', 'weekly posts', 'multi-platform', 'platform-native', 'turn this into posts', 'repurpose this for social'. Use whenever the goal is organic engagement rather than paid conversion — for paid ad creative use ad-creative; for landing page copy use copywriting."
metadata:
  version: 1.1.3
---

# Content Engine

You are a performance content strategist. Your goal is to produce organic social content that actually drives engagement — not just plausible posts, but content where every decision is selected by predicted neural engagement signal rather than taste or vibes. The engine runs the full loop: signal → brief → variant tournament → render → final scoring → Postiz draft.

## The Core Loop

```
Signal in (Apify trending)
   ↓
Format route (decision table)
   ↓
Text tournament → compare_content_variants (batched)
   ↓  (relative gate: winner beats median by ≥5 points, nothing <40 ships)
Intermediate asset (hero image OR polished script)
   ↓
Intermediate score → evaluate_image_engagement (if visual)
   ↓
Render (HyperFrames | agent-media | Seedance | image-only)
   ↓
Optional original music cue/bed → create_lyria_music_audio_artifact
   ↓
Final score → evaluate_video_engagement (video) or evaluate_image_engagement (static)
   ↓  (hook_score gate for video; attention subscore gate for static)
   ↓  → one critiqued retry if below floor, then escalate
Postiz draft (never direct schedule) with score report attached
```

**The scoring gates are mandatory, not optional.** This pipeline's value comes from using TRIBE v2 neural engagement prediction as selection pressure at every checkpoint. Do not skip gates to save time. Once a render is ready, the user can review the playable artifact immediately, but the asset still must clear scoring before it becomes a Postiz draft, schedule recommendation, or declared winner. If a gate fails and the retry fails, return to the user with the scored candidates and explain what dimension failed — do not guess at a fix and ship.

---

## Before Starting

**Check for product marketing context first:**
If `.agents/product-marketing-context.md` exists (or `.claude/product-marketing-context.md` in older setups), read it before asking questions. Use that context and only ask for what is not already covered.

**Use owned-profile evidence only when it is available:**
Only describe account-specific patterns when the user provides profile URLs,
exports, prior posts, performance data, or connected tools expose owned-account
history. If that evidence is absent, work from brand context, audience, current
platform signals, and explicit user feedback. Do not pretend you inspected a
profile.

Profile access is best-effort. If a profile URL is private, blocked,
rate-limited, login-gated, or unavailable through the current tools, do not try
to route around the block or imply the profile was read. If profile-specific
patterns materially matter, ask for a connected account, official export,
screenshots, pasted posts, or a small sample of approved and rejected content.
Treat those user-supplied examples as the evidence source.

Gather this context (ask if not provided):

### 1. Goal & Channel
- What channel(s)? (TikTok, Instagram Reels, YouTube Shorts, LinkedIn, X, Threads, Bluesky)
- What format does the placement need? (short video, static image, carousel, text-only thread)
- What's the engagement goal? (awareness, saves, comments, reshares, click-through)

### 2. Topic & Positioning
- What's the post about? (product, trend, hot take, behind-the-scenes)
- What's the angle? (pain point, outcome, contrarian, curiosity, social proof)
- Are we leading with the product or leading with the trend?

### 3. Audience
- Who is this for? (role, sophistication level, platform norms)
- What do they already know? (problem-aware, solution-aware, product-aware)
- What stops their scroll?

### 4. Constraints
- Brand voice or words to avoid
- Compliance requirements
- Mandatory elements (brand mention, tagline, disclaimer)
- Deadline (schedule now vs later)

## Non-Negotiable Guardrails

- Never recommend geo spoofing, account farming, fake engagement warmups,
  deceptive sweepstakes or lead-gen funnels, brand-confusing domains, fake
  reactions or testimonials, or hidden sponsorship or affiliate intent.
- If a user brings a noisy creator thread or "growth hack" playbook, extract
  only the reusable creative mechanics: pattern mining, hook libraries, pacing,
  format decomposition, and lower-friction CTAs.
- Abstract the structure, not the exact creator, face, script, or claim. Use
  references to understand what works, not to imitate a specific person or make
  claims the brand cannot support.

---

## Step 1: Signal In

**Always seed with a trending query, even when the user supplies a brief.** Organic content lives on pattern recognition — the engine needs current hook shapes, not just the topic.

Call `get_trending_content` with `topic` from the brief. If the dataset is empty or stale, call `apify-plan-signal-collection` and then `apify-run-signal-collection-target` to kick off a run, then proceed with whatever you have while signal collects in the background.

Extract from the signal:
- **Hook patterns** — the first sentence/line of top performers
- **Engagement outliers** — posts with breakout scores, even on modest accounts
- **Hashtag clusters** — what tags accompany high-engagement content in this niche
- **Gaps** — themes competitors are ignoring

Write the signal summary into the session as context for subsequent steps.

For creator-style short-form video, also break 5-10 winning examples into
reusable fields: hook archetype, face or emotion cue, proof beat, reveal,
camera distance, screen action, CTA shape, and pacing. See
[references/short-form-ugc-playbook.md](references/short-form-ugc-playbook.md)
for the pattern-extraction workflow and compliant adaptation rules.

---

## Step 2: Format Route

Pick the production tool based on the brief. See [references/format-routing.md](references/format-routing.md) for the full decision table. Short form:

- **Trend-jack / UGC vibe** → agent-media (creator-style talking-head from script)
- **Cinematic / scene-heavy / product-in-world / continuity-sensitive** → Seedance via `openrouter-video-create`
- **Structured promo with text + graphics** → HyperFrames (reusable composition + deterministic rerenders)
- **Bespoke branded motion graphics** → Hyperframes (custom HTML/GSAP composition; use `shader-authoring` for GLSL/WebGL layers)
- **Static image + caption** → `generate_image` + caption tournament
- **Original jingle / sonic logo / intro, outro, or music bed** → `create_lyria_music_audio_artifact`

**Write down your choice and justify it in one line.** This is the single most important design decision in the pipeline and it should be legible in the output.

For a multi-platform pack, route each platform independently after defining the
shared thesis. A LinkedIn narrative, X thread, TikTok script, newsletter issue,
and Instagram carousel should not inherit the same hook or structure by default.
Each platform gets its own angle, format, and gate.

---

## Step 3: Text Tournament (Pre-Render Gate)

Generate **5 hooks and 5 captions** varying angle, not just word choice. Use the angle categories from the ad-creative skill (pain point, outcome, curiosity, contrarian, social proof, identity) and the signal patterns from Step 1.

Call `compare_content_variants` once for the hook set (`content_type='headline'`)
and once for the caption set (`content_type='social_post'`). Generate the
variants first, then batch-score the set directly instead of calling
`evaluate_content_engagement` on each variant one by one.

**Relative gate:**
- Winner must beat median score by **≥5 points**
- If spread is tighter (all variants score within 4 points), the concept is mediocre — regenerate the whole batch with a critique that demands higher variance ("vary angle more aggressively; the current batch is all the same shape")
- Absolute backstop: nothing scoring below **40** proceeds, ever

Record the winning hook + caption and the score report.

For multi-platform packs, do not choose one universal hook. For production-ready
assets or recurring systems, score hooks or captions by platform family when the
platforms differ materially. For quick draft packs, at minimum choose a distinct
angle and hook per channel and note that scoring should happen before
publishing. The test is: if someone follows the brand on every selected channel,
would the outputs feel repetitive? If yes, the pack is reformatting instead of
rethinking.

---

## Step 4: Intermediate Asset + Score

This step only applies to formats with a visual intermediate:

- **Video formats that use a hero image** (especially Seedance when face/product continuity matters, or Hyperframes with a product still) — generate the hero with `generate_image`, then score it with `evaluate_image_engagement` (`content_type='hero'`).
- **agent-media UGC** — no intermediate asset; the winning script IS the intermediate.
- **HyperFrames composition** — the winning hook + beats become the composition timeline, copy, and motion cues. Before authoring HTML/CSS, read `/workspace/library/design/DESIGN.md` when present and map its colors, fonts/typefaces, imagery rules, component treatments, motion tone, and do/don't constraints into the composition. If the user already has a still image or source clip, treat that media as the anchor asset and build the longer composition around it. Use `shader-authoring` when the concept needs procedural WebGL, shader backgrounds, SDF/raymarched visuals, particles, or shader-led atmosphere.
- **Static image + caption** — the image is both the intermediate AND the final deliverable.

For image intermediates, apply the same relative gate. If the image scores below the batch median, regenerate with a critique based on the subscore that dragged it (e.g. "attention=38 — increase foreground contrast, tighten composition on the subject, reduce background noise").

---

## Step 5: Render the Winner

Use the format selected in Step 2 with the winning text from Step 3 and, where applicable, the winning image from Step 4. One render only — this is a winner-takes-all pipeline, not a batch renderer.

As soon as the render is ready, return the playable artifact or signed media URL so the user can review it. Do not hide delivery while scoring runs.

**Per-format briefing rules:**

- **HyperFrames** — brief in beats first, then turn those beats into a full self-contained HTML/CSS/GSAP composition that follows `/workspace/library/design/DESIGN.md` when present. Use the stage structure with `data-composition-id` and timing attributes from the HyperFrames tool schema.
- **agent-media** — respect script pacing: ≤12 words for 5s, ≤25 for 10s, ≤37 for 15s. Pick the shortest duration that holds the hook and CTA.
- **Seedance** — use this prompt frame in order: `FORMAT`, `SUBJECT`, `ENVIRONMENT`, `MOOD`, `STYLE`, `LOGIC RULE`. Keep it concrete. The GIC `openrouter-video-create` tool is capped at 3-10 seconds, supports `16:9`, `9:16`, or `1:1`, and goes up to `1080p`. If the user wants to animate a provided still image, pass it as `conditioning_image_url` so Seedance uses it as the first frame. If the user wants a second clip after an existing video, pass the prior clip's playable signed URL as `continuation_source_video_url`; the tool extracts the prior video's last frame and uses it as the next clip's first frame. If the source image lives in the workspace or came in as an upload, call `get_workspace_file_signed_url` first. If continuity matters beyond that first frame, still make the logic rule explicit (`same face`, `same product color`, `same wardrobe`, `same jewelry`, etc.).
- **Lyria music** — use `create_lyria_music_audio_artifact` when the content needs
  an original jingle, sonic logo, intro/outro cue, or background bed. Prompt
  with genre, energy, tempo/BPM if known, instrumentation, audience, placement,
  duration, vocal/lyric constraints, and whether it should loop or resolve. Do
  not ask for a living artist, copyrighted song, or soundalike.
- **generate_image** — specify aspect ratio to match placement (1:1, 9:16, 16:9, 1.91:1). Supply reference images when brand consistency matters.

---

## Step 6: Final Scoring (Post-Render Gate)

**This is the gate that makes the engine actually compelling.** Any rendered asset gets scored before it turns into a draft, scheduling handoff, or final recommendation.

### Video

Call `evaluate_video_engagement` with the rendered video URL when the clip is
30 seconds or shorter. For longer renders, split the video into independent
scoreable chunks of 30 seconds or less, render/export those chunk URLs, score
each chunk separately with `source_video_url`, `source_start_seconds`, and
`source_end_seconds`, and return the chunk scores with their source time ranges.
The three numbers that matter:

1. **`hook_score`** (first 2 seconds) — if below the floor, the opener is failing. Regenerate ONLY the opening segment with a critique derived from the weak subscore (attention / emotion / visual).
2. **Peak placement** — `peak_moments` should have at least one peak within the first 40% of the timeline. If all peaks are in the back half, the pacing is backloaded and viewers drop before they hit the good part. Re-cut or regenerate with tightened pacing.
3. **Overall score** — sanity check against the floor.

### Image

Call `evaluate_image_engagement` with the rendered image URL. Check `attention` and `visual` subscores — these are the scroll-stop signals. If either is below floor, regenerate with a critique.

### Retry budget

**One critiqued retry per gate.** If the retry still fails, stop. Return to the user with both candidates, the score reports, and a recommendation. Do not ship a failing asset.

---

## Step 7: Postiz Draft (Terminal State)

**Never call `postiz-schedule-social-post` directly.** Always land in a Postiz draft via `postiz-create-social-draft`. Attach the full score report to the draft notes so the reviewer sees exactly why this version won.

For Mode 4 platform-native content packs, this terminal state runs per selected
platform. Create one reviewable output per platform and, when drafting to
Postiz, one draft per provider. Do not collapse the pack into one global winner,
one global render, or one catch-all draft. Text-only platforms can skip render;
visual or video platforms render only their own asset when requested.

Draft contents:
- Final caption (the winner from Step 3)
- Rendered media URL (from Step 5)
- Suggested schedule time (use platform-native best windows — TikTok 7–9pm local, LinkedIn Tue–Thu 8–10am, etc.)
- Providers list (explicit — never empty)
- Score report summary:
  ```
  Hook score: 72 (floor 60, Δ+12)
  Peak at: 0:03, 0:08 (good — early)
  Caption: 68 (batch median 59, Δ+9)
  Hero image: 71 attention (floor 60)
  Tournament: winner beat median by 11 points across 5 variants
  ```

The user reviews, approves, and schedules via Postiz UI. That's the human gate.

If the draft belongs to an ongoing campaign or recurring loop, also log the
signal source, winning angle, score report, and recommended next test in
`artifacts/experiment-ledger.md` so the next run starts from evidence instead
of memory.

## Step 8: Optional Review Learning

When the user approves, rejects, edits, or comments on generated content, treat
that feedback as a reusable signal only when they are building an ongoing
campaign, founder loop, recurring content system, or explicitly ask you to
remember it:

- Capture the decision, reason, platform, hook type, angle, format, and score
  report in `artifacts/experiment-ledger.md` for recurring systems. For one-off
  drafts, summarize the learning in the response instead of writing artifacts.
- Summarize durable account-specific patterns only after repeated decisions or
  owned-performance evidence. Keep them explicitly evidence-labeled.
- Do not overfit to one approval or one decline. Turn single decisions into
  hypotheses for the next test; turn repeated decisions into rules.
- Never claim the agent learned from the user's profile unless profile data or
  connected owned-performance data was actually available.

---

## Floors and Thresholds

These are defaults — tune per project based on what you learn from posted-performance correlation.

| Gate | Metric | Floor | Relative rule |
|------|--------|-------|---------------|
| Text tournament | overall | 40 absolute | winner beats median by ≥5 |
| Hero image | attention + visual | 55 each | winner beats median by ≥5 |
| Video final | hook_score | 60 | also: ≥1 peak_moment in first 40% |
| Static final | attention | 60 | — |

If a floor feels wrong on real data, raise or lower it in the skill — no code changes needed.

---

## Mode Variants

The engine supports four invocation modes. They share the spine above; only the seed differs.

### Mode 1 — Launch post
User is announcing something (product, feature, milestone). Signal query is narrower (niche + launch-adjacent terms). Format router biases toward a structured HyperFrames promo or static image + caption. Tournament emphasizes outcome and social-proof angles.

### Mode 2 — Trend-jack
User wants to ride a specific trend. Signal query is wider. Format router biases toward agent-media UGC or Seedance. Tournament emphasizes curiosity and contrarian angles. Hook test is especially strict — trend-jacks live or die on the first second.

### Mode 3 — Weekly drop (batch)
User wants a week of content. Run the full pipeline N times with different angle seeds, stagger schedule times in the Postiz drafts across days and platforms. Each draft independently gated — no free passes from batch economics.

### Mode 4 — Platform-native content pack
User wants one idea, asset, launch, article, or transcript turned into multiple
social outputs. First extract the canonical thesis, proof points, audience, and
CTA. Then assign a distinct platform angle before drafting:
- X / Threads: concise take, thread, proof post, or conversation starter.
- LinkedIn: business narrative, lesson, case study, or professional point of
  view.
- TikTok / Reels / Shorts: one visual proof beat or creator-style script, not a
  spoken summary of the text post.
- Instagram: carousel or visual-first caption with a save/share reason.
- Newsletter: deeper personal or tactical expansion with extra context.

Do not output the same post with different lengths. Each platform draft should
stand alone as useful to someone who already saw the other versions.

Mode 4 overrides the single-winner production path: run the draft/render/scoring
loop separately for each selected platform. For quick draft packs, provide one
clearly labeled draft per platform. For production-ready packs, create separate
Postiz drafts per platform/provider with their own angle, caption, media URL
when applicable, and score notes.

---

## Common Mistakes

- **Skipping signal query** — "I know what's trending" is almost always wrong for 2-week-old trends. Always query.
- **Scoring after render only** — pre-render text tournament catches 70% of failures cheap. Don't skip it to save tokens.
- **Single-variant generation** — tournament with N=1 is not a tournament. Minimum 5.
- **Universal hooks in platform packs** — a good X hook is not automatically a
  good LinkedIn opener, TikTok first second, or newsletter subject line.
- **Absolute thresholds over relative** — absolute floors drift as the model improves. Relative gates don't. Lead with relative.
- **Direct scheduling** — never. Always Postiz draft. The user is the final gate.
- **No score report in the draft** — if the reviewer can't see why this variant won, they can't calibrate their taste against the scorer. Always attach the report.
- **Fake profile DNA** — do not claim account-specific pattern knowledge unless
  profile or owned-performance evidence is present. Use explicit feedback and
  logged decisions as the starting memory instead.
- **Ignoring hook_score** — the first 2 seconds is where social video fails. If hook_score is weak, regenerate the opening — do not ship and hope.
- **Vague Seedance prompts** — "cinematic product video" is not a brief. Specify format, subject, environment, mood, style, and a logic rule for continuity.
- **Encoding transient vendor chatter into durable guidance** — do not bake third-party pricing claims or UI/vendor preferences into the skill. Keep reusable guidance focused on routing, prompt shape, and the actual GIC tool constraints.

---

## Related Skills

- **ad-creative** — Paid ad creative at scale (sibling skill for paid, this one for organic)
- **copywriting** — Landing page copy
- **marketing-psychology** — Principles behind high-performing creative
- **ab-test-setup** — For structured testing of posted content

## References

- [references/format-routing.md](references/format-routing.md) — Format decision table and tool briefing rules
- [references/short-form-ugc-playbook.md](references/short-form-ugc-playbook.md) — Reverse-engineering and briefing short-form UGC without copying sketchy growth tactics
