---
tags: [org, read]
priority: P2
phase: 3
endpoints:
  - GET /user/orgs
status: done
blockedBy: [auth-login]
blocks: [org-view]
---

# Org List

## As a

developer who belongs to one or more GitHub organizations

## I want

to list the organizations I'm a member of

## Acceptance criteria

1. Running `gor org list` lists the organizations the authenticated user belongs to
2. Each row shows: organization login, description, and avatar URL
3. `--limit` / `-L` flag caps the number of results (default: 30)
4. `--json` flag outputs as JSON with optional field selection
5. `--hostname` flag targets a specific host
6. If the user belongs to no organizations, a friendly empty message is shown

## Out of scope

- Listing organizations for another user (not exposed by the authenticated-user endpoint)
- Organization members, teams, or repositories (separate stories)
