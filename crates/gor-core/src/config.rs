//! Configuration file management for `gor`.
//!
//! Reads and writes `~/.config/gor/config.yml` (mode 0600).
//! Supports host-scoped keys via a top-level `hosts` map.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Top-level configuration structure stored in `~/.config/gor/config.yml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GorConfig {
    /// Global config keys (editor, browser, pager, git_protocol, prompt).
    #[serde(flatten)]
    pub global: BTreeMap<String, serde_yaml_ng::Value>,
    /// Host-scoped configuration overrides.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub hosts: BTreeMap<String, BTreeMap<String, serde_yaml_ng::Value>>,
}

/// Supported top-level config keys and their validation rules.
pub const SUPPORTED_KEYS: &[&str] = &["editor", "browser", "pager", "git_protocol", "prompt"];

/// Returns the path to the config file: `~/.config/gor/config.yml`.
///
/// # Errors
///
/// Returns an error if the home directory cannot be determined.
pub fn config_path() -> Result<PathBuf, ConfigError> {
    let base = dirs::config_dir().ok_or(ConfigError::NoHome)?;
    Ok(base.join("gor").join("config.yml"))
}

/// Load the config from disk. Returns a default empty config if the file does not exist.
///
/// # Errors
///
/// Returns an I/O error if the file exists but cannot be read, or if the YAML is malformed.
pub fn load() -> Result<GorConfig, ConfigError> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(GorConfig::default());
    }
    let contents = std::fs::read_to_string(&path).map_err(|e| ConfigError::Read {
        path: path.clone(),
        source: e,
    })?;
    if contents.trim().is_empty() {
        return Ok(GorConfig::default());
    }
    serde_yaml_ng::from_str(&contents).map_err(|e| ConfigError::Parse { path, source: e })
}

/// Save the config to disk, creating parent directories and setting mode 0600.
///
/// # Errors
///
/// Returns an I/O error if the file cannot be written.
pub fn save(config: &GorConfig) -> Result<(), ConfigError> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| ConfigError::Write {
            path: path.clone(),
            source: e,
        })?;
    }
    let yaml =
        serde_yaml_ng::to_string(config).map_err(|e| ConfigError::Serialize { source: e })?;
    std::fs::write(&path, &yaml).map_err(|e| ConfigError::Write {
        path: path.clone(),
        source: e,
    })?;
    set_restrictive_perms(&path)?;
    Ok(())
}

/// Set file permissions to 0600 (owner read/write only).
#[cfg(unix)]
fn set_restrictive_perms(path: &std::path::Path) -> Result<(), ConfigError> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .map_err(|e| ConfigError::Write {
            path: path.to_path_buf(),
            source: e,
        })?
        .permissions();
    perms.set_mode(0o600);
    std::fs::set_permissions(path, perms).map_err(|e| ConfigError::Write {
        path: path.to_path_buf(),
        source: e,
    })?;
    Ok(())
}

#[cfg(not(unix))]
fn set_restrictive_perms(_path: &std::path::Path) -> Result<(), ConfigError> {
    // On non-Unix platforms, we rely on the OS default permissions.
    Ok(())
}

/// Get a config value, checking host-scoped overrides first.
///
/// If `host` is provided, checks `hosts.<host>.<key>` first, then falls back to the global key.
#[must_use]
pub fn get<'a>(
    config: &'a GorConfig,
    key: &str,
    host: Option<&str>,
) -> Option<&'a serde_yaml_ng::Value> {
    if let Some(h) = host {
        if let Some(host_config) = config.hosts.get(h) {
            if let Some(value) = host_config.get(key) {
                return Some(value);
            }
        }
    }
    config.global.get(key)
}

/// Set a config value. If `host` is provided, sets it under `hosts.<host>.<key>`.
pub fn set(config: &mut GorConfig, key: &str, value: serde_yaml_ng::Value, host: Option<&str>) {
    if let Some(h) = host {
        config
            .hosts
            .entry(h.to_string())
            .or_default()
            .insert(key.to_string(), value);
    } else {
        config.global.insert(key.to_string(), value);
    }
}

/// Validate a config key and value combination.
///
/// # Errors
///
/// Returns an error string if the key is unknown or the value is invalid.
pub fn validate(key: &str, value: &str) -> Result<(), String> {
    if !SUPPORTED_KEYS.contains(&key) {
        return Err(format!(
            "unknown config key '{key}'. Supported keys: {}",
            SUPPORTED_KEYS.join(", ")
        ));
    }
    if key == "git_protocol" && !matches!(value, "https" | "ssh") {
        return Err(format!(
            "invalid value '{value}' for git_protocol: must be 'https' or 'ssh'"
        ));
    }
    Ok(())
}

/// Errors that can occur during config file operations.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Could not determine the home directory.
    #[error("could not determine config directory")]
    NoHome,
    /// Failed to read the config file.
    #[error("failed to read config file {path}: {source}")]
    Read {
        /// Path to the config file.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// Failed to parse the config file as YAML.
    #[error("failed to parse config file {path}: {source}")]
    Parse {
        /// Path to the config file.
        path: PathBuf,
        /// Underlying YAML parse error.
        source: serde_yaml_ng::Error,
    },
    /// Failed to write the config file.
    #[error("failed to write config file {path}: {source}")]
    Write {
        /// Path to the config file.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// Failed to serialize the config to YAML.
    #[error("failed to serialize config: {source}")]
    Serialize {
        /// Underlying YAML serialization error.
        source: serde_yaml_ng::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_empty() {
        let config = GorConfig::default();
        assert!(config.global.is_empty());
        assert!(config.hosts.is_empty());
    }

    #[test]
    fn get_global_key() {
        let mut config = GorConfig::default();
        config.global.insert(
            "editor".to_string(),
            serde_yaml_ng::Value::String("vim".to_string()),
        );
        let value = get(&config, "editor", None);
        assert_eq!(value.and_then(|v| v.as_str()), Some("vim"));
    }

    #[test]
    fn get_host_scoped_key_falls_back_to_global() {
        let mut config = GorConfig::default();
        config.global.insert(
            "editor".to_string(),
            serde_yaml_ng::Value::String("vim".to_string()),
        );
        // No host override — should fall back to global.
        let value = get(&config, "editor", Some("github.com"));
        assert_eq!(value.and_then(|v| v.as_str()), Some("vim"));
    }

    #[test]
    fn get_host_scoped_key_overrides_global() {
        let mut config = GorConfig::default();
        config.global.insert(
            "editor".to_string(),
            serde_yaml_ng::Value::String("vim".to_string()),
        );
        config.hosts.insert(
            "github.com".to_string(),
            BTreeMap::from([(
                "editor".to_string(),
                serde_yaml_ng::Value::String("code".to_string()),
            )]),
        );
        let value = get(&config, "editor", Some("github.com"));
        assert_eq!(value.and_then(|v| v.as_str()), Some("code"));
    }

    #[test]
    fn set_global_key() {
        let mut config = GorConfig::default();
        set(
            &mut config,
            "editor",
            serde_yaml_ng::Value::String("vim".to_string()),
            None,
        );
        assert_eq!(
            config.global.get("editor").and_then(|v| v.as_str()),
            Some("vim")
        );
    }

    #[test]
    fn set_host_scoped_key() {
        let mut config = GorConfig::default();
        set(
            &mut config,
            "editor",
            serde_yaml_ng::Value::String("code".to_string()),
            Some("github.com"),
        );
        assert_eq!(
            config
                .hosts
                .get("github.com")
                .and_then(|h| h.get("editor"))
                .and_then(|v| v.as_str()),
            Some("code")
        );
    }

    #[test]
    fn validate_known_key() {
        assert!(validate("editor", "vim").is_ok());
    }

    #[test]
    fn validate_unknown_key() {
        assert!(validate("unknown_key", "value").is_err());
    }

    #[test]
    fn validate_git_protocol_https() {
        assert!(validate("git_protocol", "https").is_ok());
    }

    #[test]
    fn validate_git_protocol_ssh() {
        assert!(validate("git_protocol", "ssh").is_ok());
    }

    #[test]
    fn validate_git_protocol_invalid() {
        assert!(validate("git_protocol", "ftp").is_err());
    }

    #[test]
    fn supported_keys_list() {
        assert!(SUPPORTED_KEYS.contains(&"editor"));
        assert!(SUPPORTED_KEYS.contains(&"browser"));
        assert!(SUPPORTED_KEYS.contains(&"pager"));
        assert!(SUPPORTED_KEYS.contains(&"git_protocol"));
        assert!(SUPPORTED_KEYS.contains(&"prompt"));
    }
}
