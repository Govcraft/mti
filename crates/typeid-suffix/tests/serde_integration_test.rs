//! Integration tests for serde functionality of `TypeIdSuffix`.
//!
//! These tests verify that `TypeIdSuffix` can be properly serialized and
//! deserialized using serde with various formats and in different contexts.

#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typeid_suffix::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: TypeIdSuffix,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Record {
    id: TypeIdSuffix,
    timestamp: u64,
    data: HashMap<String, TypeIdSuffix>,
}

#[test]
fn test_serde_in_struct() {
    let user = User {
        id: TypeIdSuffix::default(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };

    let serialized = serde_json::to_string(&user).unwrap();
    let deserialized: User = serde_json::from_str(&serialized).unwrap();

    assert_eq!(user, deserialized);
}

#[test]
fn test_serde_in_complex_struct() {
    let mut data = HashMap::new();
    let _ = data.insert("related_id".to_string(), TypeIdSuffix::default());
    let _ = data.insert("parent_id".to_string(), TypeIdSuffix::default());

    let record = Record {
        id: TypeIdSuffix::default(),
        timestamp: 1_620_000_000,
        data,
    };

    let serialized = serde_json::to_string(&record).unwrap();
    let deserialized: Record = serde_json::from_str(&serialized).unwrap();

    assert_eq!(record, deserialized);
}

#[test]
fn test_serde_with_different_formats() {
    let suffix = TypeIdSuffix::default();

    // Test with JSON
    let json = serde_json::to_string(&suffix).unwrap();
    let from_json: TypeIdSuffix = serde_json::from_str(&json).unwrap();
    assert_eq!(suffix, from_json);

    // Test with pretty-printed JSON
    let pretty_json = serde_json::to_string_pretty(&suffix).unwrap();
    let from_pretty_json: TypeIdSuffix = serde_json::from_str(&pretty_json).unwrap();
    assert_eq!(suffix, from_pretty_json);
}
