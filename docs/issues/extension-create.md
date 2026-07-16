---
tags: [extension, write]
priority: P3
phase: 4
endpoints: []
status: todo
blockedBy: [extension-list]
blocks: []
---

# Extension Create

## As a

developer building custom gor extensions

## I want

to scaffold a new gor extension

## Acceptance criteria

1. Running `gor extension create my-ext` creates a new extension directory
2. The directory contains a `gor-my-ext` executable script (or binary placeholder)
3. `--precompiled` flag creates a Go/Rust binary template instead of a shell script
4. `--title` flag sets a human-readable title for the extension
5. `--description` flag sets a description for the extension
6. A success message is printed with the path to the created extension
7. If the extension name already exists, the command exits non-zero with a clear error

## Out of scope

- Publishing extensions to a registry
- Building or compiling extensions
- Testing extensions
