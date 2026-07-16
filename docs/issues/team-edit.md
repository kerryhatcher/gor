---
tags: [team, write]
priority: P2
phase: 4
endpoints:
  - PATCH /orgs/{org}/teams/{team_slug}
status: todo
blockedBy: [team-list]
blocks: []
---

# Team Edit

## As a

team maintainer updating team settings

## I want

to edit a team's name, description, or privacy

## Acceptance criteria

1. Running `gor team edit my-team --org myorg --name "New Name"` renames the team
2. `--name` / `-n` flag sets a new display name
3. `--description` / `-d` flag sets a new description
4. `--privacy` flag changes visibility: `closed` or `secret`
5. `--parent` flag changes the parent team slug
6. `--hostname` flag targets a specific host
7. A success message is printed confirming the changes
8. If the team does not exist, the command exits non-zero with a clear error

## Out of scope

- Changing the team slug (not supported by GitHub API)
- Bulk editing multiple teams
