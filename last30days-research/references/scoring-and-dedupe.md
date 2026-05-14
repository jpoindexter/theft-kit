# Scoring, Normalization, And Dedupe

Use this file to turn raw scraped posts into a ranked, clustered shortlist. Ranking by raw view count almost always produces a bad brief because one viral creator dominates and cross-surface duplicates inflate signal.

## Step 1 — Normalize engagement per surface

Each surface weights engagement differently. Do not compare raw numbers across surfaces.

Per-post engagement score (EPS), roughly:

- **YouTube long-form**: `views + 50*likes + 200*comments`
- **YouTube shorts**: `views + 30*likes + 100*comments`
- **TikTok**: `views + 20*likes + 100*comments + 200*shares`
- **Instagram reels**: `plays + 25*likes + 100*comments + 150*saves`
- **Instagram posts**: `likes + 50*comments + 100*saves`
- **Reddit**: `upvotes + 5*comments`
- **Blog / website**: backlink count (if available) or log-scaled organic traffic estimate
- **HN**: `points + 2*comments`

These weights are coarse — the point is to not let view-heavy surfaces drown comment-heavy ones.

## Step 2 — Normalize against author baseline

A 100k-view video from a 10k-subscriber channel is a bigger signal than a 1M-view video from a 5M-subscriber channel. Compute baseline lift wherever you have enough data.

Baseline lift:
```
lift = post_EPS / median(last_20_posts_EPS_same_author)
```

When baseline data is missing, fall back to:
```
lift = post_EPS / median(all_posts_in_this_surface_this_scrape)
```

Rank by `lift`, not raw EPS. Surface posts with `lift >= 2.0` as breakouts. Lift below 1.0 is noise.

## Step 3 — Per-author cap

Cap any single author's contribution to the top-N shortlist at 2 items. Otherwise one viral creator owns the brief.

When an author appears more than twice in the raw ranking, keep their two highest-lift posts and drop the rest into a "see also" section for that author.

## Step 4 — Cross-surface dedupe

Creators cross-post. The same hook appears as a TikTok, a Reel, and a YouTube short. Count it once.

Merge into one cluster when:
- Same author AND same title/hook substring match, OR
- Same claim or demo across two surfaces within 72 hours, OR
- Same URL or landing page appears in 3+ posts regardless of creator

Keep the highest-lift surface's link as canonical; list the others as "also on".

## Step 5 — Cluster into themes

For each surviving post, tag:
- **Theme**: the topic angle (e.g., "pricing transparency", "AI codegen horror story", "founder pov day-in-life")
- **Format**: the archetype (e.g., "founder-to-camera explainer", "before/after screen recording", "talking head over b-roll", "carousel with data")
- **Hook type**: the first-3-seconds device (e.g., "bold claim", "number reveal", "question", "pattern-break visual")
- **Implied angle**: what this post tells us about what's working (one sentence)

Cluster posts sharing theme AND (format OR hook type). A cluster with fewer than 2 independent creators is not a theme — it's an outlier post.

## Step 6 — Rank clusters

For each cluster:
```
cluster_score = sum(post_lift) * sqrt(unique_creators) * surface_diversity_bonus
```

Where `surface_diversity_bonus = 1.0` if single surface, `1.3` if 2 surfaces, `1.5` if 3+.

Return the top 5-10 clusters as "themes" and the top 3-5 format archetypes as "winning formats". Anything below cluster_score threshold goes into an appendix or gets dropped.

## Step 7 — Breakout creator detection

Separate pass: for each creator in the scrape with 3+ posts in window, compute:
```
creator_lift = median(post_lift in window) / median(post_lift in prior 90 days)
```

Creators with `creator_lift >= 1.5` and at least one post above `post_lift = 2.0` are breakout candidates. Return up to 10.

## Rules of thumb

- If a theme only has one creator behind it, it's an outlier post, not a theme.
- If all your "themes" come from one surface, you scraped too narrowly — note it in the deliverable.
- If you cannot compute baseline lift (new accounts, sparse history), say so and rank by EPS + surface diversity instead.
- Prefer showing the founder 5 tight themes over 20 noisy ones.
