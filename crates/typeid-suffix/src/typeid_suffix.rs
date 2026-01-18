//! This module implements the ``TypeIdSuffix`` struct and its associated functionality.
//! ``TypeIdSuffix`` represents the suffix part of a `TypeId`, which is a base32-encoded UUID.

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use uuid::{Uuid, Variant, Version};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::encoding::{decode_base32, encode_base32};
use crate::errors::{DecodeError, InvalidSuffixReason, InvalidUuidReason};
use crate::versions::{UuidVersion, V7};

/// Represents a `TypeId` suffix, which is a 26-character base32-encoded UUID.
///
/// This struct encapsulates the suffix part of a `TypeId`, providing methods for
/// creation, conversion, and validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeIdSuffix([u8; 26]);

impl TypeIdSuffix {
    /// Creates a new ``TypeIdSuffix`` from a specific UUID version.
    ///
    /// This method generates a new UUID of the specified version and encodes it
    /// as a ``TypeIdSuffix``.
    ///
    /// # Type Parameters
    ///
    /// * `V`: A type that implements `UuidVersion` and `Default`.
    ///
    /// # Returns
    ///
    /// A new ``TypeIdSuffix`` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new::<V4>();
    /// ```
    #[cfg_attr(feature = "instrument", tracing::instrument)]
    #[inline]
    #[must_use]
    pub fn new<V>() -> Self
    where
        V: UuidVersion + Default,
    {
        Self(encode_base32(V::default().as_bytes()))
    }

    /// Checks if a given UUID is valid according to the `TypeId` specification.
    ///
    /// This method validates both the variant and version of the UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid`: A reference to the `Uuid` to be validated.
    ///
    /// # Returns
    ///
    /// `true` if the UUID is valid, `false` otherwise.
    const fn is_valid_uuid(uuid: &Uuid) -> bool {
        let is_valid_variant = matches!(
            uuid.get_variant(),
            Variant::RFC4122 | Variant::Microsoft | Variant::Future | Variant::NCS
        );

        let is_valid_version = matches!(
            uuid.get_version(),
            Some(
                Version::Max
                    | Version::Custom
                    | Version::SortMac
                    | Version::Mac
                    | Version::Dce
                    | Version::Md5
                    | Version::Random
                    | Version::Sha1
                    | Version::SortRand
                    | Version::Nil
            )
        );

        is_valid_variant || is_valid_version
    }

    /// Converts the `TypeIdSuffix` to a UUID.
    ///
    /// This method decodes the base32-encoded suffix back into a UUID.
    ///
    /// # Returns
    ///
    /// The `Uuid` represented by this `TypeIdSuffix`.
    ///
    /// # Panics
    ///
    /// This method uses `expect()` internally, but it should never panic under normal circumstances.
    /// A panic would indicate a serious internal inconsistency in the `TypeIdSuffix` struct,
    /// which should be reported as a bug in the library.
    ///
    /// The reason it shouldn't panic is that:
    /// 1. The `TypeIdSuffix` is always created from a valid UUID or a valid base32 string.
    /// 2. All creation methods (`new()`, `from_str()`, `From<Uuid>`) perform thorough validation.
    /// 3. The internal representation is immutable after creation.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new::<V4>();
    /// let uuid = suffix.to_uuid();
    /// ```
    #[inline]
    #[must_use]
    pub fn to_uuid(&self) -> Uuid {
        let decoded_bytes = decode_base32(&self.0).expect("This should never fail because we've already validated the input");
        Uuid::from_bytes(decoded_bytes)
    }

    /// Returns a string slice of the ``TypeIdSuffix``.
    ///
    /// This method provides a way to access the underlying string representation
    /// of the ``TypeIdSuffix``.
    ///
    /// # Returns
    ///
    /// A string slice containing the base32-encoded ``TypeIdSuffix``.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new::<V4>();
    /// let suffix_str = suffix.as_ref();
    /// assert_eq!(suffix_str.len(), 26);
    /// ```
    #[must_use]
    #[inline]
    fn as_str(&self) -> &str {
        // SAFETY: This unwrap is safe because we know that the internal bytes
        // are always valid ASCII characters, which are valid UTF-8
        std::str::from_utf8(&self.0).unwrap()
    }
}

impl TypeIdSuffix {
    /// Checks if the ``TypeIdSuffix`` contains a V6 or V7 UUID.
    fn is_sortable(&self) -> bool {
        matches!(self.to_uuid().get_version(), Some(Version::SortMac | Version::SortRand))
    }
}

impl Ord for TypeIdSuffix {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_sortable() && other.is_sortable() {
            self.to_uuid().cmp(&other.to_uuid())
        } else {
            // Fall back to lexicographic ordering for non-V6/V7 UUIDs
            self.0.cmp(&other.0)
        }
    }
}

impl PartialOrd for TypeIdSuffix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for TypeIdSuffix {
    /// Creates a default ``TypeIdSuffix`` using `UUIDv7`.
    ///
    /// This implementation uses `V7` (`UUIDv7`) as the default UUID version
    /// for generating a ``TypeIdSuffix``.
    ///
    /// # Returns
    ///
    /// A new ``TypeIdSuffix`` instance generated from a `UUIDv7`.
    fn default() -> Self {
        Self::new::<V7>()
    }
}

impl Deref for TypeIdSuffix {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for TypeIdSuffix {
    fn as_ref(&self) -> &str {
        self
    }
}

impl Borrow<str> for TypeIdSuffix {
    fn borrow(&self) -> &str {
        self
    }
}

impl fmt::Display for TypeIdSuffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl From<&TypeIdSuffix> for Uuid {
    /// Converts a reference to a ``TypeIdSuffix`` into a Uuid.
    ///
    /// This implementation allows for efficient conversion from a `TypeIdSuffix`
    /// reference to a Uuid without unnecessary cloning.
    ///
    /// # Arguments
    ///
    /// * `value`: A reference to the ``TypeIdSuffix`` to convert.
    ///
    /// # Returns
    ///
    /// The `Uuid` represented by the ``TypeIdSuffix``.
    fn from(value: &TypeIdSuffix) -> Self {
        value.to_uuid()
    }
}

impl From<TypeIdSuffix> for Uuid {
    /// Converts a ``TypeIdSuffix`` into a Uuid.
    ///
    /// This implementation allows for conversion from a `TypeIdSuffix`
    /// to a Uuid, consuming the original ``TypeIdSuffix``.
    ///
    /// # Arguments
    ///
    /// * `value`: The ``TypeIdSuffix`` to convert.
    ///
    /// # Returns
    ///
    /// The `Uuid` represented by the ``TypeIdSuffix``.
    fn from(value: TypeIdSuffix) -> Self {
        value.to_uuid()
    }
}

impl FromStr for TypeIdSuffix {
    type Err = DecodeError;

    /// Parses a string slice into a ``TypeIdSuffix``.
    ///
    /// This method attempts to create a ``TypeIdSuffix`` from a string representation.
    /// It performs various validations to ensure the input string is a valid ``TypeIdSuffix``.
    ///
    /// # Arguments
    ///
    /// * `input`: The string slice to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the parsed ``TypeIdSuffix`` or a `DecodeError`.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The input string is not exactly 26 characters long.
    /// - The input string contains non-ASCII characters.
    /// - The first character of the input string is greater than '7'.
    /// - The input string contains invalid base32 characters.
    /// - The decoded UUID is not valid according to the `TypeId` specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::from_str("01h455vb4pex5vsknk084sn02q").unwrap();
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != 26 {
            return Err(DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidLength));
        }
        if !input.is_ascii() {
            return Err(DecodeError::InvalidSuffix(InvalidSuffixReason::NonAsciiCharacter));
        }
        if input.as_bytes()[0] > b'7' {
            return Err(DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidFirstCharacter));
        }
        let encoded_bytes: [u8; 26] = input.as_bytes().try_into().map_err(|_| DecodeError::InvalidSuffix(InvalidSuffixReason::InvalidLength))?;
        let decoded_bytes = decode_base32(&encoded_bytes)?;
        let uuid = Uuid::from_bytes(decoded_bytes);
        if !Self::is_valid_uuid(&uuid) {
            return Err(DecodeError::InvalidUuid(InvalidUuidReason::InvalidVersion));
        }
        Ok(Self(encoded_bytes))
    }
}

impl From<Uuid> for TypeIdSuffix {
    /// Converts a Uuid into a ``TypeIdSuffix``.
    ///
    /// This implementation allows for conversion from a Uuid to a ``TypeIdSuffix``.
    ///
    /// # Arguments
    ///
    /// * `value`: The Uuid to convert.
    ///
    /// # Returns
    ///
    /// A new ``TypeIdSuffix`` instance representing the given Uuid.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    /// use typeid_suffix::prelude::*;
    ///
    /// let uuid = Uuid::new_v4();
    /// let suffix: TypeIdSuffix = uuid.into();
    /// ```
    fn from(value: Uuid) -> Self {
        // SAFETY: The Uuid crate guarantees that the bytes are always 16 bytes long
        let encoded_bytes = encode_base32(value.as_bytes());
        Self(encoded_bytes)
    }
}

#[cfg(feature = "serde")]
impl Serialize for TypeIdSuffix {
    /// Serializes the `TypeIdSuffix` as its string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "serde")] {
    /// use typeid_suffix::prelude::*;
    /// let suffix = TypeIdSuffix::default();
    /// let json = serde_json::to_string(&suffix).unwrap();
    /// // The JSON string will be the suffix string, e.g., "\"01h455vb4pex5vsknk084sn02q\""
    /// assert!(json.starts_with("\"") && json.ends_with("\""));
    /// assert_eq!(json.trim_matches('"'), suffix.as_ref());
    /// # }
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TypeIdSuffix {
    /// Deserializes a `TypeIdSuffix` from its string representation.
    ///
    /// This expects a string that is a valid `TypeID` suffix.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid `TypeIdSuffix`
    /// (e.g., incorrect length, invalid characters, invalid first character,
    /// or decodes to an invalid UUID variant/version).
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "serde")] {
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix_str = "\"01h455vb4pex5vsknk084sn02q\""; // JSON string
    /// let deserialized: TypeIdSuffix = serde_json::from_str(suffix_str).unwrap();
    ///
    /// let invalid_suffix_str = "\"invalid\"";
    /// let result: Result<TypeIdSuffix, _> = serde_json::from_str(invalid_suffix_str);
    /// assert!(result.is_err());
    /// # }
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}