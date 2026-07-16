//! Implementation of the `gor ssh-key` and `gor gpg-key` subcommands.
//!
//! Provides SSH and GPG key management for the authenticated user.

#![allow(clippy::print_stdout)]

use crate::cli::{GpgKeyCommand, SshKeyCommand};
use crate::client::Client;
use crate::output::print_json;
use anyhow::Context;
use std::fs;

/// Run the `gor ssh-key` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run_ssh(cmd: SshKeyCommand) -> anyhow::Result<()> {
    match cmd {
        SshKeyCommand::List { json, hostname } => list_ssh_keys(json, hostname.as_deref()),
        SshKeyCommand::Add {
            title,
            file,
            body,
            hostname,
        } => add_ssh_key(
            &title,
            file.as_deref(),
            body.as_deref(),
            hostname.as_deref(),
        ),
    }
}

/// Run the `gor gpg-key` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run_gpg(cmd: GpgKeyCommand) -> anyhow::Result<()> {
    match cmd {
        GpgKeyCommand::List { json, hostname } => list_gpg_keys(json, hostname.as_deref()),
        GpgKeyCommand::Add {
            file,
            body,
            hostname,
        } => add_gpg_key(file.as_deref(), body.as_deref(), hostname.as_deref()),
    }
}

/// Execute `gor ssh-key list`.
///
/// Lists SSH keys for the authenticated user.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn list_ssh_keys(json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let response = client
        .get("/user/keys")
        .context("failed to fetch SSH keys")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list SSH keys: HTTP {status}");
    }

    let keys: Vec<serde_json::Value> = response
        .json()
        .context("failed to parse SSH keys response")?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&keys, fields_ref);
        return Ok(());
    }

    if keys.is_empty() {
        println!("No SSH keys found.");
        return Ok(());
    }

    let title_width = 30;
    let type_width = 10;

    println!("{:<title_width$}  {:<type_width$}  KEY", "TITLE", "TYPE");

    for key in &keys {
        let title = key["title"].as_str().unwrap_or("—");
        let key_type = key["key"]
            .as_str()
            .unwrap_or("")
            .split(' ')
            .next()
            .unwrap_or("—");
        let key_short = key["key"]
            .as_str()
            .unwrap_or("")
            .split(' ')
            .nth(1)
            .unwrap_or("")
            .chars()
            .take(40)
            .collect::<String>();

        let title_truncated = crate::cmd::util::truncate(title, title_width);

        println!("{title_truncated:<title_width$}  {key_type:<type_width$}  {key_short}");
    }

    Ok(())
}

/// Execute `gor ssh-key add`.
///
/// Adds an SSH public key to the authenticated user's account.
///
/// # Errors
///
/// Returns an error if the key body is missing or the API request fails.
fn add_ssh_key(
    title: &str,
    file: Option<&str>,
    body: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let key_body = if let Some(b) = body {
        b.to_string()
    } else if let Some(f) = file {
        fs::read_to_string(f)
            .with_context(|| format!("failed to read SSH key file: {f}"))?
            .trim()
            .to_string()
    } else {
        anyhow::bail!("no SSH key provided (use --file or --body)");
    };

    if key_body.is_empty() {
        anyhow::bail!("SSH key body is empty");
    }

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let body_value = serde_json::json!({
        "title": title,
        "key": key_body,
    });

    let response = client
        .post("/user/keys", &body_value)
        .context("failed to add SSH key")?;

    let status = response.status();
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("add failed");
        anyhow::bail!("failed to add SSH key: {msg}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let key_id = result["id"].as_u64().unwrap_or(0);
    println!("SSH key added: {key_id} ({title})");
    Ok(())
}

/// Execute `gor gpg-key list`.
///
/// Lists GPG keys for the authenticated user.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn list_gpg_keys(json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let response = client
        .get("/user/gpg_keys")
        .context("failed to fetch GPG keys")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list GPG keys: HTTP {status}");
    }

    let keys: Vec<serde_json::Value> = response
        .json()
        .context("failed to parse GPG keys response")?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&keys, fields_ref);
        return Ok(());
    }

    if keys.is_empty() {
        println!("No GPG keys found.");
        return Ok(());
    }

    let id_width = 16;
    let name_width = 30;

    println!("{:<id_width$}  {:<name_width$}  EMAILS", "KEY ID", "NAME");

    for key in &keys {
        let key_id = key["key_id"].as_str().unwrap_or("—");
        let name = key["name"].as_str().unwrap_or("—");

        let emails: Vec<String> = key["emails"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|e| e["email"].as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let name_truncated = crate::cmd::util::truncate(name, name_width);
        let emails_str = emails.join(", ");

        println!("{key_id:<id_width$}  {name_truncated:<name_width$}  {emails_str}");
    }

    Ok(())
}

/// Execute `gor gpg-key add`.
///
/// Adds a GPG public key to the authenticated user's account.
///
/// # Errors
///
/// Returns an error if the key body is missing or the API request fails.
fn add_gpg_key(
    file: Option<&str>,
    body: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let armored_key = if let Some(b) = body {
        b.to_string()
    } else if let Some(f) = file {
        fs::read_to_string(f)
            .with_context(|| format!("failed to read GPG key file: {f}"))?
            .trim()
            .to_string()
    } else {
        anyhow::bail!("no GPG key provided (use --file or --body)");
    };

    if armored_key.is_empty() {
        anyhow::bail!("GPG key body is empty");
    }

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let body_value = serde_json::json!({
        "armored_public_key": armored_key,
    });

    let response = client
        .post("/user/gpg_keys", &body_value)
        .context("failed to add GPG key")?;

    let status = response.status();
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("add failed");
        anyhow::bail!("failed to add GPG key: {msg}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let key_id = result["key_id"].as_str().unwrap_or("—");
    println!("GPG key added: {key_id}");
    Ok(())
}
