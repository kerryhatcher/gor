---
number: 4
title: Sync-by-default concurrency model using reqwest blocking
status: accepted
date: 2026-07-16
tags: [async, architecture, dependencies]
deciders: [kwhatcher]
---

# Sync-by-default concurrency model using reqwest blocking

## Context and Problem Statement

gor is a CLI tool that makes HTTP requests to the GitHub API. Rust offers two concurrency models: synchronous (blocking I/O) and asynchronous (non-blocking I/O with an async runtime like Tokio). Which model should gor adopt as its primary approach?

## Decision Drivers

* **Simplicity** — synchronous code is easier to write, debug, and test
* **Startup time** — async runtimes add initialization overhead
* **Dependency footprint** — Tokio is a large dependency
* **Concurrency needs** — gor is a batch CLI tool; most commands make sequential API calls
* **Ecosystem alignment** — `reqwest::blocking` is well-supported and widely used

## Considered Options

* Synchronous with `reqwest::blocking` — blocking HTTP client, no async runtime
* Async with Tokio + `reqwest` — non-blocking HTTP client with Tokio runtime
* Hybrid — sync core library with async shell when needed

## Decision Outcome

Chosen option: **Synchronous with `reqwest::blocking`**, because gor is a batch CLI tool that does a task and exits. The simplicity of synchronous code outweighs the potential performance benefits of async for gor's use case. If parallel I/O is needed later, targeted async or `rayon` can be added without converting the entire codebase.

### Consequences

* Good, because simpler code — no `async`/`await`, no `Pin`, no `Send + Sync` bounds
* Good, because easier debugging — synchronous stack traces are straightforward
* Good, because faster startup — no Tokio runtime initialization
* Good, because smaller binary — no Tokio dependency
* Good, because `reqwest::blocking` is a thin wrapper around the async client, so switching later is feasible
* Bad, because sequential API calls are slower than concurrent ones for multi-entity operations
* Bad, because no built-in timeout/reactor — must manage timeouts manually
* Bad, because blocking I/O can stall if a request hangs without a timeout

### Confirmation

All HTTP client code in `src/client.rs` uses `reqwest::blocking::Client`. There is no `tokio` or `async` code in the codebase. CI verifies this via `cargo deny check` (no Tokio in the dependency graph unless explicitly needed later).

## Pros and Cons of the Options

### Synchronous with reqwest::blocking

Blocking HTTP client with no async runtime.

* Good, because simple, linear code that is easy to reason about
* Good, because no async runtime dependency — smaller binary, faster startup
* Good, because `reqwest::blocking` is a thin wrapper; switching to async is feasible
* Good, because easier to test — no need for `#[tokio::test]` or async test harnesses
* Neutral, because `reqwest::blocking` internally uses a Tokio runtime (hidden)
* Bad, because sequential requests are slower than concurrent ones
* Bad, because blocking on I/O can stall the entire process

### Async with Tokio + reqwest

Non-blocking HTTP client with Tokio runtime.

* Good, because concurrent requests are faster for multi-entity operations
* Good, because built-in timeout and cancellation support
* Good, because `reqwest` (async) is the primary maintained variant
* Bad, because adds Tokio as a dependency (~20+ crates)
* Bad, because async code is harder to debug and test
* Bad, because `Send + Sync` bounds propagate through the codebase
* Bad, because slower startup due to runtime initialization

### Hybrid

Sync core library with async shell when needed.

* Good, because keeps business logic synchronous and testable
* Good, because async can be added incrementally for specific commands
* Neutral, because requires careful API design to support both modes
* Bad, because two I/O patterns in one codebase increase complexity
* Bad, because still requires Tokio as a dependency

## More Information

This decision follows the principle "async is an optimization, not an architecture" from Microsoft's Rust training materials. If gor later needs concurrent API calls (e.g., for `run watch` polling multiple workflow runs), the recommended approach is:

1. Keep the core library synchronous
2. Add a targeted Tokio runtime in the specific command handler
3. Use `tokio::task::spawn_blocking` for any CPU-bound work

This decision should be revisited if:
- A command requires 5+ concurrent API calls for acceptable performance
- Streaming responses (SSE, WebSocket) are needed
- The `reqwest::blocking` variant is deprecated
