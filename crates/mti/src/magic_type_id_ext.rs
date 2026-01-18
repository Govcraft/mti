use std::str::FromStr;

use typeid_prefix::prelude::*;
use typeid_suffix::prelude::*;

use crate::errors::MagicTypeIdError;
use crate::magic_type_id::MagicTypeId;

#[cfg(feature = "instrument")]
use tracing::{debug, instrument, trace, warn};

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

    /// Creates a `MagicTypeId` with a V3 UUID (MD5-based name hash).
    ///
    /// This method sanitizes the prefix and creates a deterministic type ID
    /// from a namespace and name using the MD5 hashing algorithm.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace identifier for the UUID.
    /// * `name` - The byte slice to hash with the namespace.
    ///
    /// # Returns
    ///
    /// A new `MagicTypeId` with a sanitized prefix and V3 suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let domain_id = "domain".create_type_id_v3(NamespaceId::DNS, b"example.com");
    /// assert_eq!(domain_id.prefix().as_str(), "domain");
    /// ```
    fn create_type_id_v3(&self, namespace: NamespaceId, name: &[u8]) -> MagicTypeId;

    /// Creates a `MagicTypeId` with a V5 UUID (SHA-1-based name hash).
    ///
    /// This method sanitizes the prefix and creates a deterministic type ID
    /// from a namespace and name using the SHA-1 hashing algorithm. V5 is
    /// preferred over V3 due to better security properties.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace identifier for the UUID.
    /// * `name` - The byte slice to hash with the namespace.
    ///
    /// # Returns
    ///
    /// A new `MagicTypeId` with a sanitized prefix and V5 suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let domain_id = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
    /// assert_eq!(domain_id.prefix().as_str(), "domain");
    ///
    /// let url_id = "page".create_type_id_v5(NamespaceId::URL, b"https://example.com/about");
    /// assert_eq!(url_id.prefix().as_str(), "page");
    /// ```
    fn create_type_id_v5(&self, namespace: NamespaceId, name: &[u8]) -> MagicTypeId;

    /// Attempts to create a `MagicTypeId` with a V3 UUID (MD5-based name hash).
    ///
    /// This method validates the prefix strictly and creates a deterministic
    /// type ID from a namespace and name using the MD5 hashing algorithm.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace identifier for the UUID.
    /// * `name` - The byte slice to hash with the namespace.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the new `MagicTypeId` or a `MagicTypeIdError`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the prefix is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let result = "valid_prefix".try_create_type_id_v3(NamespaceId::DNS, b"example.com");
    /// assert!(result.is_ok());
    ///
    /// let result = "Invalid Prefix!".try_create_type_id_v3(NamespaceId::DNS, b"example.com");
    /// assert!(result.is_err());
    /// ```
    fn try_create_type_id_v3(
        &self,
        namespace: NamespaceId,
        name: &[u8],
    ) -> Result<MagicTypeId, MagicTypeIdError>;

    /// Attempts to create a `MagicTypeId` with a V5 UUID (SHA-1-based name hash).
    ///
    /// This method validates the prefix strictly and creates a deterministic
    /// type ID from a namespace and name using the SHA-1 hashing algorithm.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace identifier for the UUID.
    /// * `name` - The byte slice to hash with the namespace.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the new `MagicTypeId` or a `MagicTypeIdError`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the prefix is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use mti::prelude::*;
    ///
    /// let result = "valid_prefix".try_create_type_id_v5(NamespaceId::DNS, b"example.com");
    /// assert!(result.is_ok());
    ///
    /// let result = "Invalid Prefix!".try_create_type_id_v5(NamespaceId::DNS, b"example.com");
    /// assert!(result.is_err());
    /// ```
    fn try_create_type_id_v5(
        &self,
        namespace: NamespaceId,
        name: &[u8],
    ) -> Result<MagicTypeId, MagicTypeIdError>;
}

impl MagicTypeIdExt for str {
    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self), fields(input = %self)))]
    fn prefix_str(&self) -> Result<String, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Extracting prefix string from TypeID");
        let result = self.prefix().map(|p| p.to_string());
        
        #[cfg(feature = "instrument")]
        match &result {
            Ok(prefix) => debug!("Successfully extracted prefix: '{}'", prefix),
            Err(err) => warn!("Failed to extract prefix: {}", err),
        }
        
        result
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self), fields(input = %self)))]
    fn suffix_str(&self) -> Result<String, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Extracting suffix string from TypeID");
        let result = self.suffix().map(|s| s.to_string());
        
        #[cfg(feature = "instrument")]
        match &result {
            Ok(suffix) => debug!("Successfully extracted suffix: '{}'", suffix),
            Err(err) => warn!("Failed to extract suffix: {}", err),
        }
        
        result
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self), fields(input = %self)))]
    fn uuid_str(&self) -> Result<String, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Extracting UUID string from TypeID");
        let result = self.uuid().map(|u| u.to_string());
        
        #[cfg(feature = "instrument")]
        match &result {
            Ok(uuid) => debug!("Successfully extracted UUID: '{}'", uuid),
            Err(err) => warn!("Failed to extract UUID: {}", err),
        }
        
        result
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self), fields(input = %self)))]
    fn prefix(&self) -> Result<TypeIdPrefix, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Extracting TypeIdPrefix from TypeID");
        
        if self.is_empty() {
            #[cfg(feature = "instrument")]
            debug!("Input is empty, returning default TypeIdPrefix");
            return Ok(TypeIdPrefix::default());
        }
        
        if let Some((prefix, _)) = self.rsplit_once('_') {
            #[cfg(feature = "instrument")]
            trace!("Found prefix part: '{}'", prefix);
            let result = TypeIdPrefix::try_from(prefix).map_err(MagicTypeIdError::Prefix);
            
            #[cfg(feature = "instrument")]
            match &result {
                Ok(prefix) => debug!("Successfully created TypeIdPrefix: '{}'", prefix),
                Err(err) => warn!("Failed to create TypeIdPrefix: {}", err),
            }
            
            result
        } else {
            #[cfg(feature = "instrument")]
            debug!("No prefix separator found, returning default TypeIdPrefix");
            Ok(TypeIdPrefix::default())
        }
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self), fields(input = %self)))]
    fn suffix(&self) -> Result<TypeIdSuffix, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Extracting TypeIdSuffix from TypeID");
        
        if self.is_empty() {
            #[cfg(feature = "instrument")]
            warn!("Input is empty, returning InvalidSuffix error");
            return Err(MagicTypeIdError::Suffix(DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidLength)));
        }
        
        let suffix_str = self.rsplit_once('_').map_or(self, |(_, suffix)| suffix);
        #[cfg(feature = "instrument")]
        trace!("Found suffix part: '{}'", suffix_str);
        
        let result = TypeIdSuffix::from_str(suffix_str).map_err(MagicTypeIdError::Suffix);
        
        #[cfg(feature = "instrument")]
        match &result {
            Ok(suffix) => debug!("Successfully created TypeIdSuffix: '{}'", suffix),
            Err(err) => warn!("Failed to create TypeIdSuffix: {}", err),
        }
        
        result
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self), fields(input = %self)))]
    fn uuid(&self) -> Result<Uuid, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Extracting UUID from TypeID");
        
        let result = self.suffix().map(|s| s.to_uuid());
        
        #[cfg(feature = "instrument")]
        match &result {
            Ok(uuid) => debug!("Successfully extracted UUID: '{}'", uuid),
            Err(err) => warn!("Failed to extract UUID: {}", err),
        }
        
        result
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self), fields(input = %self, uuid_version = std::any::type_name::<V>())))]
    fn create_type_id<V: UuidVersion + Default>(&self) -> MagicTypeId {
        #[cfg(feature = "instrument")]
        trace!("Creating MagicTypeId with sanitized prefix");
        
        let prefix = self.create_prefix_sanitized();
        #[cfg(feature = "instrument")]
        debug!("Sanitized prefix: '{}'", prefix);
        
        let suffix = TypeIdSuffix::new::<V>();
        #[cfg(feature = "instrument")]
        debug!("Created new TypeIdSuffix: '{}'", suffix);
        
        MagicTypeId::new(prefix, suffix)
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self, suffix), fields(input = %self, suffix = %suffix)))]
    fn create_type_id_with_suffix<V: UuidVersion + Default>(&self, suffix: TypeIdSuffix) -> MagicTypeId {
        #[cfg(feature = "instrument")]
        trace!("Creating MagicTypeId with sanitized prefix and provided suffix");
        
        let prefix = self.create_prefix_sanitized();
        #[cfg(feature = "instrument")]
        debug!("Sanitized prefix: '{}'", prefix);
        
        MagicTypeId::new(prefix, suffix)
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self), fields(input = %self, uuid_version = std::any::type_name::<V>())))]
    fn try_create_type_id<V: UuidVersion + Default>(&self) -> Result<MagicTypeId, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Attempting to create MagicTypeId with validated prefix");
        
        let prefix = TypeIdPrefix::try_from(self)?;
        #[cfg(feature = "instrument")]
        debug!("Successfully validated prefix: '{}'", prefix);
        
        let suffix = TypeIdSuffix::new::<V>();
        #[cfg(feature = "instrument")]
        debug!("Created new TypeIdSuffix: '{}'", suffix);
        
        Ok(MagicTypeId::new(prefix, suffix))
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self, suffix), fields(input = %self, suffix = %suffix)))]
    fn try_create_type_id_with_suffix<V: UuidVersion + Default>(&self, suffix: TypeIdSuffix) -> Result<MagicTypeId, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Attempting to create MagicTypeId with validated prefix and provided suffix");

        let prefix = TypeIdPrefix::try_from(self)?;
        #[cfg(feature = "instrument")]
        debug!("Successfully validated prefix: '{}'", prefix);

        Ok(MagicTypeId::new(prefix, suffix))
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self, name), fields(input = %self, namespace = %namespace)))]
    fn create_type_id_v3(&self, namespace: NamespaceId, name: &[u8]) -> MagicTypeId {
        #[cfg(feature = "instrument")]
        trace!("Creating MagicTypeId with V3 UUID from namespace");

        let prefix = self.create_prefix_sanitized();
        #[cfg(feature = "instrument")]
        debug!("Sanitized prefix: '{}'", prefix);

        let suffix = TypeIdSuffix::new_v3(namespace, name);
        #[cfg(feature = "instrument")]
        debug!("Created V3 TypeIdSuffix: '{}'", suffix);

        MagicTypeId::new(prefix, suffix)
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self, name), fields(input = %self, namespace = %namespace)))]
    fn create_type_id_v5(&self, namespace: NamespaceId, name: &[u8]) -> MagicTypeId {
        #[cfg(feature = "instrument")]
        trace!("Creating MagicTypeId with V5 UUID from namespace");

        let prefix = self.create_prefix_sanitized();
        #[cfg(feature = "instrument")]
        debug!("Sanitized prefix: '{}'", prefix);

        let suffix = TypeIdSuffix::new_v5(namespace, name);
        #[cfg(feature = "instrument")]
        debug!("Created V5 TypeIdSuffix: '{}'", suffix);

        MagicTypeId::new(prefix, suffix)
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self, name), fields(input = %self, namespace = %namespace)))]
    fn try_create_type_id_v3(
        &self,
        namespace: NamespaceId,
        name: &[u8],
    ) -> Result<MagicTypeId, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Attempting to create MagicTypeId with V3 UUID from namespace");

        let prefix = TypeIdPrefix::try_from(self)?;
        #[cfg(feature = "instrument")]
        debug!("Successfully validated prefix: '{}'", prefix);

        let suffix = TypeIdSuffix::new_v3(namespace, name);
        #[cfg(feature = "instrument")]
        debug!("Created V3 TypeIdSuffix: '{}'", suffix);

        Ok(MagicTypeId::new(prefix, suffix))
    }

    #[cfg_attr(feature = "instrument", instrument(level = "debug", skip(self, name), fields(input = %self, namespace = %namespace)))]
    fn try_create_type_id_v5(
        &self,
        namespace: NamespaceId,
        name: &[u8],
    ) -> Result<MagicTypeId, MagicTypeIdError> {
        #[cfg(feature = "instrument")]
        trace!("Attempting to create MagicTypeId with V5 UUID from namespace");

        let prefix = TypeIdPrefix::try_from(self)?;
        #[cfg(feature = "instrument")]
        debug!("Successfully validated prefix: '{}'", prefix);

        let suffix = TypeIdSuffix::new_v5(namespace, name);
        #[cfg(feature = "instrument")]
        debug!("Created V5 TypeIdSuffix: '{}'", suffix);

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

#[cfg(test)]
mod namespace_ext_tests {
    use crate::prelude::*;

    #[test]
    fn create_type_id_v3_with_valid_prefix() {
        let type_id = "domain".create_type_id_v3(NamespaceId::DNS, b"example.com");
        assert_eq!(type_id.prefix().as_str(), "domain");
        assert_eq!(type_id.suffix().to_uuid().get_version(), Some(Version::Md5));
    }

    #[test]
    fn create_type_id_v5_with_valid_prefix() {
        let type_id = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
        assert_eq!(type_id.prefix().as_str(), "domain");
        assert_eq!(type_id.suffix().to_uuid().get_version(), Some(Version::Sha1));
    }

    #[test]
    fn create_type_id_v3_sanitizes_invalid_prefix() {
        let type_id = "Invalid Prefix!".create_type_id_v3(NamespaceId::DNS, b"example.com");
        assert!(type_id.prefix().as_str().starts_with("invalidprefix"));
    }

    #[test]
    fn create_type_id_v5_sanitizes_invalid_prefix() {
        let type_id = "Invalid Prefix!".create_type_id_v5(NamespaceId::DNS, b"example.com");
        assert!(type_id.prefix().as_str().starts_with("invalidprefix"));
    }

    #[test]
    fn create_type_id_v3_is_deterministic() {
        let id1 = "domain".create_type_id_v3(NamespaceId::DNS, b"example.com");
        let id2 = "domain".create_type_id_v3(NamespaceId::DNS, b"example.com");
        assert_eq!(id1, id2);
    }

    #[test]
    fn create_type_id_v5_is_deterministic() {
        let id1 = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
        let id2 = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
        assert_eq!(id1, id2);
    }

    #[test]
    fn try_create_type_id_v3_with_valid_prefix() {
        let result = "valid_prefix".try_create_type_id_v3(NamespaceId::DNS, b"example.com");
        assert!(result.is_ok());
        let type_id = result.unwrap();
        assert_eq!(type_id.prefix().as_str(), "valid_prefix");
    }

    #[test]
    fn try_create_type_id_v5_with_valid_prefix() {
        let result = "valid_prefix".try_create_type_id_v5(NamespaceId::DNS, b"example.com");
        assert!(result.is_ok());
        let type_id = result.unwrap();
        assert_eq!(type_id.prefix().as_str(), "valid_prefix");
    }

    #[test]
    fn try_create_type_id_v3_with_invalid_prefix() {
        let result = "Invalid Prefix!".try_create_type_id_v3(NamespaceId::DNS, b"example.com");
        assert!(result.is_err());
    }

    #[test]
    fn try_create_type_id_v5_with_invalid_prefix() {
        let result = "Invalid Prefix!".try_create_type_id_v5(NamespaceId::DNS, b"example.com");
        assert!(result.is_err());
    }

    #[test]
    fn v5_example_matches_docs() {
        // Reproduce the example from lib.rs docs
        let type_id = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
        assert_eq!(
            type_id.suffix().to_uuid().to_string(),
            "cfbff0d1-9375-5685-968c-48ce8b15ae17"
        );
    }

    #[test]
    fn v3_different_from_v5() {
        let id_v3 = "domain".create_type_id_v3(NamespaceId::DNS, b"example.com");
        let id_v5 = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
        assert_ne!(id_v3, id_v5);
    }

    #[test]
    fn different_namespaces_produce_different_ids() {
        let dns_id = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
        let url_id = "domain".create_type_id_v5(NamespaceId::URL, b"example.com");
        assert_ne!(dns_id, url_id);
    }

    #[test]
    fn different_names_produce_different_ids() {
        let id1 = "domain".create_type_id_v5(NamespaceId::DNS, b"example.com");
        let id2 = "domain".create_type_id_v5(NamespaceId::DNS, b"different.com");
        assert_ne!(id1, id2);
    }

    #[test]
    fn custom_namespace_works() {
        let custom_ns = NamespaceId::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        let type_id = "test".create_type_id_v5(custom_ns, b"test-name");
        assert_eq!(type_id.prefix().as_str(), "test");
        assert_eq!(type_id.suffix().to_uuid().get_version(), Some(Version::Sha1));
    }
}
