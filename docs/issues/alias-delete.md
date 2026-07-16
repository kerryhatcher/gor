---
tags: [alias, write]
priority: P2
phase: 3
endpoints: []
---

# Alias Delete

## As a

developer who no longer needs a shortcut

## I want

to remove a command alias

## Acceptance criteria

1. Running `gor alias delete co` removes the alias named `co`
2. `--hostname` flag removes an alias scoped to a specific host
3. A confirmation message is printed on success
4. If the alias does not exist, a message indicates it was not found
5. Exit code 0 on success

## Out of scope

- Bulk deletion of aliases
- Deleting aliases for all hosts at once
