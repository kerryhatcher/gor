//! Shared helpers for command implementations.
//!
//! Common utilities for pagination, body building, field selection,
//! and other cross-cutting concerns used by multiple commands.

/// Truncates a string to `max_len` characters, appending "…" if truncated.
///
/// # Examples
///
/// ```
/// use gor::cmd::util::truncate;
///
/// assert_eq!(truncate("hello", 10), "hello");
/// assert_eq!(truncate("hello world", 5), "hello…");
/// ```
#[must_use]
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut truncated = s.chars().take(max_len).collect::<String>();
        truncated.push('…');
        truncated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_string() {
        assert_eq!(truncate("hi", 10), "hi");
    }

    #[test]
    fn truncate_exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn truncate_long_string() {
        assert_eq!(truncate("hello world", 5), "hello…");
    }

    #[test]
    fn truncate_empty() {
        assert_eq!(truncate("", 5), "");
    }
}
