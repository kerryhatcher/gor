---
tags: [project, read]
priority: P2
phase: 3
endpoints:
  - GET /orgs/{org}/projects
  - GET /repos/{owner}/{repo}/projects
status: done
blockedBy: [org-view]
blocks: [project-view]
---

# Project List

## As a

developer or maintainer tracking work in projects

## I want

to list the GitHub Projects associated with an org or repository

## Acceptance criteria

1. Running `gor project list` in a repo directory lists projects linked to that repository
2. `--org` flag lists projects owned by an organization
3. `--owner` flag lists projects owned by a user
4. Each row shows: project number, title, state (open/closed), and visibility
5. `--limit` / `-L` flag caps results (default: 30)
6. `--json` flag outputs as JSON with optional field selection
7. `--hostname` flag targets a specific host

## Out of scope

- Projects V2 (GraphQL-based) — the REST endpoints cover classic Projects
- Viewing project items (separate story)
