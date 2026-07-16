---
tags: [org, read]
priority: P2
phase: 3
endpoints:
  - GET /orgs/{org}
---

# Org View

## As a

developer who wants details about a GitHub organization

## I want

to view an organization's profile and metadata

## Acceptance criteria

1. Running `gor org view myorg` shows the organization's profile
2. Displayed fields include: name, description, location, website URL, email, and member/team/repo counts
3. `--web` / `-w` flag opens the organization in the browser
4. `--json` flag outputs as JSON with optional field selection
5. `--hostname` flag targets a specific host

## Out of scope

- Organization members and teams listing (separate stories)
- Organization settings (admin-only, not in v1 scope)
