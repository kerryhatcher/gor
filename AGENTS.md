# AGENTS.md — AI Agent Development Guide

Instructions for LLM coding agents working on the `gor` repository.

---

## Project Identity

**gor** — GitHub on Rust. A fast, self-contained GitHub CLI, equivalent to `gh` but written in Rust with no external dependencies on `git` or OpenSSL.

- **Binary:** `gor`
- **Crate:** `gor` (published on crates.io)
- **License:** MIT OR Apache-2.0
- **MSRV:** 1.85
- **Edition:** 2024

---

## Architecture (non-negotiable)

```
src/
  lib.rs          # All business logic — fully unit-testable, no I/O in pure functions
  main.rs         # Thin entry point: parse args, init tracing, dispatch to lib
  cli.rs          # clap derive definitions (Args, Subcommand enums)
  client.rs       # HTTP client, auth, token resolution, URL building
  host.rs         # Multi-host support (github.com + GHES), URL derivation
  repository.rs   # Repo spec parsing (OWNER/REPO), remote URL detection via gix
  output.rs       # JSON, table, and terminal formatting
  error.rs        # Error enum (thiserror), exit codes
  config.rs       # Host config persistence (~/.config/gor/hosts.yml, mode 0600)
  keyring_store.rs # OS keyring integration
  auth/
    mod.rs         # Login/logout/status/token orchestration
    device.rs      # OAuth device flow
    token.rs       # Token verification + stdin reading
  cmd/
    mod.rs
    api.rs         # Arbitrary REST API calls
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

**Rules:**
- `src/main.rs` must remain thin — parse args, init logging, call into `lib.rs`. Never put business logic here.
- Every public item in `lib.rs` must have a doc comment with `# Examples`.
- New command groups go in `src/cmd/`. Each file gets a `mod` entry in `src/cmd/mod.rs`.
- Shared logic between commands goes in `src/cmd/util.rs`. Never duplicate pagination, body building, or field selection code.

---

## Development Workflow

All common tasks have `just` recipes. Run `just` with no arguments to list them.

```bash
just build          # Debug build
just release        # Release build (optimized, stripped, LTO)
just test           # Full test suite (nextest + doc tests)
just test-unit      # Unit tests only (src/)
just test-integration # Integration tests only (tests/)
just lint           # Format check + clippy with -D warnings
just lint-fix       # Auto-fix formatting + clippy
just doc            # Build docs and open in browser
just coverage       # Generate lcov coverage report
just audit          # cargo-deny + cargo-audit
just typos          # Spellcheck
just shear          # Unused dependency check
just ci             # Full CI gate (lint + test + audit + typos + shear)
just clean          # Remove target/
just install        # Install gor from source
```

**Always run `just ci` before pushing.** This mirrors what CI will run.

---

## Linting & Formatting

### Policy is centralized in `Cargo.toml` under `[workspace.lints]`

**Deny (hard gates — CI fails):**
- `unsafe_code` — no unsafe in application code
- `unwrap_used` — use `?` or proper error handling
- `dbg_macro` — no dbg!() in committed code
- `todo` — no todo!() in committed code

**Warn (visible, fix over time):**
- `expect_used` — prefer proper error handling over expect()
- `print_stdout`, `print_stderr` — use tracing for logging
- `pedantic` and `nursery` groups (with negative priority so they can be overridden)

**Allow (explicitly suppressed noise):**
- `module_name_repetitions`, `doc_markdown`, `must_use_candidate`, `too_many_lines`

### Formatting
- `style_edition = "2024"`, `max_width = 100`
- Run `just lint-fix` before committing

### Spellcheck
- `typos` runs in CI. Add false positives to `_typos.toml` under `[default.extend-words]`.

---

## Error Handling

- **Library code** (`client.rs`, `repository.rs`, `host.rs`, `config.rs`): Use `thiserror` — define explicit error variants in `error.rs`. Never expose third-party error types in public APIs.
- **Command code** (`cmd/*.rs`): Use `anyhow::Result` with `.context(...)` for human-readable messages.
- **User-facing diagnostics**: Use `miette` for structured errors with source snippets and help text.
- **Exit codes**: 0 (success), 1 (general error), 4 (auth/401/403).

---

## Testing

### Unit tests
- Live in `#[cfg(test)] mod tests` blocks inside source files.
- Test pure functions in `lib.rs`, `repository.rs`, `host.rs`, `output.rs`.
- Use `proptest` for parsers (repo spec, remote URL, field coercion).

### Integration tests
- Live in `tests/` directory.
- Use `assert_cmd` + `assert_fs` for black-box CLI testing.
- Use `insta` for snapshot testing of `--help` output, error messages, and table formatting.
- Use `wiremock` to mock GitHub API responses.

### Doc tests
- Every public function in `lib.rs` gets a runnable `# Examples` section.
- Use `no_run` for examples that need network access.
- Use `should_panic` for error-path examples.

---

## Commit Conventions

Use **Conventional Commits**. This is required — `release-plz` uses commit messages to generate changelogs.

```
feat: add repo create command
fix: handle 429 rate limit responses
docs: document auth flow in README
test: add integration tests for pr checkout
refactor: extract pagination logic to util.rs
chore: update dependencies
ci: add windows to test matrix
```

**Scopes** (optional but encouraged): `auth`, `api`, `repo`, `pr`, `issue`, `release`, `label`, `search`, `gist`, `workflow`, `run`, `client`, `output`, `config`, `ci`, `docs`.

---

## CI Pipeline

CI runs on every push and PR. Eight parallel jobs:

| Job | What it checks |
|-----|---------------|
| `lint` | fmt, clippy, cargo-deny, typos, cargo-shear |
| `doc` | doc generation warnings, doc tests |
| `test` | nextest + doc tests on ubuntu, macos, windows |
| `coverage` | llvm-cov + codecov upload |
| `msrv` | cargo check with Rust 1.85 |
| `security` | cargo-audit via rustsec |
| `trivy` | vuln, secret, and misconfig scanning via Trivy |
| `kingfisher` | secret scanning with live validation via Kingfisher |

**If CI fails, fix it before pushing more commits.** Do not disable lint gates with `#[allow(...)]` without a documented reason.

---

## Key Design Decisions

1. **REST-first, GraphQL later.** REST covers all P0/P1 needs. GraphQL is deferred.
2. **gix over git2.** Pure Rust, no OpenSSL, no `git` binary dependency.
3. **Multi-host from day one.** github.com and GHES use the same code paths.
4. **Sync by default.** `reqwest::blocking`. Async only if I/O concurrency demands it.
5. **Flag-driven, not interactive.** No TUI prompts in v1. All behavior controlled via CLI flags.
6. **`--json` with field selection.** Not all-or-nothing. Support `--json field1,field2` + `--jq`.
7. **Config in `~/.config/gor/`.** YAML hosts file (mode 0600), OS keyring for tokens.

---

## Adding a New Command

1. Define the CLI args in `src/cli.rs` (Args struct + Subcommand variant).
2. Create `src/cmd/<command>.rs` with a `run()` function.
3. Add `pub mod <command>;` to `src/cmd/mod.rs`.
4. Wire the dispatch in `src/main.rs` `match cli.command { ... }`.
5. Add integration tests in `tests/` using `assert_cmd`.
6. Add snapshot tests for `--help` output using `insta`.
7. Update the usage table in `README.md`.
8. Run `just ci` before committing.

---

## Dependency Policy

- **No OpenSSL.** Use `rustls` everywhere. `deny.toml` bans `openssl` and `openssl-sys`.
- **No git2.** Use `gix` for all git operations. `deny.toml` bans `git2`.
- **Minimize dependencies.** Each new dependency must justify itself. Prefer crates from the rust-cli ecosystem.
- **Pin versions** in `[workspace.dependencies]`. Member crates inherit with `{ workspace = true }`.

---

## Documentation

- `#![deny(missing_docs)]` on the library crate.
- `//!` module overviews for every public module.
- `///` on every public item with `# Examples`, `# Errors`, `# Panics` sections.
- Intra-doc links (`[`Client`]`, `[`Repository`]`) throughout.
- README synced from crate docs via `cargo-rdme`.

---

## Resources

- `docs/research/api-overview.md` — GitHub API fundamentals (auth, versioning, pagination, rate limits)
- `docs/research/api-implementation-roadmap.md` — Prioritized 4-phase implementation plan
- `docs/research/rust-best-practices-applied.md` — Crate selection, config rationale, design principles
- `docs/research/research-linting.md` — Clippy, rustfmt, supply-chain tooling details
- `docs/research/research-cicd.md` — CI/CD pipeline design and action versions
- `docs/research/research-testing.md` — Testing strategy and tooling
- `docs/research/research-architecture.md` — Crate ecosystem and architecture patterns
- `docs/research/research-docs.md` — Documentation best practices
- `docs/research/research-deployment.md` — Distribution and release automation
