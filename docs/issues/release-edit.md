---
tags: [release, write]
priority: P1
phase: 1
endpoints:
  - PATCH /repos/{owner}/{repo}/releases/{id}
status: done
blockedBy: [release-view]
blocks: []
---

# Release Edit

## As a

developer who needs to correct or update an already-published release

## I want

to edit an existing GitHub release's metadata and release notes

## Acceptance criteria

1. Running `gor release edit v1.0.0` edits the release identified by tag `v1.0.0`
2. Running `gor release edit 12345` edits the release identified by database ID `12345`
3. `--title` flag updates the release title
4. `--notes` flag updates the release body (markdown)
5. `--notes-file` flag reads the updated release body from a file
6. `--draft` flag toggles the draft status of the release (`true` marks it as a draft, `false` publishes it)
7. `--prerelease` flag toggles the prerelease status of the release (`true` marks it as a prerelease, `false` clears it)
8. `--tag` flag changes the tag the release points to
9. `--target` flag changes the target commitish (branch or commit SHA)
10. `--repo` / `-R` flag specifies the repository explicitly
11. `--hostname` flag targets a specific host
12. The updated release URL is printed on success

## Out of scope

- Creating a new release (use `gor release create`)
- Asset upload or download (`gor release upload` / `gor release download`)
- Deleting a release (`gor release delete`)
- Tag creation on the server (the `--tag` flag only re-points an existing release to an existing tag)
