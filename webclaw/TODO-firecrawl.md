# Stuff to pull from Firecrawl + Scrapy

Patterns and ideas from Firecrawl's open source (AGPL). Don't copy code, replicate the patterns.

## Engine routing

- [ ] Smart engine picker -- track success/failure per domain, auto-route to best engine
- [ ] Skip HTTP attempt for known-SPA domains (supabase.com, ycombinator.com, etc)
- [ ] Levenshtein distance comparison between HTTP vs browser results to detect incomplete renders
- [ ] Fallback chain config: HTTP -> browser -> stealth browser -> proxy+browser

## Async job system

- [ ] Don't block API response waiting for scrape to finish
- [ ] Return job ID immediately, client polls for result
- [ ] Redis + BullMQ (or Rust equivalent) for job queue
- [ ] Webhook delivery -- POST results to a URL when done
- [ ] Job priority tiers (paid users get faster processing)

## Scraping improvements

- [ ] Pre-scrape actions: click, scroll, wait, type before extracting (for cookie banners, load-more buttons)
- [ ] Media blocking option -- skip images/CSS to reduce detection surface
- [ ] Mobile user-agent option (some sites serve simpler pages to mobile)
- [ ] Geolocation/language spoofing per request
- [ ] Screenshot capture alongside extraction
- [ ] Wikipedia/Reddit/LinkedIn specialized handlers (already have LinkedIn + Reddit modules, extend them)

## Proxy and stealth

- [ ] Residential proxy pool integration (Bright Data, Oxylabs, or similar)
- [ ] Mobile proxy rotation for stealth mode
- [ ] Per-domain proxy assignment (sticky sessions for multi-page scrapes)
- [ ] Proxy health tracking -- remove dead proxies automatically

## API features

- [ ] Batch scrape endpoint -- POST /v1/batch with array of URLs
- [ ] Crawl endpoint -- POST /v1/crawl with depth/max_pages
- [ ] Map endpoint -- POST /v1/map returns all URLs on a site
- [ ] Search endpoint -- POST /v1/search (web search + scrape results)
- [ ] Extract endpoint -- POST /v1/extract with JSON schema or prompt (LLM-powered)
- [ ] Async versions of all endpoints with job polling

## Auth and billing

- [ ] API key generation and validation
- [ ] Credit-based billing (1 credit per scrape)
- [ ] Rate limiting per API key (Redis-backed)
- [ ] Team/org support
- [ ] Usage dashboard
- [ ] Stripe integration for payments
- [ ] Free tier (500 credits one-time)
- [ ] Zero-data-retention flag per request

## From Scrapy

- [ ] AutoThrottle -- adjust request delay dynamically based on response latency, not fixed timing
- [ ] Per-domain concurrency slots -- separate rate limits per domain instead of global
- [ ] Request fingerprinting -- hash URL+method+body for dedup during crawls
- [ ] robots.txt middleware -- respect robots.txt by default (can override)
- [ ] Middleware chain pattern -- pluggable request/response transformers
- [ ] Retry middleware -- configurable retry on 500/502/503/408 with backoff
- [ ] Redirect cycle detection -- catch infinite redirect loops

## Infrastructure

- [ ] Playwright as separate microservice (not spawned per-request)
- [ ] Browser pool -- keep N browsers warm, reuse across requests
- [ ] Redis for caching, rate limiting, job state
- [ ] Health monitoring and alerting
- [ ] Request logging and analytics

## Web app

- [ ] Landing page with try-it input
- [ ] API docs page
- [ ] Dashboard (API keys, usage, billing)
- [ ] Playground (paste URL, see result, get code snippet)
- [ ] "Get code" button -- generates curl/Python/Node snippet for the request
