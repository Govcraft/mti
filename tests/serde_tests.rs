#![doc = "Tests for Serde serialization and deserialization of `MagicTypeId`."]
//! Tests for Serde serialization and deserialization of `MagicTypeId`.
//!
//! This module verifies that `MagicTypeId` instances can be correctly
//! serialized to and deserialized from various formats using Serde.

use mti::prelude::*;
use serde_json;
use std::str::FromStr;

#[test]
fn test_serialize_deserialize_with_prefix() {
    // Create a MagicTypeId with a prefix
    let prefix = TypeIdPrefix::from_str("user").unwrap();
    let suffix = TypeIdSuffix::new::<Nil>(); // Using Nil for deterministic testing
    let magic_id = MagicTypeId::new(prefix, suffix);
    
    // Serialize to JSON
    let json = serde_json::to_string(&magic_id).unwrap();
    
    // Verify it's serialized as a string
    assert_eq!(json, "\"user_00000000000000000000000000\"");
    
    // Deserialize from JSON
    let deserialized: MagicTypeId = serde_json::from_str(&json).unwrap();
    
    // Verify it matches the original
    assert_eq!(deserialized, magic_id);
    assert_eq!(deserialized.prefix().as_str(), "user");
    assert_eq!(deserialized.suffix().to_string(), "00000000000000000000000000");
}

#[test]
fn test_serialize_deserialize_without_prefix() {
    // Create a MagicTypeId without a prefix
    let prefix = TypeIdPrefix::default();
    let suffix = TypeIdSuffix::new::<Nil>(); // Using Nil for deterministic testing
    let magic_id = MagicTypeId::new(prefix, suffix);
    
    // Serialize to JSON
    let json = serde_json::to_string(&magic_id).unwrap();
    
    // Verify it's serialized as a string
    assert_eq!(json, "\"00000000000000000000000000\"");
    
    // Deserialize from JSON
    let deserialized: MagicTypeId = serde_json::from_str(&json).unwrap();
    
    // Verify it matches the original
    assert_eq!(deserialized, magic_id);
    assert!(deserialized.prefix().as_str().is_empty());
    assert_eq!(deserialized.suffix().to_string(), "00000000000000000000000000");
}

#[test]
fn test_deserialize_invalid_format() {
    // Try to deserialize an invalid string
    let result: Result<MagicTypeId, _> = serde_json::from_str("\"invalid!_00000000000000000000000000\"");
    
    // Verify it fails
    assert!(result.is_err());
}

#[test]
fn test_in_complex_structure() {
    // Test serialization/deserialization in a more complex structure
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct User {
        id: MagicTypeId,
        name: String,
    }
    
    let user = User {
        id: MagicTypeId::from_str("user_00000000000000000000000000").unwrap(),
        name: "John Doe".to_string(),
    };
    
    // Serialize to JSON
    let json = serde_json::to_string(&user).unwrap();
    
    // Deserialize from JSON
    let deserialized: User = serde_json::from_str(&json).unwrap();
    
    // Verify it matches the original
    assert_eq!(deserialized, user);
    assert_eq!(deserialized.id.prefix().as_str(), "user");
    assert_eq!(deserialized.id.suffix().to_string(), "00000000000000000000000000");
}