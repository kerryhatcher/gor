# Research: Rust Crate Deployment & Distribution (2025–2026)

## Summary
The 2026 best practice is to **automate everything in CI and ship pre-built binaries, not source**. For library crates, publish to crates.io with **Trusted Publishing (OIDC)** — no long-lived API tokens — and gate releases with **cargo-semver-checks**. For a CLI tool, the recommended distribution stack is **cargo-dist** (multi-platform builds + installers + GitHub Releases) plus **cargo-binstall** (binary-first `cargo install` replacement) and a **Homebrew tap / one-line installer script**, with **Docker** (multi-stage + cargo-chef + distroless/scratch musl binary) as the container path. The best end-user install experience is a layered one: installer script / Homebrew / cargo-binstall, all sourcing artifacts from GitHub Releases.

## Findings

### 1. crates.io publishing workflow — Trusted Publishing is the standard
Trusted Publishing (RFC 3691) lets CI authenticate to crates.io via OpenID Connect, exchanging an OIDC token for a short-lived (~30 min) publish token. There are **no long-lived API tokens to store or rotate**, and repository/workflow verification prevents unauthorized publishes. Initial crate publish still requires a manual API token, but all subsequent CI-driven publishes use OIDC. GitHub is GA; GitLab is public beta. The official `rust-lang/crates-io-auth-action@v1` performs the token exchange. [Source: crates.io Trusted Publishing docs](https://crates.io/docs/trusted-publishing)

Best-practice CI shape: trigger on `v*` tags, set `permissions: { id-token: write }`, use an optional GitHub **environment** named `release` with protection rules (required reviewers), run `cargo publish --dry-run` first to validate metadata/compilation, then `cargo publish`. [Source: crates.io Trusted Publishing docs](https://crates.io/docs/trusted-publishing)

### 2. cargo-release vs release-plz — different philosophies
- **cargo-release** is a **CLI-first** tool for manual/local releases: it bumps versions, runs hooks, and publishes. It supports `--unpublished` for automation but has **no first-class changelog management** (long-standing gap) and is not designed around CI release PRs. It does support release hooks that release-plz lacks.
- **release-plz** is **CI-first**: it generates/updates **release pull requests**, detects API breaking changes via **cargo-semver-checks**, generates changelogs with **git-cliff**, and verifies publish status against the **cargo registry** (not just git tags). It requires **no config files** (reads `Cargo.toml`), does **not require conventional commits**, and supports `pr_per_package` for workspaces.

Recommendation: use **release-plz** for automated, CI-driven crate publishing (especially workspaces); **cargo-release** remains viable for manual/local control with custom hooks. [Source: release-plz "Why yet another release tool"](https://release-plz.dev/docs/why)

### 3. Binary distribution — cargo-dist (maintainer) vs cargo-binstall (user)
- **cargo-dist** (v0.32.0, 2026-05) is the maintainer-side automation tool: it generates CI, **cross-compiles for many targets**, produces **shell + PowerShell + MSI + Homebrew installers**, and publishes to **GitHub Releases**. It removed the cargo-auditable + cargo-zigbuild cross-compile limitation and refreshed default GitHub Action versions. [Source: axodotdev/cargo-dist](https://github.com/axodotdev/cargo-dist) / [v0.32.0 release](https://github.com/axodotdev/cargo-dist/releases/tag/v0.32.0)
- **cargo-binstall** (v1.20.0) is the end-user/CI tool: a **binary-first replacement for `cargo install`**. `cargo binstall <pkg>` fetches pre-built binaries from the repo's releases and **falls back to compiling from source** only if no prebuilt exists. [Source: cargo-bins/cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

Layered end-user install paths: (1) Homebrew tap `brew install …`; (2) installer script `curl …/install.sh | sh` (cargo-dist-generated); (3) `cargo binstall …`. [Source: rs4ts.dev Distributing CLI Tools](https://rs4ts.dev/18-cli-tools/10-distribution/)

### 4. Container images for CLI tools — multi-stage + cargo-chef + distroless
Standard 2026 pattern: **multi-stage Dockerfile** isolating the heavy Rust toolchain from a minimal runtime. Use **cargo-chef** to cache dependency layers (recipe → cook → build), then copy only the compiled binary into **distroless** (`gcr.io/distroless/cc-debian12`) or **scratch**. For `scratch` you must build a **statically linked musl binary** (`x86_64-unknown-linux-musl`). Shrink the binary with `opt-level="z"`, `strip=true`, `lto=true` in the release profile, and run as a non-root user. [Source: Rust Project Primer — Containers](https://www.rustprojectprimer.com/releasing/containers.html) / [Docker Rust guide](https://docs.docker.com/guides/rust/build-images/)

### 5. Versioning & semver compliance — cargo-semver-checks as a gate
**cargo-semver-checks** (v0.48.0) analyzes the public API (via rustdoc JSON) against a baseline (default: latest on crates.io; pin with `--baseline-version`/`--baseline-rev`) to detect breaking changes. Run locally (`cargo semver-checks`) and as a CI gate via `obi1kenobi/cargo-semver-checks-action@v2`. Note: it depends on **unstable rustdoc JSON**, so keep the tool in sync with the Rust toolchain. It is a Rust Project Goal for 2025H2 to move it toward being merged into cargo. [Source: cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) / [Project Goal](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-semver-checks.html)

release-plz integrates cargo-semver-checks so a detected breaking change **auto-bumps the major version** in the release PR — making semver compliance automatic rather than a manual review step. [Source: release-plz docs](https://release-plz.dev/docs/why)

### 6. Cross-compilation for distribution
Most popular CLI tools still use **custom GitHub Actions matrix builds**, but the common cross tooling is: **cargo-zigbuild** (Zig-based, easy musl/glibc cross without target sysroots) and **cross** via `taiki-e/setup-cross-toolchain-action` (or `cross` Docker images). `taiki-e/upload-rust-binary-action` and `houseabsolute/actions-rust-release` are reusable Actions that build + package + upload to GitHub Releases with SHA256 checksums. cargo-dist itself orchestrates cross-compilation across a target matrix. For static Linux binaries, target `x86_64-unknown-linux-musl` (+ cargo-zigbuild) for maximum portability. [Source: rust-porting-playbook research](https://github.com/jlevy/rust-porting-playbook/blob/main/docs/project/research/research-rust-cli-binary-distribution.md) / [taiki-e/setup-cross-toolchain-action](https://github.com/taiki-e/setup-cross-toolchain-action)

### 7. Answer: best way to distribute a Rust CLI to end users in 2026
1. Build with **cargo-dist** → produces cross-compiled binaries + installers + GitHub Releases artifacts.
2. Offer end users **three install paths**: cargo-dist **installer script** (`curl … | sh`), **Homebrew tap**, and **`cargo binstall`** (binary-first, instant).
3. For container-first users, publish a **multi-stage, cargo-chef-baked, distroless/scratch image** with a musl static binary.
4. Gate releases with **release-plz** (auto version bump + changelog + publish PR) and **cargo-semver-checks** (semver safety), and publish the underlying library via **Trusted Publishing** if also on crates.io.

This layering maximizes reach (curl/brew/cargo/Docker), speed (prebuilt binaries, no compile wait), and supply-chain safety (OIDC publish, semver gate, checksums).

## Sources
- Kept: [crates.io Trusted Publishing](https://crates.io/docs/trusted-publishing) — authoritative, defines the recommended publish flow + security model.
- Kept: [release-plz "Why yet another release tool"](https://release-plz.dev/docs/why) — primary source comparing cargo-release vs release-plz.
- Kept: [axodotdev/cargo-dist](https://github.com/axodotdev/cargo-dist) + [v0.32.0 release](https://github.com/axodotdev/cargo-dist/releases/tag/v0.32.0) — current release automation + installer capabilities.
- Kept: [cargo-bins/cargo-binstall](https://github.com/cargo-bins/cargo-binstall) — binary-first install path for end users.
- Kept: [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) + [Project Goal 2025H2](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-semver-checks.html) — semver CI gate, current roadmap.
- Kept: [Rust Project Primer — Containers](https://www.rustprojectprimer.com/releasing/containers.html) + [Docker Rust guide](https://docs.docker.com/guides/rust/build-images/) — container best practices.
- Kept: [rust-porting-playbook binary distribution research](https://github.com/jlevy/rust-porting-playbook/blob/main/docs/project/research/research-rust-cli-binary-distribution.md) — real-world cross-compile + release action patterns.
- Kept: [rs4ts.dev Distributing CLI Tools](https://rs4ts.dev/18-cli-tools/10-distribution/) — end-user install layering (brew/installer/binstall).
- Dropped: generic "How to publish your first crate" blog posts and SEO listicles — redundant with official crates.io docs.
- Dropped: LinkedIn/promotional summaries of release-plz — superseded by the official docs.

## Gaps
- No release-plz vs cargo-release **quantitative benchmark** (build time, maintainer effort) — comparison is qualitative from docs.
- cargo-dist **0.33** does not exist yet (latest is 0.32.0, 2026-05); future installer features unverified.
- Exact **minimal image size** numbers for distroless vs scratch were not benchmarked here (depends on crate deps).
- Windows/Mac signing/notarization for cargo-dist MSI/pkg installers was not deeply covered (relevant for production trust).

## Supervisor coordination
No blocking decisions. Research completed within scope; writing artifact only (no code changes), consistent with a research subagent role.

---

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Research brief authored strictly on the assigned topic (crates.io publishing, cargo-release vs release-plz, binary distribution, container images, semver/versioning, cross-compilation, and the 2026 CLI distribution recommendation) with no scope widening."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "Findings cite primary/authoritative sources (crates.io Trusted Publishing docs, release-plz official docs, cargo-dist GitHub + v0.32.0 release, cargo-binstall, cargo-semver-checks + Rust Project Goal, Rust Project Primer/Docker container guides) with URLs, plus a Sources section listing kept and dropped sources and a Gaps section."
    }
  ],
  "changedFiles": [
    "home/kwhatcher/projects/gor/.pi-subagents/artifacts/outputs/57baf442/research-deployment.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "web_search: crates.io publishing workflow best practices 2025/2026",
      "result": "passed",
      "summary": "Returned Trusted Publishing (OIDC) as recommended flow."
    },
    {
      "command": "web_search: cargo-release vs release-plz comparison 2025",
      "result": "passed",
      "summary": "Returned philosophy + feature differences."
    },
    {
      "command": "web_search: cargo-dist vs cargo-binstall binary distribution 2025/2026",
      "result": "passed",
      "summary": "Returned maintainer vs user tool roles."
    },
    {
      "command": "web_search: cargo-semver-checks CI, cross-compilation GitHub Actions, container image best practices",
      "result": "passed",
      "summary": "Returned semver gate, cross tooling, Docker patterns."
    },
    {
      "command": "web_search: best way distribute Rust CLI 2026 + cargo-dist 0.33 + release-plz 2026",
      "result": "passed",
      "summary": "Confirmed layered install recommendation; latest cargo-dist is 0.32.0."
    },
    {
      "command": "fetch_content: crates.io/docs/trusted-publishing",
      "result": "passed",
      "summary": "Verified OIDC flow, 30-min tokens, GitHub GA / GitLab beta."
    },
    {
      "command": "fetch_content: release-plz.dev/docs/why",
      "result": "passed",
      "summary": "Verified cargo-semver-checks integration and differences."
    }
  ],
  "validationOutput": [
    "Output written to authoritative path /home/kwhatcher/projects/gor/.pi-subagents/artifacts/outputs/57baf442/research-deployment.md",
    "Covered all 7 requested topics; answer to 'best way to distribute a Rust CLI in 2026' provided as a layered strategy.",
    "Sources verified primary/authoritative; redundant SEO/blog sources dropped with rationale."
  ],
  "residualRisks": [
    "No quantitative benchmarks for cargo-release vs release-plz effort/build time.",
    "cargo-dist 0.33 not yet released (latest verified 0.32.0, 2026-05).",
    "Windows/Mac installer signing/notarization not deeply covered."
  ],
  "noStagedFiles": true,
  "diffSummary": "New research artifact created; no code or repository files modified (research-only task).",
  "reviewFindings": [
    "no blockers: artifact complete and within scope; acceptance is 'reviewed' per contract."
  ],
  "manualNotes": "This is a research-only deliverable (no code changes). Progress file at .pi-subagents/artifacts/progress/57baf442/progress.md should be updated by the parent or a separate step; research content is finalized."
}
```
