---
tags: [cache, write]
priority: P4
phase: 4
endpoints:
  - DELETE /repos/{owner}/{repo}/actions/caches
status: done
blockedBy: [cache-list]
blocks: []
---

# Cache Delete

## As a

developer who needs to invalidate stale CI caches

## I want

to delete one or more caches by key

## Acceptance criteria

1. Running `gor cache delete my-key` deletes the cache with the exact key `my-key`
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--all` flag deletes all caches in the repository
4. `--key-prefix` flag deletes caches whose keys start with the given prefix
5. `--ref` flag scopes deletion to a specific branch/tag ref
6. A confirmation message is printed showing the number of deleted caches
7. `--hostname` flag targets a specific host
8. Exit code 0 on success

## Out of scope

- Partial cache invalidation (only full-key, prefix, or all)
- Bulk deletion across multiple repositories
