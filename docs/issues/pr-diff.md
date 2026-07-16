---
tags: [pr, read]
priority: P1
phase: 1
endpoints:
  - GET /repos/{owner}/{repo}/pulls/{number}
status: done
blockedBy: [pr-view]
blocks: []
---

# PR Diff

## As a

developer reviewing code changes

## I want

to see the diff of a pull request

## Acceptance criteria

1. Running `gor pr diff 42` shows the unified diff of PR #42
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--color` flag controls colorized output (`always`, `never`, `auto`)
4. `--patch` flag shows the diff in patch format (default)
5. `--name-only` flag shows only the names of changed files
6. `--hostname` flag targets a specific host
7. The diff is streamed to stdout for piping to `less` or other tools

## Out of scope

- Side-by-side diff view
- Word-level diff highlighting
- Diff between any two arbitrary refs (use `gor api`)
