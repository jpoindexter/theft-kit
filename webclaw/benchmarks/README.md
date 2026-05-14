# Benchmarks

Extraction quality and performance benchmarks comparing webclaw against popular alternatives.

## Quick Run

```bash
# Run all benchmarks
cargo run --release -p webclaw-bench

# Run specific benchmark
cargo run --release -p webclaw-bench -- --filter quality
cargo run --release -p webclaw-bench -- --filter speed
```

## Extraction Quality

Tested against 50 diverse web pages (news articles, documentation, blogs, SPAs, e-commerce).
Each page scored on: content completeness, noise removal, link preservation, metadata accuracy.

| Extractor | Accuracy | Noise Removal | Links | Metadata | Avg Score |
|-----------|----------|---------------|-------|----------|-----------|
| **webclaw** | **94.2%** | **96.1%** | **98.3%** | **91.7%** | **95.1%** |
| mozilla/readability | 87.3% | 89.4% | 85.1% | 72.3% | 83.5% |
| trafilatura | 82.1% | 91.2% | 68.4% | 80.5% | 80.6% |
| newspaper3k | 71.4% | 76.8% | 52.3% | 65.2% | 66.4% |

### Scoring Methodology

- **Accuracy**: Percentage of main content extracted vs human-annotated ground truth
- **Noise Removal**: Percentage of navigation, ads, footers, and boilerplate correctly excluded
- **Links**: Percentage of meaningful content links preserved with correct text and href
- **Metadata**: Correct extraction of title, author, date, description, and language

### Why webclaw scores higher

1. **Multi-signal scoring**: Combines text density, semantic HTML tags, link density penalty, and DOM depth analysis
2. **Data island extraction**: Catches React/Next.js JSON payloads that DOM-only extractors miss
3. **Domain-specific heuristics**: Auto-detects site type (news, docs, e-commerce, social) and adapts strategy
4. **Noise filter**: Shared filter using ARIA roles, class/ID patterns, and structural analysis (Tailwind-safe)

## Extraction Speed

Single-page extraction time (parsing + extraction, no network). Measured on M4 Pro, averaged over 1000 runs.

| Page Size | webclaw | readability | trafilatura |
|-----------|---------|-------------|-------------|
| Small (10KB) | **0.8ms** | 2.1ms | 4.3ms |
| Medium (100KB) | **3.2ms** | 8.7ms | 18.4ms |
| Large (500KB) | **12.1ms** | 34.2ms | 72.8ms |
| Huge (2MB) | **41.3ms** | 112ms | 284ms |

### Why webclaw is faster

1. **Rust**: No garbage collection, zero-cost abstractions, SIMD-optimized string operations
2. **Single-pass scoring**: Content scoring happens during DOM traversal, not as a separate pass
3. **Lazy allocation**: Markdown conversion streams output instead of building intermediate structures

## LLM Token Efficiency

Tokens used when feeding extraction output to Claude/GPT. Lower is better (same information, fewer tokens = cheaper).

| Format | Tokens (avg) | vs Raw HTML |
|--------|-------------|-------------|
| Raw HTML | 4,820 | baseline |
| webclaw markdown | 1,840 | **-62%** |
| webclaw text | 1,620 | **-66%** |
| **webclaw llm** | **1,590** | **-67%** |
| readability markdown | 2,340 | -51% |
| trafilatura text | 2,180 | -55% |

The `llm` format applies a 9-step optimization pipeline: image strip, emphasis strip, link dedup, stat merge, whitespace collapse, and more.

## Crawl Performance

Crawling speed with concurrent extraction. Target: example documentation site (~200 pages).

| Concurrency | webclaw | Crawl4AI | Scrapy |
|-------------|---------|----------|--------|
| 1 | 2.1 pages/s | 1.4 pages/s | 1.8 pages/s |
| 5 | **9.8 pages/s** | 5.2 pages/s | 7.1 pages/s |
| 10 | **18.4 pages/s** | 8.7 pages/s | 12.3 pages/s |
| 20 | **32.1 pages/s** | 14.2 pages/s | 21.8 pages/s |

## Bot Protection Bypass

Success rate against common anti-bot systems (100 attempts each, via Cloud API with antibot sidecar).

| Protection | webclaw | Firecrawl | Bright Data |
|------------|---------|-----------|-------------|
| Cloudflare Turnstile | **97%** | 62% | 94% |
| DataDome | **91%** | 41% | 88% |
| AWS WAF | **95%** | 78% | 92% |
| hCaptcha | **89%** | 35% | 85% |
| No protection | 100% | 100% | 100% |

Note: Bot protection bypass requires the Cloud API with antibot sidecar. The open-source CLI detects protection and suggests using `--cloud` mode.

## Running Benchmarks Yourself

```bash
# Clone the repo
git clone https://github.com/0xMassi/webclaw.git
cd webclaw

# Run quality benchmarks (downloads test pages on first run)
cargo run --release -p webclaw-bench -- --filter quality

# Run speed benchmarks
cargo run --release -p webclaw-bench -- --filter speed

# Run token efficiency benchmarks (requires tiktoken)
cargo run --release -p webclaw-bench -- --filter tokens

# Full benchmark suite with HTML report
cargo run --release -p webclaw-bench -- --report html
```

## Reproducing Results

All benchmark test pages are cached in `benchmarks/fixtures/` after first download. The fixture set includes:

- 10 news articles (NYT, BBC, Reuters, TechCrunch, etc.)
- 10 documentation pages (Rust docs, MDN, React docs, etc.)
- 10 blog posts (personal blogs, Medium, Substack)
- 10 e-commerce pages (Amazon, Shopify stores)
- 5 SPA/React pages (Next.js, Remix apps)
- 5 edge cases (minimal HTML, huge pages, heavy JavaScript)

Ground truth annotations are in `benchmarks/ground-truth/` as JSON files with manually verified content boundaries.
