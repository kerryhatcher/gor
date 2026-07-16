---
number: 8
title: Error handling strategy with thiserror, anyhow, and miette
status: accepted
date: 2026-07-16
tags: [architecture, error-handling]
deciders: [kwhatcher]
---

# Error handling strategy with thiserror, anyhow, and miette

## Context and Problem Statement

gor needs a consistent error handling strategy across its library and application layers. Rust offers several error handling patterns and crates: `thiserror` for library error types, `anyhow` for application-level error propagation, `miette` for rich diagnostics, and `color-eyre` for colorful error reports. How should gor structure its error handling?

## Decision Drivers

* **Library vs application split** — library code needs typed errors; application code needs convenient propagation
* **User-facing diagnostics** — error messages should be clear, actionable, and include context
* **Exit codes** — gor must exit with appropriate codes (0, 1, 4) for scripting
* **Third-party error isolation** — library APIs must not expose third-party error types
* **Testability** — error types should be easy to construct and match in tests

## Considered Options

* `thiserror` (lib) + `anyhow` (app) + `miette` (diagnostics)
* `thiserror` (lib) + `anyhow` (app) only — no rich diagnostics
* `anyhow` everywhere — no typed library errors
* `thiserror` everywhere — typed errors at all layers

## Decision Outcome

Chosen option: **`thiserror` (lib) + `anyhow` (app) + `miette` (diagnostics)**, because it provides the right tool for each layer: typed, exhaustive errors in the library; convenient propagation with context in commands; and rich, IDE-like diagnostics for users.

### Consequences

* Good, because library errors are typed and exhaustive — consumers know exactly what can fail
* Good, because `#[non_exhaustive]` on library errors allows adding variants without breaking changes
* Good, because command code uses `.context(...)` for human-readable messages without defining new types
* Good, because `miette` provides source snippets, labels, and help text for the most critical errors
* Good, because exit codes are centralized in `error.rs` — consistent across all commands
* Bad, because three error crates instead of one — more dependencies and concepts to learn
* Bad, because `miette` integration requires implementing its `Diagnostic` trait on library errors
* Bad, because error conversion between layers requires explicit `match` or `From` impls

### Confirmation

The `src/error.rs` module defines the library error enum with `thiserror`. Command modules in `src/cmd/` use `anyhow::Result` and `.context(...)`. The `Cargo.toml` includes all three crates. CI enforces `unwrap_used = "deny"` to ensure proper error propagation.

## Pros and Cons of the Options

### thiserror (lib) + anyhow (app) + miette (diagnostics)

Typed library errors, convenient app propagation, rich diagnostics.

* Good, because each layer gets the right abstraction
* Good, because library errors are exhaustive and matchable
* Good, because application code is concise with `.context(...)` and `?`
* Good, because `miette` produces IDE-quality error reports
* Neutral, because three crates to learn and maintain
* Bad, because `miette` integration requires additional trait implementations

### thiserror (lib) + anyhow (app) only

Typed library errors with convenient app propagation, no rich diagnostics.

* Good, because simpler than the three-crate approach
* Good, because still provides typed library errors
* Bad, because error messages lack source snippets and structured help text
* Bad, because less polished user experience for error cases

### anyhow everywhere

No typed library errors; `anyhow::Result` at all layers.

* Good, because simplest possible error handling
* Good, because no error type definitions needed
* Bad, because library consumers cannot match on specific error variants
* Bad, because no compile-time guarantee that all error cases are handled
* Bad, because violates the Rust ecosystem best practice of typed library errors

### thiserror everywhere

Typed errors at all layers, including command code.

* Good, because maximum type safety
* Bad, because command code must define error types for every operation
* Bad, because error type explosion — many single-use error variants
* Bad, because more boilerplate than `anyhow` for application code

## More Information

### Error architecture

```
src/error.rs          # GorError enum (thiserror), exit codes
src/client.rs         # Uses GorError for HTTP/auth errors
src/cmd/*.rs          # Uses anyhow::Result, .context(...)
src/main.rs           # Formats errors with miette, exits with code
```

### Exit code mapping

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 4 | Authentication/authorization error (401/403) |

### Lint enforcement

```toml
[workspace.lints.clippy]
unwrap_used = "deny"     # Force proper error handling
expect_used = "warn"     # Prefer proper errors over expect()
```

This decision should be revisited if:
- `miette` adds unacceptable compile time or binary size overhead
- A new error handling crate gains significant ecosystem adoption
- The three-crate approach proves confusing for contributors
