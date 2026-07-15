//! Error types for the `gor` library.
//!
//! Uses [`thiserror`] for ergonomic error definitions with proper
//! `Display` and [`std::error::Error`] implementations.

use thiserror::Error;

/// Top-level error type for all `gor` operations.
///
/// Each variant corresponds to a distinct failure mode. Library code
/// returns these errors; command code wraps them with [`anyhow`].
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GorError {
    /// An HTTP request to the GitHub API failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Authentication failed (invalid or expired token).
    #[error("authentication failed: {0}")]
    Auth(String),

    /// The requested resource was not found (404).
    #[error("not found: {0}")]
    NotFound(String),

    /// Rate limit exceeded (429 or secondary rate limit).
    #[error("rate limit exceeded: {0}")]
    RateLimit(String),

    /// Invalid input from the user (bad repo spec, etc.).
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
