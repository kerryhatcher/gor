---
tags: [codespace, write]
priority: P3
phase: 4
endpoints:
  - POST /user/codespaces
---

# Codespace Create

## As a

developer who wants a cloud development environment

## I want

to create a new codespace for a repository

## Acceptance criteria

1. Running `gor codespace create --repo owner/repo` creates a codespace for the given repo
2. `--branch` / `-b` flag selects the branch to base the codespace on
3. `--machine` / `-m` flag selects the machine type (e.g., `standardLinux32gb`)
4. `--devcontainer-path` flag selects a custom devcontainer path
5. `--idle-timeout` flag sets the idle timeout duration
6. `--display-name` flag sets a friendly display name
7. `--repo` / `-R` flag specifies the repository explicitly
8. Only available on github.com — a clear error is shown when used on GHES
9. The codespace URL and connection details are printed on success
10. Exit code 0 on success

## Out of scope

- Waiting for the codespace to become ready (user polls or uses `gor codespace list`)
- SSH access setup (separate story)
