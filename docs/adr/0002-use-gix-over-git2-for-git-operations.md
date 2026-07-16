---
number: 2
title: Use gix over git2 for git operations
status: accepted
date: 2026-07-16
tags: [git, dependencies]
deciders: [kwhatcher]
---

# Use gix over git2 for git operations

## Context and Problem Statement

gor needs to interact with local git repositories for operations like detecting the current repository from a working directory, resolving remote URLs, and cloning repositories. The two main Rust crates for git operations are `git2` (bindings to `libgit2`) and `gix` (pure Rust, formerly `gitoxide`). Which should gor use?

## Decision Drivers

* **Zero system dependencies** — gor must not require `git`, `cmake`, `pkg-config`, or any C compiler at build time or runtime
* **No OpenSSL** — `git2` transitively pulls in OpenSSL on many platforms; gor uses `rustls`
* **Pure Rust** — aligns with gor's "single `cargo install`" distribution goal
* **Active maintenance** — the crate must be actively developed and responsive to issues
* **Sufficient API coverage** — must support remote URL parsing, repository discovery, and cloning

## Considered Options

* `gix` — pure Rust git implementation (formerly `gitoxide`)
* `git2` — Rust bindings to `libgit2`
* Shell out to `git` CLI — run `git` as a subprocess

## Decision Outcome

Chosen option: **`gix`**, because it is the only option that satisfies all decision drivers: pure Rust with no system dependencies, no OpenSSL, and no `git` binary requirement. It is actively maintained with a responsive author and has reached sufficient API maturity for gor's needs (repository discovery, remote URL parsing).

### Consequences

* Good, because no C compiler, `cmake`, or `pkg-config` needed at build time
* Good, because no `git` binary needed at runtime — true self-contained binary
* Good, because no OpenSSL dependency — consistent with gor's `rustls`-only TLS policy
* Good, because `gix` is actively developed with frequent releases and a responsive maintainer
* Bad, because `gix` API is less stable than `git2` — breaking changes occur between minor versions
* Bad, because `gix` has less community documentation and fewer Stack Overflow answers than `git2`
* Bad, because some advanced operations (e.g., merge, rebase) are not yet implemented in `gix`

### Confirmation

Enforced via `deny.toml` which bans `git2` from the dependency graph. CI runs `cargo deny check` on every push.

## Pros and Cons of the Options

### gix

Pure Rust git implementation with no C dependencies.

* Good, because pure Rust — compiles on any target that Rust supports
* Good, because no system libraries needed at build time or runtime
* Good, because actively maintained with ~weekly releases
* Good, because supports repository discovery, remote URL parsing, and cloning
* Neutral, because API surface is large and still evolving
* Bad, because API breaking changes are more frequent than `git2`
* Bad, because smaller community and fewer usage examples

### git2

Rust bindings to `libgit2`, a mature C library.

* Good, because mature and battle-tested — used by `cargo` itself
* Good, because stable API with strong backward compatibility guarantees
* Good, because extensive documentation and community knowledge
* Bad, because requires `cmake`, `pkg-config`, and a C compiler at build time
* Bad, because transitively depends on OpenSSL on most platforms
* Bad, because cross-compilation is painful due to C dependencies

### Shell out to git CLI

Run the `git` binary as a subprocess.

* Good, because zero additional Rust dependencies
* Good, because always up-to-date with the user's git version
* Bad, because requires `git` to be installed — breaks the "self-contained binary" goal
* Bad, because parsing CLI output is fragile and locale-dependent
* Bad, because subprocess management adds error-handling complexity

## More Information

The `deny.toml` configuration enforces this decision by banning `git2`:

```toml
[bans]
deny = [
    { name = "git2", reason = "Use gix for all git operations" },
]
```

This decision should be revisited if `gix` development stalls or if gor needs git operations (merge, rebase) that `gix` does not yet support.
