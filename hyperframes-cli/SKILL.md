---
name: hyperframes-cli
description: HyperFrames CLI tool — hyperframes init, lint, preview, render, transcribe, tts, doctor, browser, info, upgrade, compositions, docs, benchmark. Use when scaffolding a project, linting or validating compositions, previewing in the studio, rendering to video, transcribing audio, generating TTS, or troubleshooting the HyperFrames environment.
---

# HyperFrames CLI

Everything runs through `npx hyperframes`. Requires Node.js >= 22 and FFmpeg.

## Workflow

1. **Scaffold** — `npx hyperframes init my-video`
2. **Write** — author HTML composition (see the `hyperframes` skill)
3. **Publish + auto-render** — call the `create_hyperframes_composition` tool. It writes the HTML under `artifacts/compositions/`, renders it to an MP4 under `artifacts/renders/`, uploads the MP4 to storage, and returns a signed `video_mp4_url` you can give the user.
4. **(Optional) Lint** — `npx hyperframes lint artifacts/` (only needed if you want stricter validation)
5. **(Optional) Preview** — `cd artifacts && npx hyperframes preview` (studio/hot-reload iteration)

**Do NOT separately run `npx hyperframes render`** — `create_hyperframes_composition` already rendered the MP4 and registered a tracked video artifact. Running `render` a second time would overwrite the MP4 but skip storage upload + artifact registration, and the user would lose the signed URL. If you need to re-render (e.g. you changed the HTML), just call `create_hyperframes_composition` again with the same `path` — the tool upserts.

**Success criteria:** `render_status=ready` plus a playable `video_mp4_url` means the render pipeline completed. It does **not** guarantee the composition is visually good. A READY render can still be a blank frame, a static frame, clipped text, or bad choreography if the HTML/CSS/GSAP is wrong.

Lint before preview — catches missing `data-composition-id`, overlapping tracks, unregistered timelines.

**Agent sandbox note:** inside the Cofounder sandbox, `create_hyperframes_composition` writes to `artifacts/compositions/*.html`, stages the render in hidden workspace scratch if needed, promotes the finished MP4 into `artifacts/renders/<slug>.mp4`, and registers `artifacts/compositions/<slug>.video.json`. Lint and preview target `artifacts/` so the CLI scans the same tree the tool writes to.

### Publish via the tool — never `cp`/`mv` and never a separate render

After authoring, call the `create_hyperframes_composition` tool with the final `path` (under `artifacts/compositions/`), `title`, `description`, and `html`. The tool will:

1. Write the HTML to the workspace (as a tracked artifact — library metadata row, title/description, UI visibility).
2. Render the MP4 in-sandbox via the bundled Chromium + ffmpeg stack.
3. Upload the MP4 to user-scoped Supabase storage.
4. Mint a 2-hour signed URL.
5. Register a `VideoArtifactFile` at `artifacts/compositions/<slug>.video.json` with `render_status=ready`.

Do NOT use `cp`, `mv`, `Write`, or any shell/filesystem command to move HTML into `artifacts/compositions/`. Do NOT invoke `npx hyperframes render` yourself — raw file moves and standalone render calls leave the composition on disk but invisible to the artifact UI, and the user will not get a playable MP4. Do NOT copy a finished MP4 into `artifacts/` hoping the UI will discover it — that does not publish it to storage or mint a signed URL.

If the tool returns `render_status=failed`, share the `render_error_message` with the user and check `npx hyperframes doctor` output. If it returns `render_status=ready`, give the user the `video_mp4_url` directly — that is the playable MP4 the user asked for. If the rendered video is blank or visually wrong, fix the composition and call `create_hyperframes_composition` again rather than trying to patch the MP4 file manually.

## Scaffolding

```bash
npx hyperframes init my-video                        # interactive wizard
npx hyperframes init my-video --example warm-grain   # pick an example
npx hyperframes init my-video --video clip.mp4        # with video file
npx hyperframes init my-video --audio track.mp3       # with audio file
npx hyperframes init my-video --non-interactive       # skip prompts (CI/agents)
```

Templates: `blank`, `warm-grain`, `play-mode`, `swiss-grid`, `vignelli`, `decision-tree`, `kinetic-type`, `product-promo`, `nyt-graph`.

`init` creates the right file structure, copies media, transcribes audio with Whisper, and installs AI coding skills. Use it instead of creating files by hand.

## Linting

```bash
npx hyperframes lint                  # current directory
npx hyperframes lint ./my-project     # specific project
npx hyperframes lint --verbose        # info-level findings
npx hyperframes lint --json           # machine-readable
```

Lints `index.html` and all files in `compositions/`. Reports errors (must fix), warnings (should fix), and info (with `--verbose`).

## Previewing

```bash
npx hyperframes preview                   # serve current directory
npx hyperframes preview --port 4567       # custom port (default 3002)
```

Hot-reloads on file changes. Opens the studio in your browser automatically.

## Rendering

**Agents should NOT call `npx hyperframes render` directly** — the `create_hyperframes_composition` tool renders automatically and registers the MP4 as a tracked artifact with a signed URL. The flags below are reference material for human operators working outside the agent loop (e.g. running `npx hyperframes` locally, debugging a failed render via `doctor`, or scripting a bespoke pipeline).

```bash
npx hyperframes render                                # standard MP4
npx hyperframes render --output final.mp4             # named output
npx hyperframes render --quality draft                # fast iteration
npx hyperframes render --fps 60 --quality high        # final delivery
npx hyperframes render --format webm                  # transparent WebM
npx hyperframes render --docker                       # byte-identical
```

| Flag           | Options               | Default                    | Notes                       |
| -------------- | --------------------- | -------------------------- | --------------------------- |
| `--output`     | path                  | renders/name_timestamp.mp4 | Output path                 |
| `--fps`        | 24, 30, 60            | 30                         | 60fps doubles render time   |
| `--quality`    | draft, standard, high | standard                   | draft for iterating         |
| `--format`     | mp4, webm             | mp4                        | WebM supports transparency  |
| `--workers`    | 1-8 or auto           | auto                       | Each spawns Chrome          |
| `--docker`     | flag                  | off                        | Reproducible output         |
| `--gpu`        | flag                  | off                        | GPU-accelerated encoding    |
| `--strict`     | flag                  | off                        | Fail on lint errors         |
| `--strict-all` | flag                  | off                        | Fail on errors AND warnings |

**Quality guidance:** `draft` while iterating, `standard` for review, `high` for final delivery.

## Transcription

```bash
npx hyperframes transcribe audio.mp3
npx hyperframes transcribe video.mp4 --model medium.en --language en
npx hyperframes transcribe subtitles.srt   # import existing
npx hyperframes transcribe subtitles.vtt
npx hyperframes transcribe openai-response.json
```

## Text-to-Speech

```bash
npx hyperframes tts "Text here" --voice af_nova --output narration.wav
npx hyperframes tts script.txt --voice bf_emma
npx hyperframes tts --list  # show all voices
```

## Troubleshooting

```bash
npx hyperframes doctor       # check environment (Chrome, FFmpeg, Node, memory)
npx hyperframes browser      # manage bundled Chrome
npx hyperframes info         # version and environment details
npx hyperframes upgrade      # check for updates
```

Run `doctor` first if rendering fails. Common issues: missing FFmpeg, missing Chrome, low memory.

## Other

```bash
npx hyperframes compositions   # list compositions in project
npx hyperframes docs           # open documentation
npx hyperframes benchmark .    # benchmark render performance
```
