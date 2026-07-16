---
tags: [ruleset, read]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/rulesets
---

# Ruleset List

## As a

repository administrator managing branch and tag protection

## I want

to list the rulesets configured for a repository

## Acceptance criteria

1. Running `gor ruleset list` in a repo directory lists all rulesets
2. Each row shows: ruleset name, enforcement state (active/disabled/evaluate), and target (branch/tag)
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--limit` / `-L` flag caps results (default: 30)
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host

## Out of scope

- Creating or editing rulesets (admin-only, not in v1 scope)
- Viewing the detailed rules within each ruleset (separate story)
