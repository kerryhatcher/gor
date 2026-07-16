//! Implementation of the `gor auth` subcommand.
//!
//! Handles login (OAuth device flow), logout, and status.

#![allow(clippy::print_stdout)]

use crate::auth::{device, token};
use crate::cli::AuthCommand;
use crate::client::Client;
use crate::host::Host;

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
    }
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
