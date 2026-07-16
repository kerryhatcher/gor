---
tags: [extension, write]
priority: P2
phase: 3
endpoints: []
status: done
blockedBy: [auth-login]
blocks: []
---

# Extension Install

## As a

developer who wants to extend `gor` with community commands

## I want

to install a `gor` extension from a git repository

## Acceptance criteria

1. Running `gor extension install owner/repo` installs the extension from that GitHub repo
2. A full `https://github.com/owner/repo` URL is also accepted
3. The extension binary is fetched/built and placed in the extensions directory (`~/.local/share/gor/extensions` or equivalent)
4. `--force` flag reinstalls over an existing extension of the same name
5. A success message shows the installed extension name and version
6. `--hostname` flag scopes installation to a specific host
7. Invalid or missing extension repositories fail with a clear error and non-zero exit code

## Out of scope

- Automatic dependency resolution for extensions
- Trust verification of third-party extension code
