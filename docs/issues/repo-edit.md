---
tags: [repo, write]
priority: P1
phase: 1
endpoints:
  - PATCH /repos/{owner}/{repo}
status: done
blockedBy: [repo-view]
blocks: []
---

# Repo Edit

## As a

developer maintaining a repository's settings

## I want

to edit a repository's metadata and feature toggles from the command line

## Acceptance criteria

1. Running `gor repo edit owner/repo` edits the specified repository's settings
2. `--description` flag updates the repository description
3. `--visibility` flag changes the repository visibility (`public`, `private`, or `internal` on GHES)
4. `--add-topic` flag adds a topic to the repository (repeatable)
5. `--remove-topic` flag removes a topic from the repository (repeatable)
6. `--default-branch` flag changes the repository's default branch
7. `--enable-wiki` flag turns the wiki on or off (`true`/`false`)
8. `--enable-issues` flag turns the issues tracker on or off (`true`/`false`)
9. `--enable-projects` flag turns the projects board on or off (`true`/`false`)
10. `--repo` / `-R` flag specifies the repository explicitly when not in a repo directory
11. `--hostname` flag targets a specific host
12. The updated repository's details are printed on success (name, description, visibility, topics, default branch, and feature toggles)

## Out of scope

- Renaming the repository (see repo-rename.md)
- Archiving or deleting the repository (see repo-delete.md, repo-archive.md)
- Managing branch protection rules or collaborators
