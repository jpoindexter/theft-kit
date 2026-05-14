---
name: name
description: "Generate 15-20 product name candidates, check npm and domain availability for each, and filter to only available options. Use when someone says: name this, brainstorm names, what should I call this, name ideas."
---

# Name

Generate product name candidates and check availability. Filter to only names that are actually available.

## Naming Principles

- Short (1-2 words, ideally under 8 characters)
- Punchy. Memorable. Says something.
- Lowercase preferred
- Design-related, technical, or cleverly abstract
- No generic SaaS names (AppifyHub, CloudSync, DataFlow)
- No forced portmanteaus (IntelliSync, SmartFlow)
- Examples of strong names: sharp, precise, evocative, visual, sonic

## Process

1. **Understand the product**: Ask what it does if not clear from context. Key questions:
   - What does it do in one sentence?
   - Who is it for?
   - What feeling should the name evoke?

2. **Generate 15-20 candidates** across these categories:
   - **Literal**: Names that describe what it does (3-4 options)
   - **Abstract**: Names that evoke the feeling or concept (3-4 options)
   - **Metaphor**: Names drawn from other domains that fit (3-4 options)
   - **Short/Punchy**: Single syllable or very short options (3-4 options)
   - **Wordplay**: Clever twists, dropped letters, respellings (3-4 options)

3. **Check availability for each name**:
   - npm: Run `npm view [name]` to check if the package name is taken
   - Domains: Check .com, .dev, .io availability using web search or domain check tools
   - Note: If tools for domain checking are unavailable, flag which names to check manually

4. **Filter and present**: Only show names where at least npm OR a good domain is available.

5. **Iterate**: Ask for feedback. Generate more in the direction the user likes.

## Output Format

```
## Name Candidates for [Product]

| Name | npm | .com | .dev | .io | Vibe |
|------|-----|------|------|-----|------|
| kern | TAKEN | taken | available | taken | Sharp, precise, design-y |
| gripe | TAKEN | taken | available | available | Edgy, complaint-focused |
| [name] | available | available | ? | ? | [one-line description of the vibe] |

### Top 3 Recommendations
1. **[name]** — Why this works: [reason]. Available on: [platforms].
2. **[name]** — Why this works: [reason]. Available on: [platforms].
3. **[name]** — Why this works: [reason]. Available on: [platforms].

### Honorable Mentions
- [name]: Would be great but [platform] is taken
- [name]: Available but might be too [adjective]
```

## Rules

- Never present a name without checking npm availability first.
- If npm is taken, still show the name but mark it clearly. Sometimes npm conflicts don't matter (e.g., abandoned packages with 0 downloads).
- For npm packages with very low downloads (under 100/week), note this — the name might still be claimable or the user might not need npm.
- Quality over quantity. 10 good names beat 20 mediocre ones.
- Avoid names that are hard to spell, hard to pronounce, or easy to confuse with existing popular tools.
- Avoid names that are too generic to trademark.
- If the user is building for enterprise, the name should feel professional, not cute.
- Check that the name doesn't mean something unfortunate in major languages.
