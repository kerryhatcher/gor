//! OAuth device flow implementation.
//!
//! Implements the GitHub OAuth device flow as described in
//! <https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps#device-flow>.
//!
//! The flow:
//! 1. Request a device code from `POST /login/device/code`
//! 2. Show the user a one-time code and URL
//! 3. Poll `POST /login/oauth/access_token` until the user completes authorization
//! 4. Return the access token

use crate::error::GorError;
use crate::host::Host;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};

/// The OAuth client ID for the `gor` CLI app.
///
/// This is a public client ID registered for the device flow.
/// It follows GitHub's guidance for CLI tools.
const CLIENT_ID: &str = "Iv23liqA1L1VQn1AOc1s";

/// Response from `POST /login/device/code`.
#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    /// The device verification code (shown to the user).
    pub user_code: String,
    /// The device code (used for polling).
    pub device_code: String,
    /// The URL the user should visit to enter the code.
    pub verification_uri: String,
    /// The polling interval in seconds.
    pub interval: u64,
    /// The lifetime of the device code in seconds.
    #[serde(default)]
    pub expires_in: u64,
}

/// Response from `POST /login/oauth/access_token` (success).
#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    /// The OAuth access token.
    pub access_token: String,
    /// The token type (should be `bearer`).
    #[allow(dead_code)]
    pub token_type: String,
    /// The scopes granted.
    #[allow(dead_code)]
    #[serde(default)]
    pub scope: String,
}

/// Error response from `POST /login/oauth/access_token`.
#[derive(Debug, Deserialize)]
struct OAuthError {
    error: String,
    #[allow(dead_code)]
    error_description: Option<String>,
}

/// Request a device code from GitHub.
///
/// # Errors
///
/// Returns an error if the HTTP request fails or GitHub returns an error.
pub fn request_device_code(
    host: &Host,
    scopes: Option<&str>,
) -> Result<DeviceCodeResponse, GorError> {
    let client = reqwest::blocking::Client::new();
    let url = host.device_code_url();

    let mut params = HashMap::new();
    params.insert("client_id", CLIENT_ID);
    let default_scopes = "repo,read:org,workflow,gist";
    let scope_str = scopes.unwrap_or(default_scopes);
    params.insert("scope", scope_str);

    tracing::info!("Requesting device code from {url}");
    let response = client
        .post(&url)
        .header("Accept", "application/json")
        .json(&params)
        .send()
        .map_err(GorError::Http)?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().unwrap_or_default();
        return Err(GorError::Auth(format!(
            "device code request failed ({status}): {body}"
        )));
    }

    response
        .json()
        .map_err(|e| GorError::Auth(format!("failed to parse device code response: {e}")))
}

/// Poll for the access token until the user completes authorization or the flow times out.
///
/// Displays a spinner and status messages to stderr while polling.
///
/// # Errors
///
/// Returns [`GorError::DeviceTimeout`] if the user doesn't authorize within the timeout.
/// Returns [`GorError::DeviceDeclined`] if the user declines authorization.
/// Returns [`GorError::Auth`] for other errors.
pub fn poll_for_token(
    host: &Host,
    device_code: &str,
    interval: u64,
    expires_in: u64,
) -> Result<String, GorError> {
    let client = reqwest::blocking::Client::new();
    let url = host.access_token_url();
    let deadline = Instant::now() + Duration::from_secs(expires_in);
    let poll_interval = Duration::from_secs(interval);

    let mut params = HashMap::new();
    params.insert("client_id", CLIENT_ID);
    params.insert("device_code", device_code);
    params.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");

    loop {
        if Instant::now() > deadline {
            return Err(GorError::DeviceTimeout(
                "timed out waiting for authorization".to_string(),
            ));
        }

        let response = client
            .post(&url)
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .map_err(GorError::Http)?;

        let status = response.status();

        if status.is_success() {
            let token_resp: AccessTokenResponse = response.json().map_err(|e| {
                GorError::Auth(format!("failed to parse access token response: {e}"))
            })?;
            return Ok(token_resp.access_token);
        }

        // Check for specific OAuth errors
        let error_body: OAuthError = response.json().unwrap_or_else(|_| OAuthError {
            error: "unknown".to_string(),
            error_description: None,
        });

        match error_body.error.as_str() {
            "authorization_pending" => {
                // User hasn't completed auth yet — keep polling.
                tracing::debug!("Authorization pending, waiting {poll_interval:?}");
                std::thread::sleep(poll_interval);
            }
            "slow_down" => {
                // GitHub asks us to slow down — increase interval.
                tracing::debug!("Slowing down polling");
                std::thread::sleep(poll_interval + Duration::from_secs(5));
            }
            "expired_token" => {
                return Err(GorError::DeviceTimeout(
                    "device code expired before authorization".to_string(),
                ));
            }
            "access_denied" => {
                return Err(GorError::DeviceDeclined);
            }
            other => {
                return Err(GorError::Auth(format!(
                    "OAuth error during polling: {other}"
                )));
            }
        }
    }
}

/// Display the device activation instructions to the user.
///
/// Prints the one-time code and activation URL to stderr.
#[allow(clippy::print_stderr)]
pub fn display_instructions(user_code: &str, verification_uri: &str) {
    let msg = format!("Open {verification_uri} and enter the following code:\n\n  {user_code}\n");
    // Write directly to stderr for immediate display.
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    let _ = writeln!(handle, "{msg}");
    let _ = handle.flush();
}
