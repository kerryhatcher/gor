//! Command implementations for `gor`.
//!
//! Each subcommand lives in its own module. The [`dispatch`] function
//! routes parsed CLI arguments to the appropriate command handler.

pub mod alias;
pub mod api;
pub mod auth;
pub mod browse;
pub mod config;
pub mod gist;
pub mod issue;
pub mod keys;
pub mod label;
pub mod org;
pub mod pr;
pub mod release;
pub mod repo;
pub mod search;
pub mod util;
pub mod workflow;

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
        Command::Label(cmd) => label::run(cmd),
        Command::Release(cmd) => release::run(cmd),
        Command::Browse(cmd) => browse::run(cmd, args.hostname.as_deref()),
        Command::Gist(cmd) => gist::run(cmd),
        Command::Search(cmd) => search::run(cmd),
        Command::Workflow(cmd) => workflow::run(cmd),
        Command::Alias(cmd) => alias::run(cmd),
        Command::Org(cmd) => org::run(cmd),
        Command::SshKey(cmd) => keys::run_ssh(cmd),
        Command::GpgKey(cmd) => keys::run_gpg(cmd),
        Command::Secret(_cmd) => anyhow::bail!("secret command not yet implemented"),
        Command::Variable(_cmd) => anyhow::bail!("variable command not yet implemented"),
        Command::Run(_cmd) => anyhow::bail!("run command not yet implemented"),
        Command::Cache(_cmd) => anyhow::bail!("cache command not yet implemented"),
        Command::Ruleset(_cmd) => anyhow::bail!("ruleset command not yet implemented"),
        Command::Extension(_cmd) => anyhow::bail!("extension command not yet implemented"),
        Command::Codespace(_cmd) => anyhow::bail!("codespace command not yet implemented"),
    }
}
