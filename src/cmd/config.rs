//! Implementation of the `gor config` subcommand.
//!
//! Manages configuration values stored in `~/.config/gor/config.yml`.

#![allow(clippy::print_stdout)]

use crate::cli::ConfigCommand;
use crate::config::{self, GorConfig};

/// Run the `gor config` subcommand.
///
/// # Errors
///
/// Returns an error if the config file cannot be read or written, or if the
/// user provides an invalid key or value.
pub fn run(cmd: ConfigCommand, hostname: Option<&str>) -> anyhow::Result<()> {
    match cmd {
        ConfigCommand::Get { key } => {
            let config = config::load()?;
            match config::get(&config, &key, hostname) {
                Some(value) => {
                    // Print scalar values as plain strings, complex values as YAML.
                    if let Some(s) = value.as_str() {
                        println!("{s}");
                    } else {
                        let yaml = serde_yaml_ng::to_string(value).unwrap_or_default();
                        print!("{yaml}");
                    }
                }
                None => {
                    anyhow::bail!("config key '{key}' is not set");
                }
            }
        }
        ConfigCommand::Set { key, value } => {
            config::validate(&key, &value).map_err(|e| anyhow::anyhow!("{e}"))?;
            let mut config = config::load()?;
            config::set(
                &mut config,
                &key,
                serde_yaml_ng::Value::String(value.clone()),
                hostname,
            );
            config::save(&config)?;
            if let Some(h) = hostname {
                println!("Set '{key}' to '{value}' for host '{h}'");
            } else {
                println!("Set '{key}' to '{value}'");
            }
        }
        ConfigCommand::List => {
            let config = config::load()?;
            print_config(&config, hostname);
        }
    }
    Ok(())
}

/// Print the current configuration as YAML to stdout.
fn print_config(config: &GorConfig, hostname: Option<&str>) {
    if let Some(h) = hostname {
        if let Some(host_config) = config.hosts.get(h) {
            if host_config.is_empty() {
                println!("# No host-scoped config for '{h}'");
            } else {
                let wrapper = serde_yaml_ng::Value::Mapping(
                    host_config
                        .iter()
                        .map(|(k, v)| (serde_yaml_ng::Value::String(k.clone()), v.clone()))
                        .collect(),
                );
                let yaml = serde_yaml_ng::to_string(&wrapper).unwrap_or_default();
                print!("{yaml}");
            }
        } else {
            println!("# No host-scoped config for '{h}'");
        }
    } else {
        // Print global config. If empty, print a comment.
        if config.global.is_empty() && config.hosts.is_empty() {
            println!("# No configuration set");
        } else {
            let yaml = serde_yaml_ng::to_string(config).unwrap_or_default();
            print!("{yaml}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_config_empty() {
        let config = GorConfig::default();
        // Should not panic.
        print_config(&config, None);
    }

    #[test]
    fn print_config_with_global() {
        let mut config = GorConfig::default();
        config.global.insert(
            "editor".to_string(),
            serde_yaml_ng::Value::String("vim".to_string()),
        );
        print_config(&config, None);
    }

    #[test]
    fn print_config_host_scoped() {
        let mut config = GorConfig::default();
        config.hosts.insert(
            "github.com".to_string(),
            std::collections::BTreeMap::from([(
                "editor".to_string(),
                serde_yaml_ng::Value::String("code".to_string()),
            )]),
        );
        print_config(&config, Some("github.com"));
    }
}
