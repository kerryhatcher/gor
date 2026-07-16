---
tags: [repo, deploy-key, read, write]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/keys
  - POST /repos/{owner}/{repo}/keys
  - DELETE /repos/{owner}/{repo}/keys/{key_id}
status: todo
blockedBy: [repo-view]
blocks: []
---

# Repo Deploy Key

## As a

developer setting up automated deployments

## I want

to list, add, and delete deploy keys for a repository

## Acceptance criteria

1. Running `gor repo deploy-key list` lists deploy keys for the repository
2. Each row shows: key ID, title, SHA-256 fingerprint, and read-only status
3. Running `gor repo deploy-key add --title "CI Server" --file ~/.ssh/id_rsa.pub` adds a deploy key
4. `--allow-write` flag grants write access to the deploy key (default: read-only)
5. Running `gor repo deploy-key delete 12345` deletes a deploy key by database ID
6. `--repo` / `-R` flag specifies the repository explicitly
7. `--hostname` flag targets a specific host
8. `--json` flag outputs as JSON with optional field selection
9. A confirmation prompt is shown before deletion (bypassed with `--yes`)

## Out of scope

- Managing deploy keys at the organization level
- Editing existing deploy keys (delete and re-add)
