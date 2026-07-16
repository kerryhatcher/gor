---
tags: [team, write]
priority: P2
phase: 4
endpoints:
  - POST /orgs/{org}/teams
status: todo
blockedBy: [team-list]
blocks: []
---

# Team Create

## As a

organization admin structuring the org

## I want

to create a new team

## Acceptance criteria

1. Running `gor team create my-team --org myorg --name "My Team"` creates a new team
2. The team slug is a required positional argument
3. `--org` flag specifies the organization (required)
4. `--name` / `-n` flag sets a display name (defaults to slug)
5. `--description` / `-d` flag sets a team description
6. `--parent` flag sets a parent team slug (for nested teams)
7. `--privacy` flag sets visibility: `closed` (default) or `secret`
8. `--hostname` flag targets a specific host
9. A success message is printed with the team slug and URL

## Out of scope

- Adding members during creation
- Setting team repositories during creation
