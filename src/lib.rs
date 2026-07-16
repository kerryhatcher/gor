//! # gor — GitHub on Rust
//!
//! A fast, self-contained GitHub CLI written in Rust. Equivalent to `gh` but
//! with no external dependencies on `git` or OpenSSL.
//!
//! ## Architecture
//!
//! This crate uses a **library + binary** split:
//! - `lib.rs` — all business logic, fully unit-testable
//! - `main.rs` — thin entry point: parse args, init tracing, dispatch
//!
//! ## Quick start
//!
//! ```rust
//! use gor::Gor;
//!
//! # fn main() -> anyhow::Result<()> {
//! let app = Gor::new();
//! // app.run() would parse args and dispatch
//! # Ok(())
//! # }
//! ```
//!
//! ## Feature flags
//!
//! Currently there are no feature flags. All functionality is included by default.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod cli;
pub mod cmd;
pub mod config;
pub mod error;

use clap::Parser;
use cli::Args;

/// The main application entry point for the library.
///
/// Construct with [`Gor::new`] and call [`Gor::run`] to execute.
///
/// # Examples
///
/// ```no_run
/// use gor::Gor;
///
/// let app = Gor::new();
/// // app.run() parses CLI args and dispatches to the appropriate command
/// ```
pub struct Gor;

impl Gor {
    /// Create a new `Gor` application instance.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Parse CLI arguments and run the requested command.
    ///
    /// # Errors
    ///
    /// Returns an error if argument parsing fails or if the command execution fails.
    pub fn run(self) -> anyhow::Result<()> {
        let args = Args::parse();
        cmd::dispatch(args)
    }
}

impl Default for Gor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of_val;

    #[test]
    fn gor_new_creates_instance() {
        let app = Gor::new();
        // Unit struct — verify it exists and has zero size.
        assert_eq!(size_of_val(&app), 0);
    }
}
