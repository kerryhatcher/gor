//! Implementation of the `gor ssh-key` and `gor gpg-key` subcommands.

#![allow(clippy::print_stdout)]

use crate::cli::{GpgKeyCommand, SshKeyCommand};
use crate::cmd::util::truncate;
use crate::render::print_json;
use gor_core::Client;
use gor_core::keys;
use std::fs;
use std::io::Write;

/// Run the `gor ssh-key` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run_ssh(cmd: SshKeyCommand) -> anyhow::Result<()> {
    match cmd {
        SshKeyCommand::List { json, hostname } => list_ssh(json, hostname.as_deref()),
        SshKeyCommand::Add {
            title,
            file,
            body,
            hostname,
        } => add_ssh(
            &title,
            file.as_deref(),
            body.as_deref(),
            hostname.as_deref(),
        ),
        SshKeyCommand::Delete {
            key_id,
            yes,
            hostname,
        } => delete_ssh(&key_id, yes, hostname.as_deref()),
    }
}

/// Run the `gor gpg-key` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run_gpg(cmd: GpgKeyCommand) -> anyhow::Result<()> {
    match cmd {
        GpgKeyCommand::List { json, hostname } => list_gpg(json, hostname.as_deref()),
        GpgKeyCommand::Add {
            file,
            body,
            hostname,
        } => add_gpg(file.as_deref(), body.as_deref(), hostname.as_deref()),
        GpgKeyCommand::Delete {
            key_id,
            yes,
            hostname,
        } => delete_gpg(&key_id, yes, hostname.as_deref()),
    }
}

fn client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("{e}"))
}

fn list_ssh(json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let c = client(hostname)?;
    let keys = keys::list_ssh(&c)?;
    if let Some(fields) = json {
        let fields_ref = if fields.is_empty() {
            None
        } else {
            Some(fields.as_slice())
        };
        let v: Vec<serde_json::Value> = keys
            .into_iter()
            .map(|k| serde_json::to_value(k).unwrap_or_default())
            .collect();
        print_json(&v, fields_ref);
        return Ok(());
    }
    if keys.is_empty() {
        println!("No SSH keys found.");
        return Ok(());
    }
    println!("{:<30}  {:<10}  KEY", "TITLE", "TYPE");
    for k in &keys {
        let t = truncate(&k.title, 30);
        let kt = k.key.split(' ').next().unwrap_or("—");
        let ks: String = k
            .key
            .split(' ')
            .nth(1)
            .unwrap_or("")
            .chars()
            .take(40)
            .collect();
        println!("{t:<30}  {kt:<10}  {ks}");
    }
    Ok(())
}

fn add_ssh(
    title: &str,
    file: Option<&str>,
    body: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let kb = match (body, file) {
        (Some(b), _) => b.to_string(),
        (_, Some(f)) => fs::read_to_string(f)
            .map_err(|e| anyhow::anyhow!("failed to read key file {f}: {e}"))?
            .trim()
            .to_string(),
        _ => anyhow::bail!("no SSH key provided (use --file or --body)"),
    };
    if kb.is_empty() {
        anyhow::bail!("SSH key body is empty");
    }
    let c = client(hostname)?;
    let result = keys::add_ssh(&c, title, &kb)?;
    println!("SSH key added: {} ({})", result.id, title);
    Ok(())
}

fn delete_ssh(key_id: &str, yes: bool, hostname: Option<&str>) -> anyhow::Result<()> {
    if !yes {
        print!("Delete SSH key '{key_id}'? [y/N] ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {}
            _ => {
                println!("Cancelled.");
                return Ok(());
            }
        }
    }
    let c = client(hostname)?;
    keys::delete_ssh(
        &c,
        key_id
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid key ID: {key_id}"))?,
    )?;
    println!("SSH key '{key_id}' deleted.");
    Ok(())
}

fn list_gpg(json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let c = client(hostname)?;
    let gpg_keys = keys::list_gpg(&c)?;
    if let Some(fields) = json {
        let fields_ref = if fields.is_empty() {
            None
        } else {
            Some(fields.as_slice())
        };
        let v: Vec<serde_json::Value> = gpg_keys
            .into_iter()
            .map(|k| serde_json::to_value(k).unwrap_or_default())
            .collect();
        print_json(&v, fields_ref);
        return Ok(());
    }
    if gpg_keys.is_empty() {
        println!("No GPG keys found.");
        return Ok(());
    }
    println!("{:<16}  {:<30}  EMAILS", "KEY ID", "NAME");
    for k in &gpg_keys {
        let kid = k.key_id.as_deref().unwrap_or("—");
        let name = truncate(k.name.as_deref().unwrap_or("—"), 30);
        println!("{kid:<16}  {name:<30}");
    }
    Ok(())
}

fn add_gpg(file: Option<&str>, body: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
    let key = match (body, file) {
        (Some(b), _) => b.to_string(),
        (_, Some(f)) => fs::read_to_string(f)
            .map_err(|e| anyhow::anyhow!("failed to read key file {f}: {e}"))?
            .trim()
            .to_string(),
        _ => anyhow::bail!("no GPG key provided (use --file or --body)"),
    };
    if key.is_empty() {
        anyhow::bail!("GPG key body is empty");
    }
    let c = client(hostname)?;
    let result = keys::add_gpg(&c, &key)?;
    println!("GPG key added: {}", result.key_id.as_deref().unwrap_or("—"));
    Ok(())
}

fn delete_gpg(key_id: &str, yes: bool, hostname: Option<&str>) -> anyhow::Result<()> {
    if !yes {
        print!("Delete GPG key '{key_id}'? [y/N] ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {}
            _ => {
                println!("Cancelled.");
                return Ok(());
            }
        }
    }
    let c = client(hostname)?;
    keys::delete_gpg(&c, key_id)?;
    println!("GPG key '{key_id}' deleted.");
    Ok(())
}
