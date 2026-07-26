//! Secure token storage for `gor`.
//!
//! Stores OAuth tokens in `~/.config/gor/hosts.yml` (mode 0600).
//! Tokens are stored per-host, keyed by hostname.
//!
//! When the `keyring` feature is enabled, the OS keyring is used
//! as the primary store with the hosts file as fallback.

#![allow(clippy::missing_const_for_thread_local)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::error::GorError;

/// Per-host token storage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HostEntry {
    /// The OAuth access token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The authenticated user's login name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Top-level hosts file structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HostsFile {
    /// Host-scoped entries keyed by hostname.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub hosts: BTreeMap<String, HostEntry>,
}

/// Returns the path to the hosts file: `~/.config/gor/hosts.yml`.
fn hosts_path() -> Result<PathBuf, GorError> {
    // Allow test override via a thread-local.
    #[cfg(test)]
    {
        let override_path = TEST_PATH.with(|cell| cell.borrow().clone());
        if let Some(path) = override_path {
            return Ok(path);
        }
    }
    let base = dirs::config_dir().ok_or(GorError::Keyring(
        "could not determine config directory".to_string(),
    ))?;
    Ok(base.join("gor").join("hosts.yml"))
}

#[cfg(test)]
std::thread_local! {
    static TEST_PATH: std::cell::RefCell<Option<PathBuf>> = std::cell::RefCell::new(None);
}

/// Load the hosts file from disk.
fn load_hosts() -> Result<HostsFile, GorError> {
    let path = hosts_path()?;
    if !path.exists() {
        return Ok(HostsFile::default());
    }
    let contents = std::fs::read_to_string(&path).map_err(|e| {
        GorError::Keyring(format!(
            "failed to read hosts file {path}: {e}",
            path = path.display()
        ))
    })?;
    if contents.trim().is_empty() {
        return Ok(HostsFile::default());
    }
    serde_yaml_ng::from_str(&contents).map_err(|e| {
        GorError::Keyring(format!(
            "failed to parse hosts file {path}: {e}",
            path = path.display()
        ))
    })
}

/// Save the hosts file to disk with restrictive permissions.
fn save_hosts(hosts: &HostsFile) -> Result<(), GorError> {
    let path = hosts_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| GorError::Keyring(format!("failed to create config directory: {e}")))?;
    }
    let yaml = serde_yaml_ng::to_string(hosts)
        .map_err(|e| GorError::Keyring(format!("failed to serialize hosts file: {e}")))?;
    std::fs::write(&path, &yaml).map_err(|e| {
        GorError::Keyring(format!(
            "failed to write hosts file {path}: {e}",
            path = path.display()
        ))
    })?;
    set_restrictive_perms(&path)?;
    Ok(())
}

/// Set file permissions to 0600 (owner read/write only).
#[cfg(unix)]
fn set_restrictive_perms(path: &std::path::Path) -> Result<(), GorError> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .map_err(|e| {
            GorError::Keyring(format!(
                "failed to stat hosts file {path}: {e}",
                path = path.display()
            ))
        })?
        .permissions();
    perms.set_mode(0o600);
    std::fs::set_permissions(path, perms).map_err(|e| {
        GorError::Keyring(format!(
            "failed to set permissions on hosts file {path}: {e}",
            path = path.display()
        ))
    })?;
    Ok(())
}

#[cfg(not(unix))]
fn set_restrictive_perms(_path: &std::path::Path) -> Result<(), GorError> {
    Ok(())
}

/// Store an OAuth token for the given host.
///
/// # Errors
///
/// Returns [`GorError::Keyring`] if the file cannot be written.
pub fn set_token(host: &str, token: &str) -> Result<(), GorError> {
    let mut hosts = load_hosts()?;
    let entry = hosts.hosts.entry(host.to_string()).or_default();
    entry.token = Some(token.to_string());
    save_hosts(&hosts)
}

/// Retrieve an OAuth token for the given host.
///
/// Returns `Ok(None)` if no token is stored for this host.
///
/// # Errors
///
/// Returns [`GorError::Keyring`] if the file cannot be read.
pub fn get_token(host: &str) -> Result<Option<String>, GorError> {
    let hosts = load_hosts()?;
    Ok(hosts.hosts.get(host).and_then(|entry| entry.token.clone()))
}

/// Delete an OAuth token for the given host.
///
/// # Errors
///
/// Returns [`GorError::Keyring`] if the file cannot be written.
pub fn delete_token(host: &str) -> Result<(), GorError> {
    let mut hosts = load_hosts()?;
    if let Some(entry) = hosts.hosts.get_mut(host) {
        entry.token = None;
        entry.user = None;
    }
    save_hosts(&hosts)
}

/// Store the authenticated user's login name for the given host.
///
/// # Errors
///
/// Returns [`GorError::Keyring`] if the file cannot be written.
pub fn set_user(host: &str, user: &str) -> Result<(), GorError> {
    let mut hosts = load_hosts()?;
    let entry = hosts.hosts.entry(host.to_string()).or_default();
    entry.user = Some(user.to_string());
    save_hosts(&hosts)
}

/// Retrieve the stored user login for the given host.
///
/// # Errors
///
/// Returns [`GorError::Keyring`] if the file cannot be read.
pub fn get_user(host: &str) -> Result<Option<String>, GorError> {
    let hosts = load_hosts()?;
    Ok(hosts.hosts.get(host).and_then(|entry| entry.user.clone()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// Helper to run a test with an isolated hosts file.
    fn with_temp_hosts<F: FnOnce()>(f: F) {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("gor-test-{}-{}", std::process::id(), id));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("hosts.yml");
        let _ = std::fs::remove_file(&path);
        TEST_PATH.with(|cell| {
            *cell.borrow_mut() = Some(path);
        });
        f();
        TEST_PATH.with(|cell| {
            *cell.borrow_mut() = None;
        });
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn get_token_no_entry_returns_none() {
        with_temp_hosts(|| {
            let result = get_token("gor-test-nonexistent-host.invalid");
            assert!(result.is_ok());
            assert!(result.unwrap().is_none());
        });
    }

    #[test]
    fn set_and_get_token() {
        with_temp_hosts(|| {
            let host = "gor-test-token.invalid";
            set_token(host, "test-token-123").unwrap();
            let token = get_token(host).unwrap();
            assert_eq!(token, Some("test-token-123".to_string()));
        });
    }

    #[test]
    fn delete_token_removes_entry() {
        with_temp_hosts(|| {
            let host = "gor-test-delete.invalid";
            set_token(host, "test-token-456").unwrap();
            delete_token(host).unwrap();
            let token = get_token(host).unwrap();
            assert!(token.is_none());
        });
    }

    #[test]
    fn set_and_get_user() {
        with_temp_hosts(|| {
            let host = "gor-test-user.invalid";
            set_user(host, "testuser").unwrap();
            let user = get_user(host).unwrap();
            assert_eq!(user, Some("testuser".to_string()));
        });
    }
}
