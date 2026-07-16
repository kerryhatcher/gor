---
tags: [project, read, write]
priority: P3
phase: 4
endpoints: []
status: done
blockedBy: [project-list]
blocks: []
---

# Project V2

## As a

developer or maintainer using GitHub's newer Projects experience

## I want

to list, view, and manage Projects V2 via the GraphQL API

## Acceptance criteria

1. Running `gor project list --v2` lists Projects V2 for the current repository or org context
2. `--v2` flag switches from classic Projects (REST) to Projects V2 (GraphQL)
3. `--org` flag lists Projects V2 owned by an organization
4. `--owner` flag lists Projects V2 owned by a user
5. Each row shows: project number, title, and closed status
6. `--limit` / `-L` flag caps results (default: 30)
7. `--json` flag outputs as JSON with optional field selection
8. `--hostname` flag targets a specific host
9. The GraphQL query uses the `organization.projectsV2` or `user.projectsV2` connection

## Out of scope

- Full CRUD for Projects V2 items and fields (deferred to future stories)
- Migrating classic Projects to Projects V2
