---
tags: [extension, write]
priority: P2
phase: 3
endpoints: []
---

# Extension Upgrade

## As a

developer who wants the latest fixes in installed extensions

## I want

to upgrade my installed extensions to their latest versions

## Acceptance criteria

1. Running `gor extension upgrade` upgrades all installed extensions to their latest versions
2. `gor extension upgrade <name>` upgrades a single named extension
3. `--hostname` flag upgrades extensions for a specific host
4. Each upgraded extension's name and new version are printed on success
5. If an extension is already up to date, a message indicates no update available
6. Exit code 0 on success

## Out of scope

- Downgrading extensions
- Upgrading to a specific version tag (always latest)
