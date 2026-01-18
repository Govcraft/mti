use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use crate::ValidationError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a valid `TypeID` prefix as defined by the `TypeID` specification.
///
/// A `TypeIdPrefix` is guaranteed to:
/// - Have a maximum length of 63 characters
/// - Contain only lowercase ASCII letters and underscores
/// - Not start or end with an underscore
/// - Start and end with a lowercase letter
///
/// # Examples
///
/// ```
/// use typeid_prefix::TypeIdPrefix;
/// use std::convert::TryFrom;
///
/// let prefix = TypeIdPrefix::try_from("valid_prefix").unwrap();
/// assert_eq!(prefix.as_str(), "valid_prefix");
///
/// let invalid = TypeIdPrefix::try_from("Invalid_Prefix");
/// assert!(invalid.is_err());
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct TypeIdPrefix(String);

#[cfg(feature = "serde")]
impl Serialize for TypeIdPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize TypeIdPrefix as a string
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TypeIdPrefix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize as a string first
        let s = String::deserialize(deserializer)?;

        // Then validate according to TypeID specification
        Self::validate(&s).map_err(serde::de::Error::custom)
    }
}

impl PartialEq<str> for TypeIdPrefix {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<TypeIdPrefix> for str {
    fn eq(&self, other: &TypeIdPrefix) -> bool {
        self == other.0
    }
}

impl Borrow<str> for TypeIdPrefix {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TypeIdPrefix {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for TypeIdPrefix {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<String> for TypeIdPrefix {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<TypeIdPrefix> for String {
    fn eq(&self, other: &TypeIdPrefix) -> bool {
        self == &other.0
    }
}

// You can also implement PartialEq<&str> if needed
impl PartialEq<&str> for TypeIdPrefix {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<TypeIdPrefix> for &str {
    fn eq(&self, other: &TypeIdPrefix) -> bool {
        self == &other.0
    }
}

/// Implements the `FromStr` trait for `TypeIdPrefix`.
///
/// This implementation allows creating a `TypeIdPrefix` from a string slice,
/// validating the input according to the `TypeID` specification.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use typeid_prefix::TypeIdPrefix;
///
/// let valid_prefix = TypeIdPrefix::from_str("user").expect("Valid prefix");
/// assert_eq!(valid_prefix.as_str(), "user");
///
/// let invalid_prefix = TypeIdPrefix::from_str("123");
/// assert!(invalid_prefix.is_err());
/// ```
///
/// # Errors
///
/// This method will return a `ValidationError` if the input string does not meet
/// the requirements of a valid `TypeID` prefix. Possible error conditions include:
///
/// - The input exceeds the maximum allowed length of 63 characters.
/// - The input contains characters other than lowercase ASCII letters and underscores.
/// - The input starts or ends with an underscore.
/// - The input does not start or end with a lowercase alphabetic character.
///
/// For more details on error conditions, see the `ValidationError` enum.
impl FromStr for TypeIdPrefix {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)
    }
}

impl TryFrom<String> for TypeIdPrefix {
    type Error = ValidationError;

    /// Attempts to create a `TypeIdPrefix` from a `String`.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if the input string is not a valid `TypeID` prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::TypeIdPrefix;
    /// use std::convert::TryFrom;
    ///
    /// let valid = TypeIdPrefix::try_from("valid_prefix".to_string()).unwrap();
    /// assert_eq!(valid.as_str(), "valid_prefix");
    ///
    /// let invalid = TypeIdPrefix::try_from("Invalid_Prefix".to_string());
    /// assert!(invalid.is_err());
    /// ```
    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::validate(input.as_ref())
    }
}

impl TryFrom<&str> for TypeIdPrefix {
    type Error = ValidationError;

    /// Attempts to create a `TypeIdPrefix` from a string slice.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if the input string is not a valid `TypeID` prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::TypeIdPrefix;
    /// use std::convert::TryFrom;
    ///
    /// let valid = TypeIdPrefix::try_from("valid_prefix").unwrap();
    /// assert_eq!(valid.as_str(), "valid_prefix");
    ///
    /// let invalid = TypeIdPrefix::try_from("Invalid_Prefix");
    /// assert!(invalid.is_err());
    /// ```
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::validate(input)
    }
}

impl TypeIdPrefix {
    pub(crate) fn validate(input: &str) -> Result<Self, ValidationError> {
        if input.len() > 63 {
            return Err(ValidationError::ExceedsMaxLength);
        }

        if input.is_empty() {
            return Err(ValidationError::IsEmpty);
        }

        if !input.is_ascii() {
            return Err(ValidationError::ContainsInvalidCharacters);
        }

        if input.starts_with('_') {
            return Err(ValidationError::StartsWithUnderscore);
        }

        if input.ends_with('_') {
            return Err(ValidationError::EndsWithUnderscore);
        }

        if !input.starts_with(|c: char| c.is_ascii_lowercase()) {
            return Err(ValidationError::InvalidStartCharacter);
        }

        if !input.ends_with(|c: char| c.is_ascii_lowercase()) {
            return Err(ValidationError::InvalidEndCharacter);
        }

        if !input.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Err(ValidationError::ContainsInvalidCharacters);
        }

        Ok(Self(input.to_string()))
    }

    pub(crate) fn clean_inner(input: &str) -> String {
        let mut result = input.to_string();
        result = result.to_lowercase();
        // Safely truncate to 63 characters if necessary
        if result.len() > 63 {
            result = result.chars().take(63).collect();
        }

        result = result
            .to_ascii_lowercase()
            .chars()
            .filter(|&c| (c.is_ascii_lowercase() || c == '_') && c.is_ascii())
            .collect::<String>();

        // Remove leading and trailing underscores safely using trim_matches
        // This avoids potential panics when the string is empty or contains only underscores
        result = result.trim_matches('_').to_string();

        result
    }

    /// Returns a string slice of the `TypeID` prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let prefix = TypeIdPrefix::try_from("valid_prefix").unwrap();
    /// assert_eq!(prefix.as_str(), "valid_prefix");
    /// ```
    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for TypeIdPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
