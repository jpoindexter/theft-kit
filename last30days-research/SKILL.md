---
name: last30days-research
description: "When the user wants an ad hoc scan of what has actually worked in their niche over a recent window (usually the last 30 days) on YouTube, TikTok, Instagram, Reddit, blogs, or podcasts. Also use when the user says 'what's working right now', 'what's trending in [niche]', 'what's going viral this month', 'what are the breakout posts in our space', 'show me top content from our competitors recently', 'pull a last30days brief', 'do a trend scan', or wants a one-shot research brief before writing content, choosing a format, or prepping a launch. This is the one-off discovery complement to `external-signal-monitoring` (which sets up recurring loops). Use this to turn raw social/web activity into ranked themes, formats, creators, and breakout posts with clear routing into content, positioning, or comparison work."
metadata:
  version: 1.0.0
---

# Last 30 Days Research

You turn a recent window of social, video, and web activity into a ranked, routed brief the founder can act on today.

The goal is not to dump a spreadsheet of scraped posts. The goal is to answer: "What themes, formats, and creators are actually working in our niche right now, and what should we do about it?"

This is the one-off discovery complement to [external-signal-monitoring](../external-signal-monitoring/SKILL.md). Reach here when the user wants a single deep pass before writing content, picking a format, or prepping a launch. Reach for `external-signal-monitoring` when they want a recurring loop.

## Workflow

### Step 1: Check Shared Context First

Before scraping anything, check whether these exist:
- `.agents/product-marketing-context.md`
- `.claude/product-marketing-context.md` for older setups
- `.agents/brand-page-context.md`
- `.claude/brand-page-context.md` for older setups

If these files exist, read them first.

Use them to pull:
- ICP, positioning, core competitors, category terms, proof angles
- Brand voice and the formats the user can realistically produce
- Any stated founder priorities (launches, comparison content, hiring pushes)

If none exist, ask two to three targeted questions before spending research budget: niche or category, named competitors, and what the user plans to do with the brief (content, positioning, launch, comparison).

### Step 2: Define The Seed Set And Window

Be explicit before touching any tool. Write down:
- **Window**: usually last 30 days. Use last 14 or last 7 if the user asked for something sharper. Use last 90 only for evergreen category scans.
- **Surfaces**: YouTube, TikTok, Instagram, Reddit, blogs, podcasts. Pick the two or three that actually matter for the niche. Do not scrape all six by default.
- **Seeds**: the exact accounts, channels, subreddits, hashtags, or keywords to hit. Group them into three buckets:
  - `niche` — topic/keyword/hashtag/subreddit searches
  - `competitor` — named competitor accounts, channels, or domains
  - `owned` — the user's own handles when comparing against their own output

Call these out in the plan so the user can correct the seed set before cost is spent.

### Step 3: Plan The Collection

Call `apify-plan-signal-collection` with the niche, seed set (competitor_seeds, owned_seeds), and surfaces (sources). This returns the structured collection plan (targets, estimated cost per target, actors used) without spending money yet. The planner tool itself does not take a time-window argument — apply the 30-day (or whatever) window when you filter results after collection, not at plan time.

Show the plan to the user. Confirm:
- Expected cost against any stated budget
- That seeds cover the right creators and keywords
- That the surface mix is correct (e.g., don't scrape TikTok for a B2B SaaS niche)

Only proceed once the plan looks right. Scrapes are cheap individually but add up quickly across many targets.

### Step 4: Run Collection In The Right Tiers

Use two tiers and be explicit about which you are in:

**Deep tier (paid, apify)** — use when the signals must be fresh, complete, or include engagement metrics:
- `apify-run-signal-collection-target` to run each planned target (preferred; wraps the actor + enum + ingest).
- `apify-run-actor` + `apify-get-dataset-items` only if you need a custom actor that isn't already mapped in the signal collection types.

Supported apify surfaces today: YouTube, TikTok, Instagram, and website scrapes. See [references/source-priority.md](references/source-priority.md) for details.

**Free tier (no budget)** — always cheap, use first when possible or as a complement:
- Reddit public JSON (e.g., `https://www.reddit.com/r/<sub>/top.json?t=month`) for subreddit signal.
- Hacker News Algolia API for tech/dev niche scans.
- GitHub API via `gh` for repo momentum and README breakouts.
- Direct `WebFetch` on blogs, competitor sitemaps, or category RSS feeds.

If apify is unavailable or budget is constrained, run the whole pass on the free tier and say so clearly in the deliverable.

Reddit scraping via apify is not yet wired in this codebase — when the user asks for Reddit at scale beyond public JSON limits, say so and note it as a follow-up. Do not fake it.

### Step 5: Score And Cluster

Use [references/scoring-and-dedupe.md](references/scoring-and-dedupe.md).

For each post collected:
- Normalize engagement per surface (views, likes, comments, shares relative to author baseline where possible).
- Cap any single author's contribution to the ranking so a viral anomaly does not own the brief.
- Deduplicate cross-surface (same hook or claim on TikTok and Reels = one cluster, not two).
- Tag each cluster with theme, format, hook type, and implied angle.

Build a ranked shortlist of:
- **Themes** — topic angles that over-indexed across multiple creators.
- **Formats** — format archetypes that outperformed (e.g., "founder-to-camera pricing breakdown", "before/after screen recording").
- **Breakout creators** — accounts with unusual recent lift relative to their baseline.
- **Outlier posts** — single posts that dramatically over-indexed and deserve a specific response.

### Step 6: Route Into The Next Workflow

Hand each ranked item into the right execution path. Do not write the assets here — just point to the right skill and hand over a sharp brief:

- `content-strategy` → convert hot themes into a content calendar or pillar plan.
- `social-content` → convert winning formats into specific reaction posts or series briefs.
- `copywriting` → convert breakout angles into landing page or section updates.
- `competitor-alternatives` → when a competitor's content dominates, refresh the comparison page or build a new alternative angle.
- `external-signal-monitoring` → convert recurring patterns into a scheduled monitor so the founder doesn't re-run this scan by hand every month.
- `ad-creative` → when a format is visibly winning in paid placements, hand it over for ad variants.

If an accepted cluster has no sensible downstream skill, write it into `.agents/product-marketing-context.md` as an observed pattern rather than inventing make-work.

---

## Deliverable Format

Use this structure:

1. **Scope** — window, surfaces, seed set, tier(s) used, actual cost spent
2. **Top Themes** — ranked, each with a one-line "why now" and 2-3 supporting post links
3. **Winning Formats** — ranked, each with an archetype description and example links
4. **Breakout Creators** — 3-10 names with baseline-vs-recent lift and why they matter
5. **Outlier Posts** — up to 5 specific posts that warrant a direct response
6. **Recommended Next Moves** — 3-7 concrete handoffs routed into the skills above
7. **Follow-up Monitor** — one line: should this become a recurring `external-signal-monitoring` loop, yes/no, at what cadence

Keep the brief short enough that a founder reads it in one sitting. Put raw data and CSV exports in `artifacts/` if needed; do not dump them into chat.

---

## Guardrails

- Do not scrape without a confirmed plan from `apify-plan-signal-collection`. Seed sprawl is the main way cost gets wasted.
- Do not treat raw view counts as truth. Normalize against author baseline before ranking.
- Do not let one viral post or one creator dominate the brief. Cap per-author contribution.
- Do not invent Reddit-at-scale apify scraping. If the user needs it, say it's not wired and either fall back to Reddit public JSON or scope the work to surfaces that are wired (YouTube, TikTok, Instagram, websites).
- Do not claim a pattern is "trending" off a single post. Require at least two independent creators or two surfaces.
- Do not recommend shipping any asset from this skill. Route into the right downstream skill with a sharp brief instead.
- If apify is not connected or the user has no budget, run the full free tier pass and say so in the deliverable. Never fake engagement numbers.
- Keep the window honest. If the request is "last 30 days" but the strongest signal sits at day 35, note it rather than lying about the window.

---

## What Good Looks Like

A strong output should make the team say:
- "I can see what's working in our niche right now without opening a browser"
- "Each theme has a clear next move and it's already routed to the right skill"
- "We know which of these are durable enough to put a recurring monitor on"

A weak output:
- lists every post scraped without ranking or clustering
- mixes surfaces without normalizing engagement
- recommends work that duplicates what `external-signal-monitoring` should be doing
- silently skips apify budget discussion and bills up cost the user did not authorize

---

## Related Skills

- **external-signal-monitoring**: For the *recurring* version of this work. Use after a one-off `last30days-research` pass if the user wants the signal loop to keep running.
- **content-strategy**: For turning winning themes into a pillar plan or calendar.
- **social-content**: For converting winning formats into specific post briefs.
- **competitor-alternatives**: For responding when competitor content dominates the scan.
- **voice-of-customer-synthesis**: Complementary — that skill mines customer-owned data (calls, tickets, reviews); this one mines public-web activity.
- **ad-creative**: For turning organic winners into paid variants.
