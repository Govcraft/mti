//! Namespace identifiers for V3 and V5 UUID generation.
//!
//! This module provides the [`NamespaceId`] type, which represents a validated
//! namespace identifier used for generating name-based UUIDs (V3 and V5).
//!
//! # Well-Known Namespaces
//!
//! RFC 4122 defines four well-known namespace identifiers:
//!
//! - [`NamespaceId::DNS`] - For domain names
//! - [`NamespaceId::URL`] - For URLs
//! - [`NamespaceId::OID`] - For ISO OIDs
//! - [`NamespaceId::X500`] - For X.500 DNs
//!
//! # Examples
//!
//! ```
//! use typeid_suffix::prelude::*;
//! use std::str::FromStr;
//!
//! // Use a well-known namespace
//! let dns_namespace = NamespaceId::DNS;
//!
//! // Parse a custom namespace from a UUID string
//! let custom = NamespaceId::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
//!
//! // Create from a Uuid directly
//! let from_uuid = NamespaceId::from(Uuid::new_v4());
//! ```

use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

use crate::errors::DecodeError;

/// A validated namespace identifier for V3 and V5 UUIDs.
///
/// Namespaces provide a way to create deterministic UUIDs from names.
/// This type wraps a UUID that is used as the namespace parameter for
/// generating name-based UUIDs (V3 with MD5, V5 with SHA-1).
///
/// # Well-Known Namespaces
///
/// RFC 4122 defines four well-known namespace identifiers available as
/// associated constants:
///
/// - [`NamespaceId::DNS`] - For domain names (`6ba7b810-9dad-11d1-80b4-00c04fd430c8`)
/// - [`NamespaceId::URL`] - For URLs (`6ba7b811-9dad-11d1-80b4-00c04fd430c8`)
/// - [`NamespaceId::OID`] - For ISO OIDs (`6ba7b812-9dad-11d1-80b4-00c04fd430c8`)
/// - [`NamespaceId::X500`] - For X.500 DNs (`6ba7b814-9dad-11d1-80b4-00c04fd430c8`)
///
/// # Examples
///
/// ```
/// use typeid_suffix::prelude::*;
///
/// // Use a well-known namespace
/// let dns_namespace = NamespaceId::DNS;
///
/// // Parse a custom namespace from string
/// let custom = NamespaceId::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
///
/// // Create from a Uuid directly
/// let from_uuid = NamespaceId::from(Uuid::new_v4());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamespaceId(Uuid);

impl NamespaceId {
    /// The DNS namespace as defined in RFC 4122.
    ///
    /// Use this namespace for domain names.
    ///
    /// UUID: `6ba7b810-9dad-11d1-80b4-00c04fd430c8`
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new_v5(NamespaceId::DNS, b"example.com");
    /// ```
    pub const DNS: Self = Self(Uuid::from_u128(0x6ba7_b810_9dad_11d1_80b4_00c0_4fd4_30c8));

    /// The URL namespace as defined in RFC 4122.
    ///
    /// Use this namespace for URLs.
    ///
    /// UUID: `6ba7b811-9dad-11d1-80b4-00c04fd430c8`
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new_v5(NamespaceId::URL, b"https://example.com/path");
    /// ```
    pub const URL: Self = Self(Uuid::from_u128(0x6ba7_b811_9dad_11d1_80b4_00c0_4fd4_30c8));

    /// The OID namespace as defined in RFC 4122.
    ///
    /// Use this namespace for ISO OIDs.
    ///
    /// UUID: `6ba7b812-9dad-11d1-80b4-00c04fd430c8`
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new_v5(NamespaceId::OID, b"1.3.6.1");
    /// ```
    pub const OID: Self = Self(Uuid::from_u128(0x6ba7_b812_9dad_11d1_80b4_00c0_4fd4_30c8));

    /// The X500 namespace as defined in RFC 4122.
    ///
    /// Use this namespace for X.500 DNs.
    ///
    /// UUID: `6ba7b814-9dad-11d1-80b4-00c04fd430c8`
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let suffix = TypeIdSuffix::new_v5(NamespaceId::X500, b"cn=John Doe,o=Acme,c=US");
    /// ```
    pub const X500: Self = Self(Uuid::from_u128(0x6ba7_b814_9dad_11d1_80b4_00c0_4fd4_30c8));

    /// Creates a new `NamespaceId` from a UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID to use as the namespace identifier.
    ///
    /// # Returns
    ///
    /// A new `NamespaceId` wrapping the provided UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let uuid = Uuid::new_v4();
    /// let namespace = NamespaceId::new(uuid);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns a reference to the inner UUID value.
    ///
    /// # Returns
    ///
    /// A reference to the wrapped UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let namespace = NamespaceId::DNS;
    /// let uuid_ref = namespace.as_uuid();
    /// assert_eq!(uuid_ref.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Converts the `NamespaceId` into its inner UUID.
    ///
    /// # Returns
    ///
    /// The wrapped UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_suffix::prelude::*;
    ///
    /// let namespace = NamespaceId::DNS;
    /// let uuid = namespace.into_uuid();
    /// assert_eq!(uuid.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    /// ```
    #[must_use]
    #[inline]
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl fmt::Display for NamespaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for NamespaceId {
    /// Converts a UUID into a `NamespaceId`.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID to convert.
    ///
    /// # Returns
    ///
    /// A new `NamespaceId` wrapping the UUID.
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl From<NamespaceId> for Uuid {
    /// Converts a `NamespaceId` into its inner UUID.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The `NamespaceId` to convert.
    ///
    /// # Returns
    ///
    /// The wrapped UUID.
    fn from(namespace: NamespaceId) -> Self {
        namespace.0
    }
}

impl FromStr for NamespaceId {
    type Err = DecodeError;

    /// Parses a string slice into a `NamespaceId`.
    ///
    /// This method attempts to parse a UUID string and wrap it in a `NamespaceId`.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the parsed `NamespaceId` or a `DecodeError`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the string is not a valid UUID format.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use typeid_suffix::prelude::*;
    ///
    /// let namespace = NamespaceId::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
    /// assert_eq!(namespace, NamespaceId::DNS);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid =
            Uuid::parse_str(s).map_err(|_| DecodeError::InvalidNamespace(s.to_string()))?;
        Ok(Self::new(uuid))
    }
}

impl AsRef<Uuid> for NamespaceId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_dns_constant_matches_rfc4122() {
        assert_eq!(
            NamespaceId::DNS.to_string(),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn namespace_url_constant_matches_rfc4122() {
        assert_eq!(
            NamespaceId::URL.to_string(),
            "6ba7b811-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn namespace_oid_constant_matches_rfc4122() {
        assert_eq!(
            NamespaceId::OID.to_string(),
            "6ba7b812-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn namespace_x500_constant_matches_rfc4122() {
        assert_eq!(
            NamespaceId::X500.to_string(),
            "6ba7b814-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn from_str_valid_uuid() {
        let result = NamespaceId::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NamespaceId::DNS);
    }

    #[test]
    fn from_str_invalid_uuid() {
        let result = NamespaceId::from_str("not-a-uuid");
        assert!(result.is_err());
        match result {
            Err(DecodeError::InvalidNamespace(s)) => assert_eq!(s, "not-a-uuid"),
            _ => panic!("Expected InvalidNamespace error"),
        }
    }

    #[test]
    fn display_formats_as_uuid() {
        let namespace = NamespaceId::DNS;
        assert_eq!(
            format!("{namespace}"),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn from_uuid_conversion() {
        let uuid = Uuid::new_v4();
        let namespace = NamespaceId::from(uuid);
        assert_eq!(*namespace.as_uuid(), uuid);
    }

    #[test]
    fn into_uuid_conversion() {
        let namespace = NamespaceId::DNS;
        let uuid: Uuid = namespace.into();
        assert_eq!(uuid.to_string(), "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    }

    #[test]
    fn as_ref_provides_uuid_reference() {
        let namespace = NamespaceId::DNS;
        let uuid_ref: &Uuid = namespace.as_ref();
        assert_eq!(
            uuid_ref.to_string(),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn new_creates_namespace_from_uuid() {
        let uuid = Uuid::new_v4();
        let namespace = NamespaceId::new(uuid);
        assert_eq!(namespace.into_uuid(), uuid);
    }

    #[test]
    fn as_uuid_returns_reference() {
        let namespace = NamespaceId::DNS;
        assert_eq!(
            namespace.as_uuid().to_string(),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn namespace_equality() {
        let ns1 = NamespaceId::DNS;
        let ns2 = NamespaceId::from_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        assert_eq!(ns1, ns2);
    }

    #[test]
    fn namespace_inequality() {
        assert_ne!(NamespaceId::DNS, NamespaceId::URL);
        assert_ne!(NamespaceId::OID, NamespaceId::X500);
    }
}
