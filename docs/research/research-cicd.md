# Research: Production-grade Rust CI/CD with GitHub Actions (2025–2026)

## Summary
A production Rust CI in 2026 is split into focused jobs (lint, doc, test matrix, coverage, MSRV, security) rather than one monolithic job, installed via `dtolnay/rust-toolchain`, cached via `Swatinem/rust-cache@v2` (and/or `sccache` for large workspaces), tested with `cargo-nextest` plus `cargo test --doc`, covered with `cargo-llvm-cov` + `codecov`, hardened with `rustsec/audit-check` + `cargo-deny`, and released with `release-plz` or `cargo-release`. Action *versions are pinned* (tags plus, for security-sensitive actions, full commit SHAs), concurrency groups cancel superseded runs, and `--locked` is used everywhere to respect `Cargo.lock`.

## Findings

### 1. Optimal CI workflow structure (separate jobs, matrix, concurrency)
Modern pipelines decompose CI into small, parallel jobs (lint, doc, test, coverage, msrv, security) so failures surface fast and cache/runner cost is isolated. A `strategy.matrix` fans tests across OSes/toolchains, with `fail-fast: false` so one platform failing doesn't cancel the others, and a top-level `concurrency` group (`github.head_ref || github.run_id`, `cancel-in-progress: true`) cancels superseded runs on push. [Source](https://github.com/swatinem/rust-gha-workflows/blob/e62fb0323b2e571f7fd85888f567a8f81bf997b6/.github/workflows/complete-ci.yml)

```yaml
name: CI
on: [push, pull_request]
concurrency:
  group: ${{ github.head_ref || github.run_id }}
  cancel-in-progress: true
env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -Dwarnings
  RUSTDOCFLAGS: -Dwarnings
```
[Source](https://www.shuttle.dev/blog/2025/01/23/setup-rust-ci-cd)

### 2. Toolchain install, formatting, and linting
Use `dtolnay/rust-toolchain` (pin with `toolchain: stable`/`beta`/`nightly` and `components: rustfmt, clippy`), and pair it with a repo `rust-toolchain.toml` so the version lives in one place. Lint with `cargo fmt --all -- --check` (fail on any diff) and `cargo clippy --locked --workspace --all-features --all-targets -- -D warnings`. For supply-chain safety, pin actions to full commit SHAs, not just `@stable` tags. [Source](https://www.rustprojectprimer.com/ci/github.html) [Source](https://github.com/dtolnay/rust-toolchain)

### 3. Caching strategies — `Swatinem/rust-cache@v2`, `sccache`, `cargo-chef`
- **`Swatinem/rust-cache@v2`** is the standard: it caches the cargo registry, git deps, and the `target/` dir keyed on a hash of your manifests. Use it on essentially every job. [Source](https://github.com/swatinem/rust-cache)
- **`sccache`** (via `Mozilla-Actions/sccache-action`) caches *individual compiled object files* and supports shared backends (S3, GHA cache, Redis). Best for large workspaces or when you want cross-job/cross-run object reuse; set `CARGO_INCREMENTAL=0` to avoid cache poisoning. Often used *together* with rust-cache — rust-cache for the directory-level state, sccache for fine-grained objects. [Source](https://github.com/mozilla/sccache)
- **`cargo-chef`** (`LukeMathWalker/cargo-chef`) is for Docker: `cargo chef prepare` emits a `recipe.json`; a multi-stage Dockerfile cooks dependencies in an early layer so the dependency build only re-runs when `Cargo.toml`/`Cargo.lock` change, not on every source edit. [Source](https://github.com/LukeMathWalker/cargo-chef)
- General speed tips: install prebuilt binaries via `taiki-e/install-action@v2` (`tool: nextest`, `cargo-llvm-cov`) instead of `cargo install`, and always pass `--locked`. [Source](https://www.shuttle.dev/blog/2025/01/23/setup-rust-ci-cd)

```yaml
- uses: Swatinem/rust-cache@v2
- uses: Mozilla-Actions/sccache-action@v0.2.0   # optional, large workspaces
  env: { SCCACHE_GHA_ENABLED: "true", RUSTC_WRAPPER: "sccache" }
```

### 4. Cross-platform testing (Linux/macOS/Windows) + `cargo-nextest`
Run the test matrix `os: [ubuntu-latest, macos-latest, windows-latest]` (or pin versions like `ubuntu-24.04`, `macos-13`, `windows-2022` to avoid surprise image migrations). Use `cargo-nextest` (install via `taiki-e/install-action@nextest`) for fast, parallel unit/integration tests, but **nextest does not run doctests**, so add a separate `cargo test --doc` step. Upload JUnit XML via `actions/upload-artifact@v4` with `if: always()` and feed it to `codecov/test-results-action@v1`. For extra Linux *targets* (musl, arm, etc.) use `cross`/`cargo-zigbuild` rather than native macOS/Windows emulation. [Source](https://nexte.st/docs/integrations/test-coverage/) [Source](https://github.com/fujiapple852/trippy/blob/823cd21e6a5865091735287b4e78936c8f27c1cd/.github/workflows/ci.yml)

```yaml
- uses: taiki-e/install-action@nextest
- run: cargo nextest run --workspace --all-features
- run: cargo test --workspace --all-features --doc
```

### 5. MSRV testing
Declare `rust-version` in `Cargo.toml` and verify it in CI with `cargo-msrv verify` (or `cargo +<msrv> check`). For stricter "true" MSRV checking against the *minimum dependency versions*, use `cargo-minimal-versions`. Keep the MSRV job on `ubuntu-latest` and pin the exact toolchain version. [Source](https://github.com/foresterre/cargo-msrv) [Source](https://doc.rust-lang.org/stable/cargo/reference/rust-version.html)

```yaml
- run: rustup toolchain install 1.74.0 --profile minimal --no-self-update
- run: cargo +1.74.0 check --locked --all-features
# or: cargo install cargo-msrv && cargo msrv verify -- cargo check --all-features
```

### 6. Code coverage — `cargo-llvm-cov` (preferred) vs `cargo-tarpaulin`
`cargo-llvm-cov` (install via `taiki-e/install-action@cargo-llvm-cov`) uses LLVM source-based instrumentation: cross-platform (Linux/macOS/Windows), accurate, and integrates with nextest via `cargo llvm-cov nextest`. Emit LCOV and upload with `codecov/codecov-action@v4` (token via `secrets.CODECOV_TOKEN`). `cargo-tarpaulin` is the older alternative (historically Linux/ptrace, now also has an LLVM engine) but is less accurate and Linux-centric. [Source](https://www.rustprojectprimer.com/measure/coverage.html) [Source](https://github.com/taiki-e/cargo-llvm-cov)

```yaml
- uses: taiki-e/install-action@cargo-llvm-cov
- run: cargo llvm-cov nextest --workspace --all-features --lcov --output-path lcov.info
- uses: codecov/codecov-action@v4
  with: { token: ${{ secrets.CODECOV_TOKEN }}, files: lcov.info, disable_search: true }
```

### 7. Security scanning in CI
- **`rustsec/audit-check@v2.0.0`** runs `cargo-audit` against the RustSec advisory DB (RUSTSEC, CVEs). Needs `secrets.GITHUB_TOKEN`.
- **`EmbarkStudios/cargo-deny-action@v2`** enforces a `deny.toml` policy: advisories, licenses, banned crates, and source allow-lists. Use `command: check bans licenses sources` and `continue-on-error: true` during rollout.
- Optionally add `obi1kenobi/cargo-semver-checks-action@v2` in the lint job to catch unintended semver breaks, and Dependabot (`dependabot.yml`) for dependency updates. Pin all of these to full SHAs. [Source](https://github.com/rustsec/audit-check) [Source](https://github.com/EmbarkStudios/cargo-deny-action/blob/main/README.md)

```yaml
security_audit:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: rustsec/audit-check@v2.0.0
      with: { token: ${{ secrets.GITHUB_TOKEN }} }
```

### 8. Release automation — `release-plz`, `cargo-release`, `semantic-release`
- **`release-plz`** (Rust-native, recommended): a `release-pr` job opens/updates a PR with version bumps + CHANGELOG from Conventional Commits and `cargo-semver-checks`; a `release` job publishes to crates.io and tags on merge. Run via `release-plz/release-plz-action@v0.5` with `command: release-pr` / `release`. Needs `contents: write`, `pull-requests: write`, `CARGO_REGISTRY_TOKEN`. [Source](https://release-plz.dev/docs/github/quickstart)
- **`cargo-release`** (CLI, more manual): pairs with `cargo-bins/release-pr` to produce a release PR, then you trigger the publish; good when you want explicit human control.
- **`semantic-release`** (JS-ecosystem tool) via `semantic-release-cargo` / `semantic-release-action/rust` for teams already standardized on semantic-release. [Source](https://github.com/release-plz/release-plz) [Source](https://github.com/semantic-release-cargo/semantic-release-cargo)

```yaml
release-plz-pr:
  runs-on: ubuntu-latest
  permissions: { contents: write, pull-requests: write }
  concurrency: { group: release-plz-${{ github.ref }}, cancel-in-progress: false }
  steps:
    - uses: actions/checkout@v4
      with: { fetch-depth: 0, persist-credentials: false }
    - uses: dtolnay/rust-toolchain@stable
    - uses: release-plz/release-plz-action@v0.5
      with: { command: release-pr }
      env: { GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }} }
```

### 9. Pre-commit hooks
Use the pre-commit framework with community Rust hooks (e.g. `AndrejOrsula/pre-commit-cargo` rev `0.4.0`, hooks `cargo-fmt`, `cargo-clippy`, `cargo-test`, `cargo-check`). **Caveat:** pre-commit.ci's hosted runners lack a full Rust toolchain, so Rust hooks typically fail there — add `ci: skip: [cargo-fmt, cargo-clippy]` or run hooks in a self-hosted/container environment. A Rust-native alternative is `j178/prek`. These hooks are a local/PR-gate convenience and do *not* replace CI. [Source](https://github.com/AndrejOrsula/pre-commit-cargo/) [Source](https://heuristicpedals.com/blog/posts/rust-pre-commits/)

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/AndrejOrsula/pre-commit-cargo
    rev: 0.4.0
    hooks:
      - id: cargo-fmt
      - id: cargo-clippy
        args: ["--all-targets", "--all-features", "--", "-D", "warnings"]
ci:
  skip: [cargo-fmt, cargo-clippy]
```

## Production-grade reference wiring (consolidated)
Fuse `test` + `coverage` into one job to avoid double compilation (as in Swatinem's `complete-ci.yml`): install `llvm-tools` + `cargo-llvm-cov` + `nextest` + `rust-cache`, run `cargo llvm-cov nextest ... --lcov --output-path lcov.info`, then upload JUnit (`codecov/test-results-action@v1`) and coverage (`codecov/codecov-action@v4`/`@v5`). A separate `lint` job runs `fmt`/`clippy`/`cargo-semver-checks`; a `doc` job runs `cargo test --doc` + `cargo doc`; an `msrv` job pins the minimum toolchain; a `security` job runs `audit-check` + `cargo-deny`; a `release.yml` uses `release-plz`. Pin all actions (tags; SHAs for security-critical), set `CARGO_TERM_COLOR`/`RUSTFLAGS=-Dwarnings` globally, and use `--locked` everywhere. Prefer explicit runner versions (`ubuntu-24.04`, `macos-14`, `windows-2022`) over `-latest` to avoid silent image upgrades (GitHub is migrating `-latest` runners toward Node.js 24, dropping Node 20 ~Sept 2026). [Source](https://github.com/actions/runner-images/blob/main/README.md)

## Sources
- Kept: Swatinem/rust-gha-workflows `complete-ci.yml` (https://github.com/swatinem/rust-gha-workflows/blob/e62fb0323b2e571f7fd85888f567a8f81bf997b6/.github/workflows/complete-ci.yml) — authoritative full reference workflow with real action versions (rust-cache@v2, codecov@v5, taiki-e/install-action, cargo-semver-checks@v2, CodSpeedHQ@v3).
- Kept: Shuttle — Effective Rust CI/CD primer (https://www.shuttle.dev/blog/2025/01/23/setup-rust-ci-cd) — fundamentals, sccache, release-plz, Dependabot rationale.
- Kept: Release-plz Quickstart (https://release-plz.dev/docs/github/quickstart) — official release-plz GitHub Action wiring, permissions, concurrency.
- Kept: Rust Project Primer — CI (https://www.rustprojectprimer.com/ci/github.html) and Coverage (https://www.rustprojectprimer.com/measure/coverage.html) — toolchain pinning, llvm-cov vs tarpaulin guidance.
- Kept: cargo-llvm-cov (https://github.com/taiki-e/cargo-llvm-cov), Swatinem/rust-cache (https://github.com/swatinem/rust-cache), cargo-chef (https://github.com/LukeMathWalker/cargo-chef), sccache (https://github.com/mozilla/sccache), cargo-msrv (https://github.com/foresterre/cargo-msrv), audit-check (https://github.com/rustsec/audit-check), cargo-deny-action (https://github.com/EmbarkStudios/cargo-deny-action), dtolnay/rust-toolchain (https://github.com/dtolnay/rust-toolchain), AndrejOrsula/pre-commit-cargo (https://github.com/AndrejOrsula/pre-commit-cargo/), trippy ci.yml (https://github.com/fujiapple852/trippy/blob/823cd21e6a5865091735287b4e78936c8f27c1cd/.github/workflows/ci.yml), nextest coverage (https://nexte.st/docs/integrations/test-coverage/).
- Dropped: SEO/duplicate StackOverflow-style and generic "Rust CI tutorial" reposts — redundant with primary sources above.
- Dropped: rustdesk/crossbeam ci.yml — used only as secondary matrix examples; covered sufficiently by trippy.

## Gaps
- Exact `codecov/codecov-action` major: the reference workflow uses `@v5` while the coverage guidance and many 2025 blogs use `@v4`; both exist — pin to whichever your org's token setup requires (v4 vs v5 differ in required `token` and uploader).
- sccache-action exact version tag varies by repo; `Mozilla-Actions/sccache-action` (commonly `@v0.2.0`) should be verified against the action's current release.
- `cargo-chef` Dockerfile is shown conceptually; a full multi-stage Dockerfile snippet was not fetched — recommend pulling the canonical example from the cargo-chef README before implementation.
- Pre-2026 `actions/checkout` major (v4 vs v6 per docs drift) should be confirmed against your GitHub Enterprise / runner version.

## Supervisor coordination
No escalation needed — research completed within scope; all findings sourced from primary/authoritative references.

---

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Delivered a scoped research brief covering only the requested Rust CI/CD topics (workflow structure, caching, cross-platform/MSRV, coverage, security, release automation, pre-commit) with no code changes or scope widening."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "Brief cites primary/authoritative sources inline (Swatinem complete-ci.yml, Shuttle primer, release-plz quickstart, Rust Project Primer, cargo-llvm-cov, rust-cache, cargo-chef, sccache, cargo-msrv, audit-check, cargo-deny, dtolnay/rust-toolchain, pre-commit-cargo, trippy ci.yml, nextest) and includes copy-ready YAML patterns with specific action versions."
    }
  ],
  "changedFiles": [
    "/home/kwhatcher/projects/gor/.pi-subagents/artifacts/outputs/57baf442/research-cicd.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [],
  "validationOutput": [
    "Research synthesized from 6 web_search passes (12+ queries) and 3 fetched primary sources.",
    "Output written to the authoritative path: /home/kwhatcher/projects/gor/.pi-subagents/artifacts/outputs/57baf442/research-cicd.md (13.7 KB).",
    "All 9 requested topic areas covered with inline citations and YAML snippets."
  ],
  "residualRisks": [
    "codecov/codecov-action major (v4 vs v5) needs org-specific confirmation.",
    "sccache-action exact tag (@v0.2.0 approximate) should be verified against current release.",
    "cargo-chef full multi-stage Dockerfile not fetched — pull canonical example before implementing Docker builds.",
    "actions/checkout major drift (v4 vs v6) should be matched to the target GitHub/runner version."
  ],
  "noStagedFiles": true,
  "diffSummary": "Added a single research artifact (research-cicd.md) summarizing 2025-2026 Rust GitHub Actions CI/CD best practices with sources and YAML patterns. No source code modified.",
  "reviewFindings": [
    "no blockers: research-only deliverable; all claims sourced; gaps explicitly documented in the Gaps section."
  ],
  "manualNotes": "This is a research/research-writing task, not an implementation task — no repositories, tests, or build commands were executed. The deliverable is the brief at the output path. Several version pins (codecov, sccache-action, checkout) are noted as needing org-specific verification before being copied into a live workflow."
}
```
