---
tags: [release, read]
priority: P1
phase: 1
endpoints:
  - GET /repos/{owner}/{repo}/releases
status: todo
blockedBy: [repo-view]
blocks: [release-view]
---

# Release List

## As a

developer checking available software versions

## I want

to list releases for a repository

## Acceptance criteria

1. Running `gor release list` in a repo directory lists releases
2. Each row shows: tag name, release name, published date, draft/prerelease status
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--limit` / `-L` flag caps results (default: 30)
5. `--exclude-drafts` flag hides draft releases
6. `--exclude-prereleases` flag hides prereleases
7. `--json` flag outputs as JSON with optional field selection
8. `--hostname` flag targets a specific host

## Out of scope

- Sorting by semantic version
- Release comparison
