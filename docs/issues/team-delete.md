---
tags: [team, write]
priority: P2
phase: 4
endpoints:
  - DELETE /orgs/{org}/teams/{team_slug}
status: todo
blockedBy: [team-list]
blocks: []
---

# Team Delete

## As a

organization admin cleaning up teams

## I want

to delete a team

## Acceptance criteria

1. Running `gor team delete my-team --org myorg` deletes the team
2. The team slug is a required positional argument
3. `--org` flag specifies the organization (required)
4. A confirmation prompt is shown before deletion (bypassed with `--yes`)
5. `--hostname` flag targets a specific host
6. A success message is printed after deletion
7. If the team does not exist, the command exits non-zero with a clear error

## Out of scope

- Deleting teams with child teams (must delete children first)
- Bulk team deletion
