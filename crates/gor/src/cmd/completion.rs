//! Implementation of the `gor completion` subcommand.
//!
//! Generates shell completion scripts for bash, zsh, fish, and PowerShell.

#![allow(clippy::print_stdout)]

use clap::CommandFactory;
use clap_complete::{Generator, Shell};

/// Run the `gor completion` subcommand.
///
/// # Errors
///
/// Returns an error if the shell type is not recognized.
pub fn run(shell: &str) -> anyhow::Result<()> {
    let shell = shell.parse::<Shell>().map_err(|_| {
        anyhow::anyhow!("unknown shell '{shell}'. Supported shells: bash, zsh, fish, powershell")
    })?;

    let cmd = crate::cli::Args::command();
    let name = cmd.get_name().to_string();
    shell.generate(&cmd, &mut std::io::stdout());

    tracing::info!("Generated {shell} completion script for {name}");
    Ok(())
}
