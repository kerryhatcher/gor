---
tags: [project, read]
priority: P3
phase: 3
endpoints:
  - GET /orgs/{org}/projects
  - GET /repos/{owner}/{repo}/projects
---

# Project List

## As a

developer tracking work in GitHub Projects

## I want

to list projects for an organization or repository

## Acceptance criteria

1. Running `gor project list --owner myorg` lists projects owned by `myorg`
2. Running `gor project list --repo owner/repo` lists projects for that repository
3. Each row shows: project number, title, state (open/closed), visibility
4. `--owner` / `-o` flag lists projects for a user or org
5. `--repo` / `-R` flag lists projects for a repository
6. `--limit` / `-L` flag caps results (default: 30)
7. `--json` flag outputs as JSON with optional field selection
8. `--hostname` flag targets a specific host
9. Exit code 0 on success

## Out of scope

- Project item management (separate story)
- Creating projects
