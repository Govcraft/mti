//! # `TypeID` Suffix
//!
//! `typeid_suffix` is a Rust library that implements the suffix portion of the
//! [TypeID Specification](https://github.com/jetpack-io/typeid). It provides
//! functionality to work with `TypeID` suffixes, which are base32-encoded
//! representations of UUIDs used in the `TypeID` system.
//!
//! This crate offers a robust, efficient, and user-friendly way to generate,
//! encode, decode, and validate `TypeID` suffixes, supporting various UUID versions.
//!
//! ## Features
//!
//! - **UUID Version Support**: Implements support for `UUIDv7` and other UUID versions.
//! - **Flexible Architecture**: Generic implementation allows for handling various UUID versions.
//! - **Base32 Encoding/Decoding**: Efficient encoding and decoding of UUIDs to/from base32 `TypeID` suffixes.
//! - **Error Handling**: Comprehensive error types for invalid suffixes and UUIDs.
//! - **Validation**: Robust validation for `TypeID` suffixes and UUIDs.
//! - **Zero-cost Abstractions**: Designed to have minimal runtime overhead.
//! - **Optional Tracing**: Integrates with the `tracing` crate for logging (optional feature).
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! typeid_suffix = "1.3.0"
//! ```
//!
//! ## Usage Examples
//!
//! ### Creating a `TypeID` Suffix
//!
//! By default, calling `TypeIdSuffix::default()` produces a suffix made from a `UUIDv7`
//! using the current timestamp.
//!
//! ```rust
//! use typeid_suffix::prelude::*;
//!
//! let default_suffix = TypeIdSuffix::default();
//! println!("Default `TypeID` suffix: {}", default_suffix);
//! ```
//!
//! You can also create a `TypeID` suffix for a specific UUID version:
//!
//! ```rust
//! use typeid_suffix::prelude::*;
//!
//! // Create a `TypeID` suffix from a `UUIDv7`
//! let suffix = TypeIdSuffix::new::<V7>();
//! println!("TypeID suffix: {}", suffix);
//! ```
//!
//! ### Parsing a `TypeID` Suffix
//!
//! ```rust
//! use std::str::FromStr;
//! use typeid_suffix::prelude::*;
//!
//! let suffix_str = "01h455vb4pex5vsknk084sn02q";
//! let parsed_suffix = TypeIdSuffix::from_str(suffix_str).expect("Valid suffix");
//! println!("Parsed suffix: {}", parsed_suffix);
//! ```
//!
//! ### Converting Between UUID and `TypeID` Suffix
//!
//! ```rust
//! use typeid_suffix::prelude::*;
//! use uuid::Uuid;
//!
//! let uuid = Uuid::new_v4();
//! let suffix: TypeIdSuffix = uuid.into();
//! println!("TypeID suffix: {}", suffix);
//!
//! let recovered_uuid: Uuid = suffix.try_into().expect("Valid UUID");
//! assert_eq!(uuid, recovered_uuid);
//! ```
//!
//! ### Error Handling
//!
//! ```rust
//! use typeid_suffix::prelude::*;
//! use std::str::FromStr;
//!
//! let result = TypeIdSuffix::from_str("invalid_suffix");
//! match result {
//!     Ok(_) => println!("Valid suffix"),
//!     Err(e) => println!("Invalid suffix: {}", e),
//! }
//! ```
//!
//! ## Optional Features
//!
//! - `instrument`: Enables logging with the `tracing` crate.
//! - `serde`: Enables serialization and deserialization support using the `serde` crate.
//!
//! To enable optional features, add them to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! typeid_suffix = { version = "1.2.0", features = ["instrument", "serde"] }
//! ```
//!
//! ### Serde Support
//!
//! When the `serde` feature is enabled, `TypeIdSuffix` implements `serde::Serialize` and `serde::Deserialize`.
//! This allows `TypeIdSuffix` instances to be easily serialized to and deserialized from various formats
//! like JSON, YAML, CBOR, etc., that are supported by Serde.
//!
//! `TypeIdSuffix` is serialized as its string representation and deserialized from a string.
//!
//! ```rust
//! # #[cfg(feature = "serde")] {
//! // Enable the serde feature in your Cargo.toml:
//! // typeid_suffix = { version = "1.2.0", features = ["serde"] }
//!
//! use typeid_suffix::prelude::*;
//! // Note: serde_json is used here as an example and would be a separate dependency.
//! use serde_json;
//!
//! // Serializing a TypeIdSuffix
//! let suffix = TypeIdSuffix::default();
//! let json_string = serde_json::to_string(&suffix).unwrap();
//! println!("Serialized: {}", json_string); // e.g., "01h455vb4pex5vsknk084sn02q" (with quotes)
//!
//! // Deserializing a TypeIdSuffix
//! let deserialized_suffix: TypeIdSuffix = serde_json::from_str(&json_string).unwrap();
//! assert_eq!(suffix, deserialized_suffix);
//!
//! // Example of deserialization error
//! let invalid_json = "\"invalid_suffix_too_long_or_invalid_chars\"";
//! let result: Result<TypeIdSuffix, _> = serde_json::from_str(invalid_json);
//! assert!(result.is_err());
//! # }
//! ```
//!
//! ## License
//!
//! This project is licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
//! * MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))
//!
//! at your option.
//!
//! ## Contributing
//!
//! Contributions are welcome! Please feel free to submit a Pull Request.

mod encoding;
mod errors;
mod namespace;
mod typeid_suffix;
mod versions;

/// The prelude module provides a convenient way to import commonly used items.
///
/// By adding `use typeid_suffix::prelude::*;` to your code, you can easily
/// access the most frequently used types and traits from this crate.
pub mod prelude {
    pub use std::str::FromStr;

    pub use uuid::{Uuid, Version};

    pub use crate::errors::*;
    pub use crate::namespace::NamespaceId;
    pub use crate::typeid_suffix::TypeIdSuffix;
    pub use crate::versions::*;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proptest::prelude::*;
    use uuid::{Uuid, Version};

    use crate::prelude::*;

    #[test]
    fn test_typeid_suffix_default() {
        let suffix = TypeIdSuffix::default();
        let uuid: Uuid = suffix.into();
        assert_eq!(uuid.get_version(), Some(Version::SortRand));
    }

    #[test]
    fn test_typeid_suffix_explicit_version() {
        let _suffix = TypeIdSuffix::new::<V4>();
    }

    prop_compose! {
        fn arbitrary_uuidv7()(timestamp in 0..=0xFFFF_FFFF_FFFF_FFFFu64, counter in 0..0x3FFFu16) -> Uuid {
            let mut bytes = [0u8; 16];
            bytes[0..8].copy_from_slice(&timestamp.to_be_bytes());
            bytes[8..10].copy_from_slice(&counter.to_be_bytes());
            bytes[6] = (bytes[6] & 0x0F) | 0x70; // Set version to 7
            bytes[8] = (bytes[8] & 0x3F) | 0x80; // Set variant to RFC4122
            Uuid::from_bytes(bytes)
        }

    }
    prop_compose! {
        fn arbitrary_uuid_other()(version in 1u8..=5u8, bytes in proptest::array::uniform16(any::<u8>())) -> Uuid {
            let mut uuid_bytes = bytes;
            uuid_bytes[6] = (uuid_bytes[6] & 0x0F) | (version << 4); // Set version
            uuid_bytes[8] = (uuid_bytes[8] & 0x3F) | 0x80; // Set variant to RFC4122
            Uuid::from_bytes(uuid_bytes)
        }
    }

    #[cfg(test)]
    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;

        #[test]
        fn test_typeid_suffix_serialize() {
            let suffix = TypeIdSuffix::default();
            let serialized = serde_json::to_string(&suffix).unwrap();
            assert!(serialized.starts_with('"') && serialized.ends_with('"'));
            assert_eq!(serialized.len(), 28); // 26 chars + 2 quotes
        }

        #[test]
        fn test_typeid_suffix_deserialize() {
            let suffix = TypeIdSuffix::default();
            let serialized = serde_json::to_string(&suffix).unwrap();
            let deserialized: TypeIdSuffix = serde_json::from_str(&serialized).unwrap();
            assert_eq!(suffix, deserialized);
        }

        #[test]
        fn test_typeid_suffix_deserialize_error() {
            let result = serde_json::from_str::<TypeIdSuffix>("\"invalid_suffix\"");
            assert!(result.is_err());
        }
    }

    proptest! {

        #[test]
        fn test_uuidv7_roundtrip(uuid in arbitrary_uuidv7()) {
            let suffix: TypeIdSuffix = uuid.into();
            let suffix_str = suffix.to_uuid().to_string();
            let v7_uuid = Uuid::from_str(&suffix_str).unwrap();
            let decoded: Uuid = v7_uuid;
            prop_assert_eq!(v7_uuid, decoded);
            prop_assert_eq!(suffix.len(), 26);
        }

        #[test]
        fn test_uuid_other_roundtrip(uuid in arbitrary_uuid_other()) {
            let suffix: TypeIdSuffix = uuid.into();
            let v4_uuid = Uuid::from_str(suffix.to_uuid().to_string().as_str()).unwrap();
            let decoded: Uuid = v4_uuid;
            prop_assert_eq!(v4_uuid, decoded);
            prop_assert_eq!(suffix.len(), 26);
        }

        #[test]
        fn test_uuidv7_fromstr(uuid in arbitrary_uuid_other()) {
            let suffix: TypeIdSuffix = uuid.into();
            let from_str = TypeIdSuffix::from_str(&suffix).unwrap();
            prop_assert_eq!(suffix, from_str);
        }

        #[test]
        fn test_uuid_other_fromstr(uuid in arbitrary_uuid_other()) {
            let suffix: TypeIdSuffix = uuid.into();
            let from_str = TypeIdSuffix::from_str(&suffix).unwrap();
            prop_assert_eq!(suffix, from_str);
        }

        #[test]
        fn test_invalid_suffix(s in "[0-9a-zA-Z]{26}") {
            if s.as_bytes()[0] > b'7' {
                prop_assert!(TypeIdSuffix::from_str(&s).is_err());
                prop_assert!(TypeIdSuffix::from_str(&s).is_err());
            }
        }


    }
}
