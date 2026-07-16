//! JSON and table formatting utilities for `gor`.
//!
//! Provides helpers for printing JSON output with optional field selection,
//! date formatting, and number formatting. All output functions print to
//! stdout and are intended for use by command implementations.

#![allow(clippy::print_stdout)]

use serde::Serialize;
use serde_json::Value;

/// Print a serializable value as pretty JSON to stdout.
///
/// If `fields` is `Some`, only the specified top-level fields are included
/// in the output. If `fields` is `None`, the entire value is printed.
///
/// # Panics
///
/// Panics if the value cannot be serialized to JSON (this should never
/// happen for well-formed data).
///
/// # Examples
///
/// ```no_run
/// use gor::output::print_json;
/// use serde_json::json;
///
/// let data = json!({"name": "hello-world", "stars": 42});
/// print_json(&data, None::<&[String]>);
/// ```
#[allow(clippy::expect_used)]
pub fn print_json<T: Serialize>(value: &T, fields: Option<&[String]>) {
    let json_value = serde_json::to_value(value).expect("value must be serializable to JSON");

    let output = match fields {
        Some(field_list) => {
            if field_list.is_empty() {
                json_value
            } else {
                let mut filtered = serde_json::Map::new();
                if let Value::Object(map) = &json_value {
                    for field in field_list {
                        if let Some(val) = map.get(field) {
                            filtered.insert(field.clone(), val.clone());
                        }
                    }
                }
                Value::Object(filtered)
            }
        }
        None => json_value,
    };

    let pretty = serde_json::to_string_pretty(&output).expect("value must be pretty-printable");
    println!("{pretty}");
}

/// Format an ISO 8601 date string into a human-readable form.
///
/// Input format: `2024-01-15T10:30:00Z` or similar ISO 8601.
/// Output format: `Jan 15, 2024`.
///
/// If the input cannot be parsed, the original string is returned.
///
/// # Examples
///
/// ```
/// use gor::output::format_date;
///
/// assert_eq!(format_date("2024-01-15T10:30:00Z"), "Jan 15, 2024");
/// assert_eq!(format_date("2023-12-25T00:00:00Z"), "Dec 25, 2023");
/// ```
#[must_use]
pub fn format_date(iso_date: &str) -> String {
    // Try to parse the date portion (first 10 characters: YYYY-MM-DD)
    let date_part = iso_date.get(..10).unwrap_or(iso_date);

    // Parse year, month, day
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() != 3 {
        return iso_date.to_string();
    }

    let year = parts[0];
    let month: u32 = parts[1].parse().unwrap_or(0);
    let day = parts[2];

    let month_name = match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => return iso_date.to_string(),
    };

    // Strip leading zeros from day
    let day_stripped = day.trim_start_matches('0');

    format!("{month_name} {day_stripped}, {year}")
}

/// Format a number with comma separators for thousands.
///
/// # Examples
///
/// ```
/// use gor::output::format_count;
///
/// assert_eq!(format_count(0), "0");
/// assert_eq!(format_count(42), "42");
/// assert_eq!(format_count(1_234), "1,234");
/// assert_eq!(format_count(1_000_000), "1,000,000");
/// ```
#[must_use]
pub fn format_count(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);

    for (i, c) in s.chars().enumerate() {
        // Insert a comma every 3 digits from the right
        if i > 0 && (s.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    result
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_json_full_output() {
        let data = json!({"name": "hello-world", "stars": 42, "forks": 7});
        // Just verify it doesn't panic
        print_json(&data, None::<&[String]>);
    }

    #[test]
    fn print_json_with_fields() {
        let data = json!({"name": "hello-world", "stars": 42, "forks": 7});
        let fields = vec!["name".to_string(), "stars".to_string()];
        // Just verify it doesn't panic
        print_json(&data, Some(&fields));
    }

    #[test]
    fn print_json_with_empty_fields() {
        let data = json!({"name": "hello-world", "stars": 42});
        let fields: Vec<String> = vec![];
        // Empty fields list should print everything
        print_json(&data, Some(&fields));
    }

    #[test]
    fn print_json_with_missing_field() {
        let data = json!({"name": "hello-world"});
        let fields = vec!["name".to_string(), "missing".to_string()];
        // Missing fields are silently omitted
        print_json(&data, Some(&fields));
    }

    #[test]
    fn format_date_standard() {
        assert_eq!(format_date("2024-01-15T10:30:00Z"), "Jan 15, 2024");
    }

    #[test]
    fn format_date_december() {
        assert_eq!(format_date("2023-12-25T00:00:00Z"), "Dec 25, 2023");
    }

    #[test]
    fn format_date_single_digit_day() {
        assert_eq!(format_date("2024-03-05T12:00:00Z"), "Mar 5, 2024");
    }

    #[test]
    fn format_date_invalid() {
        assert_eq!(format_date("not-a-date"), "not-a-date");
    }

    #[test]
    fn format_date_empty() {
        assert_eq!(format_date(""), "");
    }

    #[test]
    fn format_date_bad_month() {
        assert_eq!(format_date("2024-13-01"), "2024-13-01");
    }

    #[test]
    fn format_count_zero() {
        assert_eq!(format_count(0), "0");
    }

    #[test]
    fn format_count_small() {
        assert_eq!(format_count(42), "42");
    }

    #[test]
    fn format_count_thousands() {
        assert_eq!(format_count(1_234), "1,234");
    }

    #[test]
    fn format_count_millions() {
        assert_eq!(format_count(1_000_000), "1,000,000");
    }

    #[test]
    fn format_count_billions() {
        assert_eq!(format_count(1_234_567_890), "1,234,567,890");
    }

    #[test]
    fn format_count_ten_thousands() {
        assert_eq!(format_count(10_000), "10,000");
    }

    #[test]
    fn format_count_hundred_thousands() {
        assert_eq!(format_count(100_000), "100,000");
    }
}
