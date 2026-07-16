---
tags: [extension, write]
priority: P2
phase: 3
endpoints: []
---

# Extension Upgrade

## As a

developer who wants the latest version of installed extensions

## I want

to upgrade one or all installed `gor` extensions

## Acceptance criteria

1. Running `gor extension upgrade repo` upgrades the named extension to its latest release/tag
2. Running `gor extension upgrade --all` upgrades every installed extension
3. The upgrade pulls the latest from the extension's source repository
4. `--hostname` flag scopes the upgrade to a specific host
5. Extensions already at the latest version are skipped with a message
6. A summary of upgraded (and skipped) extensions is printed

## Out of scope

- Downgrading an extension to a prior version
- Upgrading extensions with local uncommitted changes
