---
tags: [issue, read]
priority: P2
phase: 4
endpoints:
  - GET /search/issues
status: todo
blockedBy: [issue-list]
blocks: []
---

# Issue Status

## As a

developer tracking my work across repositories

## I want

to see a summary of issues I'm involved in

## Acceptance criteria

1. Running `gor issue status` shows issues relevant to the current user
2. The output is grouped by: "Assigned to you", "Mentioned you", "Created by you"
3. Each issue shows: number, title, repository, state, and labels
4. `--repo` / `-R` flag scopes results to a specific repository
5. `--hostname` flag targets a specific host
6. `--json` flag outputs as JSON with optional field selection
7. If no relevant issues are found, a clear message is shown

## Out of scope

- Cross-organization issue status
- Filtering by milestone or project
