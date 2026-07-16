---
tags: [release, read]
priority: P1
phase: 1
endpoints:
  - GET /repos/{owner}/{repo}/releases/{id}
  - GET /repos/{owner}/{repo}/releases/tags/{tag}
---

# Release View

## As a

developer inspecting a specific release

## I want

to see the full details of a release including its assets

## Acceptance criteria

1. Running `gor release view v1.0.0` shows the release for that tag
2. Running `gor release view 12345` shows the release by database ID
3. Tag name, release name, body, author, and published date are displayed
4. Assets are listed with name, size, and download count
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--web` / `-w` flag opens the release in the browser
7. `--json` flag outputs as JSON with optional field selection
8. `--hostname` flag targets a specific host

## Out of scope

- Release notes diffing
- Asset download (separate story)
