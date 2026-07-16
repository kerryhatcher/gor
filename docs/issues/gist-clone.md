---
tags: [gist, read, write]
priority: P2
phase: 4
endpoints:
  - GET /gists/{gist_id}
status: todo
blockedBy: [gist-view]
blocks: []
---

# Gist Clone

## As a

developer who wants to work with a gist locally

## I want

to clone a gist to my local machine

## Acceptance criteria

1. Running `gor gist clone <gist-id>` clones the gist into a local directory
2. The directory name defaults to the gist ID
3. `--dir` / `-d` flag specifies a custom target directory
4. The cloned directory is initialized as a git repository with the gist's remote
5. `--hostname` flag targets a specific host
6. A success message is printed with the path to the cloned directory
7. If the gist does not exist, the command exits non-zero with a clear error
8. If the target directory already exists, the command exits non-zero with a clear error

## Out of scope

- Cloning multiple gists at once
- Updating an already-cloned gist (use `git pull` in the cloned directory)
