use std::fmt;

/// Represents errors that can occur during validation of `TypeID` prefixes.
///
/// This enum encapsulates various error conditions that may arise when validating
/// a `TypeID` prefix according to the `TypeID` specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The input exceeds the maximum allowed length of 63 characters.
    ExceedsMaxLength,

    /// The input contains characters that are not allowed in a `TypeID` prefix.
    ///
    /// Valid characters are lowercase ASCII letters and underscores.
    ContainsInvalidCharacters,

    /// The input starts with an underscore, which is not allowed.
    StartsWithUnderscore,

    /// The input ends with an underscore, which is not allowed.
    EndsWithUnderscore,

    /// The input does not start with a lowercase alphabetic character.
    InvalidStartCharacter,

    /// The input does not end with a lowercase alphabetic character.
    InvalidEndCharacter,

    /// The input is an empty string, which is not allowed.
    IsEmpty,
}

impl fmt::Display for ValidationError {
    /// Formats the `ValidationError` for display.
    ///
    /// This implementation provides human-readable error messages for each variant
    /// of the `ValidationError` enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::ValidationError;
    ///
    /// let error = ValidationError::ExceedsMaxLength;
    /// assert_eq!(error.to_string(), "Input exceeds 63 characters");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match self {
            Self::ExceedsMaxLength => {
                "Input exceeds 63 characters"
            }
            Self::ContainsInvalidCharacters => {
                "Input contains invalid characters: only lowercase ASCII letters and underscores are allowed"
            }
            Self::StartsWithUnderscore => {
                "Input cannot start with an underscore"
            }
            Self::EndsWithUnderscore => {
                "Input cannot end with an underscore"
            }
            Self::InvalidStartCharacter => {
                "Input must start with a lowercase alphabetic character"
            }
            Self::InvalidEndCharacter => {
                "Input must end with a lowercase alphabetic character"
            }
            Self::IsEmpty => {
                "Input cannot be empty"
            }
        };

        #[cfg(feature = "instrument")]
        tracing::error!("ValidationError: {}", error_message);

        write!(f, "{error_message}")
    }
}

/// Implements the standard Error trait for `ValidationError`.
///
/// This allows `ValidationError` to be used with the `std::error::Error` trait,
/// enabling better interoperability with error handling mechanisms in Rust.
impl std::error::Error for ValidationError {}
