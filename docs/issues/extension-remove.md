---
tags: [extension, write]
priority: P2
phase: 3
endpoints: []
---

# Extension Remove

## As a

developer who no longer wants an installed extension

## I want

to uninstall a `gor` extension

## Acceptance criteria

1. Running `gor extension remove repo` removes the installed extension named `repo`
2. The extension's files are deleted from the extensions directory
3. `--hostname` flag scopes removal to a specific host
4. Removing a non-existent extension is a no-op with a clear message
5. A confirmation message is printed on success

## Out of scope

- Disabling an extension without deleting it
- Removing multiple extensions at once
