---
tags: [secret, read]
priority: P2
phase: 3
endpoints:
  - GET /repos/{owner}/{repo}/actions/secrets
  - GET /orgs/{org}/actions/secrets
status: todo
blockedBy: [repo-view]
blocks: [secret-delete]

# Secret List

## As a

developer managing CI secrets for a repository or organization

## I want

to list the encrypted secrets available to GitHub Actions

## Acceptance criteria

1. Running `gor secret list` in a repo directory lists repository secrets
2. `--org` flag lists organization secrets instead of repository secrets
3. Each row shows: secret name, and whether it requires a specific environment or visibility
4. `--json` flag outputs as JSON with optional field selection
5. `--hostname` flag targets a specific host

## Out of scope

- Showing secret values (GitHub never returns them)
- Setting or deleting secrets (separate stories)
