# Research: Modern Rust CLI Tool Architecture & Crate Ecosystem (2025-2026)

## Summary
A modern, polished Rust CLI is built on a small, well-established core: **clap** (derive API) for arguments, **anyhow/thiserror** for errors with **miette** or **color-eyre** for user-facing diagnostics, **tracing** (+ `tracing-subscriber`) for structured logging, and a **lib + bin** crate layout that keeps business logic testable. Terminal polish comes from **indicatif** (progress), **ratatui** + **crossterm** (TUIs), and **console** (coloring); configuration from **config**, **figment**, or **confy** depending on layering needs. Async (Tokio) is treated as an optimization, not a default — most CLIs stay synchronous.

## Findings

### 1. Argument parsing — clap v4 is stable; v5 is still unreleased (behind `unstable-v5`)
- clap 4.6.x is the current stable line (4.6.2 released 2026-07-15; MSRV bumped to 1.85 in 4.6.0). clap **5.0.0 is marked "TBD"** and is only available via the `unstable-v5` feature flag, so production tooling should target clap 4. [Source](https://github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md)
- Key v4/v5 derive patterns: a top-level `#[derive(Parser)]` `Cli` struct holds global args (`#[arg(global = true)]`), and an enum `#[derive(Subcommand)]` models subcommands so `match` is exhaustive and compile-time safe. `#[command(subcommand)]` and `#[arg(...)]` attributes handle the rest. [Source](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- Notable v5 breaking changes (when it lands): `Vec<Vec<T>>` now captures/group values per occurrence; `ValueEnum` variants use the full doc comment (not just the summary) for help text; default `max_term_width` becomes 100; lazy subcommand init via `#[command(defer = true)]` (mirrors builder's `Command::defer`) to cut startup time on large CLI trees. [Source](https://github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md), [Source](https://github.com/clap-rs/clap/pull/6256)
- Idiomatic v4 migration detail: args now use `ArgAction` (`SetTrue`, `Count`, `Set`, `Append`) rather than the old `takes_value`/`multiple_values`; verify CLIs with `Cli::command().debug_assert()` (trycmd is recommended for snapshot `--help`/error tests). [Source](https://github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md)

### 2. Error handling — split by role: thiserror (lib) + anyhow (app) + miette/color-eyre (UX)
- **Libraries**: define explicit error enums with `thiserror` (`#[derive(Error, Debug)]`, `#[non_exhaustive]`, `#[from]` for auto-`From`/`?` conversion, `#[error("...")]` for `Display`). Wrap third-party errors; never leak them in the public API. Use `Result<T>` type aliases. [Source](https://www.azdanov.dev/articles/2025/rust-error-guidelines)
- **Applications**: use `anyhow::Result` with `.context(...)` to attach human-readable context during propagation without defining new types. [Source](https://www.azdanov.dev/articles/2025/rust-error-guidelines)
- **Rich CLI diagnostics**: `miette` provides IDE-like reports (source snippets, `#[label]`, diagnostic `code`, `help`) and pairs cleanly with `thiserror` via its `Diagnostic` trait. `color-eyre` is the lighter "install a global handler" option for colorful panic/error traces (span + backtrace). Pick `miette` for structured, code-bearing diagnostics; `color-eyre` for quick drop-in polish. [Source](https://docs.rs/miette/latest/miette/), [Source](https://docs.rs/color-eyre/latest/color_eyre/)
- `tracing-error`'s `ErrorLayer` can attach span context to errors for production debugging. [Source](https://www.azdanov.dev/articles/2025/rust-error-guidelines)

### 3. Logging/tracing — `log`+`env_logger` for simple tools; `tracing` for anything structured/async
- Simple CLIs: depend on the `log` facade and an `env_logger` impl, initialized once in `main` with `env_logger::init()`; control verbosity via `RUST_LOG=my_crate=debug`. [Source](https://docs.rs/env_logger/latest/env_logger/), [Source](https://rs4ts.dev/23-ecosystem/03-logging/)
- Complex/async/multithreaded tools: use the `tracing` ecosystem (spans, fields, JSON/OTel export). Wire up with `tracing_subscriber::registry().with(EnvFilter::from_default_env()).with(fmt::layer()).init()`. Libraries should depend only on `log`/`tracing` facades, not a concrete subscriber, to stay implementation-agnostic. Avoid `println!` for program output in production. [Source](https://docs.rs/env_logger/latest/env_logger/), [Source](https://oneuptime.com/blog/post/2026-01-07-rust-tracing-structured-logs/view)
- The `rust-clitool-template` uses `tracing` + `tracing-subscriber` (env-filter) as its default, reflecting the ecosystem shift toward tracing. [Source](https://github.com/abigagli/rust-clitool-template)

### 4. Terminal output — indicatif (progress), ratatui + crossterm (TUIs), console (color)
- **Progress**: `indicatif` is the standard for inline progress bars/spinners; it auto-detects TTY vs piped output and styles itself (it builds on the `console` crate). Typical pins: `indicatif = "0.18"`. [Source](https://docs.rs/indicatif/latest/indicatif/), [Source](https://rs4ts.dev/18-cli-tools/04-progress-bars/)
- **Full-screen TUIs**: `ratatui` (e.g. `0.29`) is the leading TUI framework and typically runs on the **crossterm** backend (`0.28`) for raw mode, alternate screen, and input events. [Source](https://github.com/ratatui/ratatui/tree/v0.29.0)
- **Color/style helpers**: `console` (used by indicatif) and `anstyle`/`anstream` (clap's own styling stack) provide portable ANSI coloring that respects `CLICOLOR`/`NO_COLOR`. Prefer them over raw ANSI escapes so output degrades gracefully when piped. [Source](https://docs.rs/indicatif/latest/indicatif/)

### 5. Configuration management — config (layered), figment (ergonomic + provenance), confy (zero-boilerplate)
- **config** (`config-rs`): mature, widely used; builds hierarchical config from layered sources (files, env vars, CLI overrides); supports reloading. Best for established, complex layered setups. [Source](https://github.com/rust-cli/config-rs)
- **figment**: modern and composable; uniquely tracks the *provenance* of every value, producing precise error messages that pinpoint exactly which source misconfigured a key. Best when type-safety, composability, and clear debugging matter. [Source](https://docs.rs/figment/latest/figment/), [Source](https://github.com/rust-cli/config-rs/issues/371)
- **confy** (`rust-cli/confy`): zero-boilerplate per-user config file storage in OS-standard locations (XDG/Native). Best for simple CLI tools that persist user state without layering or env overrides. [Source](https://github.com/rust-cli/confy)
- Practical guidance: choose Config for complex hierarchies, Figment for developer-experience/debuggability, Confy for drop-in user config. [Source](https://github.com/rust-lang-nursery/rust-cookbook/pull/808)

### 6. Async vs sync — sync by default; async only when concurrency justifies it
- Default to **synchronous** code: simpler to debug, test, and maintain; adequate for batch tools that do a task and exit. [Source](https://users.rust-lang.org/t/is-there-any-benefits-to-using-async-await-for-a-cli-tool/85761), [Source](https://microsoft.github.io/RustTraining/async-book/ch14-async-is-an-optimization-not-an-architecture.html)
- Use **Tokio** only for high I/O concurrency (many simultaneous network connections, concurrent streaming). If used, follow the **"sync core, async shell"** pattern: keep business logic synchronous and testable, wrap it in an async `main` (`#[tokio::main]` or `Runtime::block_on`). Use `rayon` for CPU-bound parallelism (async does not speed up compute). [Source](https://microsoft.github.io/RustTraining/async-book/ch14-async-is-an-optimization-not-an-architecture.html), [Source](https://rs4ts.dev/11-async/13-async-vs-sync/)
- Pitfalls: never block an executor thread — use `tokio::task::spawn_blocking` for blocking/CPU work so you don't stall the runtime. [Source](https://rust-lang.github.io/async-book/part-guide/io.html)

### 7. Project structure / organization — lib + bin, thin main, src/bin for multi-binary
- Use a **combined library + binary** layout: `src/lib.rs` holds business logic (unit/integration testable); `src/main.rs` is a thin entry point that only parses args and orchestrates. This is the dominant scalable pattern and the basis of popular templates. [Source](https://learnrust.net/chapter-13/binary-and-library-crates/), [Source](https://dev.to/sgchris/crate-layout-best-practices-librs-modrs-and-srcbin-4abd)
- Multiple related executables go in `src/bin/` (each an independent binary importing `src/lib.rs`). [Source](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- Concrete template (`rust-clitool-template`) ships: `args.rs` (clap definitions), `error.rs` (error utilities), `lib.rs` (logic), `main.rs` (logging init + dispatch); deps: clap, anyhow, tracing + tracing-subscriber, serde, tokio (minimal), plus strict Clippy lints and `just` for cross-compile. [Source](https://github.com/abigagli/rust-clitool-template)
- Complement with: clap `trycmd`/`assert_cmd` for CLI snapshot tests, `cargo-release`/cross-compilation (zigbuild) for distribution, and strict lints for polish.

## Sources
- Kept: clap CHANGELOG (github.com/clap-rs/clap) — authoritative v4/v5 status and derive changes
- Kept: Anton Ždanov, "Rust Error Guidelines" (azdanov.dev, 2025) — thiserror/anyhow/miette role split
- Kept: rust-clitool-template (github.com/abigagli/rust-clitool-template) — real-world lib+bin layout and crate set
- Kept: miette docs (docs.rs/miette) & color-eyre docs (docs.rs/color_eyre) — diagnostic UX crates
- Kept: env_logger docs & rs4ts logging guide — log/tracing guidance
- Kept: indicatif docs & ratatui repo — progress/TUI libraries
- Kept: config-rs, figment docs, confy repo — configuration crate comparison
- Kept: Microsoft Async Training "Async Is an Optimization" & Rust users forum — async vs sync rationale
- Dropped: SEO/cheat-sheet reposts (techbytes, devproportal, masturbyte) — derivative, lower authority; used only for corroboration
- Dropped: Medium "how I built" posts — anecdotal, not authoritative

## Gaps
- No hands-on benchmarking of crate overhead was performed; recommendations reflect documented ecosystem consensus, not measured performance.
- clap v5 final API is still in flux (behind `unstable-v5`); some v5 specifics may shift before release.
- Version pins (e.g., ratatui 0.29/0.30-alpha, indicatif 0.18) reflect mid-2026 snapshots and should be re-verified against crates.io at implementation time.
- Next steps: if building a concrete tool, pin exact versions via `cargo add`, validate clap with `trycmd`, and decide config strategy (Figment vs Config vs Confy) based on whether layering/env-overrides are needed.

## Supervisor coordination
No blocking decisions required; delivering research-only artifact as specified.
