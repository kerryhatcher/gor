//! Command implementations for `gor`.
//!
//! Each subcommand lives in its own module. The [`dispatch`] function
//! routes parsed CLI arguments to the appropriate command handler.

pub mod config;
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
        Command::Repo { owner_repo } => {
            tracing::info!("Viewing repository: {owner_repo}");
            println!("Repository: {owner_repo}");
            println!("This command is not yet implemented.");
            Ok(())
        }
        Command::Config(cmd) => config::run(cmd, args.hostname.as_deref()),
    }
}
