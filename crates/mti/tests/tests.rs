//! General integration and core functionality tests for the `mti` crate.
//!
//! This module covers various aspects of `MagicTypeId` creation, parsing,
//! and manipulation, ensuring the fundamental features work as expected.
use mti::prelude::*;
use std::collections::HashMap;

fn takes_str(s: &str) {
    assert!(s.starts_with("user_"));
}

fn takes_as_ref<T: AsRef<str>>(s: T) {
    assert!(s.as_ref().starts_with("user_"));
}

#[test]
fn test_magic_typeid_capabilities() {
    // Create a `MagicTypeId`
    let prefix: TypeIdPrefix = "user".try_into().expect("Valid prefix");
    let suffix = TypeIdSuffix::new::<Nil>();  // Using Nil for deterministic testing
    let magic_id = MagicTypeId::new(prefix, suffix);

    // Test string-like behavior
    let magic_str = magic_id.to_string();
    assert!(magic_id.starts_with("user_"));
    assert_eq!(magic_id.len(), 31);  // "user" + "_" + 26 chars for suffix
    assert_eq!(magic_id, "user_00000000000000000000000000");  // "user" + "_" + Nil UUID

    // Can be used as a string slice
    let len = magic_id.len();
    assert_eq!(len, 31);
    assert!(magic_id.contains("user_"));
    assert!(!magic_id.contains("admin_"));

    // Can be compared directly with strings
    assert_eq!(magic_id, magic_str);
    assert_eq!(magic_str, magic_id);
    assert!(magic_id.starts_with("user_"));
    assert!(magic_id.ends_with(&magic_str[5..]));

    // Can be used with string methods
    let upper = magic_id.to_uppercase();
    assert!(upper.starts_with("USER_"));

    // Test Deref coercion
    takes_str(&magic_id);

    // Test AsRef<str>
    takes_as_ref(&magic_id);

    // Test Borrow<str>
    let mut map = HashMap::new();
    let _ = map.insert(magic_id.clone(), "value");
    assert!(map.contains_key(magic_str.as_str()));

    // Test component access
    assert_eq!(magic_id.prefix().as_str(), "user");
    assert_eq!(magic_id.suffix().to_string().len(), 26);

    // Test FromStr
    let parsed: MagicTypeId = magic_str.parse().expect("Valid MagicTypeId");
    assert_eq!(parsed, magic_id);

    // Test with no prefix
    let no_prefix_id = MagicTypeId::new(TypeIdPrefix::default(), TypeIdSuffix::new::<V4>());
    assert!(!no_prefix_id.contains('_'));
    assert_eq!(no_prefix_id.len(), 26);

    // Test PartialEq implementations
    assert_eq!(magic_id, magic_str.as_str());
    assert_eq!(magic_str.as_str(), magic_id);
}
#[test]
fn test_general() {
    const PREFIX: &str = "user";
    assert!(PREFIX.try_create_prefix().is_ok());
    assert!("HelloWorld".try_create_prefix().is_err());
    assert!("HelloWorld".try_create_type_id::<V7>().is_err());
    assert_eq!("HelloWorld".create_type_id::<V7>().prefix(), "helloworld");
}

