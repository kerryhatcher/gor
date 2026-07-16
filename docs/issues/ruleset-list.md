---
tags: [ruleset, read]
priority: P4
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/rulesets
  - GET /orgs/{org}/rulesets
---

# Ruleset List

## As a

developer managing branch and tag protection

## I want

to list the rulesets configured for a repository or organization

## Acceptance criteria

1. Running `gor ruleset list` in a repo directory lists repository rulesets
2. Each row shows: ruleset name, enforcement state (active/disabled/evaluate), target (branch/tag)
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--org` flag lists organization-level rulesets
5. `--limit` / `-L` flag caps results (default: 30)
6. `--json` flag outputs as JSON with optional field selection
7. `--hostname` flag targets a specific host
8. Exit code 0 on success

## Out of scope

- Creating or editing rulesets
- Viewing individual ruleset details (separate story)
