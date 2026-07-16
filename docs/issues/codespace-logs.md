---
tags: [codespace, read]
priority: P2
phase: 4
endpoints:
  - GET /user/codespaces/{codespace_name}/logs
---

# Codespace Logs

## As a

developer debugging a codespace that failed to start

## I want

to view the creation and lifecycle logs of a GitHub Codespace

## Acceptance criteria

1. Running `gor codespace logs <name>` streams the lifecycle logs for the named codespace
2. The logs are fetched from `GET /user/codespaces/{name}/logs`
3. `--repo` flag scopes selection when only a partial name is given
4. `--json` flag outputs as JSON with optional field selection
5. `-f` / `--follow` flag tails the logs until the codespace is ready
6. Codespaces are only available on github.com

## Out of scope

- Editing the codespace environment
- Application-level logs inside the running codespace
