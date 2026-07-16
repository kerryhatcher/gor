---
tags: [codespace, write]
priority: P2
phase: 4
endpoints:
  - POST /repos/{owner}/{repo}/codespaces
status: todo
blockedBy: [auth-login]
blocks: []
---

# Codespace Create

## As a

developer who wants a cloud dev environment for a branch

## I want

to create a new GitHub Codespace

## Acceptance criteria

1. Running `gor codespace create` in a repo directory creates a codespace for the default branch
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--branch` / `-b` flag creates the codespace for a specific branch
4. `--machine` / `-m` flag selects the machine type/size
5. `--idle-timeout` flag sets the auto-stop idle timeout
6. `--retention-period` flag sets how long the codespace is kept after stopping
7. `--default-permissions` / `--repo-permissions` flags control pre-authorization
8. The new codespace name and web URL are printed on success
9. Codespaces are only available on github.com

## Out of scope

- Waiting for the codespace to finish provisioning
- Auto-opening an editor (use `gor codespace code` / `ssh`)
