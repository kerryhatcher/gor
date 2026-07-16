---
tags: [ruleset, read]
priority: P3
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/rulesets/{ruleset_id}
status: todo
blockedBy: [ruleset-list]
blocks: []
---

# Ruleset Check

## As a

developer verifying branch protection compliance

## I want

to check which rulesets apply to a branch or tag

## Acceptance criteria

1. Running `gor ruleset check --branch main` shows rulesets that apply to the `main` branch
2. `--branch` / `-b` flag specifies the branch to check
3. `--tag` / `-t` flag specifies the tag to check (mutually exclusive with `--branch`)
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--hostname` flag targets a specific host
6. Each matching ruleset shows: name, ID, enforcement level, and bypass actors
7. `--json` flag outputs as JSON with optional field selection
8. If no rulesets match, a clear message is shown

## Out of scope

- Checking rulesets at the organization level
- Simulating a push to see which rules would block it
