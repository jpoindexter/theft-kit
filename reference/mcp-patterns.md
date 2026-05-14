---
name: MCP Patterns
description: Model Context Protocol server patterns for Hashmark's codebase intelligence MCP server
tools: [Read, Write, Edit, Glob, Grep, Bash, LSP]
---

# MCP Server Patterns — Hashmark

## Project Context

Hashmark ships an MCP server that exposes scanned repository intelligence to AI coding assistants. The MCP server enables tools like Claude Code and Cursor to query AGENTS.md content, search chunks, and codebase metrics directly.

## MCP Server Location

The MCP server lives in `packages/cli/` alongside the CLI scanner. It shares types and utilities with the CLI package.

## Tool Design Principles

### Tools should be narrow and composable
Each MCP tool does one thing. Don't build a mega-tool that accepts 10 parameters.

```typescript
// Good — focused tools
server.tool('get_agents_md', { repoPath: z.string() }, async ({ repoPath }) => { ... });
server.tool('search_codebase', { query: z.string(), limit: z.number().default(10) }, async ({ query }) => { ... });
server.tool('get_complexity_report', { repoPath: z.string() }, async ({ repoPath }) => { ... });

// Bad — everything in one tool
server.tool('query_repo', { action: z.enum(['get', 'search', 'complexity']), ... }, ...);
```

### Always validate with Zod
Every tool input schema uses Zod. No raw `any` in tool parameters.

```typescript
import { z } from 'zod';

const SearchInput = z.object({
  query: z.string().min(1).max(200),
  limit: z.number().int().min(1).max(50).default(10),
  sectionType: z.enum(['overview', 'architecture', 'api', 'commands']).optional(),
});
```

### Return structured content
MCP tools return `{ content: [{ type: 'text', text: string }] }`. For structured data, serialize to JSON or markdown — don't return raw objects.

```typescript
return {
  content: [{
    type: 'text',
    text: JSON.stringify(results, null, 2),
  }],
};
```

## Search Tool Pattern

Hashmark uses BM25 keyword search in-process (no DB round-trip for MCP). The search chunks are loaded from the scanned AGENTS.md content.

```typescript
server.tool(
  'search_agents_md',
  'Search the AGENTS.md content for relevant sections',
  {
    query: z.string().describe('Search query'),
    limit: z.number().default(5).describe('Max results to return'),
  },
  async ({ query, limit }) => {
    const chunks = loadSearchChunks(); // loaded from local scan output
    const results = bm25Search(chunks, query, limit);
    return {
      content: [{
        type: 'text',
        text: results.map(r => `## ${r.heading}\n\n${r.content}`).join('\n\n---\n\n'),
      }],
    };
  }
);
```

## Error Handling

MCP tool errors should be user-readable, not raw stack traces. Never expose file paths, tokens, or internal state in error messages.

```typescript
async ({ repoPath }) => {
  try {
    const result = await processRepo(repoPath);
    return { content: [{ type: 'text', text: result }] };
  } catch (err) {
    const msg = err instanceof Error ? err.message : 'Unknown error';
    return {
      content: [{ type: 'text', text: `Error: ${msg}` }],
      isError: true,
    };
  }
}
```

## CLI Integration

The MCP server is invoked via `npx hashmark --mcp` or the installed binary. The CLI scanner and MCP server share:
- `packages/cli/src/types.ts` — shared types
- `packages/cli/src/scanners/` — AST complexity, imports, framework detection
- `packages/cli/src/generator.ts` — AGENTS.md generator

## AST Complexity Tools

Hashmark's custom AST complexity module (`packages/cli/src/scanners/ast-complexity.ts`) exposes 4 metrics:
- **Cyclomatic** (McCabe) — decision paths
- **Cognitive** (SonarQube) — mental complexity
- **Halstead** — vocabulary/effort
- **Maintainability Index** (Visual Studio variant) — `max(0, (171 - 5.2*ln(V) - 0.23*CC - 16.2*ln(LOC)) * 100/171)`

Cognitive gotcha: `a && b && c` = +1 (same-op chain), not +3. Count only when operator changes.

## Testing MCP Tools

Use the MCP inspector for manual testing:
```bash
npx @modelcontextprotocol/inspector packages/cli/dist/mcp-server.js
```

For unit tests (Vitest), mock the file system and test tool handlers directly without spinning up the full MCP server.

## Key Files

- `packages/cli/src/mcp-server.ts` — MCP server entry point
- `packages/cli/src/scanners/ast-complexity.ts` — complexity metrics (~700 lines)
- `packages/cli/src/generator.ts` — AGENTS.md content generator
- `packages/cli/src/commands.ts` — CLI + MCP command routing
