---
tags: [codespace, read]
priority: P2
phase: 4
endpoints:
  - GET /user/codespaces/{codespace_name}/ports
status: done
blockedBy: [codespace-list]
blocks: []
---

# Codespace Ports

## As a

developer running a web server in my codespace

## I want

to list and manage forwarded ports in a GitHub Codespace

## Acceptance criteria

1. Running `gor codespace ports <name>` lists all forwarded ports for the named codespace
2. Each port row shows: label, source port, visibility (public/private/organization)
3. `--json` flag outputs as JSON with optional field selection
4. `--hostname` flag targets a specific host (codespaces only on github.com)
5. The command exits non-zero if the codespace is not running

## Out of scope

- Forwarding new ports (use the codespace's built-in port forwarding)
- Changing port visibility (use the web UI or `gh codespace ports visibility`)
