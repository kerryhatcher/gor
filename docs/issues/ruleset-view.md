---
tags: [ruleset, read]
priority: P4
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/rulesets/{ruleset_id}
  - GET /orgs/{org}/rulesets/{ruleset_id}
---

# Ruleset View

## As a

developer auditing repository protection rules

## I want

to see the full details of a ruleset including its conditions and rules

## Acceptance criteria

1. Running `gor ruleset view 42` shows ruleset #42
2. The ruleset name, enforcement state, target, and source type are displayed
3. All rules (e.g., `required_linear_history`, `pull_request`, `required_signatures`) are listed with their parameters
4. Conditions (ref name patterns, repository names) are shown
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--org` flag views an organization-level ruleset
7. `--json` flag outputs as JSON with optional field selection
8. `--hostname` flag targets a specific host
9. Exit code 0 on success

## Out of scope

- Editing or deleting rulesets
- Bypass list management
