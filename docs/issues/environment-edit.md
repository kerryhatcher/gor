---
tags: [environment, write]
priority: P2
phase: 4
endpoints:
  - PUT /repos/{owner}/{repo}/environments/{env}
status: todo
blockedBy: [environment-list]
blocks: []
---

# Environment Edit

## As a

DevOps engineer adjusting environment settings

## I want

to edit an environment's protection rules

## Acceptance criteria

1. Running `gor environment edit staging --wait-timer 5` sets a 5-minute wait timer
2. `--wait-timer` flag sets the wait timer in minutes (0 to disable)
3. `--branch` / `-b` flag changes the deployment branch restriction
4. `--add-reviewer` flag adds a required reviewer (repeatable)
5. `--remove-reviewer` flag removes a required reviewer (repeatable)
6. `--repo` / `-R` flag specifies the repository explicitly
7. `--hostname` flag targets a specific host
8. A success message is printed confirming the changes
9. If the environment does not exist, the command exits non-zero with a clear error

## Out of scope

- Editing environment secrets or variables
- Custom deployment protection rules
