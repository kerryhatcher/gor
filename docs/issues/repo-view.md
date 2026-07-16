---
tags: [repo, read]
priority: P0
phase: 0
endpoints:
  - GET /repos/{owner}/{repo}
status: done
blockedBy: [auth-login]
blocks: [repo-delete, repo-edit, repo-fork, repo-sync, repo-transfer, pr-list, issue-list, release-create, release-list, label-create, label-list, workflow-list, secret-set, secret-list, variable-set, variable-list, ruleset-list, cache-list, browse]
---

# Repo View

## As a

developer exploring a repository

## I want

to see a repository's description, stats, and metadata

## Acceptance criteria

1. Running `gor repo view owner/repo` displays the repository name, description, and URL
2. Star count, fork count, and open issue count are shown
3. Primary language, license, and last-pushed date are shown
4. Default branch is displayed
5. Repository visibility (public/private) is shown
6. `--web` / `-w` flag opens the repository in the default browser
7. `--json` flag outputs the full API response as JSON
8. `--json field1,field2` outputs only the specified fields
9. `--hostname` flag targets a specific host
10. If no `owner/repo` is given, auto-detects from the current directory's git remote

## Implementation notes

- `gor repo` is now a subcommand group (like `gor auth`), with `gor repo view` as the first subcommand.
- `owner_repo` is optional; if omitted, it auto-detects from the current directory's git remote via `gix`.
- `--json` accepts optional comma-separated field names; with no value, outputs the full API response.
- `--web` / `-w` opens the repository URL in the default browser using `xdg-open` (Linux), `open` (macOS), or `start` (Windows).
- Created `src/repository.rs` for repo spec parsing and remote URL detection.
- Created `src/output.rs` for JSON formatting, date formatting, and number formatting utilities.
- The `serde_json` crate was added to the package dependencies (was only in workspace dependencies).

## Out of scope

- README rendering
- File tree browsing
- Language breakdown percentages
