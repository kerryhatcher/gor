---
number: 5
title: Multi-host support for github.com and GHES from day one
status: accepted
date: 2026-07-16
tags: [architecture, auth, config]
deciders: [kwhatcher]
---

# Multi-host support for github.com and GHES from day one

## Context and Problem Statement

gor must support both github.com and GitHub Enterprise Server (GHES) instances. GHES has different base URLs (e.g., `https://github.mycompany.com`) and different API endpoint paths (`/api/v3` vs. `/api`). Should gor support multiple hosts from the start, or target github.com first and add GHES later?

## Decision Drivers

* **Parity with `gh`** — `gh` supports GHES; gor should not regress on this capability
* **Architectural cost** — retrofitting multi-host support is more expensive than designing for it
* **API differences** — GHES has different base URL patterns and may lack some endpoints
* **Auth isolation** — each host needs its own token and configuration

## Considered Options

* Multi-host from day one — design the host abstraction before any API calls
* github.com first, GHES later — hardcode github.com; refactor when GHES is needed
* Single-host only — never support GHES

## Decision Outcome

Chosen option: **Multi-host from day one**, because the architectural cost of retrofitting multi-host support is far higher than designing it upfront. The host abstraction (`src/host.rs`) is a small, well-defined module that pays for itself immediately by keeping URL construction centralized and testable.

### Consequences

* Good, because URL construction is centralized in `host.rs` — no hardcoded URLs anywhere
* Good, because auth is scoped per-host — tokens are never shared between hosts
* Good, because GHES users are first-class citizens from v1
* Good, because the host abstraction is simple: a struct with `base_url`, `api_endpoint`, and token
* Good, because `--hostname` flag works uniformly across all commands
* Bad, because every command must be aware of the host concept (minor API complexity)
* Bad, because testing must cover both github.com and GHES URL patterns
* Bad, because GHES API differences (missing endpoints, different pagination) add edge cases

### Confirmation

The `src/host.rs` module provides `Host` struct and URL derivation. The `src/client.rs` module accepts a `Host` parameter for all API calls. The `src/config.rs` module stores per-host configuration. The `--hostname` flag is a global CLI argument available to all commands.

## Pros and Cons of the Options

### Multi-host from day one

Design the host abstraction before any API calls.

* Good, because no hardcoded URLs — all URL construction goes through `host.rs`
* Good, because auth is properly scoped per-host from the start
* Good, because GHES support is not an afterthought
* Good, because the abstraction is small and well-defined
* Neutral, because requires a `Host` parameter on every client method
* Bad, because slightly more upfront design work

### github.com first, GHES later

Hardcode github.com; refactor when GHES is needed.

* Good, because faster initial implementation
* Good, because simpler code with fewer abstractions
* Bad, because retrofitting multi-host support touches every API call site
* Bad, because hardcoded URLs become tech debt that is hard to find and replace
* Bad, because auth is initially single-host and must be restructured
* Bad, because GHES users are locked out until the refactor is complete

### Single-host only

Never support GHES.

* Good, because simplest possible implementation
* Bad, because excludes enterprise users — a significant portion of `gh`'s user base
* Bad, because not competitive with `gh` which supports GHES

## More Information

The host resolution order is:
1. `--hostname` CLI flag (explicit)
2. `GH_HOST` environment variable
3. `github.com` (default)

GHES API endpoint derivation follows GitHub's conventions:
- `https://github.mycompany.com` → API at `https://github.mycompany.com/api/v3`
- Uploads at `https://github.mycompany.com/api/uploads`
- Web UI at `https://github.mycompany.com`

This decision is unlikely to be revisited — multi-host support is a foundational capability.
