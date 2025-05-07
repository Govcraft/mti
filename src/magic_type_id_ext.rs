use std::str::FromStr;

use typeid_prefix::prelude::*;
use typeid_suffix::prelude::*;

use crate::errors::MagicTypeIdError;
use crate::magic_type_id::MagicTypeId;

/// Extends string-like types with `TypeID` functionality.
///
/// This trait provides methods to parse, validate, and create `TypeIDs` and their components.
/// It allows for easy manipulation of `TypeID` strings and creation of `MagicTypeId` instances.
///
/// # Examples
///
/// ```
/// use mti::prelude::*;
///
/// let type_id_str = "user_01h2xcejqg4wh1r27hsdgzeqp4";
///
/// // Extracting components
/// assert_eq!(type_id_str.prefix_str().unwrap(), "user");
/// assert_eq!(type_id_str.suffix_str().unwrap(), "01h2xcejqg4wh1r27hsdgzeqp4");
///
/// // Creating a new TypeID
/// let new_id = "product".create_type_id::<V7>();
/// assert!(new_id.to_string().starts_with("product_"));
///
/// // Parsing an existing TypeID
/// let parsed_id = "user".try_create_type_id::<V7>().unwrap();
/// assert_eq!(parsed_id.prefix().as_str(), "user");
/// ```
pub trait MagicTypeIdExt {
    /// Extracts and validates the prefix string from a `TypeID`.
    ///
    /// This method attempts to extract the prefix part of a `TypeID` string,
    /// validate it, and return it as a `String`.
    ///
    /// # Returns
    ///
    /// - `Ok(String)` containing the validated prefix if successful.
    /// - `Err(MagicTypeIdError)` if the prefix is invalid or the `TypeID` format is incorrect.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The prefix part of the `TypeID` is invalid according to the `TypeID` specification.
    /// - The `TypeID` string format is incorrect (e.g., missing separator).
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// assert_eq!("user_01h2xcejqg4wh1r27hsdgzeqp4".prefix_str().unwrap(), "user");
    /// assert_eq!("01h2xcejqg4wh1r27hsdgzeqp4".prefix_str().unwrap(), ""); // No prefix
    /// assert!("123_01h2xcejqg4wh1r27hsdgzeqp4".prefix_str().is_err()); // Invalid prefix
    /// ```
    fn prefix_str(&self) -> Result<String, MagicTypeIdError>;

    /// Extracts and validates the suffix string from a `TypeID`.
    ///
    /// This method attempts to extract the suffix part of a `TypeID` string,
    /// validate it as a valid base32-encoded UUID, and return it as a `String`.
    ///
    /// # Returns
    ///
    /// - `Ok(String)` containing the validated suffix if successful.
    /// - `Err(MagicTypeIdError)` if the suffix is invalid or the `TypeID` format is incorrect.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The suffix is not a valid base32-encoded UUID.
    /// - The `TypeID` string is empty or improperly formatted.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let suffix = "user_01h2xcejqg4wh1r27hsdgzeqp4".suffix_str().unwrap();
    /// assert_eq!(suffix, "01h2xcejqg4wh1r27hsdgzeqp4");
    /// assert!("user_invalid".suffix_str().is_err());
    /// ```
    fn suffix_str(&self) -> Result<String, MagicTypeIdError>;

    /// Extracts and decodes the UUID string from a `TypeID`.
    ///
    /// This method attempts to extract the suffix part of a `TypeID` string,
    /// decode it from base32 to a standard UUID format, and return it as a `String`.
    ///
    /// # Returns
    ///
    /// - `Ok(String)` containing the decoded UUID in standard format if successful.
    /// - `Err(MagicTypeIdError)` if the suffix is invalid or the `TypeID` format is incorrect.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The suffix is not a valid base32-encoded UUID.
    /// - The `TypeID` string is empty or improperly formatted.
    /// - The decoded suffix cannot be converted to a valid UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let uuid = "user_01h2xcejqg4wh1r27hsdgzeqp4".uuid_str().unwrap();
    /// assert_eq!(uuid.len(), 36); // Standard UUID string length
    /// assert!(uuid.contains('-')); // Contains hyphens in standard format
    /// ```
    fn uuid_str(&self) -> Result<String, MagicTypeIdError>;

    /// Extracts and validates the prefix from a `TypeID`, returning a `TypeIdPrefix`.
    ///
    /// This method attempts to extract the prefix part of a `TypeID` string
    /// and return it as a validated `TypeIdPrefix` instance.
    ///
    /// # Returns
    ///
    /// - `Ok(TypeIdPrefix)` containing the validated prefix if successful.
    /// - `Err(MagicTypeIdError)` if the prefix is invalid or the `TypeID` format is incorrect.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The prefix part of the `TypeID` is invalid according to the `TypeID` specification.
    /// - The `TypeID` string format is incorrect (e.g., missing separator).
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let prefix = "user_01h2xcejqg4wh1r27hsdgzeqp4".prefix().unwrap();
    /// assert_eq!(prefix.as_str(), "user");
    /// assert!("123_01h2xcejqg4wh1r27hsdgzeqp4".prefix().is_err());
    /// ```
    fn prefix(&self) -> Result<TypeIdPrefix, MagicTypeIdError>;

    /// Extracts and validates the suffix from a `TypeID`, returning a `TypeIdSuffix`.
    ///
    /// This method attempts to extract the suffix part of a `TypeID` string
    /// and return it as a validated `TypeIdSuffix` instance.
    ///
    /// # Returns
    ///
    /// - `Ok(TypeIdSuffix)` containing the validated suffix if successful.
    /// - `Err(MagicTypeIdError)` if the suffix is invalid or the `TypeID` format is incorrect.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The suffix is not a valid base32-encoded UUID.
    /// - The `TypeID` string is empty or improperly formatted.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let suffix = "user_01h2xcejqg4wh1r27hsdgzeqp4".suffix().unwrap();
    /// assert_eq!(suffix.to_string(), "01h2xcejqg4wh1r27hsdgzeqp4");
    /// assert!("user_invalid".suffix().is_err());
    /// ```
    fn suffix(&self) -> Result<TypeIdSuffix, MagicTypeIdError>;

    /// Extracts and decodes the UUID from a `TypeID`, returning a `Uuid`.
    ///
    /// This method attempts to extract the suffix part of a `TypeID` string,
    /// decode it from base32, and return it as a `Uuid` instance.
    ///
    /// # Returns
    ///
    /// - `Ok(Uuid)` containing the decoded UUID if successful.
    /// - `Err(MagicTypeIdError)` if the suffix is invalid or the `TypeID` format is incorrect.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The suffix is not a valid base32-encoded UUID.
    /// - The `TypeID` string is empty or improperly formatted.
    /// - The decoded suffix cannot be converted to a valid UUID.
    ///
    /// # Examples
    ///
    /// ```
    ///     use mti::prelude::*;
    ///
    ///     let type_id_str = "user_01h455vb4pex5vsknk084sn02q";
    ///     let uuid = type_id_str.uuid().unwrap();
    ///     assert_eq!(uuid.get_version_num(), 7); // UUIDv7
    ///     assert_eq!(uuid.to_string(), "01890a5d-ac96-774b-bcce-b302099a8057");
    /// ```
    fn uuid(&self) -> Result<Uuid, MagicTypeIdError>;

    /// Creates a new `MagicTypeId` with the string as prefix and a new UUID of the specified version.
    ///
    /// This method sanitizes the input string to ensure a valid prefix is created. The sanitization process:
    /// - Converts the input to lowercase.
    /// - Removes all characters that are not lowercase ASCII letters or underscores.
    /// - Trims leading and trailing underscores.
    /// - Truncates the result to a maximum of 63 characters.
    /// - If the result is empty after sanitization, it returns an empty prefix (which is valid).
    ///
    /// # Type Parameters
    ///
    /// * `V`: A type that implements `UuidVersion` and `Default`.
    ///
    /// # Returns
    ///
    /// A new `MagicTypeId` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let id = "user".create_type_id::<V7>();
    /// assert!(id.to_string().starts_with("user_"));
    ///
    /// // Sanitization examples:
    /// let sanitized_id = "Invalid Prefix!123".create_type_id::<V4>();
    /// assert!(sanitized_id.to_string().starts_with("invalidprefix_"));
    ///
    ///     let sanitized_id = "  _USER_NAME_  ".create_type_id::<V4>();
    ///     assert!(sanitized_id.to_string().starts_with("user_name_"));
    ///
    /// let sanitized_id = "a".repeat(100).create_type_id::<V4>();
    /// assert_eq!(sanitized_id.prefix().as_str().len(), 63);
    /// ```
    fn create_type_id<V: UuidVersion + Default>(&self) -> MagicTypeId;

    /// Creates a new `MagicTypeId` with the string as prefix and the provided suffix.
    ///
    /// This method sanitizes the input string to ensure a valid prefix is created. The sanitization process:
    /// - Converts the input to lowercase.
    /// - Removes all characters that are not lowercase ASCII letters or underscores.
    /// - Trims leading and trailing underscores.
    /// - Truncates the result to a maximum of 63 characters.
    /// - If the result is empty after sanitization, it returns an empty prefix (which is valid).
    ///
    /// # Parameters
    ///
    /// * `suffix`: A `TypeIdSuffix` to use for the new `MagicTypeId`.
    ///
    /// # Returns
    ///
    /// A new `MagicTypeId` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new::<V7>();
    ///
    /// let id = "user".create_type_id_with_suffix::<V7>(suffix.clone());
    /// assert!(id.to_string().starts_with("user_"));
    ///
    /// // Sanitization examples:
    /// let sanitized_id = "PRODUCT_ID_123".create_type_id_with_suffix::<V7>(suffix.clone());
    /// assert!(sanitized_id.to_string().starts_with("product_id_"));
    ///
    /// let sanitized_id = "__inva!id__".create_type_id_with_suffix::<V7>(suffix);
    /// assert!(sanitized_id.to_string().starts_with("invaid_"));
    /// ```
    fn create_type_id_with_suffix<V: UuidVersion + Default>(&self, suffix: TypeIdSuffix) -> MagicTypeId;

    /// Attempts to create a new `MagicTypeId` with the string as prefix and a new UUID of the specified version.
    ///
    /// This method does not sanitize the input, so it will fail if the prefix is invalid.
    ///
    /// # Type Parameters
    ///
    /// * `V`: A type that implements `UuidVersion` and `Default`.
    ///
    /// # Returns
    ///
    /// - `Ok(MagicTypeId)` if the prefix is valid and the `TypeID` was successfully created.
    /// - `Err(MagicTypeIdError)` if the prefix is invalid.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The input string is not a valid `TypeID` prefix according to the [specification](https://github.com/jetify-com/typeid/tree/main/spec).
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let id = "user".try_create_type_id::<V7>().unwrap();
    /// assert!(id.to_string().starts_with("user_"));
    ///
    /// assert!("Invalid Prefix!".try_create_type_id::<V4>().is_err());
    /// ```
    fn try_create_type_id<V: UuidVersion + Default>(&self) -> Result<MagicTypeId, MagicTypeIdError>;

    /// Attempts to create a new `MagicTypeId` with the string as prefix and the provided suffix.
    ///
    /// This method does not sanitize the input, so it will fail if the prefix is invalid.
    ///
    /// # Parameters
    ///
    /// * `suffix`: A `TypeIdSuffix` to use for the new `MagicTypeId`.
    ///
    /// # Returns
    ///
    /// - `Ok(MagicTypeId)` if the prefix is valid and the `TypeID` was successfully created.
    /// - `Err(MagicTypeIdError)` if the prefix is invalid.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The input string is not a valid `TypeID` prefix according to the specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new::<V7>();
    /// let id = "user".try_create_type_id_with_suffix::<V7>(suffix).unwrap();
    /// assert!(id.to_string().starts_with("user_"));
    ///
    /// let suffix = TypeIdSuffix::new::<V7>();
    /// assert!("Invalid Prefix!".try_create_type_id_with_suffix::<V7>(suffix).is_err());
    /// ```
    fn try_create_type_id_with_suffix<V: UuidVersion + Default>(&self, suffix: TypeIdSuffix) -> Result<MagicTypeId, MagicTypeIdError>;
}

impl MagicTypeIdExt for str {
    fn prefix_str(&self) -> Result<String, MagicTypeIdError> {
        self.prefix().map(|p| p.to_string())
    }

    fn suffix_str(&self) -> Result<String, MagicTypeIdError> {
        self.suffix().map(|s| s.to_string())
    }

    fn uuid_str(&self) -> Result<String, MagicTypeIdError> {
        self.uuid().map(|u| u.to_string())
    }

    fn prefix(&self) -> Result<TypeIdPrefix, MagicTypeIdError> {
        if self.is_empty() {
            return Ok(TypeIdPrefix::default());
        }
        match self.rsplit_once('_') {
            Some((prefix, _)) => TypeIdPrefix::try_from(prefix).map_err(MagicTypeIdError::Prefix),
            None => Ok(TypeIdPrefix::default()),
        }
    }

    fn suffix(&self) -> Result<TypeIdSuffix, MagicTypeIdError> {
        if self.is_empty() {
            return Err(MagicTypeIdError::Suffix(DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidLength)));
        }
        let suffix_str = self.rsplit_once('_').map_or(self, |(_, suffix)| suffix);
        TypeIdSuffix::from_str(suffix_str).map_err(MagicTypeIdError::Suffix)
    }

    fn uuid(&self) -> Result<Uuid, MagicTypeIdError> {
        self.suffix().map(|s| s.to_uuid())
    }

    fn create_type_id<V: UuidVersion + Default>(&self) -> MagicTypeId {
        let prefix = self.create_prefix_sanitized();
        let suffix = TypeIdSuffix::new::<V>();
        MagicTypeId::new(prefix, suffix)
    }

    fn create_type_id_with_suffix<V: UuidVersion + Default>(&self, suffix: TypeIdSuffix) -> MagicTypeId {
        let prefix = self.create_prefix_sanitized();
        MagicTypeId::new(prefix, suffix)
    }

    fn try_create_type_id<V: UuidVersion + Default>(&self) -> Result<MagicTypeId, MagicTypeIdError> {
        let prefix = TypeIdPrefix::try_from(self)?;
        let suffix = TypeIdSuffix::new::<V>();
        Ok(MagicTypeId::new(prefix, suffix))
    }

    fn try_create_type_id_with_suffix<V: UuidVersion + Default>(&self, suffix: TypeIdSuffix) -> Result<MagicTypeId, MagicTypeIdError> {
        let prefix = TypeIdPrefix::try_from(self)?;
        Ok(MagicTypeId::new(prefix, suffix))
    }
}

#[cfg(test)]
mod ext_tests {
    #[test]
    fn test_prefix() {
        use crate::prelude::*;

        assert_eq!("user_01h455vb4pex5vsknk084sn02q".prefix_str().unwrap(), "user");
        assert_eq!("Invalid_Prefix_01h455vb4pex5vsknk084sn02q".create_prefix_sanitized().as_str(), "invalid_prefix_hvbpexvsknksnq");
        assert_eq!("123InvalidStart".prefix_str().unwrap(), "");
        assert_eq!("".prefix_str().unwrap(), "");
        //create a new type id from an extracted prefix
        let prefix = "user_00000000000000000000000000".prefix().unwrap();
        let new_id = prefix.create_type_id::<Nil>();
        assert_eq!(new_id.to_string(), "user_00000000000000000000000000");
        assert_eq!(new_id.uuid_str().unwrap(), "00000000-0000-0000-0000-000000000000");
    }
}
