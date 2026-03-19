// ============================================================
// E04: Collections and Error Handling (Ch 8-9)
// ============================================================
use std::collections::HashMap;

// --- Exercise 1: Vec operations ---
// Given a vector of integers, return (mean, median, mode).
// Mean as f64, median as f64, mode as i32.
// You may assume the vector is non-empty.
pub fn stats(v: &[i32]) -> (f64, f64, i32) {
    // TODO: Calculate mean, median, and mode.
    // Hint for mode: use a HashMap to count occurrences.
    todo!()
}

// --- Exercise 2: String manipulation ---
// Convert a string to Pig Latin:
// - If starts with a vowel (a, e, i, o, u), append "-hay"
// - Otherwise, move first consonant cluster to end and append "-ay"
// For simplicity, assume lowercase ASCII input, single word.
pub fn pig_latin(word: &str) -> String {
    // TODO
    todo!()
}

// --- Exercise 3: HashMap ---
// Given a list of (department, employee) pairs, build a HashMap
// from department -> sorted Vec of employees.
pub fn department_employees(entries: &[(&str, &str)]) -> HashMap<String, Vec<String>> {
    // TODO
    todo!()
}

// --- Exercise 4: Result and the ? operator ---
// Parse a string like "3,4" into a tuple (i32, i32).
// Return an error if the format is wrong or parsing fails.
#[derive(Debug, PartialEq)]
pub enum ParsePairError {
    MissingComma,
    BadInt(String),
}

pub fn parse_pair(s: &str) -> Result<(i32, i32), ParsePairError> {
    // TODO: Split on comma, parse each half. Use ? or map_err.
    todo!()
}

// --- Exercise 5: Error propagation ---
// Read numbers from a vec of strings, return their sum.
// If any string is not a valid i32, return an error.
pub fn sum_strings(strings: &[&str]) -> Result<i32, String> {
    // TODO: Use ? with map_err to propagate parse errors.
    todo!()
}

// --- Exercise 6: Custom error with From ---
// Combine IO-like and parse errors into a custom error type.
#[derive(Debug)]
pub enum AppError {
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        // TODO
        todo!()
    }
}

// Parse a string to i32. If the number is negative, return Custom error.
pub fn parse_positive(s: &str) -> Result<i32, AppError> {
    // TODO: Use ? to auto-convert ParseIntError. Then check sign.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats() {
        let (mean, median, mode) = stats(&[1, 2, 2, 3, 4]);
        assert!((mean - 2.4).abs() < 1e-6);
        assert_eq!(median, 2.0);
        assert_eq!(mode, 2);
    }

    #[test]
    fn test_pig_latin() {
        assert_eq!(pig_latin("apple"), "apple-hay");
        assert_eq!(pig_latin("first"), "irst-fay");
        assert_eq!(pig_latin("string"), "ing-stray");
    }

    #[test]
    fn test_department_employees() {
        let entries = vec![
            ("Engineering", "Alice"),
            ("Sales", "Bob"),
            ("Engineering", "Charlie"),
            ("Sales", "Alice"),
        ];
        let map = department_employees(&entries);
        assert_eq!(map["Engineering"], vec!["Alice", "Charlie"]);
        assert_eq!(map["Sales"], vec!["Alice", "Bob"]);
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("3,4"), Ok((3, 4)));
        assert_eq!(parse_pair("-1,100"), Ok((-1, 100)));
        assert_eq!(parse_pair("no comma"), Err(ParsePairError::MissingComma));
        assert!(matches!(parse_pair("a,b"), Err(ParsePairError::BadInt(_))));
    }

    #[test]
    fn test_sum_strings() {
        assert_eq!(sum_strings(&["1", "2", "3"]), Ok(6));
        assert!(sum_strings(&["1", "abc", "3"]).is_err());
    }

    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42").unwrap(), 42);
        assert!(matches!(parse_positive("abc"), Err(AppError::Parse(_))));
        assert!(matches!(parse_positive("-5"), Err(AppError::Custom(_))));
    }
}
