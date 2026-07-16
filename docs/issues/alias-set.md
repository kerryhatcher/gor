---
tags: [alias, write]
priority: P2
phase: 3
endpoints: []
status: done
blockedBy: [auth-login]
blocks: []
---

# Alias Set

## As a

developer who repeats long `gor` command sequences

## I want

to define a short alias that expands to a longer `gor` subcommand

## Acceptance criteria

1. Running `gor alias set co 'pr checkout'` creates an alias named `co` that expands to `pr checkout`
2. Aliases expand only to `gor` subcommands (no arbitrary shell)
3. `--clobber` flag overwrites an existing alias of the same name
4. Without `--clobber`, defining a duplicate alias fails with a clear error and non-zero exit code
5. Aliases are persisted in `~/.config/gor/config.yml` (mode 0600)
6. The alias definition is printed on success
7. `--hostname` flag scopes the alias to a specific host

## Out of scope

- Aliases that shell out to arbitrary binaries
- Importing/exporting alias sets (separate story)
