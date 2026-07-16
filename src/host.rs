//! Multi-host support for `gor`.
//!
//! Derives REST API base URLs for github.com and GitHub Enterprise Server
//! instances. All URL construction flows through [`Host`] to ensure consistent
//! host handling across the codebase.

/// Represents a GitHub host (github.com or a GHES instance).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    /// The hostname, e.g. `github.com` or `git.example.com`.
    hostname: String,
    /// The REST API base URL for this host.
    api_base: String,
}

impl Host {
    /// Create a new `Host` for the given hostname.
    ///
    /// For `github.com`, the API base is `https://api.github.com`.
    /// For GHES instances, the API base is `https://<hostname>/api/v3`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gor::host::Host;
    ///
    /// let host = Host::new("github.com");
    /// assert_eq!(host.api_base(), "https://api.github.com");
    ///
    /// let ghes = Host::new("git.example.com");
    /// assert_eq!(ghes.api_base(), "https://git.example.com/api/v3");
    /// ```
    #[must_use]
    pub fn new(hostname: &str) -> Self {
        let api_base = if hostname == "github.com" {
            "https://api.github.com".to_string()
        } else {
            format!("https://{hostname}/api/v3")
        };
        Self {
            hostname: hostname.to_string(),
            api_base,
        }
    }

    /// Returns the hostname (e.g. `github.com`).
    #[must_use]
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    /// Returns the REST API base URL (e.g. `https://api.github.com`).
    #[must_use]
    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    /// Build a full API URL by joining the API base with the given path.
    ///
    /// The path should start with a `/`, e.g. `/user` or `/repos/owner/repo`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gor::host::Host;
    ///
    /// let host = Host::new("github.com");
    /// assert_eq!(host.api_url("/user"), "https://api.github.com/user");
    /// ```
    #[must_use]
    pub fn api_url(&self, path: &str) -> String {
        format!("{}{path}", self.api_base)
    }

    /// Build the OAuth device code URL for this host.
    ///
    /// # Examples
    ///
    /// ```
    /// use gor::host::Host;
    ///
    /// let host = Host::new("github.com");
    /// assert_eq!(
    ///     host.device_code_url(),
    ///     "https://github.com/login/device/code"
    /// );
    /// ```
    #[must_use]
    pub fn device_code_url(&self) -> String {
        format!("https://{}/login/device/code", self.hostname)
    }

    /// Build the OAuth access token URL for this host.
    ///
    /// # Examples
    ///
    /// ```
    /// use gor::host::Host;
    ///
    /// let host = Host::new("github.com");
    /// assert_eq!(
    ///     host.access_token_url(),
    ///     "https://github.com/login/oauth/access_token"
    /// );
    /// ```
    #[must_use]
    pub fn access_token_url(&self) -> String {
        format!("https://{}/login/oauth/access_token", self.hostname)
    }

    /// Build the device activation URL shown to the user.
    ///
    /// # Examples
    ///
    /// ```
    /// use gor::host::Host;
    ///
    /// let host = Host::new("github.com");
    /// assert_eq!(host.device_activation_url(), "https://github.com/login/device");
    /// ```
    #[must_use]
    pub fn device_activation_url(&self) -> String {
        format!("https://{}/login/device", self.hostname)
    }
}

impl Default for Host {
    fn default() -> Self {
        Self::new("github.com")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_github_dot_com() {
        let host = Host::default();
        assert_eq!(host.hostname(), "github.com");
        assert_eq!(host.api_base(), "https://api.github.com");
    }

    #[test]
    fn github_api_url() {
        let host = Host::new("github.com");
        assert_eq!(host.api_url("/user"), "https://api.github.com/user");
        assert_eq!(
            host.api_url("/repos/owner/repo"),
            "https://api.github.com/repos/owner/repo"
        );
    }

    #[test]
    fn ghes_api_url() {
        let host = Host::new("git.example.com");
        assert_eq!(host.api_base(), "https://git.example.com/api/v3");
        assert_eq!(host.api_url("/user"), "https://git.example.com/api/v3/user");
    }

    #[test]
    fn device_code_url_github() {
        let host = Host::new("github.com");
        assert_eq!(
            host.device_code_url(),
            "https://github.com/login/device/code"
        );
    }

    #[test]
    fn device_code_url_ghes() {
        let host = Host::new("git.example.com");
        assert_eq!(
            host.device_code_url(),
            "https://git.example.com/login/device/code"
        );
    }

    #[test]
    fn access_token_url_github() {
        let host = Host::new("github.com");
        assert_eq!(
            host.access_token_url(),
            "https://github.com/login/oauth/access_token"
        );
    }

    #[test]
    fn device_activation_url() {
        let host = Host::new("github.com");
        assert_eq!(
            host.device_activation_url(),
            "https://github.com/login/device"
        );
    }
}
