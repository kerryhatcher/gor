---
tags: [project, write]
priority: P3
phase: 4
endpoints:
  - POST /orgs/{org}/projects
  - POST /repos/{owner}/{repo}/projects
  - POST /users/{owner}/projects
status: todo
blockedBy: [project-list]
blocks: [project-delete, project-edit, project-close, project-copy]
---

# Project Create

## As a

maintainer setting up project tracking

## I want

to create a new project board

## Acceptance criteria

1. Running `gor project create "Sprint 42"` creates a new project with the given title
2. `--org` flag creates the project under an organization
3. `--owner` flag creates the project under a user account
4. `--repo` / `-R` flag creates the project under a repository
5. `--body` / `-b` flag sets a description for the project
6. `--template` flag creates from a template (e.g., `basic-kanban`, `automated-kanban`)
7. `--hostname` flag targets a specific host
8. A success message is printed with the project number and URL

## Out of scope

- Creating Projects V2 (use `gor project create --v2` in a future story)
- Adding columns or items during creation
