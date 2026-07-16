---
tags: [release, write]
priority: P1
phase: 1
endpoints:
  - POST /repos/{owner}/{repo}/releases
---

# Release Create

## As a

developer shipping a new version of software

## I want

to create a new GitHub release with release notes

## Acceptance criteria

1. Running `gor release create v1.0.0` creates a release for the given tag
2. `--title` flag sets the release title (defaults to tag name)
3. `--notes` flag sets the release body (markdown)
4. `--notes-file` flag reads release notes from a file
5. `--draft` flag creates the release as a draft
6. `--prerelease` flag marks the release as a prerelease
7. `--target` flag specifies the target commitish (branch or commit SHA)
8. `--discussion-category` flag sets the discussion category for the release
9. `--repo` / `-R` flag specifies the repository explicitly
10. `--hostname` flag targets a specific host
11. The release URL is printed on success

## Out of scope

- Auto-generating release notes from merged PRs
- Asset upload (separate story)
- Tag creation (the tag must already exist)
