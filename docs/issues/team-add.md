---
tags: [team, write]
priority: P2
phase: 4
endpoints:
  - PUT /orgs/{org}/teams/{team_slug}/memberships/{username}
  - DELETE /orgs/{org}/teams/{team_slug}/memberships/{username}
status: todo
blockedBy: [team-list]
blocks: []
---

# Team Add

## As a

team maintainer managing membership

## I want

to add or remove members from a team

## Acceptance criteria

1. Running `gor team add my-team --org myorg --user octocat` adds a user to the team
2. `--user` / `-u` flag specifies the username to add (required)
3. `--role` flag specifies the role: `member` (default) or `maintainer`
4. Running `gor team remove my-team --org myorg --user octocat` removes a user from the team
5. A confirmation prompt is shown before removal (bypassed with `--yes`)
6. `--hostname` flag targets a specific host
7. A success message is printed confirming the operation
8. If the user is already a member (or not a member for removal), the command exits gracefully

## Out of scope

- Bulk adding/removing multiple users
- Managing team repositories
