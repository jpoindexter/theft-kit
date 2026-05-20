# Source Priority And Cost Tradeoffs

Use this file to decide which surfaces to hit in what order for a last-30-days scan. Pick the smallest set that covers the niche.

## Tier 1 — Free, always cheap, use first when possible

These do not touch apify budget. Use them before spending anything.

### Reddit public JSON
- Append `.json` to any listing URL: `https://www.reddit.com/r/<sub>/top.json?t=month`
- Respect user-agent and rate limits; batch small requests.
- Best for: early-stage B2C, dev tools, prosumer niches, community sentiment.
- Limitation: no paid apify actor is wired yet in this codebase for deeper Reddit scraping. Call this out when scope exceeds public JSON.

### Hacker News via Algolia
- `https://hn.algolia.com/api/v1/search?tags=story&numericFilters=created_at_i>{ts}`
- Best for: dev/infra/AI niches.

### GitHub via `gh`
- Use a date qualifier to bound to the window, e.g. `gh search repos "<keywords> pushed:>$(date -u -v-30d +%Y-%m-%d)" --sort stars` (macOS) or the GNU equivalent `pushed:>$(date -u -d '30 days ago' +%Y-%m-%d)` (Linux). `gh api repos/<owner>/<repo>/stargazers` for star lift.
- Best for: open-source momentum, README changes, trending repos.

### Direct web fetch
- `WebFetch` on competitor blogs, sitemaps, category RSS feeds, comparison pages.
- Best for: blog-shaped content, pricing changes, positioning shifts on owned properties.

### Polymarket and public prediction markets
- `https://gamma-api.polymarket.com/markets` for narrative-market signal.
- Best for: topical/news niches where narrative matters.

## Tier 2 — Paid via apify, use when freshness and engagement metrics matter

All paid scrapes must go through the apify planning flow:

1. `apify-plan-signal-collection` — builds the plan without spending.
2. Confirm with user.
3. `apify-run-signal-collection-target` — runs a planned target end-to-end.

Supported surfaces today (mapped in `ApifyContentSource`):

### YouTube
- Default actor: `streamers/youtube-scraper`.
- Good for: long-form breakouts, channel-level baselines, comparison videos, tutorial content.
- Seed shapes: channel URLs, search keywords, playlist URLs.

### TikTok
- Default actor: `clockworks/tiktok-scraper`.
- Good for: short-form breakouts, hook patterns, creator-level lift.
- Seed shapes: hashtags, usernames, search keywords.

### Instagram
- Default actor: `apify/instagram-api-scraper`.
- Good for: reels performance, brand accounts, creator partnerships, UGC patterns.
- Seed shapes: usernames, hashtags, location tags.

### Website (generic)
- Actor routing goes through the site-scraper flow for blogs, landing pages, and comparison pages.
- Good for: competitor blog inventory, sitemap diffs, pricing-page changes.

## Not yet wired

- **Reddit at scale via apify**: no actor is currently in `ApifyContentSource`. For Reddit, use Tier 1 (public JSON) until a Reddit actor is wired in a follow-up. When the ask exceeds what public JSON can deliver, say so explicitly in the deliverable instead of silently skipping.
- **X/Twitter**: no default actor. Scope out or fall back to manual handles + WebFetch when absolutely needed.
- **LinkedIn**: not wired; do not attempt.

## Picking the surface mix

Start by asking: where does this niche actually buy attention?

- **B2B SaaS, infra, dev tools** → YouTube long-form + HN + GitHub + blogs. TikTok usually noise.
- **Consumer, creator economy, lifestyle** → TikTok + Instagram + YouTube shorts. Reddit for community voice.
- **Prosumer tools** → YouTube + Reddit (public JSON) + blogs.
- **News-driven / narrative** → Polymarket gamma + HN + Reddit public JSON + competitor blogs.
- **Physical/commerce** → Instagram + TikTok + blogs.

Two to three surfaces is almost always correct. Four or more wastes budget on signal that does not cluster.

## Cost discipline

- The apify cost constants in `backend/gic_backend/services/apify_service/types.py` are per-1000-results. Inflate by expected result count per target to estimate.
- If total estimated cost exceeds the user's stated budget or a reasonable default (~$5 for ad hoc scans), stop and confirm.
- Prefer narrower seeds (specific creators, specific keywords) over broad ones (entire hashtag).
- Use `maxItems` caps on each target when the actor supports it; do not run open-ended scrapes.
