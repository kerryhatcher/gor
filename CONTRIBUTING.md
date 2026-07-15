# Contributing to gor

Thanks for your interest in contributing! gor is a Rust rewrite of the GitHub CLI — fast, self-contained, and pure Rust (no `git` binary, no OpenSSL).

---

## Getting Started

### Prerequisites

| Tool | Purpose | Official Docs |
|------|---------|---------------|
| **Rust** 1.85+ | Compiler & toolchain | [rustup.rs](https://rustup.rs) |
| **just** | Command runner | [just.systems](https://just.systems) |
| **cargo-nextest** | Test runner | [nexte.st](https://nexte.st) |
| **cargo-deny** | Supply-chain checks | [cargo-deny](https://embarkstudios.github.io/cargo-deny/) |
| **typos-cli** | Spellcheck | [typos](https://github.com/crate-ci/typos) |
| **Trivy** | Vuln, secret & misconfig scanning | [trivy.dev](https://trivy.dev) |
| **Kingfisher** | Secret scanning with live validation | [Kingfisher](https://mongodb.github.io/kingfisher/) |

#### Rust (rustup)

The recommended way to install Rust is via [rustup](https://rustup.rs):

```bash
# Linux / macOS / WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (PowerShell)
# Download and run: https://win.rustup.rs
```

After installation, ensure the stable toolchain is active:

```bash
rustup toolchain install stable
rustup default stable
```

#### just, cargo-nextest, cargo-deny, typos-cli

```bash
# All platforms (via cargo)
cargo install just cargo-nextest cargo-deny typos-cli

# macOS (via Homebrew)
brew install just nextest cargo-deny typos-cli

# Linux — use cargo or distro packages where available
```

#### Trivy

```bash
# macOS
brew install trivy

# Linux (Debian/Ubuntu)
sudo apt-get install -y wget apt-transport-https gnupg
wget -qO - https://aquasecurity.github.io/trivy-repo/deb/public.key | sudo apt-key add -
echo "deb https://aquasecurity.github.io/trivy-repo/deb $(lsb_release -sc) main" | sudo tee /etc/apt/sources.list.d/trivy.list
sudo apt-get update && sudo apt-get install -y trivy

# Linux (RHEL/CentOS/Fedora)
sudo rpm -ivh https://github.com/aquasecurity/trivy/releases/latest/download/trivy.rpm

# Windows (Scoop / Chocolatey)
scoop install trivy
# or
choco install trivy

# Docker (no local install needed)
docker run --rm -v "$PWD":/src aquasec/trivy fs /src
```

See the [Trivy installation docs](https://trivy.dev/docs/latest/getting-started/installation/) for more options.

#### Kingfisher

```bash
# macOS
brew install kingfisher

# Linux / macOS (installer script)
curl -sSL https://raw.githubusercontent.com/mongodb/kingfisher/main/scripts/install-kingfisher.sh | bash

# PyPI
uv tool install kingfisher-bin
# or
pip install kingfisher-bin

# Docker (no local install needed)
docker run --rm -v "$PWD":/src ghcr.io/mongodb/kingfisher:latest scan /src
```

See the [Kingfisher installation docs](https://mongodb.github.io/kingfisher/getting-started/installation/) for more options.

### Setup

```bash
git clone https://github.com/gor-sh/gor.git
cd gor
just build     # verify everything compiles
just test      # run the test suite
just ci        # full CI gate — run this before pushing
```

Run `just` with no arguments to see all available recipes.

---

## Finding Something to Work On

- **Good first issues** are tagged with [`good first issue`](https://github.com/gor-sh/gor/labels/good%20first%20issue) — these are small, self-contained tasks ideal for new contributors.
- **Help wanted** issues are tagged with [`help wanted`](https://github.com/gor-sh/gor/labels/help%20wanted) — these are larger features where design direction is already established.
- Check the [API Implementation Roadmap](docs/research/api-implementation-roadmap.md) for the big-picture plan.
- If you have an idea that isn't captured in an issue, **open an issue first** to discuss it before writing code.

---

## Development Workflow

### Branching

- Fork the repo and create a feature branch from `main`.
- Branch naming: `feat/short-description`, `fix/short-description`, `docs/short-description`.

### Commit Messages

We use **Conventional Commits**. This is required — our release automation generates changelogs from commit messages.

```
feat: add repo create command
fix: handle 429 rate limit responses
docs: document auth flow
test: add integration tests for pr checkout
refactor: extract pagination to util module
chore: update dependencies
```

Keep commits focused. One logical change per commit. If you need to fix something from review, squash rather than adding fixup commits.

### Before Submitting

```bash
just ci    # lint + test + audit + typos + shear
```

This runs the same checks as CI. If it fails, CI will fail too.

---

## Code Style

### Architecture

gor uses a **library + binary** split:

- `src/lib.rs` — all business logic, fully unit-testable
- `src/main.rs` — thin entry point: parse args, init tracing, dispatch

**Do not put business logic in `main.rs`.** If you're writing more than argument parsing and dispatch, it belongs in `lib.rs` or a `cmd/` module.

### Error Handling

- **Library code** (`client.rs`, `repository.rs`, etc.): Use `thiserror` — add variants to `error.rs`.
- **Command code** (`cmd/*.rs`): Use `anyhow::Result` with `.context(...)`.
- **User-facing errors**: Use `miette` for structured diagnostics with source snippets.

### Linting

Lint policy is centralized in `Cargo.toml` under `[workspace.lints]`. Key rules:

- **No `unwrap()`** — use `?` or proper error handling. `unwrap_used` is `deny` at the workspace level.
- **No `dbg!()`** — use `tracing::debug!()` instead.
- **No `todo!()`** — open an issue or use a proper error.
- **No unsafe code** — `unsafe_code` is `deny`.

If you need to suppress a lint, do it at the smallest possible scope with a comment explaining why:

```rust
#[allow(clippy::unwrap_used)]
fn example() {
    // Safe: this can never fail because we validated the input above
    let value = optional.unwrap();
}
```

### Formatting

Run `just lint-fix` before committing. This auto-formats with `cargo fmt` and auto-fixes clippy lints.

---

## Testing

### Writing Tests

- **Unit tests** go in `#[cfg(test)] mod tests` blocks at the bottom of source files.
- **Integration tests** go in `tests/` and use `assert_cmd` + `assert_fs` for CLI testing.
- **Snapshot tests** use `insta` — run `cargo insta review` to accept new snapshots.
- **Doc tests** go in `///` comments on public items. Every public function should have a runnable example.

### Running Tests

```bash
just test               # full suite
just test-unit          # unit tests only
just test-integration   # integration tests only
just coverage           # with coverage report
```

### Test Philosophy

- Test behavior, not implementation. Integration tests that invoke the binary are preferred over unit tests that test internal details.
- Mock external services with `wiremock`. Never hit the real GitHub API in tests.
- Snapshot test all `--help` output and error messages. This catches accidental CLI changes.

---

## Adding a New Command

1. Define CLI args in `src/cli.rs` (Args struct + Subcommand variant).
2. Create `src/cmd/<name>.rs` with a public `run()` function.
3. Add `pub mod <name>;` to `src/cmd/mod.rs`.
4. Wire the dispatch in `src/main.rs`.
5. Add integration tests in `tests/`.
6. Add snapshot tests for `--help` output.
7. Update the usage table in `README.md`.
8. Run `just ci` before committing.

---

## Documentation

- Every public item must have a doc comment (`///` or `//!`). `missing_docs` is `deny`.
- Use `# Examples`, `# Errors`, and `# Panics` sections where applicable.
- Use intra-doc links (`[`Client`]`, `[`Repository`]`) to cross-reference types.
- If you change public API, update the relevant examples.

---

## Review Process

1. Open a pull request against `main`.
2. CI must pass (lint, test, coverage, msrv, security).
3. A maintainer will review within a few days.
4. Address feedback in new commits or by squashing.
5. Once approved, a maintainer will merge.

**Tips for a smooth review:**
- Keep PRs small and focused. A 50-line change is reviewed in minutes; a 500-line change takes days.
- Write a clear PR description: what, why, and any design decisions.
- Link to the issue your PR addresses.
- If your PR is a work in progress, mark it as a draft.

---

## Dependency Policy

gor aims to be lightweight and self-contained:

- **No OpenSSL.** Use `rustls`. `deny.toml` bans `openssl` and `openssl-sys`.
- **No git2.** Use `gix` for all git operations. `deny.toml` bans `git2`.
- **New dependencies must be justified.** What does it do that existing deps can't? How many transitive deps does it pull in?
- **Pin versions** in `[workspace.dependencies]` in the root `Cargo.toml`.

---

## Project Resources

| Resource | Location |
|----------|----------|
| API fundamentals | [`docs/research/api-overview.md`](docs/research/api-overview.md) |
| Implementation roadmap | [`docs/research/api-implementation-roadmap.md`](docs/research/api-implementation-roadmap.md) |
| Rust best practices | [`docs/research/rust-best-practices-applied.md`](docs/research/rust-best-practices-applied.md) |
| Linting deep dive | [`docs/research/research-linting.md`](docs/research/research-linting.md) |
| CI/CD deep dive | [`docs/research/research-cicd.md`](docs/research/research-cicd.md) |
| Testing deep dive | [`docs/research/research-testing.md`](docs/research/research-testing.md) |
| Architecture deep dive | [`docs/research/research-architecture.md`](docs/research/research-architecture.md) |
| AI agent guide | [`AGENTS.md`](AGENTS.md) |

---

## Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). Be respectful, constructive, and welcoming. Harassment of any kind will not be tolerated.

---

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project: **MIT OR Apache-2.0**.
