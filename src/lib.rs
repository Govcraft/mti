//! # Magic Type ID (MTI): Empowering Distributed Systems with Intelligent Identifiers
//!
//! Welcome to `mti`, a Rust crate that brings the power of type-safe, prefix-enhanced identifiers to your distributed systems.
//! Built on the [TypeID Specification](https://github.com/jetpack-io/typeid), `mti` combines the uniqueness of UUIDs with
//! the readability and type safety of prefixed identifiers, offering a robust solution for managing identifiers across your applications.
//!
//! ## Why Magic Type ID?
//!
//! In the world of distributed systems and microservices, having unique, type-safe, and human-readable identifiers
//! is crucial. Magic Type ID solves this challenge by providing:
//!
//! - **Type Safety**: Embed type information directly in your identifiers.
//! - **Readability**: Human-readable prefixes make identifiers self-descriptive.
//! - **Uniqueness**: Utilizes UUIDs (including `UUIDv7`) for guaranteed global uniqueness.
//! - **Sortability**: When using `UUIDv7`, identifiers are inherently time-sortable.
//! - **Flexibility**: Support for various UUID versions and custom prefixes.
//! - **Ease of Use**: Intuitive API with "magical" creation methods.
//!
//! ## Quick Start
//!
//! Add `mti` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mti = "0.1.0"
//! ```
//!
//! Then, in your Rust code:
//!
//! ```rust
//! use std::str::FromStr;
//! use mti::prelude::*;
//!
//! // Create a MagicTypeId for a user
//! let user_id = "user".create_type_id::<V7>();
//! println!("New User ID: {}", user_id); // e.g., "user_01h455vb4pex5vsknk084sn02q"
//!
//! // Parse an existing MagicTypeId
//! let order_id = MagicTypeId::from_str("order_01h455vb4pex5vsknk084sn02q").unwrap();
//! assert_eq!(order_id.prefix().as_str(), "order");
//! ```
//!
//! ## Key Features
//!
//! ### 1. Effortless Creation with Type Safety
//!
//! Create identifiers with type information embedded:
//!
//! ```rust
//! use mti::prelude::*;
//!
//! let product_id = "product".create_type_id::<V4>();
//! let user_id = "user".create_type_id::<V7>();
//! ```
//!
//! ### 2. Flexible Prefix Handling
//!
//! Magic Type ID offers both infallible and fallible prefix creation:
//!
//! ```rust
//! use mti::prelude::*;
//!
//! // Infallible - always produces a valid prefix
//! let sanitized_id = "Invalid Prefix!".create_type_id::<V7>();
//! assert!(sanitized_id.to_string().starts_with("invalidprefix_"));
//!
//! // Fallible - returns an error for invalid prefixes
//! let result = "Invalid Prefix!".try_create_type_id::<V7>();
//! assert!(result.is_err());
//! ```
//!
//! ### 3. UUID Version Flexibility
//!
//! Support for UUID versions 1 through 7, defaulting to the time-sortable `UUIDv7`:
//!
//! ```rust
//! use mti::prelude::*;
//!
//! let v4_id = "data".create_type_id::<V4>();
//! let v7_id = "data".create_type_id::<V7>();
//! ```
//!
//! ### 4. Seamless String-Like Behavior
//!
//! `MagicTypeId`s can be used in most contexts where you'd use a string:
//!
//! ```rust
//! use mti::prelude::*;
//!
//! let id = "user".create_type_id::<V7>();
//! assert!(id.starts_with("user_"));
//! assert_eq!(id.len(), 31);
//! ```
//!
//! ### 5. Robust Error Handling
//!
//! Comprehensive error types for invalid prefixes or UUIDs:
//!
//! ```rust
//! use std::str::FromStr;
//! use mti::prelude::*;
//!
//! let result = MagicTypeId::from_str("invalid!_01h455vb4pex5vsknk084sn02q");
//! match result {
//!     Ok(_) => println!("Valid MagicTypeId"),
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```
//!
//! ## Advanced Usage
//!
//! ### Custom Type-Safe ID Types
//!
//! Create custom ID types for enhanced type safety:
//!
//! ```rust
//! use mti::prelude::*;
//!
//! struct UserId(MagicTypeId);
//! struct OrderId(MagicTypeId);
//!
//! impl UserId {
//!     fn new() -> Self {
//!         Self("user".create_type_id::<V7>())
//!     }
//! }
//!
//! impl OrderId {
//!     fn new() -> Self {
//!         Self("order".create_type_id::<V7>())
//!     }
//! }
//!
//! let user_id = UserId::new();
//! let order_id = OrderId::new();
//!
//! // Compile-time type safety
//! fn process_user(id: UserId) { /* ... */ }
//! fn process_order(id: OrderId) { /* ... */ }
//!
//! process_user(user_id);
//! process_order(order_id);
//! // process_user(order_id); // This would cause a compile-time error!
//! ```
//!
//! ### Seamless Database Integration
//!
//! `MagicTypeId`s can be easily integrated with database libraries:
//!
//! ```rust,ignore
//! use mti::prelude::*;
//! use some_db_library::Database;
//!
//! #[derive(Debug)]
//! struct User {
//!     id: MagicTypeId,
//!     name: String,
//! }
//!
//! fn create_user(db: &Database, name: &str) -> User {
//!     let user = User {
//!         id: "user".magic_type_id::<V7>(),
//!         name: name.to_string(),
//!     };
//!     db.insert("users", &user);
//!     user
//! }
//!
//! fn get_user(db: &Database, id: &MagicTypeId) -> Option<User> {
//!     db.get("users", id)
//! }
//! ```
//!
//! ## Performance and Safety
//!
//! Magic Type ID is designed with performance and safety in mind:
//!
//! - Zero-cost abstractions for string-like operations.
//! - Built on top of the thoroughly tested and verified `TypeIdPrefix` and `TypeIdSuffix` crates.
//! - Efficient UUID generation and manipulation.
//!
//! ## Learn More
//!
//! - Explore the [`MagicTypeId`] struct for core functionality.
//! - Check out the [`MagicTypeIdExt`] trait for powerful string extension methods.
//! - See the `errors` module for comprehensive error handling.
//!
//! ## Contributing
//!
//! We welcome contributions! Please see our [GitHub repository](https://github.com/Govcraft/mti) for issues, feature requests, and pull requests.
//!
//! ## License
//!
//! This project is licensed under either of
//!
//! - Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
//! - MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))
//!
//! at your option.
//!
//! Happy coding with Magic Type ID! 🎩✨

mod magic_type_id_ext;
mod magic_type_id;
mod errors;

/// A prelude module that re-exports the most commonly used types and traits.
///
/// This module provides a convenient way to import all the essential components
/// of the `mti` crate with a single `use` statement.
///
/// # Example
///
/// ```
/// use mti::prelude::*;
///
/// let user_id = "user".create_type_id::<V7>();
/// println!("Generated User ID: {}", user_id);
/// ```
pub mod prelude {
    /// Re-exports from the `typeid_prefix` crate, including `TypeIdPrefix` and related types.
    pub use typeid_prefix::prelude::*;

    /// Re-exports from the `typeid_suffix` crate, including `TypeIdSuffix`, `UuidVersion`, and UUID version types (e.g., `V4`, `V7`).
    pub use typeid_suffix::prelude::*;

    /// Re-exports error types from this crate, primarily `MagicTypeIdError`.
    pub use crate::errors::*;

    /// Re-exports the `MagicTypeId` struct, the core type of this crate.
    ///
    /// `MagicTypeId` represents a type-safe identifier combining a prefix and a UUID-based suffix.
    pub use crate::magic_type_id::MagicTypeId;

    /// Re-exports the `MagicTypeIdExt` trait, which provides extension methods for creating and manipulating `MagicTypeId`s.
    ///
    /// This trait is implemented for `str`, allowing for easy creation of `MagicTypeId`s from string literals.
    pub use crate::magic_type_id_ext::MagicTypeIdExt;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use typeid_prefix::prelude::PrefixFactory;
    use typeid_prefix::TypeIdPrefix;
    use typeid_suffix::prelude::*;

    use crate::errors::MagicTypeIdError;
    use crate::magic_type_id::MagicTypeId;
    use crate::magic_type_id_ext::MagicTypeIdExt;

    #[test]
    fn test_slug_id() {
        let suffix = TypeIdSuffix::default();
        let slug_id = MagicTypeId::new("prefix".create_prefix_sanitized(), suffix);
        let slug_two: MagicTypeId = "another_prefix_01h455vb4pex5vsknk084sn02q".parse().unwrap();
        let slug_bad: Result<MagicTypeId, MagicTypeIdError> = "another_prefix_01h455vb4pNOPEsknk084sn02q".parse();

        assert_eq!(slug_two.prefix().as_str(), "another_prefix");
        assert_eq!(slug_id.prefix().to_string().as_str(), "prefix");
        assert!(slug_bad.is_err());
    }

    #[test]
    fn test_slug_id_ext() {
        let slug_str = "prefix_01h455vb4pex5vsknk084sn02q";
        assert_eq!(slug_str.prefix_str().unwrap(), "prefix");
        assert_eq!(slug_str.prefix_str().unwrap().as_str(), "prefix");
        assert_eq!(slug_str.uuid_str().unwrap().len(), 36);

        let no_prefix = "01h455vb4pex5vsknk084sn02q";
        assert_eq!(no_prefix.prefix_str().unwrap(), "");
        assert!(no_prefix.prefix_str().unwrap().is_empty());
        assert_eq!(no_prefix.uuid_str().unwrap().len(), 36);

        let invalid_prefix = "Invalid_Prefix_01h455vb4pex5vsknk084sn02q";
        assert_eq!(invalid_prefix.create_prefix_sanitized().as_str(), "invalid_prefix_hvbpexvsknksnq");
        assert!(invalid_prefix.prefix_str().is_err());
        assert_eq!(invalid_prefix.uuid_str().unwrap().len(), 36);

        let invalid_suffix = "prefix_invalid";
        assert_eq!(invalid_suffix.prefix_str().unwrap(), "prefix");
        assert_eq!(invalid_suffix.prefix_str().unwrap().as_str(), "prefix");
        assert!(invalid_suffix.suffix_str().is_err());
    }

    #[test]
    fn test_nil() {
        let slug_id: MagicTypeId = "00000000000000000000000000".parse().unwrap();
        assert!(slug_id.prefix().is_empty());
        assert_eq!(slug_id.suffix().to_uuid().to_string(), "00000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn test_new_creation_methods() {
        // Test create_slug
        let slug: MagicTypeId = "test_prefix".create_type_id::<V7>();
        assert_eq!(slug.prefix().as_str(), "test_prefix");
        assert_eq!(slug.suffix().to_string().len(), 26);

        // Test create_slug with different UUID version
        let slug_other: MagicTypeId = "other_prefix".create_type_id::<V3>();
        assert_eq!(slug_other.prefix().as_str(), "other_prefix");
        assert_eq!(slug_other.suffix().to_string().len(), 26);

        // Test try_create_slug with valid prefix
        let slug_try: Result<MagicTypeId, _> = "valid_prefix".try_create_type_id::<V7>();
        assert!(slug_try.is_ok());
        assert_eq!(slug_try.unwrap().prefix().as_str(), "valid_prefix");

        // Test try_create_slug with invalid prefix
        let invalid_result: Result<MagicTypeId, _> = "Invalid Prefix".try_create_type_id::<V6>();
        assert!(invalid_result.is_err());
    }
    #[test]
    fn doc_smoke_tests(){
        // Sanitized creation (always produces a valid prefix)
        let sanitized_id = "Invalid Prefix!".create_type_id::<V7>();
        assert!(sanitized_id.to_string().starts_with("invalidprefix_"));
        // Strict creation (returns an error for invalid prefixes)
        let result = "Invalid Prefix!".try_create_type_id::<V7>();
        assert!(result.is_err());

        let id = "user".create_type_id::<V7>();
        assert!(id.starts_with("user_"));
        assert_eq!(id.len(), 31);

        // Use in string comparisons
        assert_eq!(id.as_str(), id.to_string());

        let id_str = "product_01h455vb4pex5vsknk084sn02q";
        let magic_id = MagicTypeId::from_str(id_str).unwrap();

        assert_eq!(magic_id.prefix().as_str(), "product");
        assert_eq!(magic_id.suffix().to_string(), "01h455vb4pex5vsknk084sn02q");

        // Extract UUID
        let uuid = magic_id.suffix().to_uuid();
        println!("Extracted UUID: {}", uuid);

        let namespace = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        let name = "example.com";
        let v5_uuid = Uuid::new_v5( & namespace, name.as_bytes());
        let domain_id = MagicTypeId::new(
            TypeIdPrefix::from_str("domain").unwrap(),
            TypeIdSuffix::from(v5_uuid)
        );
        assert_eq!(domain_id.uuid_str().unwrap(), "cfbff0d1-9375-5685-968c-48ce8b15ae17");
    }

}