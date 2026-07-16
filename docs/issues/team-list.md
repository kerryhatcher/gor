---
tags: [team, read]
priority: P2
phase: 4
endpoints:
  - GET /orgs/{org}/teams
status: todo
blockedBy: [org-list]
blocks: [team-members, team-add, team-create, team-delete, team-edit]
---

# Team List

## As a

organization member browsing teams

## I want

to list teams in an organization

## Acceptance criteria

1. Running `gor team list --org myorg` lists all teams in the organization
2. `--org` flag specifies the organization (required)
3. Each row shows: team slug, name, description, and member count
4. `--limit` / `-L` flag caps results (default: 30)
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host
7. If no teams are found, a clear message is shown

## Out of scope

- Listing child teams (nested teams)
- Filtering by team visibility
