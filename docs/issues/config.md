---
tags: [config, read, write]
priority: P2
phase: 3
endpoints: []
---

# Config

## As a

developer who wants to customize `gor`'s behavior

## I want

to view and set `gor` configuration values stored in `~/.config/gor/config.yml`

## Acceptance criteria

1. Running `gor config get <key>` prints the value of a top-level config key
2. Running `gor config set <key> <value>` sets a top-level config key
3. Running `gor config list` prints all config keys and values as YAML
4. `--host` flag scopes the config to a specific GitHub host (host-scoped keys)
5. Supported keys include at least: `editor`, `browser`, `pager`, `git_protocol`, `prompt`
6. `git_protocol` accepts `https` or `ssh`
7. Setting an unknown key produces a clear error and a non-zero exit code
8. `--hostname` flag targets a specific host
9. Invalid values (e.g. `git_protocol: ftp`) produce a clear error and non-zero exit code

## Out of scope

- Deleting a config key (treat as setting to empty or use `gor config set key ""`)
- Encrypted config values (tokens live in the keyring, not config)
- Per-repository config overrides (not supported in v1)
