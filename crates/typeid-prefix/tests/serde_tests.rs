//! Tests for serde serialization and deserialization of TypeIdPrefix.
//!
//! This module tests the serde implementation for the TypeIdPrefix struct,
//! ensuring that it correctly serializes to and deserializes from strings
//! while maintaining the validation rules of the TypeID specification.

#![cfg(feature = "serde")]
#![doc(hidden)]

use std::convert::TryFrom;
use typeid_prefix::TypeIdPrefix;

#[test]
fn test_serialize_typeidprefix() {
    // Create a valid TypeIdPrefix
    let prefix = TypeIdPrefix::try_from("valid_prefix").unwrap();

    // Serialize to JSON
    let serialized = serde_json::to_string(&prefix).unwrap();

    // Verify it serializes as a string
    assert_eq!(serialized, "\"valid_prefix\"");
}

#[test]
fn test_deserialize_valid_typeidprefix() {
    // Valid JSON string
    let json = "\"valid_prefix\"";

    // Deserialize
    let prefix: TypeIdPrefix = serde_json::from_str(json).unwrap();

    // Verify it deserializes correctly
    assert_eq!(prefix.as_str(), "valid_prefix");
}

#[test]
fn test_deserialize_invalid_typeidprefix() {
    // Invalid JSON strings (violate TypeIdPrefix rules)
    let long_string = format!("\"{}\"", "a".repeat(64)); // Too long (> 63 chars)
    let invalid_cases = vec![
        "\"UPPERCASE\"",               // Contains uppercase
        "\"_starts_with_underscore\"", // Starts with underscore
        "\"ends_with_underscore_\"",   // Ends with underscore
        "\"contains-hyphen\"",         // Contains invalid character
        &long_string,                  // Too long (> 63 chars)
    ];

    for invalid_json in invalid_cases {
        // Deserialization should fail
        let result: Result<TypeIdPrefix, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "Should fail to deserialize: {invalid_json}"
        );
    }
}

#[test]
fn test_roundtrip_serialization() {
    // Create a valid TypeIdPrefix
    let original = TypeIdPrefix::try_from("test_prefix").unwrap();

    // Serialize to JSON
    let serialized = serde_json::to_string(&original).unwrap();

    // Deserialize back
    let deserialized: TypeIdPrefix = serde_json::from_str(&serialized).unwrap();

    // Verify roundtrip
    assert_eq!(original, deserialized);
}

#[test]
fn test_empty_string() {
    // Empty string is NOT valid according to the validation rules
    let json = "\"\"";

    // Deserialization should fail
    let result: Result<TypeIdPrefix, _> = serde_json::from_str(json);
    assert!(result.is_err(), "Should fail to deserialize empty string");
}
