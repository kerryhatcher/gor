---
tags: [gist, write]
priority: P2
phase: 2
endpoints:
  - POST /gists
status: done
blockedBy: [auth-login]
blocks: []
---

# Gist Create

## As a

developer sharing a code snippet

## I want

to create a new gist with one or more files

## Acceptance criteria

1. Running `gor gist create file.rs` creates a gist with the given file
2. `--desc` flag sets the gist description
3. `--public` flag creates a public gist (default: secret)
4. `--filename` flag overrides the filename in the gist
5. `--web` / `-w` flag opens the new gist in the browser
6. Multiple files can be passed as positional arguments
7. The gist URL and ID are printed on success
8. `--hostname` flag targets a specific host

## Out of scope

- Creating a gist from stdin (use `gor api` with `--input -`)
- Updating an existing gist (separate story)
