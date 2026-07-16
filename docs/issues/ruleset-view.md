---
tags: [ruleset, read]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/rulesets/{ruleset_id}
status: todo
blockedBy: [ruleset-list]
blocks: []
---

# Ruleset View

## As a

repository administrator inspecting a protection policy

## I want

to view the full details of a single ruleset

## Acceptance criteria

1. Running `gor ruleset view 42` shows ruleset #42 for the current repository
2. `--repo` / `-R` flag specifies the repository explicitly
3. Displayed fields include: name, enforcement state, target, source branch/tag, and the list of rules (with parameters)
4. `--web` / `-w` flag opens the ruleset settings in the browser
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host

## Out of scope

- Editing ruleset rules
- Organization-level rulesets (separate story if supported)
