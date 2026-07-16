//! Command implementations for `gor`.
//!
//! Each subcommand lives in its own module. The [`dispatch`] function
//! routes parsed CLI arguments to the appropriate command handler.

pub mod api;
pub mod auth;
pub mod config;
pub mod issue;
pub mod pr;
pub mod repo;
pub mod util;

use crate::cli::{Args, Command};

/// Dispatch a parsed CLI command to its handler.
///
/// # Errors
///
/// Returns an error if the command execution fails.
#[allow(clippy::print_stdout)]
pub fn dispatch(args: Args) -> anyhow::Result<()> {
    match args.command {
        Command::Api(cmd) => api::run(&cmd, args.hostname.as_deref()),
        Command::Auth(cmd) => auth::run(cmd),
        Command::Repo(cmd) => repo::run(cmd),
        Command::Pr(cmd) => pr::run(cmd),
        Command::Issue(cmd) => issue::run(cmd),
        Command::Config(cmd) => config::run(cmd, args.hostname.as_deref()),
    }
}
