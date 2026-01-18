use std::str::FromStr;

use crate::{TypeIdPrefix, ValidationError};

/// A trait for creating valid `TypeIdPrefix`s from a given input.
///
/// This trait is implemented for any type that can be converted to a string slice (`AsRef<str>`).
/// It provides a method to clean and create a valid `TypeIdPrefix`, even from invalid input as well as a fallible creation method.
///
/// # Examples
///
/// ```
/// use typeid_prefix::prelude::PrefixFactory;
///
/// let sanitized = "Invalid String 123!@#".create_prefix_sanitized();
/// assert_eq!(sanitized.as_str(), "invalidstring");
/// ```
pub trait PrefixFactory {
    /// Sanitizes the input and creates a valid `TypeIdPrefix`.
    ///
    /// This method will modify the input to conform to the `TypeID` specification by:
    /// - Removing invalid characters
    /// - Converting all characters to lowercase
    /// - Truncating to the maximum allowed length if necessary
    /// - Ensuring the result starts and ends with a lowercase alphabetic character
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::prelude::*;
    ///
    /// let valid_input = "User123";
    /// let prefix = valid_input.create_prefix_sanitized();
    /// assert_eq!(prefix.as_str(), "user");
    ///
    /// let invalid_input = "123_USER_456";
    /// let prefix = invalid_input.create_prefix_sanitized();
    /// assert_eq!(prefix.as_str(), "user");
    ///
    /// let empty_input = "123";
    /// let prefix = empty_input.create_prefix_sanitized();
    /// assert_eq!(prefix.as_str(), "");
    /// ```
    ///
    /// # Return Value
    ///
    /// - If the input can be sanitized into a valid prefix, returns a `TypeIdPrefix` containing the sanitized value.
    /// - If the input is invalid and cannot be sanitized into a valid prefix (e.g., contains no valid characters),
    ///   returns an empty `TypeIdPrefix`.
    ///
    /// # Note
    ///
    /// This method will always return a `TypeIdPrefix`, even if it's empty. If you need to ensure
    /// the input is valid without modification, use `try_create_prefix` instead.
    fn create_prefix_sanitized(&self) -> TypeIdPrefix
    where
        Self: AsRef<str>;

    /// Attempts to create a `TypeIdPrefix` from the input without modifying it.
    ///
    /// This method validates the input according to the `TypeID` specification
    /// and returns a `Result` containing either the valid `TypeIdPrefix` or a
    /// `ValidationError` describing why the input is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::prelude::*;
    ///
    /// let valid_input = "user";
    /// assert!(valid_input.try_create_prefix().is_ok());
    ///
    /// let invalid_input = "User123";
    /// assert!(invalid_input.try_create_prefix().is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return a `ValidationError` if the input does not meet
    /// the requirements of a valid `TypeID` prefix. Possible error conditions include:
    ///
    /// - The input exceeds the maximum allowed length of 63 characters.
    /// - The input contains characters other than lowercase ASCII letters and underscores.
    /// - The input starts or ends with an underscore.
    /// - The input does not start or end with a lowercase alphabetic character.
    ///
    /// For more details on specific error conditions, see the `ValidationError` enum.
    ///
    /// # Note
    ///
    /// Unlike `create_prefix_sanitized`, this method does not modify the input.
    /// If you need to automatically correct invalid inputs, use `create_prefix_sanitized` instead.
    fn try_create_prefix(&self) -> Result<TypeIdPrefix, ValidationError>
    where
        Self: AsRef<str>;
}

#[allow(unused_variables)]
impl<T> PrefixFactory for T
where
    T: AsRef<str>,
{
    fn create_prefix_sanitized(&self) -> TypeIdPrefix {
        let input = TypeIdPrefix::clean_inner(self.as_ref());
        TypeIdPrefix::validate(&input).unwrap_or_else(|e| {
            #[cfg(feature = "instrument")]
            tracing::warn!("Invalid TypeIdPrefix: {:?}. Using empty string instead.", e);
            TypeIdPrefix::default()
        })
    }
    fn try_create_prefix(&self) -> Result<TypeIdPrefix, ValidationError> {
        TypeIdPrefix::from_str(self.as_ref())
    }
}
