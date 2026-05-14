---
name: post
description: "Generate platform-specific posts for Reddit, X, Bluesky, Dev.to, or Hacker News with authentic voice. Use when someone says: write me a post, post for reddit, post for x, post for bluesky, post for dev.to, post for hackernews."
---

# Post

Generate copy-pasteable posts tailored to specific platforms with authentic, direct voice.

## Voice Rules

- Casual and direct. Conversational, not performative.
- No em-dashes. Use periods or commas instead.
- No corporate speak. No "leverage", "synergy", "unlock", "empower", "revolutionize".
- No "I'm excited to announce" or "thrilled to share".
- Asks questions. Engages the reader.
- Shares what was learned, not what was achieved.
- Shows the work. Specific numbers, specific tools, specific decisions.
- Self-aware. Can acknowledge limitations or mistakes.
- No hashtags unless the platform requires them.
- No emoji spam. Zero or one emoji max per post.

## Platform Rules

### Reddit
- Respect subreddit culture. Ask which subreddit if not specified.
- Not promotional. Share value first, mention the product only if relevant and natural.
- Use the subreddit's typical post format (question, discussion, show-off, etc.).
- Include a TL;DR for longer posts.
- Be ready to engage in comments. The post should invite discussion.

### X (Twitter)
- 280 character limit. Be punchy.
- One clear idea per tweet.
- If a thread is needed, write each tweet to stand alone.
- No "thread incoming" or "1/" unless it's actually a thread.
- Hook in the first line.

### Bluesky
- 300 character limit.
- Similar to X but the audience skews more technical and less engagement-bait.
- No algorithm gaming. Just say the thing.

### Dev.to
- Technical blog format with proper markdown.
- Include a cover image suggestion (describe what it should show).
- Use headers, code blocks, and lists.
- Start with the problem, then the solution, then what was learned.
- Include a "What I'd do differently" section if relevant.
- Tag suggestions at the end.

### Hacker News
- "Show HN:" format for launches. Plain title for discussions.
- Zero hype. Understated. Let the work speak.
- The HN audience hates marketing language. Be technical and honest.
- Comment with context after posting (first comment explaining what it is and why).
- Acknowledge limitations upfront. HN respects honesty.

## Process

1. **Ask which platform** if not specified in the request.
2. **Ask what the post is about** if not clear from context.
3. **Draft the post** following platform rules and voice rules.
4. **Output the post as copy-pasteable text** in a code block so formatting is preserved.
5. **If Dev.to**: Output the full markdown including frontmatter.
6. **If X thread**: Number each tweet and show character count.
7. **Offer to iterate** based on feedback.

## Output Format

```
## [Platform] Post

[Copy-pasteable text in a code block]

Character count: X/Y (for X, Bluesky)
Suggested subreddit: r/example (for Reddit)
Suggested tags: #tag1 #tag2 (for Dev.to)
```

## Rules

- Never output multiple platform versions unless asked. One platform at a time.
- Always show character count for character-limited platforms.
- For Reddit, always suggest a subreddit and explain why.
- For Dev.to, suggest 3-4 tags.
- If the user wants to promote a product, find the non-promotional angle first. What did they learn? What problem did they hit? What's the interesting technical decision?
- Read any relevant project files or READMEs to understand what's being posted about.
