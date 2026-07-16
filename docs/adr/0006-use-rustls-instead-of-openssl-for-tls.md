---
number: 6
title: Use rustls instead of OpenSSL for TLS
status: accepted
date: 2026-07-16
tags: [security, dependencies]
deciders: [kwhatcher]
---

# Use rustls instead of OpenSSL for TLS

## Context and Problem Statement

gor makes HTTPS requests to the GitHub API and therefore needs a TLS implementation. Rust HTTP clients (`reqwest`) support two TLS backends: `native-tls` (OpenSSL on Linux, Secure Transport on macOS, SChannel on Windows) and `rustls` (pure Rust TLS). Which should gor use?

## Decision Drivers

* **Zero system dependencies** — gor must compile with a single `cargo install` without system library prerequisites
* **Cross-compilation** — gor targets multiple platforms (Linux glibc/musl, macOS, Windows) via `cargo-dist`
* **Static linking** — musl targets require static linking; OpenSSL makes this difficult
* **Security** — OpenSSL has a history of CVEs; `rustls` has a smaller attack surface
* **Build speed** — compiling OpenSSL from source is slow and error-prone

## Considered Options

* `rustls` — pure Rust TLS implementation
* `native-tls` / OpenSSL — platform-native TLS with OpenSSL on Linux
* Platform-specific — use each platform's native TLS stack directly

## Decision Outcome

Chosen option: **`rustls`**, because it is the only option that enables true single-command installation across all target platforms. Pure Rust TLS eliminates the most common build failure in the Rust ecosystem: OpenSSL compilation errors.

### Consequences

* Good, because no system library dependencies — `cargo install gor` just works
* Good, because cross-compilation is straightforward — no C toolchain needed for TLS
* Good, because static musl builds are trivial — no dynamic linking to OpenSSL
* Good, because smaller attack surface — `rustls` is ~20K LOC vs OpenSSL's ~500K
* Good, because `rustls` is the recommended default for new Rust projects
* Good, because `reqwest` supports `rustls` as a first-class backend via `rustls-tls` feature
* Bad, because `rustls` does not support TLS 1.0/1.1 (not needed for GitHub API)
* Bad, because some niche platforms may have `rustls` ring/aws-lc-rs compatibility issues
* Bad, because `rustls` is less battle-tested than OpenSSL (though rapidly maturing)

### Confirmation

Enforced via `deny.toml` which bans `openssl` and `openssl-sys` from the dependency graph. CI runs `cargo deny check` on every push. The `Cargo.toml` specifies `reqwest` with `default-features = false` and `rustls-tls` feature.

## Pros and Cons of the Options

### rustls

Pure Rust TLS implementation.

* Good, because pure Rust — compiles anywhere Rust compiles
* Good, because no C toolchain needed for TLS
* Good, because static linking is trivial
* Good, because smaller, auditable codebase
* Good, because `reqwest` has first-class `rustls-tls` support
* Neutral, because uses `ring` or `aws-lc-rs` for cryptography (both in Rust/C)
* Bad, because no TLS 1.0/1.1 support (not relevant for GitHub API)
* Bad, because less battle-tested than OpenSSL

### native-tls / OpenSSL

Platform-native TLS with OpenSSL on Linux.

* Good, because uses the platform's trusted certificate store
* Good, because OpenSSL is battle-tested and widely audited
* Good, because supports all TLS versions
* Bad, because requires OpenSSL development libraries at build time
* Bad, because cross-compilation is painful — need target-platform OpenSSL
* Bad, because static musl builds require compiling OpenSSL from source
* Bad, because OpenSSL version mismatches cause runtime errors
* Bad, because larger attack surface and history of CVEs

### Platform-specific

Use each platform's native TLS stack directly.

* Good, because uses the OS vendor's TLS implementation
* Bad, because three different code paths to maintain
* Bad, because no unified API — each platform has different TLS configuration
* Bad, because Linux has no single "native" TLS stack (OpenSSL is de facto but not built-in)

## More Information

The `deny.toml` configuration enforces this decision:

```toml
[bans]
deny = [
    { name = "openssl", reason = "Use rustls for all TLS" },
    { name = "openssl-sys", reason = "Use rustls for all TLS" },
]
```

The `Cargo.toml` configures `reqwest` accordingly:

```toml
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "blocking", "json"] }
```

This decision should be revisited if:
- `rustls` development stalls or the crate is abandoned
- A platform gor targets does not support `rustls`'s crypto backend
- GitHub API begins requiring TLS 1.0/1.1 (extremely unlikely)
