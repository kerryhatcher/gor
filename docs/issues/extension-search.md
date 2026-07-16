---
tags: [extension, read]
priority: P3
phase: 4
endpoints: []
status: todo
blockedBy: [extension-list]
blocks: []
---

# Extension Search

## As a

developer looking for useful extensions

## I want

to search for available gor extensions

## Acceptance criteria

1. Running `gor extension search <query>` searches for extensions matching the query
2. Results show: extension name, description, author, and star count
3. `--limit` / `-L` flag caps results (default: 30)
4. `--json` flag outputs as JSON with optional field selection
5. `--hostname` flag targets a specific host
6. If no results are found, a clear message is shown
7. The search queries a well-known extension registry (e.g., GitHub topics)

## Out of scope

- Installing extensions from search results (use `gor extension install`)
- Filtering by category or tag
