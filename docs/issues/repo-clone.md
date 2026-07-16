---
tags: [repo, git]
priority: P0
phase: 0
endpoints:
  - GET /repos/{owner}/{repo}
status: done
blockedBy: [auth-login]
blocks: []
---

# Repo Clone

## As a

developer who wants to get a local copy of a repository

## I want

to clone a GitHub repository to my local machine

## Acceptance criteria

1. Running `gor repo clone owner/repo` clones the repository via `gix`
2. The clone URL is derived from the repository's `clone_url` field
3. If no `owner/repo` is given, the argument is treated as a full repo URL
4. `--directory` / `-d` flag specifies the target directory name
5. `--upstream-remote-name` flag sets the name of the upstream remote (default: `upstream`)
6. For forks, the upstream remote is automatically added
7. Progress is displayed during the clone operation
8. `--hostname` flag targets a specific host

## Out of scope

- Shallow clones (`--depth`)
- Clone with SSH key selection
- Submodule initialization
