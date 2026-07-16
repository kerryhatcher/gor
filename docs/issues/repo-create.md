---
tags: [repo, write]
priority: P1
phase: 1
endpoints:
  - POST /user/repos
  - POST /orgs/{org}/repos
---

# Repo Create

## As a

developer starting a new project

## I want

to create a new GitHub repository from the command line

## Acceptance criteria

1. Running `gor repo create my-repo` creates a new repository under the authenticated user
2. `--description` flag sets the repository description
3. `--private` flag creates a private repository (default: public)
4. `--internal` flag creates an internal repository (GHES only)
5. `--org` flag creates the repository under an organization
6. `--template` flag creates from a template repository
7. `--clone` flag clones the new repository locally after creation
8. `--remote` flag sets the remote name when cloning (default: `origin`)
9. `--push` flag pushes local content to the new repository
10. `--disable-wiki` and `--disable-issues` flags control feature toggles
11. `--hostname` flag targets a specific host
12. The new repository's URL is printed on success

## Out of scope

- `.gitignore`, license, or README template selection (use `--template` for that)
- Repository creation from an existing local directory (use `--push` to push an existing local repo after creation)
