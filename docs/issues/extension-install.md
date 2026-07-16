---
tags: [extension, write]
priority: P2
phase: 3
endpoints: []
---

# Extension Install

## As a

developer who wants to add functionality to `gor`

## I want

to install an extension from a git repository

## Acceptance criteria

1. Running `gor extension install owner/repo` installs the extension from the given repository
2. A full repository URL may also be passed as the argument
3. The extension is cloned into the extensions directory (`~/.config/gor/extensions`)
4. `--hostname` flag installs an extension for a specific host
5. `--force` flag reinstalls an already-installed extension
6. On success, the installed extension's command becomes available via `gor <extension-name>`
7. The extension name and version are printed on success
8. Exit code 0 on success

## Out of scope

- Building extensions from source (they must be pre-built binaries in the repo)
- Extension discovery/marketplace browsing
