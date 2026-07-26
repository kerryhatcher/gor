# Handoff: gor-core Library Split (Issue #2)

## Project Overview

**Goal:** Split the single-crate `gor` CLI into a Cargo workspace with:
- **`gor-core`** — reusable library crate with typed GitHub API operations, models, and infrastructure
- **`gor`** — thin CLI binary for argument parsing, rendering, and dispatching to `gor-core`

**Binary:** `gor` (unchanged)  
**Package names:** `gor-cli` (crates.io), `gor-core` (new)  
**Branch:** `refactor/split-core-library`  
**Draft PR:** [#3](https://github.com/kerryhatcher/gor/pull/3)  
**Issue:** [#2](https://github.com/kerryhatcher/gor/issues/2) — full design spec

---

## What's Been Done

### Phase 0 — Setup
- Branch `refactor/split-core-library` created and pushed
- Draft PR opened, linked to issue #2
- Design spec at `docs/superpowers/specs/2026-07-20-core-library-split-design.md`

### Phase 1 — Workspace Restructuring
- Root `Cargo.toml` is a pure workspace with `members = ["crates/*"]`
- `crates/gor-core/` — library (`name = "gor_core"`), keeps: `reqwest`, `serde`, `serde_json`, `serde_yaml_ng`, `keyring` (optional), `thiserror`, `dirs`, `gix`, `tracing`
- `crates/gor/` — binary (`name = "gor"`), depends on `gor-core`, keeps: `clap`, `clap_complete`, `anyhow`, `miette`, `tracing-subscriber`, `indicatif`, `console`, `gix`, `serde_yaml_ng`, `dirs`
- All imports updated: core files use `gor_core::` prefix, `output.rs` → `render.rs`
- **Commit:** `26e8b18` — `build: split workspace into gor-core + gor crates`

### Phase 2 — Label Template
- `crates/gor-core/src/label.rs` — typed `Label` struct (`#[non_exhaustive]` + capture-rest `extra`), `Label::list/create/update/delete/clone_from` ops
- `crates/gor-core/src/util.rs` — `urlencode_label_name` helper
- CLI handler thinned to args → op → render pattern
- **Tests:** 49 gor-core unit tests + 120 gor unit tests
- **Commit:** `793f733` — `refactor(label): extract typed ops into gor-core`

### Phase 3a–3b — 8 More Domains Converted

| Domain | Core Module | CLI Handler | Commit |
|--------|------------|-------------|--------|
| **cache** | `crates/gor-core/src/cache.rs` | `crates/gor/src/cmd/cache.rs` | `02de2b3` |
| **org** | `crates/gor-core/src/org.rs` | `crates/gor/src/cmd/org.rs` | `02de2b3` |
| **secret** | `crates/gor-core/src/secret.rs` | `crates/gor/src/cmd/secret.rs` | `4621f83` |
| **variable** | `crates/gor-core/src/variable.rs` | `crates/gor/src/cmd/variable.rs` | `4621f83` |
| **project** | `crates/gor-core/src/project.rs` | `crates/gor/src/cmd/project.rs` | `0b409e2` |
| **keys** | `crates/gor-core/src/keys.rs` | `crates/gor/src/cmd/keys.rs` | `27f90b9` |
| **search** | `crates/gor-core/src/search.rs` | `crates/gor/src/cmd/search.rs` | `27f90b9` |
| **workflow** | `crates/gor-core/src/workflow.rs` | `crates/gor/src/cmd/workflow.rs` | `27f90b9` |

---

## What Remains

### Phase 3c–3d — 7 domains still to convert

| Domain | LOC | Priority | Notes |
|--------|-----|----------|-------|
| `gist` | 518 | 3c | Create/list/view/edit/delete |
| `run` | 636 | 3c | List/view/watch/cancel/rerun/download |
| `codespace` | 647 | 3c | List/create/delete/ssh/stop |
| `release` | 1313 | 3d | Create/edit/delete/list/view/upload/download |
| `repo` | 1317 | 3d | View/list/create/fork/delete/edit/clone/sync/transfer |
| `issue` | 1332 | 3d | List/view/create/edit/close/reopen/comment/transfer |
| `pr` | 2236 | 3d | List/view/create/close/reopen/merge/diff/checkout/comment/review/checks/ready/edit |

### Phase 4 — Cleanup & Publishability (not started)
- Delete dead bin helpers (e.g., old `urlencoding` in label.rs → now in `gor_core::util`)
- Move shared pagination/field-selection into `gor_core::util`
- Audit `gor-core` for any remaining `anyhow`/`print_stdout`/`console`/`indicatif` usage
- Add `gor-core` metadata: `description`, `keywords`, `categories`
- Verify `cargo doc --no-deps -p gor-core` clean

### Phase 5 — Merge & Release (not started)
- Merge PR, referencing `closes #2`
- `release-plz` publishes `gor-core` 0.1.0 + bumped `gor-cli`
- Update README with consumer example

---

## Conversion Pattern (follow this for remaining domains)

### 1. Core Module (`crates/gor-core/src/<domain>.rs`)
- Typed `struct` with `#[non_exhaustive]` and `#[serde(flatten)] extra`
- Options structs (`ListOptions`, `CreateOptions`, etc.) — use owned `String` not `&str`
- Operation functions returning `Result<T, GorError>` (never `anyhow`, never print)
- Status→error mapping: `404 → GorError::NotFound`, `422 → GorError::InvalidInput`, other failures → `GorError::InvalidInput(msg)`
- `#![allow(clippy::missing_errors_doc)]` at module top (to avoid per-function doc burden)
- `///` doc comments on all structs and fields (required by `#![deny(missing_docs)]`)

### 2. Register in `crates/gor-core/src/lib.rs`
- Add `pub mod <domain>;` in alphabetical order

### 3. Thin CLI Handler (`crates/gor/src/cmd/<domain>.rs`)
- Keep `pub fn run(cmd: DomainCommand)` — dispatches to private helpers
- Helpers: resolve args → call `gor_core::<domain>::*` → render output
- Import `gor_core::<domain>::{self, TypeName}`
- Use `Client::new(host).map_err(|e| anyhow::anyhow!("..."))` pattern
- Keep all `println!`, prompts, and rendering in the CLI handler
- Keep `#![allow(clippy::print_stdout)]` (allowed for CLI crate)
- Doc comments on `pub fn run` (required by `#![deny(missing_docs)]`)

### 4. Add `pub mod <domain>;` to `crates/gor/src/cmd/mod.rs`

### 5. Gate
- `cargo build` must pass
- `cargo clippy --all-targets` must pass
- `cargo test` must pass (existing 118 gor + 49 gor-core + doc tests = ~187 tests)
- Commit: `refactor(<domain>): extract typed ops into gor-core`

---

## Key Constraints & Pitfalls

### Lint rules (deny, will fail CI):
- **`#![deny(missing_docs)]`** in both crates — every pub item, struct field, and enum variant needs a doc comment
- **`unwrap_used = "deny"`** — no `.unwrap()` or `.unwrap_err()` ever. Use `?` or pattern matching
- **`dbg_macro = "deny"`** — no `dbg!()`
- **`todo = "deny"`** — no `todo!()`
- **`clippy::use_self`** — in `impl` blocks, use `Self::Variant` not `EnumName::Variant`
- **`clippy::missing_errors_doc`** — suppress with `#![allow(clippy::missing_errors_doc)]` at module top
- **`clippy::uninlined_format_args`** — use `format!("{var}")` not `format!("{}", var)`
- **`clippy::format_push_string`** — use `write!` macro instead of `push_str(&format!(...))`
- **`clippy::too_many_arguments`** — suppress with `#![allow(clippy::too_many_arguments)]`

### Error handling:
- Core ops return `Result<T, GorError>` — never `anyhow::Result`
- `GorError` variants: `Http(reqwest::Error)`, `Auth(String)`, `NotFound(String)`, `RateLimit(String)`, `InvalidInput(String)`, `Io(std::io::Error)`, `Keyring(String)`, `DeviceTimeout(String)`, `DeviceDeclined`
- CLI handlers return `anyhow::Result<()>` — wrap errors with `map_err(|e| anyhow::anyhow!("..."))`

### `#[non_exhaustive]` structs:
- Cannot be constructed outside the defining crate
- Test data must use `serde_json::from_value(json!({...}))` to construct instances

### `extra` field idiom:
```rust
/// Any additional fields returned by the API.
#[serde(flatten)]
pub extra: serde_json::Map<String, serde_json::Value>,
```
- Always include this on typed models to capture unknown API fields
- Requires a doc comment

### CLI handler spec resolution helper (copy-paste template):
```rust
fn resolve_spec(repo: Option<&str>) -> anyhow::Result<gor_core::RepoSplit> {
    match repo {
        Some(s) => Ok(parse_repo_spec(s)?),
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        }),
    }
}

fn build_client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("failed to create HTTP client: {e}"))
}
```

---

## Current File Structure

```
Cargo.toml                          # Workspace root
crates/
  gor-core/
    Cargo.toml
    src/
      lib.rs                        # Exports all core modules
      client.rs                     # HTTP client (reqwest::blocking)
      host.rs                       # Host URL derivation
      config.rs                     # Config management
      error.rs                      # GorError enum
      keyring_store.rs              # OS keyring integration
      repository.rs                 # Repo spec parsing, remote detection
      auth/                         # OAuth device flow, token verification
        mod.rs
        device.rs
        token.rs
      cache.rs                      # ✅ Converted
      keys.rs                       # ✅ Converted
      label.rs                      # ✅ Converted (template)
      org.rs                        # ✅ Converted
      project.rs                    # ✅ Converted
      search.rs                     # ✅ Converted
      secret.rs                     # ✅ Converted
      util.rs                       # urlencode_label_name
      variable.rs                   # ✅ Converted
      workflow.rs                   # ✅ Converted
  gor/
    Cargo.toml
    src/
      lib.rs                        # Gor struct, re-exports cli/cmd/render
      main.rs                       # Thin entry point
      cli.rs                        # clap derive structs
      render.rs                     # print_json, format_date, format_count
      cmd/
        mod.rs                      # Dispatches all subcommands
        alias.rs                    # Bin-only
        api.rs                      # Bin-only
        attestation.rs              # Bin-only
        auth.rs                     # Bin-only
        browse.rs                   # Bin-only
        cache.rs                    # ✅ Converted
        classroom.rs                # Bin-only
        codespace.rs                # 🔜 Needs conversion
        completion.rs               # Bin-only
        config.rs                   # Bin-only
        copilot.rs                  # Bin-only
        extension.rs                # Bin-only
        gist.rs                     # 🔜 Needs conversion
        issue.rs                    # 🔜 Needs conversion
        keys.rs                     # ✅ Converted
        label.rs                    # ✅ Converted
        org.rs                      # ✅ Converted
        pr.rs                       # 🔜 Needs conversion
        project.rs                  # ✅ Converted
        release.rs                  # 🔜 Needs conversion
        repo.rs                     # 🔜 Needs conversion
        ruleset.rs                  # Bin-only
        run.rs                      # 🔜 Needs conversion
        search.rs                   # ✅ Converted
        secret.rs                   # ✅ Converted
        util.rs                     # truncate helper
        variable.rs                 # ✅ Converted
        workflow.rs                 # ✅ Converted
```

---

## Quick Build & Test

```bash
just build          # cargo build
just test           # cargo test (expect ~187 tests passing)
just lint           # cargo fmt + clippy
just ci             # Full CI gate
```

---

## Critical Reminders

1. **The `#[allow(clippy::elided_lifetimes_in_paths)]` is a rustc lint, not clippy** — use `#[allow(elided_lifetimes_in_paths)]` without `clippy::` prefix
2. **After every successful conversion**, commit and push to the `refactor/split-core-library` branch
3. **All existing tests must continue to pass** — no behavior changes
4. **The `pr.rs` file (2236 LOC) is the largest and most complex** — may need to be split into submodules (`pr::list`, `pr::view`, `pr::merge`, etc.)
5. **`release`, `repo`, `issue`, `pr` have `#[cfg(test)]` blocks** — preserve these tests when converting
6. **The `just` pre-commit hooks run `cargo fmt`, `cargo clippy`, `cargo check`, and `typos`** — ensure all pass before the commit will succeed
