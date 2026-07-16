---
tags: [team, read]
priority: P2
phase: 4
endpoints:
  - GET /orgs/{org}/teams/{team_slug}/members
status: todo
blockedBy: [team-list]
blocks: []
---

# Team Members

## As a

team maintainer managing membership

## I want

to list members of a team

## Acceptance criteria

1. Running `gor team members my-team --org myorg` lists members of the team
2. The team slug is a required positional argument
3. `--org` flag specifies the organization (required)
4. Each row shows: login, role (maintainer/member), and membership status
5. `--limit` / `-L` flag caps results (default: 30)
6. `--json` flag outputs as JSON with optional field selection
7. `--hostname` flag targets a specific host
8. If the team has no members, a clear message is shown

## Out of scope

- Listing pending invitations
- Listing child team members
