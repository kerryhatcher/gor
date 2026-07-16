---
number: 7
title: Flag-driven CLI with no interactive prompts in v1
status: accepted
date: 2026-07-16
tags: [ux, architecture]
deciders: [kwhatcher]
---

# Flag-driven CLI with no interactive prompts in v1

## Context and Problem Statement

CLI tools can interact with users in two ways: flag-driven (all behavior controlled via command-line arguments) or interactive (prompts, TUI pickers, confirmation dialogs). `gh` uses both — it has flags for scripting but also interactive prompts for common workflows. Which approach should gor take for v1?

## Decision Drivers

* **Scriptability** — gor must be usable in shell scripts and CI pipelines without TTY detection hacks
* **Simplicity** — interactive prompts require terminal handling, which adds complexity
* **Speed to v1** — interactive features take longer to implement and test
* **Parity with `gh`** — `gh` supports both modes; gor can add interactive features later
* **User expectations** — some users expect interactive prompts for auth and common workflows

## Considered Options

* Flag-driven only — all behavior controlled via CLI flags; no interactive prompts
* Interactive-first — prompts and pickers for common workflows; flags for scripting
* Hybrid — interactive by default with `--flag` overrides (like `gh`)

## Decision Outcome

Chosen option: **Flag-driven only**, because it maximizes scriptability, minimizes implementation complexity, and gets gor to a usable v1 faster. Interactive features can be added in v2 based on user feedback about which workflows benefit most from prompts.

### Consequences

* Good, because every operation is scriptable — no TTY required
* Good, because simpler implementation — no terminal handling, no prompt libraries
* Good, because faster to implement and test — fewer interaction states to cover
* Good, because consistent behavior across TTY and non-TTY environments
* Good, because no dependency on `dialoguer`, `inquire`, or similar prompt crates
* Bad, because auth flow requires more manual steps (user must paste token or complete device flow in browser)
* Bad, because some workflows are less discoverable without interactive prompts
* Bad, because users coming from `gh` may expect interactive prompts

### Confirmation

The codebase has no interactive prompt dependencies. The `src/cli.rs` defines all behavior via clap derive attributes. The auth flow uses OAuth device flow (browser-based) rather than terminal prompts.

## Pros and Cons of the Options

### Flag-driven only

All behavior controlled via CLI flags; no interactive prompts.

* Good, because fully scriptable — works in CI, cron, and shell scripts
* Good, because no TTY detection or terminal handling needed
* Good, because consistent behavior in all environments
* Good, because simpler to test — no interaction state machine
* Good, because faster to implement v1
* Bad, because less discoverable for new users
* Bad, because auth requires browser-based device flow or manual token entry
* Bad, because long command lines for complex operations

### Interactive-first

Prompts and pickers for common workflows; flags for scripting.

* Good, because more discoverable and user-friendly
* Good, because guided workflows reduce errors
* Good, because familiar to `gh` users
* Bad, because requires TTY detection and graceful degradation
* Bad, because more complex to implement and test
* Bad, because interactive prompts break in CI/scripts without `--yes` flags
* Bad, because adds dependencies on prompt libraries

### Hybrid

Interactive by default with `--flag` overrides (like `gh`).

* Good, because best of both worlds — interactive for humans, flags for scripts
* Good, because matches `gh` behavior — familiar to existing users
* Bad, because most complex to implement — two code paths for every input
* Bad, because TTY detection is fragile (pipes, redirects, CI environments)
* Bad, because significantly more test surface area
* Bad, because delays v1 significantly

## More Information

The one exception to "no interactive prompts" is the OAuth device flow for `gor auth login`, which opens the user's browser. This is not a terminal prompt — it's a web-based interaction that works in any environment with a browser.

This decision should be revisited when:
- User feedback indicates specific workflows that are painful without prompts
- v1 is stable and interactive features can be added without breaking changes
- A `--interactive` / `-i` flag can be added to opt into prompts on a per-command basis
