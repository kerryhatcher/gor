---
tags: [classroom, read]
priority: P4
phase: 4
endpoints:
  - GET /classrooms
  - GET /classrooms/{classroom_id}/assignments
status: done
blockedBy: [auth-login]
blocks: []
---

# Classroom

## As a

student or educator using GitHub Classroom

## I want

to list classrooms and assignments from the command line

## Acceptance criteria

1. Running `gor classroom list` lists all classrooms the authenticated user belongs to
2. Each row shows: classroom ID, name, and organization
3. Running `gor classroom assignments <id>` lists assignments for a classroom
4. Each assignment row shows: ID, title, deadline, and invitation status
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host
7. An empty classroom list prints a clear message

## Out of scope

- Accepting or submitting assignments
- Classroom administration (creating classrooms, managing rosters)
- Viewing individual student submissions
