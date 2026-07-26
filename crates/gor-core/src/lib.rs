//! # gor-core — Core library for the gor GitHub CLI
//!
//! Provides typed operations, client infrastructure, and domain models
//! for the GitHub REST API. Used by the `gor` CLI binary and available
//! as a reusable library for other Rust applications.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod host;
pub mod keyring_store;
pub mod repository;

pub mod label;
pub mod util;

/// Convenience re-exports of key types.
pub use client::Client;
pub use error::GorError;
pub use repository::RepoSplit;
