---
number: 9
title: Config and credential storage in XDG directory with OS keyring
status: accepted
date: 2026-07-16
tags: [config, auth, security]
deciders: [kwhatcher]
---

# Config and credential storage in XDG directory with OS keyring

## Context and Problem Statement

gor needs to persist two kinds of data: host configuration (API endpoints, preferred protocols) and authentication tokens (OAuth access tokens). Configuration is non-sensitive and should be human-readable; tokens are secrets and must be stored securely. How should gor store each?

## Decision Drivers

* **Security** — tokens must not be stored in plaintext in predictable locations
* **XDG compliance** — configuration should follow XDG Base Directory Specification
* **Cross-platform** — must work on Linux, macOS, and Windows
* **Human-editable** — host configuration should be easy to inspect and edit
* **Multi-host** — each host (github.com, GHES instances) needs its own config and token

## Considered Options

* YAML config in `~/.config/gor/` + OS keyring for tokens
* Single JSON file with encrypted tokens
* Environment variables only — no persistent storage
* `confy` for config + encrypted file for tokens

## Decision Outcome

Chosen option: **YAML config in `~/.config/gor/` + OS keyring for tokens**, because it separates concerns cleanly: human-readable, XDG-compliant YAML for configuration, and platform-native secure storage for secrets. This matches `gh`'s approach, making it familiar to users migrating from `gh`.

### Consequences

* Good, because tokens are stored in the OS keyring — encrypted and protected by OS authentication
* Good, because host config is human-readable YAML — easy to inspect, edit, and version-control (without tokens)
* Good, because XDG-compliant — respects `XDG_CONFIG_HOME` on Linux, `~/Library/Application Support` on macOS
* Good, because file permissions are enforced — `hosts.yml` is created with mode `0600`
* Good, because multi-host is natural — each host is a YAML key with its own settings
* Bad, because OS keyring access adds a dependency on `keyring` crate and platform-specific libraries (e.g., `libdbus` on Linux)
* Bad, because keyring access can fail in headless environments (CI, SSH sessions without a keyring daemon)
* Bad, because YAML is more error-prone for hand-editing than TOML (indentation sensitivity)

### Confirmation

The `src/config.rs` module reads/writes `~/.config/gor/hosts.yml` with mode `0600`. The `src/keyring_store.rs` module integrates with the OS keyring via the `keyring` crate. CI installs `libdbus-1-dev` on Linux for keyring support.

## Pros and Cons of the Options

### YAML config + OS keyring

Human-readable config files with platform-native secret storage.

* Good, because tokens are encrypted at rest by the OS
* Good, because config is human-readable and editable
* Good, because matches `gh`'s approach — familiar to existing users
* Good, because XDG-compliant and cross-platform
* Good, because multi-host is natural in YAML
* Neutral, because requires `libdbus` on Linux for keyring access
* Bad, because keyring may not be available in headless environments
* Bad, because YAML indentation errors can break config

### Single JSON file with encrypted tokens

One file containing both config and encrypted tokens.

* Good, because single file to manage
* Bad, because encryption key must be stored somewhere (chicken-and-egg problem)
* Bad, because not human-readable without decryption
* Bad, because custom encryption is error-prone and less secure than OS keyring

### Environment variables only

No persistent storage; all config via environment variables.

* Good, because no files to manage or secure
* Good, because works in all environments
* Bad, because requires setting variables in every shell session
* Bad, because multi-host configuration is awkward in environment variables
* Bad, because no secure storage for long-lived tokens

### confy + encrypted file

Zero-boilerplate config with `confy` crate.

* Good, because minimal code to write
* Bad, because `confy` uses TOML, not YAML — less familiar for `gh` users
* Bad, because `confy` has limited multi-host support
* Bad, because still requires a separate solution for token storage

## More Information

### Config file structure (`~/.config/gor/hosts.yml`)

```yaml
github.com:
  user: kwhatcher
  git_protocol: ssh
  editor: vim
github.mycompany.com:
  user: kwhatcher
  git_protocol: https
```

### Token resolution order

1. `GITHUB_TOKEN` or `GH_TOKEN` environment variable (per-command override)
2. OS keyring (persistent, secure)
3. `~/.config/gor/hosts.yml` `token` field (fallback, discouraged)

### File permissions

`hosts.yml` is created with mode `0600` (owner read/write only) to protect any tokens stored there. The `keyring` crate uses platform-specific secure storage:
- Linux: `secret-service` (dbus) or `keyutils` (kernel keyring)
- macOS: Keychain
- Windows: Credential Manager

This decision should be revisited if:
- The `keyring` crate becomes unmaintained
- A new cross-platform secret storage standard emerges
- User feedback indicates strong preference for a different config format (TOML, JSON)
