# Research: Rust CLI Linting, Formatting & Static Analysis (2025–2026 Best Practices)

## Summary
A best-of-breed Rust CLI project in 2025–2026 centralizes lint policy in `Cargo.toml` via the `[workspace.lints]` table (stable since Rust 1.74), sets `clippy.toml` only for behavioral tunables (MSRV, complexity thresholds, disallowed items), formats with `rustfmt.toml` using `style_edition = "2024"`, and gates CI with `cargo clippy --all-targets --all-features -- -D warnings`. Supply-chain hygiene is enforced by `cargo-deny` (license/security/source bans), `cargo-audit` (advisory DB), `typos-cli` (spellcheck), and the newer `cargo-shear` / `cargo-vet` / `cargo-semver-checks` tools. The `[lints.cargo]` manifest-lint table is still **nightly-only** and should not be relied on for stable CI.

## Key Versions (as of mid-2026)
| Tool | Version / status | Notes |
|---|---|---|
| Rust toolchain | 1.85+ for `style_edition = "2024"`; `[lints]` table stable since **1.74** | `[lints.cargo]` still **nightly** (`-Zlints`) |
| cargo-deny | **0.20.2** | 4 checks: advisories, licenses, bans, sources |
| cargo-audit (RustSec) | **0.22.2** (0.22.0 released 2025-11-07) | Scans `Cargo.lock` vs RustSec DB; `cargo audit bin` for auditable binaries |
| typos-cli | **1.48.0** | `_typos.toml` config |
| cargo-shear | **1.12.4** | Successor to machete/udeps; finds unused/misplaced/unlinked deps, can remove them |
| cargo-machete | **0.9.2** (0.9.0/0.9.1 Aug 2025) | Fast text/regex unused-dep detection, **no nightly** |
| cargo-udeps | **0.1.61** | Compiler-based, needs **nightly** (`cargo +nightly udeps`) |
| cargo-outdated | **0.19.0** (Apr 2026) | Reports newer dependency versions |
| cargo-semver-checks | **0.48.0** | Detects semver-breaking public-API changes |
| cargo-dylint | **6.0.1** | Run custom lint libraries |
| cargo-vet (Mozilla) | current | Human audit certification of every dependency; shareable audits |

---

## Findings

### 1. Clippy lint levels: configure via `Cargo.toml`, not scattered attributes
Since Rust 1.74 you centralize lint policy in the `[lints.clippy]` / `[lints.rust]` tables; this is preferred over `#![allow(...)]` attributes scattered through source. Levels are `allow`, `warn`, `deny`, `forbid`. Use `deny` for quality gates and `allow` to suppress noisy lints; prefer `deny` over `forbid` for style because `forbid` blocks all downstream suppression. [Source](https://doc.rust-lang.org/stable/clippy/configuration.html) [Source](https://rustprojectprimer.com/checks/lints.html)

### 2. Workspace inheritance pattern
Define policy once in the root `Cargo.toml` under `[workspace.lints.*]` and inherit per crate with `[lints] workspace = true`. The `missing_lints_inheritance` cargo lint (warn) flags members that forget to set `lints.workspace = true`. [Source](https://rustprojectprimer.com/checks/lints.html) [Source](https://doc.rust-lang.org/stable/cargo/reference/lints.html)

### 3. `pedantic` / `nursery` / `restriction` are not deny-by-default — cherry-pick
`clippy::pedantic` is allow-by-default because some lints have false positives. Best practice is to enable the group at `warn` with a negative priority (so it can be overridden), then selectively `deny` the specific lints you want as hard gates, and `allow` the noisy ones. Do **not** blanket-`deny` the entire pedantic/nursery/restriction groups. [Source](https://rs4ts.dev/24-tooling/07-ci-cd/) [Source](https://rustprojectprimer.com/checks/lints.html)

### 4. CI hard gate
`cargo clippy --all-targets --all-features -- -D warnings` turns every warn into a deny, so the job fails on any warning (clippy or rustc). Run on every PR. [Source](https://rustprojectprimer.com/checks/lints.html) [Source](https://rs4ts.dev/24-tooling/07-ci-cd/)

### 5. `clippy.toml` is for behavioral tunables, not lint levels
`clippy.toml` / `.clippy.toml` configures *thresholds and behavior*, not allow/warn/deny (those go in `Cargo.toml`). Key options: `msrv` (or rely on `rust-version` in `Cargo.toml`), `cognitive-complexity-threshold` (default 25), `type-complexity-threshold`, `large-error-threshold` (default 128, drives `result_large_err`), `enum-variant-size-threshold` (default 200), `avoid-breaking-exported-api` (default true), `doc-valid-idents`, and restriction-oriented `disallowed-methods` / `disallowed-types` / `disallowed-macros`. [Source](https://doc.rust-lang.org/clippy/lint_configuration.html) [Source](https://doc.rust-lang.org/clippy/configuration.html)

### 6. rustfmt: use `style_edition = "2024"`
A `rustfmt.toml` with `style_edition = "2024"` aligns formatting with the 2024 style guide (Rust 1.85+). Keep most settings at defaults (`max_width = 100`, `hard_tabs = false`, `newline_style = "Unix"`, `use_small_heuristics = "Default"`). Import-granularity options (`imports_granularity`, `group_imports`) are **unstable** and require nightly + `unstable_features = true`; best-of-breed CLI projects generally keep the config minimal and stable to avoid forcing nightly. [Source](https://doc.rust-lang.org/edition-guide/rust-2024/rustfmt-style-edition.html) [Source](https://github.com/rust-lang/rustfmt/blob/HEAD/Configurations.md)

### 7. `[lints.cargo]` manifest lints are still nightly-only
The `cargo::` manifest lint group (e.g. `unused_dependencies`, `non_kebab_case_bins`, `unused_workspace_dependencies`) is documented as unstable and gated behind `-Zlints` on nightly; it is **not** available on stable as of mid-2026. Use `cargo-machete` / `cargo-shear` / `cargo-udeps` for unused-dependency detection on stable instead. [Source](https://doc.rust-lang.org/nightly/cargo/reference/lints.html) [Source](https://github.com/rust-lang/cargo/issues/12115)

### 8. cargo-deny: the supply-chain gate
`cargo-deny` (v0.20.2) lints the dependency graph across four independently-configurable checks — **advisories** (RustSec DB), **licenses** (allow/deny SPDX), **bans** (forbidden crates, duplicate versions, wildcards), **sources** (allowed registries/git). Each check emits error or warning so policies can be adopted incrementally. Its advisory check subsumes `cargo-audit`, but `cargo-audit` is lighter weight when you only need advisory scanning. Generate a starter `deny.toml` with `cargo deny init`. [Source](https://embarkstudios.github.io/cargo-deny/checks/cfg.html) [Source](https://rustprojectprimer.com/checks/audit.html)

### 9. cargo-audit, cargo-outdated, cargo-udeps, cargo-machete
- **cargo-audit** (v0.22.2): scans `Cargo.lock` against RustSec DB; exits non-zero on vulns. `cargo audit bin` inspects compiled binaries built with `cargo-auditable`. [Source](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- **cargo-outdated** (v0.19.0): `cargo outdated` lists deps with newer versions. [Source](https://crates.io/crates/cargo-outdated)
- **cargo-udeps** (v0.1.61): compiler-based, most accurate unused-dep detection but **requires nightly** (`cargo +nightly udeps`). [Source](https://crates.io/crates/cargo-udeps)
- **cargo-machete** (v0.9.2): regex/text scan, fast, **no nightly**; good first-pass. `cargo machete`. [Source](https://github.com/bnjbvr/cargo-machete)

### 10. typos-cli: spellcheck on every PR
`typos-cli` (v1.48.0) is a code-aware spellchecker with low false positives; run `typos` on every PR. Suppress false positives via `_typos.toml` using `[default.extend-words]` (word = "word"), `[default.extend-identifiers]`, or `extend-ignore-identifiers-re`. [Source](https://rustprojectprimer.com/checks/lints.html) [Source](https://github.com/crate-ci/typos/blob/HEAD/docs/reference.md)

### 11. Newer / emerging tooling
- **cargo-shear** (v1.12.4): modern replacement for machete/udeps; detects unused, misplaced, and unlinked deps and can auto-remove them — the recommended default for unused-dep checks today. [Source](https://github.com/Boshen/cargo-shear)
- **cargo-vet** (Mozilla): enforces that *every* dependency is human-certified (shareable audit records from Google/Mozilla); stronger than advisory scanning. [Source](https://rustprojectprimer.com/checks/audit.html)
- **cargo-semver-checks** (v0.48.0): detects public-API changes that violate semver; essential before publishing/version-bumping a CLI library. [Source](https://github.com/obi1kenobi/cargo-semver-checks)
- **dylint** (cargo-dylint v6.0.1): run custom lints from dynamic libraries. [Source](https://github.com/trailofbits/dylint/)
- **cargo-geiger**: counts unsafe code in the dependency tree. [Source](https://rustprojectprimer.com/checks/audit.html)
- **sarif-rs** (`clippy-sarif`, `audit-sarif`): convert clippy/audit output to SARIF for inline GitHub PR annotations. [Source](https://rustprojectprimer.com/checks/lints.html)

---

## Best-of-Breed Config Examples

### A. Workspace root `Cargo.toml` (lint policy + dependency inheritance)
```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
edition = "2024"
rust-version = "1.85"
license = "MIT OR Apache-2.0"

[workspace.dependencies]
# Pin shared deps here; members inherit with `xxx = { workspace = true }`

# ---- Lint policy (inherited by every member) ----
[workspace.lints.rust]
unsafe_code = "deny"            # app/library policy; relax per-crate if needed
unused_qualifications = "warn"
rust_2018_idioms = "warn"

[workspace.lints.clippy]
# Whole groups at warn (not deny) with negative priority so members can override
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
# Hard gates (cherry-picked, not the whole groups)
unwrap_used = "deny"
expect_used = "warn"
dbg_macro = "deny"
todo = "deny"
print_stdout = "warn"           # deny for libraries; warn for binaries that legitimately print
print_stderr = "warn"
# Noisy pedantic lints we explicitly relax
module_name_repetitions = "allow"
doc_markdown = "allow"
must_use_candidate = "allow"
too_many_lines = "allow"
```

### B. Member `Cargo.toml`
```toml
[package]
name = "my-cli"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"

[dependencies]
serde = { workspace = true }

[lints]
workspace = true                # inherit [workspace.lints]; required or `missing_lints_inheritance` warns

# Per-crate escape hatches (optional)
[package.metadata.lints]        # (n/a) — instead override via source attributes:
# e.g. `#![allow(clippy::unwrap_used)]` on a test module where panicking is fine
```

### C. `clippy.toml`
```toml
# Set MSRV (clippy also reads `rust-version` from Cargo.toml; set one place)
msrv = "1.85.0"

cognitive-complexity-threshold = 25      # default; lower (e.g. 15) for stricter
type-complexity-threshold = 200
large-error-threshold = 128              # drives clippy::result_large_err
enum-variant-size-threshold = 200
avoid-breaking-exported-api = true       # default; keep true for libraries

# Restriction-oriented bans (use with `clippy::disallowed_*` lints set to deny)
[disallowed-methods]
# "std::panic::panic_any" = "use a typed error instead"
[disallowed-types]
# "std::sync::Mutex" = "prefer parking_lot or tokio::sync where applicable"
```

### D. `rustfmt.toml` (stable, recommended)
```toml
style_edition = "2024"
max_width = 100
hard_tabs = false
newline_style = "Unix"
use_small_heuristics = "Default"
edition = "2024"

# Optional, NIGHTLY-ONLY — leave commented unless you build with nightly:
# unstable_features = true
# imports_granularity = "Module"
# group_imports = "StdExternalCrate"
```

### E. `deny.toml` (cargo-deny v0.20.2, best-of-breed)
```toml
[graph]
all-features = true
targets = [
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
]

[advisories]
unmaintained = "warn"
unsound = "deny"
yanked = "warn"
ignore = []                 # add specific RUSTSEC-xxxx = "reason" to waive temporarily

[licenses]
confidence-threshold = 0.93
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-3.0",
    "Zlib",
    "MPL-2.0",
]
exceptions = []             # per-crate license exceptions with reason

[bans]
multiple-versions = "warn"  # "deny" once dupes are cleaned up
wildcards = "deny"
highlight = "all"
deny = [
    { crate = "openssl", use-instead = "rustls" },
    { crate = "openssl-sys", use-instead = "rustls" },
    { crate = "git2", use-instead = "gix" },
]
skip = []
skip-tree = []

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

### F. `_typos.toml`
```toml
[default.extend-words]
# word = "word"   # add project-specific tokens here

[files]
extend-exclude = ["Cargo.lock", "*.lock", "target/"]
```

---

## Recommended Deny / Warn / Allow Patterns (must-haves)

**Deny (hard gates, fail CI):**
- `warnings` via `cargo clippy ... -- -D warnings` (everything above).
- `clippy::unwrap_used` (or at least `warn`; `allow` only in tests via attribute).
- `clippy::dbg_macro`, `clippy::todo`, `clippy::panic` (for libraries).
- `rust::unsafe_code` (for pure-application crates; allow in low-level crates with documented justification).
- `cargo-deny` `unsound = "deny"`, `sources.unknown-* = "deny"`, `bans.wildcards = "deny"`.

**Warn (visible, fix over time):**
- `clippy::pedantic` and `clippy::nursery` groups at `warn` (priority -1).
- `clippy::expect_used`, `clippy::print_stdout`/`print_stderr`.
- `rust::unused_qualifications`, `rust::rust_2018_idioms`.
- `cargo-deny` `unmaintained`/`yanked` at `warn` during adoption.

**Allow (explicitly suppress noise):**
- `clippy::module_name_repetitions`, `clippy::doc_markdown`, `clippy::must_use_candidate`, `clippy::too_many_lines`, `clippy::float_cmp`, `clippy::single_component_path_imports`.
- `[lints.cargo]` items **only on nightly**; on stable rely on `cargo-shear`/`machete`/`udeps`.

### CI checklist (per PR)
```
rustup component add clippy rustfmt
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo deny check            # or: cargo audit
typos
cargo shear                # unused-dep check (or cargo machete)
cargo outdated --exit-code 1   # optional staleness gate
```

---

## Sources
- **Kept:** Clippy Configuration (doc.rust-lang.org/clippy/configuration.html) — lint-level + clippy.toml semantics.
- **Kept:** Clippy Lint Configuration (doc.rust-lang.org/clippy/lint_configuration.html) — full list of clippy.toml tunables.
- **Kept:** Cargo Lints reference (doc.rust-lang.org/stable/cargo/reference/lints.html) — `[lints]` tables, groups, `[lints.cargo]` nightly caveat.
- **Kept:** Rust Project Primer — Lints (rustprojectprimer.com/checks/lints.html) — workspace inheritance, CI, typos, SARIF.
- **Kept:** Rust Project Primer — Audit (rustprojectprimer.com/checks/audit.html) — cargo-audit/deny/vet, CI examples, cargo-geiger.
- **Kept:** cargo-deny config (embarkstudios.github.io/cargo-deny/checks/cfg.html) — deny.toml example covering all four checks.
- **Kept:** crate-ci/typos reference (github.com/crate-ci/typos/docs/reference.md) — `_typos.toml` extend-words.
- **Kept:** Rustfmt Style Edition (doc.rust-lang.org/edition-guide/rust-2024/rustfmt-style-edition.html) — `style_edition = "2024"`.
- **Kept:** crates.io pages for versions — cargo-deny 0.20.2, cargo-audit 0.22.2, typos-cli 1.48.0, cargo-shear 1.12.4, cargo-machete 0.9.2, cargo-outdated 0.19.0, cargo-udeps 0.1.61, cargo-semver-checks 0.48.0, cargo-dylint 6.0.1.
- **Dropped:** Mid-2020s blog posts reiterating basics (SEO-heavy) — superseded by official docs above.
- **Dropped:** Outdated version pins from older tutorials — replaced with current crates.io versions.

## Gaps
- Exact "best-of-breed" `Cargo.toml` is synthesized from official guidance + the Rust Project Primer; no single canonical upstream example pins every lint, so the example is a recommended composite rather than a copied single project.
- `[lints.cargo]` stable timeline is unresolved (still nightly-only mid-2026 per cargo issue #12115); re-check before relying on manifest-level unused-dependency lints.
- `cargo-semver-checks` / `cargo-vet` depth (full audit workflows) was scoped at a survey level; deeper integration recipes were out of scope.
- Tool versions reflect crates.io as of mid-2026 and may have advanced by the time of use.

## Supervisor coordination
No blocking decisions; research completed within scope. No `contact_supervisor` needed.
