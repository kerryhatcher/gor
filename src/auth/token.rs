//! Token verification and stdin reading utilities.
//!
//! Provides functions to verify that an OAuth token is valid by calling
//! `GET /user`, and to read tokens from stdin (for piping).

use crate::error::GorError;
use crate::host::Host;
use serde::Deserialize;

/// Minimal user response from `GET /user` for token verification.
#[derive(Debug, Deserialize)]
pub struct UserResponse {
    /// The authenticated user's login name.
    pub login: String,
}

/// Verify that an OAuth token is valid by calling `GET /user`.
///
/// Returns the authenticated user's login name on success.
///
/// # Errors
///
/// Returns [`GorError::Auth`] if the token is invalid or the request fails.
///
/// # Examples
///
/// ```no_run
/// use gor::auth::token::verify_token;
/// use gor::host::Host;
///
/// let host = Host::new("github.com");
/// let login = verify_token(&host, "gho_abc123").unwrap();
/// println!("Authenticated as {login}");
/// ```
pub fn verify_token(host: &Host, token: &str) -> Result<String, GorError> {
    let client = reqwest::blocking::Client::new();
    let url = host.api_url("/user");

    tracing::info!("Verifying token with GET /user");
    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", concat!("gor/", env!("CARGO_PKG_VERSION")))
        .send()
        .map_err(GorError::Http)?;

    let status = response.status();
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(GorError::Auth(
            "token is invalid or expired (HTTP 401)".to_string(),
        ));
    }
    if status == reqwest::StatusCode::FORBIDDEN {
        return Err(GorError::Auth(
            "token lacks required permissions (HTTP 403)".to_string(),
        ));
    }
    if !status.is_success() {
        let body = response.text().unwrap_or_default();
        return Err(GorError::Auth(format!(
            "token verification failed ({status}): {body}"
        )));
    }

    let user: UserResponse = response
        .json()
        .map_err(|e| GorError::Auth(format!("failed to parse user response: {e}")))?;

    Ok(user.login)
}

/// Read a token from stdin, trimming whitespace.
///
/// Used for piping tokens: `echo gho_abc | gor auth login --with-token`
///
/// # Errors
///
/// Returns an I/O error if stdin cannot be read.
///
/// # Examples
///
/// ```no_run
/// use gor::auth::token::read_token_from_stdin;
///
/// let token = read_token_from_stdin().unwrap();
/// ```
pub fn read_token_from_stdin() -> Result<String, GorError> {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .map_err(GorError::Io)?;
    Ok(buffer.trim().to_string())
}
