//! Error types for the `TypeID` suffix module.
//!
//! This module defines the error types used throughout the `TypeID` suffix
//! implementation, providing detailed information about various failure modes
//! during encoding, decoding, and validation processes.

use std::fmt;

#[cfg(feature = "instrument")]
use tracing::error;

/// Represents errors that can occur during `TypeID` suffix decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Represents an error with the `TypeID` suffix.
    InvalidSuffix(InvalidSuffixReason),
    /// Represents an error with the underlying UUID.
    InvalidUuid(InvalidUuidReason),
    /// The namespace UUID string is invalid.
    ///
    /// This error occurs when attempting to parse a namespace UUID
    /// from a string that doesn't match UUID format.
    InvalidNamespace(String),
}

/// Specifies the reason for an invalid `TypeID` suffix.
///
/// This enum provides more granular information about why a `TypeID` suffix
/// is considered invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidSuffixReason {
    /// The suffix does not have the required length of 26 characters.
    InvalidLength,
    /// The suffix contains one or more non-ASCII characters.
    NonAsciiCharacter,
    /// The first character of the suffix is greater than '7'.
    InvalidFirstCharacter,
    /// The suffix contains a character that is not in the base32 alphabet.
    InvalidCharacter,
}

/// Specifies the reason for an invalid UUID.
///
/// This enum provides more detailed information about why a UUID
/// is considered invalid in the context of `TypeID`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidUuidReason {
    /// The UUID version is not valid for this `TypeID`.
    InvalidVersion,
    /// The UUID variant is not RFC4122.
    InvalidVariant,
    /// The UUID bytes are invalid.
    InvalidBytes,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidSuffix(reason) => format!("Invalid `TypeID` suffix: {reason}"),
            Self::InvalidUuid(reason) => format!("Invalid UUID: {reason}"),
            Self::InvalidNamespace(s) => format!(
                "invalid namespace UUID '{s}': expected format like '6ba7b810-9dad-11d1-80b4-00c04fd430c8'"
            ),
        };

        #[cfg(feature = "instrument")]
        error!("{msg}");

        write!(f, "{msg}")
    }
}

impl fmt::Display for InvalidSuffixReason {
    /// Provides a human-readable description of the invalid suffix reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidLength => "Suffix must be exactly 26 characters long",
            Self::NonAsciiCharacter => "Suffix contains non-ASCII characters",
            Self::InvalidFirstCharacter => "First character of suffix must be '7' or less",
            Self::InvalidCharacter => "Suffix contains characters not in the base32 alphabet",
        };

        #[cfg(feature = "instrument")]
        error!("{}", msg);

        write!(f, "{msg}")
    }
}

impl fmt::Display for InvalidUuidReason {
    /// Provides a human-readable description of the invalid UUID reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidVersion => "UUID version is not valid for this TypeID",
            Self::InvalidVariant => "UUID variant is not RFC4122",
            Self::InvalidBytes => "UUID bytes are invalid",
        };

        #[cfg(feature = "instrument")]
        error!("{msg}");

        write!(f, "{msg}")
    }
}

/// Implement the standard Error trait for `DecodeError`.
impl std::error::Error for DecodeError {}
