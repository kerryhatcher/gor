---
tags: [extension, read]
priority: P3
phase: 4
endpoints: []
status: todo
blockedBy: [extension-list]
blocks: []
---

# Extension Exec

## As a

developer using installed extensions

## I want

to execute an installed gor extension by name

## Acceptance criteria

1. Running `gor my-ext` executes the installed extension named `my-ext`
2. Arguments after the extension name are forwarded to the extension binary
3. The extension binary is resolved from `$PATH` or the gor extensions directory
4. If the extension is not installed, the command exits non-zero with a clear error
5. The extension's exit code is propagated to the caller
6. `--help` on an extension delegates to the extension's own help

## Out of scope

- Extension sandboxing or permission management
- Extension version pinning
