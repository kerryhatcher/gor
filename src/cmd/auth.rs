//! Implementation of the `gor auth` subcommand.
//!
//! Handles login (OAuth device flow), logout, and status.

#![allow(clippy::print_stdout)]

use crate::auth::{device, token};
use crate::cli::AuthCommand;
use crate::client::Client;
use crate::host::Host;
use anyhow::Context;

/// Run the `gor auth` subcommand.
///
/// # Errors
///
/// Returns an error if authentication fails, the keyring is unavailable,
/// or the network request fails.
pub fn run(cmd: AuthCommand) -> anyhow::Result<()> {
    match cmd {
        AuthCommand::Login {
            hostname,
            scopes,
            with_token,
        } => login(
            hostname.as_deref().unwrap_or("github.com"),
            scopes.as_deref(),
            with_token,
        ),
        AuthCommand::Logout { hostname } => logout(hostname.as_deref().unwrap_or("github.com")),
        AuthCommand::Status { hostname } => status(hostname.as_deref().unwrap_or("github.com")),
        AuthCommand::SetupGit { hostname } => {
            setup_git(hostname.as_deref().unwrap_or("github.com"))
        }
        AuthCommand::Token {
            hostname,
            refresh,
            scopes,
            secure,
        } => token(
            hostname.as_deref().unwrap_or("github.com"),
            refresh,
            &scopes,
            secure,
        ),
    }
}

/// Configure git to use gor as a credential helper for HTTPS.
///
/// Writes `credential.https://<host>.helper` and `credential.helper`
/// git config entries that invoke `gor auth git-credential`.
///
/// # Errors
///
/// Returns an error if git config commands fail.
fn setup_git(hostname: &str) -> anyhow::Result<()> {
    use std::process::Command;

    let helper_value = "!gor auth git-credential";
    let cred_key = format!("credential.https://{hostname}.helper");

    // Check if gor is already configured for this host
    let existing = Command::new("git")
        .args(["config", "--global", "--get", &cred_key])
        .output()
        .ok();

    let already_configured = existing.as_ref().is_some_and(|o| {
        o.status.success() && String::from_utf8_lossy(&o.stdout).trim() == helper_value
    });

    if already_configured {
        println!("git is already configured to use gor for {hostname} credentials");
        return Ok(());
    }

    // Check if a non-gor credential helper exists for this host
    let has_existing = existing.as_ref().is_some_and(|o| {
        o.status.success() && !String::from_utf8_lossy(&o.stdout).trim().is_empty()
    });

    if has_existing {
        // Preserve existing non-gor helper by using --add
        println!("$ git config --global --add {cred_key} {helper_value}");
        let output = Command::new("git")
            .args(["config", "--global", "--add", &cred_key, helper_value])
            .output()
            .context("failed to run git config --add")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git config failed: {stderr}");
        }
    } else {
        // Set the credential helper for the specific host
        println!("$ git config --global {cred_key} {helper_value}");
        let output = Command::new("git")
            .args(["config", "--global", &cred_key, helper_value])
            .output()
            .context("failed to run git config")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git config failed: {stderr}");
        }
    }

    // Also set the generic credential.helper if not already set
    let generic_key = "credential.helper";
    let generic_existing = Command::new("git")
        .args(["config", "--global", "--get", generic_key])
        .output()
        .ok();

    let has_generic = generic_existing.as_ref().is_some_and(|o| {
        o.status.success() && !String::from_utf8_lossy(&o.stdout).trim().is_empty()
    });

    if !has_generic {
        println!("$ git config --global {generic_key} {helper_value}");
        let output = Command::new("git")
            .args(["config", "--global", generic_key, helper_value])
            .output()
            .context("failed to run git config")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git config failed: {stderr}");
        }
    }

    println!("✓ git is now configured to use gor for {hostname} credentials");
    Ok(())
}

/// Log in to a GitHub account using the OAuth device flow.
fn login(hostname: &str, scopes: Option<&str>, with_token: bool) -> anyhow::Result<()> {
    let host = Host::new(hostname);

    if with_token {
        // Read token from stdin.
        let token = token::read_token_from_stdin()
            .map_err(|e| anyhow::anyhow!("failed to read token from stdin: {e}"))?;
        if token.is_empty() {
            anyhow::bail!("no token provided on stdin");
        }

        // Verify the token.
        let login = token::verify_token(&host, &token)
            .map_err(|e| anyhow::anyhow!("token verification failed: {e}"))?;

        // Store it.
        let client = Client::with_token(hostname, &token)?;
        client.save_token()?;
        client.save_user(&login)?;

        println!("Authenticated as {login}");
        return Ok(());
    }

    // Start the OAuth device flow.
    let device_resp = device::request_device_code(&host, scopes)
        .map_err(|e| anyhow::anyhow!("failed to request device code: {e}"))?;

    // Show the user the code and URL.
    device::display_instructions(&device_resp.user_code, &device_resp.verification_uri);

    // Poll for the token.
    let access_token = device::poll_for_token(
        &host,
        &device_resp.device_code,
        device_resp.interval,
        device_resp.expires_in,
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;

    // Verify the token.
    let login = token::verify_token(&host, &access_token)
        .map_err(|e| anyhow::anyhow!("token verification failed: {e}"))?;

    // Store it.
    let client = Client::with_token(hostname, &access_token)?;
    client.save_token()?;
    client.save_user(&login)?;

    println!("Authenticated as {login}");
    Ok(())
}

/// Log out of a GitHub account by removing the stored token.
fn logout(hostname: &str) -> anyhow::Result<()> {
    let client = Client::new(hostname)?;
    if !client.is_authenticated() {
        println!("Not logged in to {hostname}");
        return Ok(());
    }

    crate::keyring_store::delete_token(hostname)
        .map_err(|e| anyhow::anyhow!("failed to remove token: {e}"))?;
    println!("Logged out of {hostname}");
    Ok(())
}

/// Print or refresh the authentication token for the given host.
///
/// When `refresh` is true, runs the OAuth device flow to obtain a new token.
/// Optional `scopes` can be specified to request additional OAuth scopes.
/// When `secure` is true, only the first and last 4 characters are shown.
///
/// # Errors
///
/// Returns an error if no token is found, the device flow fails,
/// or the keyring cannot be read.
fn token(hostname: &str, refresh: bool, scopes: &[String], secure: bool) -> anyhow::Result<()> {
    if refresh {
        let host = Host::new(hostname);

        // Build the scopes string from the provided scopes, or use defaults.
        let scope_str = if scopes.is_empty() {
            None
        } else {
            Some(scopes.join(","))
        };

        // Start the OAuth device flow.
        let device_resp = device::request_device_code(&host, scope_str.as_deref())
            .map_err(|e| anyhow::anyhow!("failed to request device code: {e}"))?;

        // Show the user the code and URL.
        device::display_instructions(&device_resp.user_code, &device_resp.verification_uri);

        // Poll for the token.
        let access_token = device::poll_for_token(
            &host,
            &device_resp.device_code,
            device_resp.interval,
            device_resp.expires_in,
        )
        .map_err(|e| anyhow::anyhow!("{e}"))?;

        // Store the new token.
        crate::keyring_store::set_token(hostname, &access_token)
            .map_err(|e| anyhow::anyhow!("failed to store token: {e}"))?;

        // Print the token.
        print_token(&access_token, secure);
        return Ok(());
    }

    // Try to get the token from the keyring first.
    let token = crate::keyring_store::get_token(hostname)
        .map_err(|e| anyhow::anyhow!("failed to read token: {e}"))?
        .or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .or_else(|| std::env::var("GH_TOKEN").ok());

    match token {
        Some(t) => {
            print_token(&t, secure);
            Ok(())
        }
        None => anyhow::bail!(
            "no token found for {hostname}. Run 'gor auth login' first or set GITHUB_TOKEN."
        ),
    }
}

/// Print a token to stdout, optionally masking it for security.
///
/// When `secure` is true, only the first 4 and last 4 characters are shown
/// with `...` in between. If the token is 8 characters or fewer, it is
/// printed in full even in secure mode.
fn print_token(token: &str, secure: bool) {
    if secure && token.len() > 8 {
        let first4 = &token[..4];
        let last4 = &token[token.len() - 4..];
        println!("{first4}...{last4}");
    } else {
        println!("{token}");
    }
}

/// Show the current authentication status.
fn status(hostname: &str) -> anyhow::Result<()> {
    let client = Client::new(hostname)?;
    match client.token() {
        Some(token) => {
            let host = Host::new(hostname);
            match token::verify_token(&host, token) {
                Ok(login) => {
                    println!("Logged in to {hostname} as {login}");
                }
                Err(_) => {
                    println!("Logged in to {hostname} but token is invalid or expired");
                }
            }
        }
        None => {
            println!("Not logged in to {hostname}");
        }
    }
    Ok(())
}
