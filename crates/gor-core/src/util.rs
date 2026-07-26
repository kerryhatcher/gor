//! Shared utility functions for gor-core.

/// URL-encode a label name for use in API paths.
///
/// GitHub API paths can contain special characters in label names
/// that must be percent-encoded.
#[must_use]
pub fn urlencode_label_name(s: &str) -> String {
    s.replace('#', "%23")
        .replace(' ', "%20")
        .replace('?', "%3F")
        .replace('&', "%26")
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn urlencode_normal() {
        assert_eq!(urlencode_label_name("bug"), "bug");
    }

    #[test]
    fn urlencode_with_hash() {
        assert_eq!(urlencode_label_name("bug#1"), "bug%231");
    }

    #[test]
    fn urlencode_with_space() {
        assert_eq!(urlencode_label_name("in progress"), "in%20progress");
    }

    #[test]
    fn urlencode_with_question_mark() {
        assert_eq!(urlencode_label_name("where?"), "where%3F");
    }

    #[test]
    fn urlencode_with_ampersand() {
        assert_eq!(urlencode_label_name("R&D"), "R%26D");
    }

    #[test]
    fn urlencode_combined() {
        assert_eq!(
            urlencode_label_name("bug #1?fix&go"),
            "bug%20%231%3Ffix%26go"
        );
    }
}
