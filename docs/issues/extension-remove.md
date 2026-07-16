---
tags: [extension, write]
priority: P2
phase: 3
endpoints: []
---

# Extension Remove

## As a

developer who no longer needs an installed extension

## I want

to uninstall an extension

## Acceptance criteria

1. Running `gor extension remove <name>` removes the extension by name
2. `--hostname` flag removes an extension for a specific host
3. A confirmation message is printed on success
4. If the extension is not installed, a message indicates it was not found
5. Exit code 0 on success

## Out of scope

- Removing all extensions at once
- Removing extension data/cache beyond the extension directory
