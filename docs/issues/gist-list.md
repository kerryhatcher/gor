---
tags: [gist, read]
priority: P2
phase: 2
endpoints:
  - GET /gists
  - GET /users/{username}/gists
status: todo
blockedBy: [auth-login]
blocks: [gist-view]
---

# Gist List

## As a

developer looking at code snippets

## I want

to list my gists or another user's public gists

## Acceptance criteria

1. Running `gor gist list` lists the authenticated user's gists
2. Each row shows: gist ID, description, file count, visibility, creation date
3. `--public` flag lists the authenticated user's public gists
4. `--secret` flag lists the authenticated user's secret gists
5. `--user` flag lists gists for a specific user (public only)
6. `--limit` / `-L` flag caps results (default: 30)
7. `--json` flag outputs as JSON with optional field selection
8. `--hostname` flag targets a specific host

## Out of scope

- Starred gists
- Forked gists
