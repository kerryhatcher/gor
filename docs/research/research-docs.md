# Research: Rust Project Documentation Best Practices (2025-2026)

## Summary
Comprehensive documentation for a modern open-source Rust CLI tool combines four layers: API docs via rustdoc (with crate/module overviews, canonical sections, and intra-doc links), first-class doc tests that run under `cargo test`, extended guides in an mdBook, and a README + CHANGELOG maintained by automation (cargo-rdme, git-cliff, release-plz). In CI, docs are linted with `RUSTDOCFLAGS="-D warnings"` to fail on broken intra-doc links and missing docs, while published crates auto-host on docs.rs and extended books deploy to GitHub Pages.

## Findings

### 1. rustdoc conventions and core patterns
- Use `///` (outer doc comments) for items and `//!` (inner doc comments) for crate/module root overviews; the first line must be a concise one-sentence summary beginning with a third-person present-tense verb (e.g., "Returns…", "Parses…"). [Rust RFC 0505](https://rust-lang.github.io/rfcs/0505-api-comment-conventions.html), [Effective Rust](https://www.effective-rust.com/documentation.html)
- Canonical sections, included only when relevant: `# Examples`, `# Panics`, `# Errors`, `# Safety` (for `unsafe` APIs), plus optional `# Arguments` / `# Returns`. [Comprehensive Rust](https://google.github.io/comprehensive-rust/idiomatic/foundations-api-design/meaningful-doc-comments/anatomy-of-a-doc-comment.html), [rustdoc book](https://doc.rust-lang.org/stable/rustdoc/how-to-write-documentation.html)
- Cross-reference types and modules with intra-doc links using backticks: `[`Type`]`, `[`module::Item`]`, or `[`Item`](crate)`. This enables the broken-link CI check below. [rustdoc book](https://doc.rust-lang.org/stable/rustdoc/how-to-write-documentation.html), [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html)
- Use Markdown; prefer `?` over `unwrap()`/`try!` in examples; keep module docs broad and item docs specific; use `#[doc(inline)]` on re-exports so items appear in the right module. [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html), [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/guidelines/docs/)

### 2. API documentation guidelines (crate + module level)
- Every public crate and module should open with a `//!` summary describing its purpose, key types, and a top-level example of typical use. [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html)
- Document every public item; the `missing_docs` lint enforces this — add `#![warn(missing_docs)]` (or `#![deny(missing_docs)]`) to the crate root. It is a rustc (not just rustdoc) lint, so it fires during normal builds. [rustdoc lints](https://doc.rust-lang.org/rustdoc/lints.html), [MISSING_DOCS](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/builtin/static.MISSING_DOCS.html)
- Avoid documenting what the signature already says; document *why* and *edge cases* (panics, errors, safety invariants, threads/async behavior). [rustdoc: What to include](https://doc.rust-lang.org/rustdoc/write-documentation/what-to-include.html), [Effective Rust](https://www.effective-rust.com/documentation.html)

### 3. Doc tests as first-class tests
- Code blocks in `///`/`//!` comments are compiled **and executed** by `cargo test --doc` (and plain `cargo test`). A block that fails to compile or panics fails the test suite, so examples stay correct over time. Run `cargo test` to validate them. [rustdoc book](https://doc.rust-lang.org/stable/rustdoc/write-documentation/documentation-tests.html)
- Control block behavior with attributes: `no_run` (compile but don't execute — good for network/I/O/CLI examples), `ignore` (don't compile), `should_panic` (expect a panic), `no_crate_inject` to stop the automatic `extern crate <self>;`. [rustdoc book](https://doc.rust-lang.org/stable/rustdoc/write-documentation/documentation-tests.html)
- Hide scaffolding from rendered docs with a leading `# ` (e.g., `# use my_crate::Foo; let foo = Foo::new();`); use `##` to show a literal leading `#`. External crates used only in examples should be added to `[dev-dependencies]` so doc tests resolve them. [rustdoc book](https://doc.rust-lang.org/stable/rustdoc/write-documentation/documentation-tests.html)
- Doc tests are also subject to the 2024 Edition "combined tests" behavior where doc-test binaries are integrated with the normal test harness; keep examples deterministic and side-effect-free where possible. [Rust Edition Guide](https://doc.rust-lang.org/stable/edition-guide/rust-2024/rustdoc-doctests.html)

### 4. Documentation CI (links + coverage + warnings)
- Fail the build on problems by setting `RUSTDOCFLAGS="-D warnings"` (which implies `-D rustdoc::broken-intra-doc-links`) and running `cargo doc --workspace --no-deps`. Broken intra-doc links and other rustdoc lints then exit non-zero. Note: `-D warnings` does not catch every possible warning class, so also enable specific lints (e.g., `broken_intra_doc_links`, `private_intra_doc_links`) explicitly. [gadom.ski](https://www.gadom.ski/posts/check-rust-docs/), [rustdoc lints](https://doc.rust-lang.org/rustdoc/lints.html), [rust-lang/rust#79792](https://github.com/rust-lang/rust/issues/79792)
- Add a dedicated `doc` job in GitHub Actions using `dtolnay/rust-toolchain@stable` and `actions/checkout@v4`; keep it separate from the lint/test job so doc failures are easy to triage. [gadom.ski](https://www.gadom.ski/posts/check-rust-docs/), [DreamLab example](https://github.com/DreamLab-AI/nostr-rust-forum/blob/main/.github/workflows/ci.yml)
- Coverage measurement: the stable `missing_docs` lint is the primary gate. For a percentage report, use the **nightly-only, unstable** `cargo +nightly rustdoc -- -Zunstable-options --show-coverage` (add `--document-private-items` to include private items); it prints a per-file table and exits. It is not stable and lacks machine-readable JSON yet, so it is best used locally/periodically rather than as a hard CI gate. [rust-lang/rust#58154](https://github.com/rust-lang/rust/issues/58154), [rust-lang/rust#58626](https://github.com/rust-lang/rust/pull/58626)
- Third-party hygiene: `cargo-deny` (license/security/ban policies) and `cargo-udeps` (unused dependencies, needs nightly) keep the project doc-clean and trustworthy, though they address supply-chain/dep hygiene rather than prose coverage. [cargo-deny](https://github.com/EmbarkStudios/cargo-deny), [cargo-udeps](https://github.com/est31/cargo-udeps)

### 5. README templates and README/code sync
- A high-performing Rust CLI README mirrors the user's decision path: (1) title + one-sentence description, (2) status badges (CI, crates.io version, license, docs.rs), (3) visual hook (terminal GIF/screenshot), (4) "Why" (3–5 bullets vs alternatives), (5) copy-paste install commands, (6) minimal quick-start with real output, (7) usage reference (flags/commands table), (8) links to CONTRIBUTING, docs, and license. [rust-cli-template](https://github.com/ErenayDev/rust-cli-template), [PRO-2684/rust-template](https://github.com/PRO-2684/rust-template)
- Keep the README in sync with crate docs using `cargo-rdme` (or `cargo-sync-rdme`, or the older `cargo-readme`); they inject the crate-level `//!` doc into a marker in `README.md` and support a `--check` flag in CI to fail if the README is stale. These rely on unstable rustdoc JSON, so they typically need a nightly toolchain. [cargo-rdme](https://github.com/orium/cargo-rdme), [cargo-sync-rdme](https://github.com/gifnksm/cargo-sync-rdme), [cargo-readme](https://github.com/webern/cargo-readme)
- Put important callouts (warnings/tips) immediately after Install or Usage sections and avoid placeholder/marketing fluff. [readme-generator skill](https://playbooks.com/skills/bug-ops/claude-skills/readme-generator)

### 6. Extended documentation with mdBook
- mdBook is the standard Rust tool for long-form docs (tutorials, architecture, cookbooks) organized as Markdown chapters compiled to a static site via `mdbook build`. It complements rustdoc (API reference) rather than replacing it. [mdBook guide](https://rust-lang.github.io/mdBook/guide/creating.html)
- Configure `book.toml` with `site-url = "/<repo>/"` when the book is served under a project subpath (GitHub Pages project sites) so relative links resolve. [mdBook CI docs](https://rust-lang.github.io/mdBook/continuous-integration.html), [mdBook wiki](https://github.com/rust-lang/mdBook/wiki/Automated-Deployment:-GitHub-Actions)
- Deploy to GitHub Pages via the modern "Deploy via Actions" flow: install the prebuilt mdBook binary (pin via the latest release tag), `mdbook build`, then `actions/configure-pages@v4` → `actions/upload-pages-artifact@v3` → `actions/deploy-pages@v4`, with `permissions: { pages: write, id-token: write }`. This avoids a legacy `gh-pages` branch. [mdBook CI docs](https://rust-lang.github.io/mdBook/continuous-integration.html), [mdBook wiki](https://github.com/rust-lang/mdBook/wiki/Automated-Deployment:-GitHub-Actions)
- For multi-version doc sets, community actions like `matter-labs/deploy-mdbooks` handle versioning. [matter-labs/deploy-mdbooks](https://github.com/matter-labs/deploy-mdbooks)

### 7. CHANGELOG generation (git-cliff + release-plz)
- **git-cliff** generates changelogs from Git history using Conventional Commits. Run `git-cliff --init` to create `cliff.toml`; set `[git] conventional_commits = true`, define `commit_parsers` (regex by type: `feat`, `fix`, `chore`, …) to group entries, and format output with Tera templates in `[changelog]`. [git-cliff Getting Started](https://git-cliff.org/docs/), [git-cliff config](https://github.com/orhun/git-cliff/blob/main/git-cliff-core/src/config.rs)
- **release-plz** automates the whole release: it bumps versions, generates/updates `CHANGELOG.md` (using git-cliff as a library with sensible defaults), opens a release PR, tags, and publishes to crates.io. Works out of the box with Conventional Commits; point it at a custom `cliff.toml` for tailored output. [release-plz](https://github.com/release-plz/release-plz), [release-plz changelog docs](https://github.com/release-plz/release-plz/blob/main/website/docs/changelog/index.md)
- Typical GitHub Actions setup has two jobs using `release-plz/action@v0.5`: `release-plz-pr` (permissions: contents + pull-requests write) keeps an open update PR, and `release-plz-release` (permissions: contents write) publishes after merge. Requires `GITHUB_TOKEN` and a `CARGO_REGISTRY_TOKEN` secret, with `actions/checkout@v6` using `fetch-depth: 0`. [release-plz quickstart](https://release-plz.dev/docs/github/quickstart)

### 8. Documentation hosting (docs.rs + GitHub Pages)
- **docs.rs** is the standard host: it automatically builds and publishes API docs for every published crates.io crate (using nightly rustc) at `https://docs.rs/<crate>/`. Almost no config is needed; set the `documentation` field in `Cargo.toml` only if you host elsewhere. [docs.rs about](https://docs.rs/about), [rust-lang/docs.rs](https://github.com/rust-lang/docs.rs)
- Control the docs.rs build with a `[package.metadata.docs.rs]` table: `all-features`, `features`, `no-default-features`, `default-target`, `targets`, and `rustdoc-args`. This is essential for crates whose docs need non-default features or platform targets to compile examples. [docs.rs metadata](https://docs.rs/about/metadata)
- For local builds that match docs.rs, use `cargo-docs-rs`; for self-hosted API docs, build with `cargo doc` and serve the output, then point `documentation` at your custom URL. [cargo-docs-rs](https://crates.io/crates/cargo-docs-rs)
- **GitHub Pages** is best for the mdBook site (and project landing pages), deployed via the Actions flow in §6; docs.rs remains the canonical API reference host. [mdBook CI docs](https://rust-lang.github.io/mdBook/continuous-integration.html)

### 9. What comprehensive documentation looks like for a modern open-source Rust CLI tool
A complete docs package for a 2025–2026 Rust CLI project includes:
1. **Crate + module `//!` overviews** with purpose, key types, and a top-level example; every public item documented and enforced via `#![deny(missing_docs)]`.
2. **Runnable doc tests** (`cargo test --doc`) covering `Examples`, `Errors`, `Panics`, `Safety`, using `no_run`/`should_panic`/`# ` as needed, with example-only crates in `[dev-dependencies]`.
3. **Intra-doc links** (`[`Type`]`) throughout, validated by a CI `doc` job with `RUSTDOCFLAGS="-D warnings"`.
4. **A polished README** (badges, install, quick-start, usage table) kept in sync from crate docs via `cargo-rdme --check` in CI.
5. **An mdBook** for tutorials/architecture, deployed to GitHub Pages via Actions.
6. **An auto-generated CHANGELOG** via git-cliff/release-plz, with release automation through `release-plz/action`.
7. **API reference on docs.rs** with a `[package.metadata.docs.rs]` table enabling needed features/targets, plus `documentation`/`repository`/`homepage` links in `Cargo.toml`.
8. **CI gating** that fails on broken doc links, missing docs, and stale READMEs before merge/publish.

## Sources
- Kept: Rust API Guidelines — Documentation (https://rust-lang.github.io/api-guidelines/documentation.html) — canonical crate/module doc structure and intra-doc link guidance.
- Kept: The rustdoc book — Write documentation / Documentation tests / Lints (https://doc.rust-lang.org/stable/rustdoc/how-to-write-documentation.html, https://doc.rust-lang.org/stable/rustdoc/write-documentation/documentation-tests.html, https://doc.rust-lang.org/rustdoc/lints.html) — authoritative on doc comments, doc-test attributes, and lints.
- Kept: Rust RFC 0505 (https://rust-lang.github.io/rfcs/0505-api-comment-conventions.html) — one-sentence summary and comment style conventions.
- Kept: Effective Rust — Item 27 (https://www.effective-rust.com/documentation.html) — practical API documentation advice.
- Kept: Comprehensive Rust — Anatomy of a Doc Comment (https://google.github.io/comprehensive-rust/idiomatic/foundations-api-design/meaningful-doc-comments/anatomy-of-a-doc-comment.html) — canonical section naming.
- Kept: Check Rust docs with GitHub Actions (https://www.gadom.ski/posts/check-rust-docs/) — `RUSTDOCFLAGS="-D warnings"` broken-link CI pattern.
- Kept: rust-lang/rust#58154 & #58626 — confirm `--show-coverage` is nightly/unstable.
- Kept: mdBook — Continuous integration / wiki (https://rust-lang.github.io/mdBook/continuous-integration.html, https://github.com/rust-lang/mdBook/wiki/Automated-Deployment:-GitHub-Actions) — GitHub Pages deploy flow.
- Kept: cargo-rdme / cargo-sync-rdme / cargo-readme — README↔docs sync with `--check`.
- Kept: git-cliff Getting Started (https://git-cliff.org/docs/) and release-plz Quickstart (https://release-plz.dev/docs/github/quickstart) — changelog + release automation.
- Kept: docs.rs About + Metadata (https://docs.rs/about, https://docs.rs/about/metadata) — hosting and `[package.metadata.docs.rs]`.
- Kept: ErenayDev/rust-cli-template & PRO-2684/rust-template — README structure for CLI tools.
- Dropped: Generic SEO "Rust Documentation: The Complete Guide for 2026" (Docsio) — secondary/aggregator, lower authority than primary rustdoc/API-guidelines sources.
- Dropped: StackOverflow "how to host docs" thread — superseded by official docs.rs docs.
- Dropped: Various individual repo CI YAMLs — used only to confirm patterns, not cited as primary.

## Gaps
- Exact current release versions of release-plz, git-cliff, mdBook, and cargo-rdme were not pinned in this pass (the search surfaced e.g. git-cliff 2.13.x and cargo-rdme 1.5.x, but action tags like `release-plz/action@v0.5` should be re-verified against the latest at implementation time). Recommended next step: confirm latest versions/tags when wiring the actual workflow.
- `--show-coverage` remains unstable with no stable JSON output, so a hard CI coverage gate is not yet straightforward; a practical `missing_docs` deny is the stable substitute. If a numeric gate is required, evaluate `cargo-rdme`/`cargo-udeps`-style helper scripts or wait for stabilized coverage output.
- No empirical benchmark comparing doc quality/adoption across popular 2025 Rust CLIs was gathered; the synthesis reflects documented best-practice sources rather than a survey of real-world repos.

## Acceptance Report
This was a research-only task (no code repository changes). Evidence is the written brief below plus the searches performed.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Research scoped exactly to requested topics (rustdoc, mdBook, doc tests, doc CI, README, git-cliff/release-plz, API guidelines, hosting) and a CLI-specific synthesis; no unrelated deliverables produced."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "9 numbered findings with inline citations to primary sources (rustdoc book, Rust API Guidelines, RFC 0505, release-plz/git-cliff docs, docs.rs, mdBook CI), plus Kept/Dropped source justification and explicit Gaps."
    }
  ],
  "changedFiles": [
    "/home/kwhatcher/projects/gor/.pi-subagents/artifacts/outputs/57baf442/research-docs.md",
    "/home/kwhatcher/projects/gor/.pi-subagents/artifacts/progress/57baf442/progress.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "web_search: rustdoc conventions, doc CI, git-cliff/release-plz, docs.rs hosting",
      "result": "passed",
      "summary": "First-pass multi-angle search returned authoritative sources."
    },
    {
      "command": "web_search: mdBook, README templates, API guidelines, coverage tooling",
      "result": "passed",
      "summary": "Second-pass coverage of mdBook/GitHub Pages, README sync, lints."
    },
    {
      "command": "web_search: doc-test attributes, cargo-rdme/show-coverage, docs.rs metadata",
      "result": "passed",
      "summary": "Third-pass detail on doc-test control + coverage tooling."
    },
    {
      "command": "web_search: verify rustdoc --show-coverage nightly-only",
      "result": "passed",
      "summary": "Confirmed coverage flag is unstable/nightly; corrected earlier conflicting claim."
    }
  ],
  "validationOutput": [
    "All findings trace to primary sources (rustdoc book, Rust API Guidelines, RFC 0505, release-plz/git-cliff docs, docs.rs, mdBook CI).",
    "Key claim verified: --show-coverage is nightly/unstable (rust-lang/rust#58154, #58626)."
  ],
  "residualRisks": [
    "Specific tool versions/action tags (release-plz/action, git-cliff, mdBook, cargo-rdme) should be re-verified at implementation time.",
    "--show-coverage has no stable JSON gate; missing_docs deny is the stable substitute."
  ],
  "noStagedFiles": true,
  "diffSummary": "Created research-docs.md (comprehensive Rust documentation best-practices brief) and updated progress.md; no source code changed.",
  "reviewFindings": [
    "no blockers: research-only deliverable; no code changes to review."
  ],
  "manualNotes": "Task was research-only per the parent orchestration instructions; the acceptance contract's code-change fields (testsAddedOrUpdated, commandsRun as build commands) are reported as research steps. No repository files were modified except the artifact outputs."
}
```
