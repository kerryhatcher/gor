---
tags: [auth, git]
priority: P0
phase: 0
endpoints: []
status: done
blockedBy: [auth-login]
blocks: []
---

# Auth Setup Git

## As a

developer who uses both `git` and `gor` with GitHub

## I want

to configure `git` to use `gor` as a credential helper so HTTPS clones and pushes are authenticated transparently

## Acceptance criteria

1. Running `gor auth setup-git` writes the `credential.https://github.com.helper` and `credential.helper` git config entries that invoke `gor auth git-credential` as the Git credential helper
2. The `git config` commands that `gor` runs are printed to stdout so the user can see exactly what was changed
3. After running, `git config --global --get-regexp credential` (or the local equivalent) reflects the written entries, confirming the config was applied correctly
4. `--hostname` flag writes the credential helper config scoped to a specific GitHub Enterprise Server host (e.g., `credential.https://<hostname>/.helper`)
5. If a `gor`-managed credential helper entry already exists, it is updated in place rather than duplicated
6. If a non-`gor` credential helper is already configured, it is preserved and the `gor` entry is added without overwriting the existing value
7. Running the command is idempotent — re-running it produces the same config state and a clear message that git is already configured
8. A success message is printed confirming git is now configured to use `gor` for credentials

## Out of scope

- Storing or modifying the GitHub token itself (handled by `gor auth login`)
- Configuring SSH instead of HTTPS credentials
- Per-repository local git config (this writes global git config only)
