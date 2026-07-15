# Rust Best Practices Applied to gor

Cross-references the Rust CLI tooling research (`research-*.md`) with gor's API implementation roadmap.
This is the "how to build it well" companion to the "what to build" in `api-implementation-roadmap.md`.

---

## Architecture Decisions

### lib + bin split (non-negotiable)
```
src/
  lib.rs          # All business logic, fully unit-testable
  main.rs         # Thin: parse args, init logging, dispatch to lib
  cli.rs          # clap derive definitions
  client.rs       # HTTP client, auth, token resolution
  host.rs         # Multi-host URL derivation
  repository.rs   # Repo spec parsing, remote detection
  output.rs       # JSON, table, and terminal formatting
  error.rs        # Error enum (thiserror)
  config.rs       # Host config persistence
  keyring_store.rs # OS keyring integration
  cmd/
    mod.rs
    api.rs         # Arbitrary API calls
    repo.rs        # repo view/list/clone/create/fork/delete
    pr.rs          # pr list/view/checkout/create/comment/close/reopen/merge/diff/review
    issue.rs       # issue list/view/create/comment/close/reopen/edit
    release.rs     # release list/view/create/edit/delete/upload/download
    label.rs       # label list/create/edit/delete
    search.rs      # search repos/issues/prs/code/commits
    gist.rs        # gist list/view/create/edit/delete
    workflow.rs    # workflow list/view/run/enable/disable
    run.rs         # run list/view/watch/cancel/rerun/download
    browse.rs      # open in browser
    util.rs        # Shared helpers (pagination, body builders, field selection)
```

### Error handling strategy
- **Library layer** (`client.rs`, `repository.rs`, `host.rs`): `thiserror` enum with `#[non_exhaustive]`
- **Application layer** (`cmd/*.rs`): `anyhow::Result` with `.context(...)` for human-readable messages
- **User-facing diagnostics**: `miette` for structured errors with source snippets and help text
- **Exit codes**: 0 (success), 1 (general error), 4 (auth/401/403), consistent with `gh`

### Async strategy: sync by default
gor is a batch CLI tool — it does a task and exits. Synchronous `reqwest::blocking` is the right default.
If parallel operations are needed later (e.g., concurrent API calls for `run watch`), use `rayon` for
CPU-bound work or a targeted Tokio runtime for I/O concurrency, keeping the core library synchronous.

---

## Linting & Formatting Configuration

### `Cargo.toml` workspace lint policy
```toml
[workspace.lints.rust]
unsafe_code = "deny"
unused_qualifications = "warn"
rust_2018_idioms = "warn"

[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
unwrap_used = "deny"
expect_used = "warn"
dbg_macro = "deny"
todo = "deny"
print_stdout = "warn"
print_stderr = "warn"
module_name_repetitions = "allow"
doc_markdown = "allow"
must_use_candidate = "allow"
too_many_lines = "allow"
```

### `clippy.toml`
```toml
msrv = "1.85.0"
cognitive-complexity-threshold = 25
type-complexity-threshold = 200
avoid-breaking-exported-api = true
```

### `rustfmt.toml`
```toml
style_edition = "2024"
max_width = 100
hard_tabs = false
newline_style = "Unix"
use_small_heuristics = "Default"
edition = "2024"
```

### `deny.toml` (cargo-deny)
- `unsound = "deny"`, `yanked = "warn"`, `unmaintained = "warn"`
- Licenses: MIT, Apache-2.0, BSD-2/3-Clause, ISC, MPL-2.0, Unicode-3.0, Zlib
- Bans: `openssl` → `rustls`, `openssl-sys` → `rustls`, `git2` → `gix`
- Sources: `unknown-registry = "deny"`, `unknown-git = "deny"`

---

## CI/CD Pipeline

### Per-PR checks (`.github/workflows/ci.yml`)
```yaml
jobs:
  lint:
    - cargo fmt --all --check
    - cargo clippy --all-targets --all-features -- -D warnings
    - cargo deny check
    - typos
    - cargo shear

  doc:
    - RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
    - cargo test --doc

  test:
    strategy:
      matrix:
        os: [ubuntu-24.04, macos-14, windows-2022]
      fail-fast: false
    - cargo nextest run --workspace --all-features
    - cargo test --doc

  coverage:
    - cargo llvm-cov nextest --workspace --all-features --lcov --output-path lcov.info
    - codecov/codecov-action@v4

  msrv:
    - cargo +1.85.0 check --locked --all-features

  security:
    - rustsec/audit-check@v2.0.0
```

### Release pipeline (`.github/workflows/release.yml`)
- `release-plz` for automated version bumps + changelogs + crates.io publish
- `cargo-dist` for cross-compiled binaries + installers + GitHub Releases
- Trusted Publishing (OIDC) for crates.io — no long-lived tokens

---

## Testing Strategy

### Unit tests (in `src/`)
- Every pure function in `lib.rs`, `repository.rs`, `host.rs`, `output.rs`
- Mock HTTP responses for `client.rs` (or use a test transport)
- Property-based tests (`proptest`) for parsers: repo spec parsing, remote URL parsing, field coercion

### Integration tests (in `tests/`)
- `assert_cmd` + `assert_fs` for black-box CLI testing
- `insta` snapshots for `--help` output, error messages, and table formatting
- Test against a real GitHub API with a test token (or recorded responses via `wiremock`/`httpmock`)

### Doc tests
- Every public API function in `lib.rs` gets a runnable example
- `no_run` for examples that need network access
- `should_panic` for error-path examples

### Scheduled (not per-PR)
- `cargo-mutants` — weekly mutation testing on core logic
- `cargo-fuzz` — fuzz targets for parsers (repo spec, remote URL, field coercion)
- `cargo-semver-checks` — run before every release

---

## Documentation Plan

### Layer 1: API docs (rustdoc)
- `#![deny(missing_docs)]` on the library crate
- `//!` module overviews for every public module
- `///` on every public item with `# Examples`, `# Errors`, `# Panics` sections
- Intra-doc links (`[`Client`]`, `[`Repository`]`) throughout

### Layer 2: README
- Badges: CI, crates.io, license, docs.rs, MSRV
- One-line description + terminal screenshot/GIF
- Quick-start: `cargo install gor` / `cargo binstall gor` / Homebrew
- Usage reference table
- Links to full docs, contributing, license

### Layer 3: mdBook
- Installation guide (cargo, binstall, Homebrew, Docker, prebuilt binaries)
- Quick-start tutorial (auth, first commands)
- Command reference (one page per command group)
- Configuration guide (hosts.yml, environment variables)
- Architecture overview (for contributors)

### Layer 4: CHANGELOG
- Auto-generated by `release-plz` (uses `git-cliff` internally)
- Conventional Commits required for all PRs

---

## Distribution

### End-user install paths (layered)
1. **Installer script:** `curl -fsSL https://gor.sh/install.sh | sh` (cargo-dist generated)
2. **Homebrew:** `brew install gor` (via custom tap)
3. **cargo-binstall:** `cargo binstall gor` (binary-first, no compile)
4. **cargo:** `cargo install gor` (compile from source, slowest)
5. **Docker:** `docker run ghcr.io/gor-sh/gor:latest` (multi-stage, distroless, musl static)

### Binary targets (cargo-dist)
- `x86_64-unknown-linux-gnu` (glibc)
- `x86_64-unknown-linux-musl` (static, for Docker/scratch)
- `aarch64-unknown-linux-gnu` (ARM64 servers)
- `x86_64-apple-darwin` (Intel Mac)
- `aarch64-apple-darwin` (Apple Silicon)
- `x86_64-pc-windows-msvc` (Windows)

---

## Crate Selection Summary

| Concern | Crate | Rationale |
|---------|-------|-----------|
| CLI parsing | `clap` 4.6 (derive) | Industry standard; v5 not stable yet |
| HTTP client | `reqwest` 0.12 (blocking, rustls) | No OpenSSL dependency |
| Git operations | `gix` 0.85 | Pure Rust; no `git` binary needed |
| Serialization | `serde` + `serde_json` + `serde_yaml_ng` | Standard Rust serialization stack |
| Error handling (lib) | `thiserror` 2 | Explicit error enums with `#[non_exhaustive]` |
| Error handling (app) | `anyhow` | `.context()` for user-friendly messages |
| Rich diagnostics | `miette` | Structured errors with source snippets |
| Logging | `tracing` + `tracing-subscriber` | Structured; env-filter for `RUST_LOG` |
| Progress bars | `indicatif` 0.18 | For clone/fetch progress |
| Terminal color | `console` / `anstyle` | Respects `NO_COLOR` |
| Config storage | `confy` or keep current YAML | Simple per-user config; no layering needed |
| OS keyring | `keyring` 3 | Cross-platform credential storage |
| Testing (CLI) | `assert_cmd` + `assert_fs` | Black-box CLI tests |
| Testing (snapshot) | `insta` | `--help` output, error messages, tables |
| Testing (property) | `proptest` | Parser/serializer invariants |
| Benchmarks | `divan` | Lower boilerplate than criterion for CLI |
| Coverage | `cargo-llvm-cov` | Cross-platform, accurate, nextest-compatible |

---

## Design Principles

These principles guide gor's architecture, drawn from the research above:

| Principle | Implementation |
|-----------|---------------|
| Pure Rust, no system deps | `gix` for git, `rustls` for TLS — no `git` binary, no OpenSSL |
| Multi-host from day one | Host resolution: env vars → keyring → config file; GHES base URL derivation |
| Sync by default | `reqwest::blocking`; async only if concurrency demands it |
| lib + bin split | `src/lib.rs` holds all logic; `src/main.rs` is a thin dispatch layer |
| Flag-driven, not interactive | No TUI prompts in v1; all behavior controlled via CLI flags |
| `--json` with field selection | Not all-or-nothing; support `--json field1,field2` + `--jq` for scripting |
| Config in `~/.config/gor/` | YAML hosts file (mode 0600), OS keyring for tokens |
