---
tags: [alias, write]
priority: P2
phase: 3
endpoints: []
status: todo
blockedBy: [alias-list]
blocks: []
---

# Alias Delete

## As a

developer who no longer needs an alias

## I want

to remove a configured `gor` alias

## Acceptance criteria

1. Running `gor alias delete co` removes the alias named `co`
2. The alias removal is persisted to `~/.config/gor/config.yml`
3. `--hostname` flag scopes the deletion to a specific host
4. Deleting a non-existent alias is a no-op with a clear message
5. A confirmation message is printed on success

## Out of scope

- Bulk alias deletion
- Restoring a deleted alias
