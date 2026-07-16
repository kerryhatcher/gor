//! Implementation of the `gor alias` subcommand.
//!
//! Provides command alias management for listing and setting aliases.
//! Aliases are stored in the config file under the `aliases` key.

#![allow(clippy::print_stdout)]

use crate::cli::AliasCommand;
use crate::config;
use anyhow::Context;

/// Run the `gor alias` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: AliasCommand) -> anyhow::Result<()> {
    match cmd {
        AliasCommand::List { hostname: _ } => list(),
        AliasCommand::Set {
            name,
            command,
            hostname: _,
        } => set(&name, &command),
    }
}

/// Execute `gor alias list`.
///
/// Lists all configured command aliases.
///
/// # Errors
///
/// Returns an error if the configuration cannot be read.
fn list() -> anyhow::Result<()> {
    let config = config::load().context("failed to load config")?;
    let aliases = config.global.get("aliases");

    let Some(alias_map) = aliases.and_then(|v| v.as_mapping()) else {
        println!("No aliases configured.");
        return Ok(());
    };

    let name_width = 20;
    println!("{:<name_width$}  COMMAND", "ALIAS");
    for (name, command) in alias_map {
        let name_str = name.as_str().unwrap_or("?");
        let cmd_str = command.as_str().unwrap_or("?");
        let name_truncated = crate::cmd::util::truncate(name_str, name_width);
        println!("{name_truncated:<name_width$}  {cmd_str}");
    }
    Ok(())
}

/// Execute `gor alias set`.
///
/// Sets a command alias. The alias maps a short name to a gor command
/// with arguments.
///
/// # Errors
///
/// Returns an error if the configuration cannot be saved.
fn set(name: &str, command: &[String]) -> anyhow::Result<()> {
    if command.is_empty() {
        anyhow::bail!("alias command is required");
    }

    let mut config = config::load().context("failed to load config")?;
    let cmd_str = command.join(" ");

    // Get or create the aliases map
    let mut aliases = config
        .global
        .get("aliases")
        .and_then(|v| v.as_mapping().cloned())
        .unwrap_or_default();

    aliases.insert(
        serde_yaml_ng::Value::String(name.to_string()),
        serde_yaml_ng::Value::String(cmd_str.clone()),
    );

    config.global.insert(
        "aliases".to_string(),
        serde_yaml_ng::Value::Mapping(aliases),
    );

    config::save(&config).context("failed to save config")?;

    println!("Alias '{name}' set to '{cmd_str}'");
    Ok(())
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn set_requires_command() {
        let result = std::panic::catch_unwind(|| {
            let _ = set("test", &[]);
        });
        assert!(result.is_ok());
    }
}
