# gor

### GitHub on Rust

[![CI](https://img.shields.io/badge/CI-passing-2ea043?style=flat-square)](https://github.com/gor-sh/gor/actions)
[![crates.io](https://img.shields.io/crates/v/gor?style=flat-square)](https://crates.io/crates/gor)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg?style=flat-square)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-blue?style=flat-square)](https://blog.rust-lang.org)

A fast, self-contained GitHub CLI written in Rust.

## Why gor?

- **Pure Rust** — no OpenSSL, no system `git` binary; everything links statically and runs anywhere.
- **No `git` subprocess** — clones, fetches, and remote detection use `gix` (gitoxide), so gor works on machines without Git installed.
- **Cross-platform** — first-class Linux, macOS, and Windows binaries, plus musl static builds for containers.
- **GitHub Enterprise Server support** — talk to `github.com` or your GHES instance from the same tool via `--hostname` / `GH_HOST`.
- **Fast** — single static binary, sub-100ms startup, no JVM or Node runtime to warm up.

## Install

```sh
# From source (compile)
cargo install gor

# Prebuilt binary, no compile step
cargo binstall gor

# Homebrew
brew install gor
```

Other options (installer script, Docker image) are published with each release.

## Quick start

```sh
# Authenticate (OAuth device flow, or paste a token)
gor auth login

# Inspect a repository
gor repo view cli/cli

# List open pull requests
gor pr list

# List open issues
gor issue list
```

## Usage reference

| Command group | Subcommands |
|---------------|-------------|
| `auth` | `login`, `logout`, `status`, `token` |
| `api` | arbitrary REST (`GET`/`POST`/...), `graphql` |
| `repo` | `view`, `list`, `clone`, `create`, `fork`, `delete`, `edit`, `archive`, `rename`, `sync` |
| `pr` | `list`, `view`, `checkout`, `create`, `comment`, `close`, `reopen`, `merge`, `diff`, `review`, `ready`, `edit`, `checks` |
| `issue` | `list`, `view`, `create`, `comment`, `close`, `reopen`, `edit`, `delete`, `lock`, `unlock`, `pin`, `unpin`, `transfer` |
| `release` | `list`, `view`, `create`, `edit`, `delete`, `upload`, `download` |
| `label` | `list`, `create`, `edit`, `delete`, `clone` |
| `search` | `repos`, `issues`, `prs`, `code`, `commits` |
| `gist` | `list`, `view`, `create`, `edit`, `delete`, `clone` |
| `workflow` | `list`, `view`, `run`, `enable`, `disable` |
| `run` | `list`, `view`, `watch`, `cancel`, `rerun`, `download` |
| `browse` | open repo / issue / PR in `$BROWSER` |
| `org` | `list`, `view` |
| `project` | `list`, `view`, `item-add` |
| `secret` | `list`, `set`, `delete` |
| `variable` | `list`, `set`, `delete` |
| `ssh-key` | `list`, `add`, `delete` |
| `gpg-key` | `list`, `add`, `delete` |
| `attestation` | `verify` |
| `ruleset` | `list`, `view` |
| `cache` | `list`, `delete` |
| `codespace` | `list`, `create`, `stop`, `delete`, `ssh` |

> Cross-cutting flags: `--json <fields>` (field selection), `--jq`, `--template`, `--web`, `--hostname` / `GH_HOST`, and `--repo` / `-R OWNER/REPO`.

## Documentation

- Extended guides and architecture: [docs/](docs/)
- API fundamentals and feature roadmap: [docs/research/](docs/research/)
- Crate API reference: published on [docs.rs](https://docs.rs/gor) with each release.

## Contributing

Contributions are welcome. Run the full check suite with `just ci`, and please follow [Conventional Commits](https://www.conventionalcommits.org/) so the changelog and releases stay automatic. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
