---
tags: [alias, write]
priority: P2
phase: 3
endpoints: []
---

# Alias Set

## As a

developer who wants to shorten frequently used `gor` commands

## I want

to define a persistent command alias

## Acceptance criteria

1. Running `gor alias set co "pr checkout"` creates an alias `co` that expands to `pr checkout`
2. The alias is stored in `gor`'s local config file (`~/.config/gor/config.yml`)
3. `--hostname` flag scopes the alias to a specific host
4. An existing alias with the same name is overwritten with a confirmation message
5. The new alias is printed on success in `name: expansion` form
6. Exit code 0 on success

## Out of scope

- Shell completion integration
- Importing aliases from `gh` config (separate story)
- Validating that the expansion is a valid `gor` command at definition time
