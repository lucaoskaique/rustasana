use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};

/// Parse date string into YYYY-MM-DD format
/// Supports "today", "tomorrow", or explicit YYYY-MM-DD dates
pub fn parse_date(date_str: &str) -> Result<String> {
    match date_str.to_lowercase().as_str() {
        "today" => Ok(Local::now().format("%Y-%m-%d").to_string()),
        "tomorrow" => Ok((Local::now() + chrono::Duration::days(1))
            .format("%Y-%m-%d")
            .to_string()),
        _ => {
            // Validate the date format
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .context("Invalid date format. Use YYYY-MM-DD, 'today', or 'tomorrow'")?;
            Ok(date_str.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_explicit_date() {
        assert_eq!(parse_date("2024-12-25").unwrap(), "2024-12-25");
    }

    #[test]
    fn test_parse_today() {
        let result = parse_date("today").unwrap();
        assert!(result.starts_with("20")); // Year starts with 20
        assert_eq!(result.len(), 10); // YYYY-MM-DD format
    }

    #[test]
    fn test_invalid_date() {
        assert!(parse_date("not-a-date").is_err());
        assert!(parse_date("2024-13-01").is_err()); // Invalid month
    }
}
