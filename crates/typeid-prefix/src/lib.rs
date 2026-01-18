#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # `TypeID` Prefix
//!
//! This crate provides a type-safe implementation of the `TypePrefix` section of the
//! [TypeID Specification](https://github.com/jetpack-io/typeid).
//!
//! The main type provided by this crate is [`TypeIdPrefix`], which represents a valid
//! `TypeID` prefix. This type ensures that all instances conform to the `TypeID` specification:
//!
//! - Maximum length of 63 characters
//! - Contains only lowercase ASCII letters and underscores
//! - Does not start or end with an underscore
//! - Starts and ends with a lowercase letter
//!
//! ## Features
//!
//! - **Type-safe**: Ensures that `TypeID` prefixes conform to the specification.
//! - **Validation**: Provides robust validation for `TypeID` prefixes.
//! - **Sanitization**: Offers methods to clean and sanitize input strings into valid `TypeID` prefixes.
//! - **Zero-cost abstractions**: Designed to have minimal runtime overhead.
//! - **Optional tracing**: Integrates with the `tracing` crate for logging (optional feature).
//!
//! ## Usage
//!
//! ```rust
//! use typeid_prefix::prelude::*;
//! use std::convert::TryFrom;
//!
//! // Create a TypeIdPrefix from a valid string
//! let prefix = TypeIdPrefix::try_from("user").unwrap();
//! assert_eq!(prefix.as_str(), "user");
//!
//! // Attempt to create from an invalid string
//! let result = TypeIdPrefix::try_from("Invalid_Prefix");
//! assert!(result.is_err());
//!
//! // Sanitize an invalid string
//! let sanitized = "Invalid_Prefix123".create_prefix_sanitized();
//! assert_eq!(sanitized.as_str(), "invalid_prefix");
//! ```
//!
//! ## Optional Tracing
//!
//! When the `instrument` feature is enabled, the crate will log validation errors
//! using the `tracing` crate.

pub use type_id_prefix::TypeIdPrefix;

pub use crate::error::ValidationError;

mod error;
mod traits;
mod type_id_prefix;

pub mod prelude {
    //! A prelude for the `TypeID` prefix crate.
    //!
    //! This module contains the most commonly used items from the crate.
    //!
    //! # Usage
    //!
    //! ```
    //! use typeid_prefix::prelude::*;
    //! ```
    pub use crate::traits::{PrefixFactory, Validate};
    pub use crate::{TypeIdPrefix, ValidationError};
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::prelude::*;

    use super::*;

    #[test]
    fn test_type_id_spaces_sanitize() {
        assert_eq!(
            "Invalid String with Spaces!!__"
                .create_prefix_sanitized()
                .as_str(),
            "invalidstringwithspaces"
        );
    }

    #[test]
    fn test_type_id_truncation() {
        assert_eq!(
            "A_valid_string_that_is_way_too_long_and_should_be_truncated_to_63_chars"
                .create_prefix_sanitized()
                .as_str(),
            "a_valid_string_that_is_way_too_long_and_should_be_truncated_to"
        );
    }

    #[test]
    fn test_type_id_underscores_sanitize() {
        assert_eq!(
            "_underscores__everywhere__"
                .create_prefix_sanitized()
                .as_str(),
            "underscores__everywhere"
        );
    }

    #[test]
    fn test_typeid_prefix_non_ascii() {
        assert!(TypeIdPrefix::try_from("🌀").is_err());
        let sanitized_input = "🌀".create_prefix_sanitized();
        assert!(
            sanitized_input.as_str().is_empty(),
            "Prefix was not empty: {sanitized_input}"
        );
    }

    #[test]
    fn test_typeid_prefix_empty() {
        assert_eq!(
            TypeIdPrefix::try_from("").unwrap_err(),
            ValidationError::IsEmpty
        );
    }

    #[test]
    fn test_typeid_prefix_single_char() {
        assert!(TypeIdPrefix::try_from("a").is_ok());
    }

    #[test]
    fn test_typeid_prefix_valid_string() {
        assert!(TypeIdPrefix::try_from("valid_string").is_ok());
    }

    #[test]
    fn test_typeid_prefix_with_underscores() {
        assert!(TypeIdPrefix::try_from("valid_string_with_underscores").is_ok());
    }

    #[test]
    fn test_typeid_prefix_exceeds_max_length() {
        let input = "a_valid_string_with_underscores_and_length_of_63_characters_____";
        assert_eq!(
            TypeIdPrefix::try_from(input).unwrap_err(),
            ValidationError::ExceedsMaxLength
        );
        assert_eq!(
            input.create_prefix_sanitized().as_str(),
            "a_valid_string_with_underscores_and_length_of__characters"
        );
    }

    #[test]
    fn test_typeid_prefix_invalid_characters() {
        assert_eq!(
            TypeIdPrefix::try_from("InvalidString").unwrap_err(),
            ValidationError::InvalidStartCharacter
        );
        assert_eq!(
            "InvalidString".create_prefix_sanitized().as_str(),
            "invalidstring"
        );
    }

    #[test]
    fn test_typeid_prefix_starts_with_underscore() {
        assert_eq!(
            TypeIdPrefix::try_from("_invalid").unwrap_err(),
            ValidationError::StartsWithUnderscore
        );
        assert_eq!("_invalid".create_prefix_sanitized().as_str(), "invalid");
    }

    #[test]
    fn test_typeid_prefix_ends_with_underscore() {
        assert_eq!(
            TypeIdPrefix::try_from("invalid_").unwrap_err(),
            ValidationError::EndsWithUnderscore
        );
        assert_eq!("invalid_".create_prefix_sanitized().as_str(), "invalid");
    }

    #[test]
    fn test_typeid_prefix_invalid_characters_with_spaces() {
        assert_eq!(
            TypeIdPrefix::try_from("invalid string with spaces").unwrap_err(),
            ValidationError::ContainsInvalidCharacters
        );
        assert_eq!(
            "invalid string with spaces"
                .create_prefix_sanitized()
                .as_str(),
            "invalidstringwithspaces"
        );
    }

    #[test]
    fn test_typeid_prefix_max_length() {
        let input = "a".repeat(63);
        assert!(TypeIdPrefix::try_from(input.as_str()).is_ok());
    }

    #[test]
    fn test_typeid_prefix_max_length_exceeded() {
        let input = "a".repeat(64);
        assert_eq!(
            TypeIdPrefix::try_from(input.as_str()).unwrap_err(),
            ValidationError::ExceedsMaxLength
        );
        assert_eq!(input.create_prefix_sanitized().as_str(), "a".repeat(63));
    }

    #[test]
    fn test_typeid_prefix_contains_uppercase() {
        assert_eq!(
            TypeIdPrefix::try_from("InvalidString").unwrap_err(),
            ValidationError::InvalidStartCharacter
        );
        assert_eq!(
            "InvalidString".create_prefix_sanitized().as_str(),
            "invalidstring"
        );
    }

    #[test]
    fn test_typeid_prefix_non_alphanumeric() {
        assert_eq!(
            TypeIdPrefix::try_from("invalid_string!").unwrap_err(),
            ValidationError::InvalidEndCharacter
        );
        assert_eq!(
            "invalid_string!".create_prefix_sanitized().as_str(),
            "invalid_string"
        );
    }

    #[test]
    fn test_typeid_prefix_numeric_start() {
        assert_eq!(
            TypeIdPrefix::try_from("1invalid").unwrap_err(),
            ValidationError::InvalidStartCharacter
        );
        assert_eq!("1invalid".create_prefix_sanitized().as_str(), "invalid");
    }

    #[test]
    fn test_typeid_prefix_numeric_end() {
        assert_eq!(
            TypeIdPrefix::try_from("invalid1").unwrap_err(),
            ValidationError::InvalidEndCharacter
        );
        assert_eq!("invalid1".create_prefix_sanitized().as_str(), "invalid");
    }

    #[test]
    fn test_clean_inner_only_underscores() {
        // This would have panicked before the fix
        let sanitized = "___".create_prefix_sanitized();
        assert!(sanitized.as_str().is_empty());
    }

    #[test]
    fn test_clean_inner_only_invalid_chars() {
        // This would have panicked before the fix if it contained only underscores
        // after filtering invalid characters
        let sanitized = "123_456".create_prefix_sanitized();
        assert!(sanitized.as_str().is_empty());
    }

    #[test]
    fn test_clean_inner_mixed_with_only_underscores_after_filtering() {
        // This would have panicked before the fix because after filtering,
        // only underscores remain, which are then stripped
        let sanitized = "123_456_789".create_prefix_sanitized();
        assert!(sanitized.as_str().is_empty());
    }

    #[test]
    fn test_clean_inner_with_valid_chars_and_underscores() {
        // This should work correctly after the fix
        let sanitized = "_abc_def_".create_prefix_sanitized();
        assert_eq!(sanitized.as_str(), "abc_def");
    }
}
