#![doc(hidden)]

use std::convert::TryFrom;

use proptest::prelude::*;
use proptest::test_runner::{Config, FileFailurePersistence};

use typeid_prefix::prelude::*;

mod proofs;

proptest! {
        #![proptest_config(Config {
        cases:1000,
        failure_persistence: Some(Box::new(FileFailurePersistence::Off)),
        .. Config::default()
    })]
    #[test]
    fn test_typeidprefix_try_from_and_sanitize(input in "\\PC*") {
        // println!("Running test with input: {:?}", input); // Print each test input
        let try_from_result = TypeIdPrefix::try_from(input.clone());
        let sanitized = input.create_prefix_sanitized();

        // Test TypeIdPrefix::try_from
        if input.len() > 63 {
            prop_assert!(try_from_result.is_err());
        } else if input.is_empty() {
            prop_assert_eq!(try_from_result.unwrap_err(), ValidationError::IsEmpty);
        } else {
            let is_ascii = input.is_ascii();
            let starts_with_valid_char = input.chars().next().is_some_and(|c| c.is_ascii_lowercase());
            let ends_with_valid_char = input.chars().last().is_some_and(|c| c.is_ascii_lowercase());
            let contains_only_valid_chars = input.chars().all(|c| c.is_ascii_lowercase() || c == '_');
            if !is_ascii || !starts_with_valid_char || !ends_with_valid_char || !contains_only_valid_chars {
                prop_assert!(try_from_result.is_err());
            } else {
                prop_assert!(try_from_result.is_ok());
            }
        }

        // Test from_sanitized
        prop_assert!(sanitized.len() <= 63);
        prop_assert!(sanitized.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
        prop_assert!(!sanitized.starts_with('_'));
        prop_assert!(!sanitized.ends_with('_'));

        // Ensure sanitized version is always valid, unless it's empty
        if sanitized.is_empty() {
            prop_assert_eq!(TypeIdPrefix::try_from(sanitized.as_str()).unwrap_err(), ValidationError::IsEmpty);
        } else {
            prop_assert!(TypeIdPrefix::try_from(sanitized.as_str()).is_ok());
        }
    }

    #[test]
    fn test_typeidprefix_try_from_str_and_sanitize(input in "\\PC*") {
        // println!("Running test with input: {:?}", input); // Print each test input
        let try_from_result = TypeIdPrefix::try_from(input.as_str());
        let sanitized = input.create_prefix_sanitized();

        // Test TypeIdPrefix::try_from for &str
        if input.len() > 63 {
            prop_assert!(try_from_result.is_err());
        } else if input.is_empty() {
            prop_assert_eq!(try_from_result.unwrap_err(), ValidationError::IsEmpty);
        } else {
            let is_ascii = input.is_ascii();
            let starts_with_valid_char = input.chars().next().is_some_and(|c| c.is_ascii_lowercase());
            let ends_with_valid_char = input.chars().last().is_some_and(|c| c.is_ascii_lowercase());
            let contains_only_valid_chars = input.chars().all(|c| c.is_ascii_lowercase() || c == '_');
            if !is_ascii || !starts_with_valid_char || !ends_with_valid_char || !contains_only_valid_chars {
                prop_assert!(try_from_result.is_err());
            } else {
                prop_assert!(try_from_result.is_ok());
            }
        }

        // Test from_sanitized (same as in previous test)
        prop_assert!(sanitized.len() <= 63);
        prop_assert!(sanitized.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
        prop_assert!(!sanitized.starts_with('_'));
        prop_assert!(!sanitized.ends_with('_'));

        // Ensure sanitized version is always valid, unless it's empty
        if sanitized.is_empty() {
            prop_assert_eq!(TypeIdPrefix::try_from(sanitized.as_str()).unwrap_err(), ValidationError::IsEmpty);
        } else {
            prop_assert!(TypeIdPrefix::try_from(sanitized.as_str()).is_ok());
        }
    }

    #[test]
    fn test_typeidprefix_clean(input in ".{0,100}") {
        // println!("Running test with input: {:?}", input); // Print each test input
        let cleaned = input.create_prefix_sanitized();
        prop_assert!(cleaned.len() <= 63);
        prop_assert!(cleaned.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
        prop_assert!(!cleaned.starts_with('_'));
        prop_assert!(!cleaned.ends_with('_'));
        if cleaned.is_empty() {
            prop_assert_eq!(TypeIdPrefix::try_from(cleaned.as_str()).unwrap_err(), ValidationError::IsEmpty);
        } else {
            prop_assert!(TypeIdPrefix::try_from(cleaned.as_str()).is_ok());
        }
    }
}