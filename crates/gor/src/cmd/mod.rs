//! Command implementations for `gor`.
//!
//! Each subcommand lives in its own module. The [`dispatch`] function
//! routes parsed CLI arguments to the appropriate command handler.

pub mod alias;
pub mod api;
pub mod attestation;
pub mod auth;
pub mod browse;
pub mod cache;
pub mod classroom;
pub mod codespace;
pub mod completion;
pub mod config;
pub mod copilot;
pub mod extension;
pub mod gist;
pub mod issue;
pub mod keys;
pub mod label;
pub mod org;
pub mod pr;
pub mod project;
pub mod release;
pub mod repo;
pub mod ruleset;
pub mod run;
pub mod search;
pub mod secret;
pub mod util;
pub mod variable;
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
        Command::Secret(cmd) => secret::run(cmd),
        Command::Variable(cmd) => variable::run(cmd),
        Command::Run(cmd) => run::run(cmd),
        Command::Cache(cmd) => cache::run(cmd),
        Command::Ruleset(cmd) => ruleset::run(cmd),
        Command::Extension(cmd) => extension::run(cmd),
        Command::Codespace(cmd) => codespace::run(cmd),
        Command::Project(cmd) => project::run(cmd),
        Command::Attestation(cmd) => attestation::run(cmd),
        Command::Classroom(cmd) => classroom::run(cmd),
        Command::Copilot(cmd) => copilot::run(cmd),
        Command::Completion { shell } => completion::run(&shell),
    }
}
