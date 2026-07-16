---
tags: [config, read, write]
priority: P2
phase: 3
endpoints: []
status: done
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
- Encrypted config values (tokens live in `~/.config/gor/hosts.yml`, not config)
- Per-repository config overrides (not supported in v1)

## Implementation

- **Module:** `src/config.rs` — YAML config file I/O
- **Command:** `src/cmd/config.rs` — `gor config {get,set,list}`
- **Config path:** `~/.config/gor/config.yml` (mode 0600 on Unix)
- **Format:** YAML with optional `hosts:` section for host-scoped overrides
- **Validation:** Key whitelist + `git_protocol` value check (https/ssh only)
- **Tests:** 20 unit tests covering load, save, get, set, validate, host scoping
