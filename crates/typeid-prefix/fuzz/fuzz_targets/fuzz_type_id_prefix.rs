#![no_main]
use libfuzzer_sys::fuzz_target;
use typeid_prefix::{TypeIdPrefix, Sanitize,};
use std::convert::TryFrom;
use libfuzzer_sys::arbitrary;
use crate::arbitrary::Arbitrary;

#[derive(Debug, Arbitrary)]
struct TypeIdPrefixInput {
    prefix: String,
}

fuzz_target!(|input: TypeIdPrefixInput| {
    // Test TypeIdPrefix::try_from with String
    let try_from_string_result = TypeIdPrefix::try_from(input.prefix.clone());

    // Test TypeIdPrefix::try_from with &str
    let try_from_str_result = TypeIdPrefix::try_from(input.prefix.as_str());

    // Test sanitize_and_create
    let sanitized = input.prefix.sanitize_and_create();

    // Ensure that try_from results are consistent for String and &str
    assert_eq!(try_from_string_result.is_ok(), try_from_str_result.is_ok());

    if let Ok(ref prefix) = try_from_string_result {
        // Ensure that the result is always valid
        assert!(TypeIdPrefix::try_from(prefix.as_str()).is_ok());
    }

    // Ensure that the sanitized version is always valid
    assert!(TypeIdPrefix::try_from(sanitized.as_str()).is_ok());

    // Check properties of sanitized output
    assert!(sanitized.len() <= 63);
    assert!(sanitized.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
    assert!(!sanitized.starts_with('_'));
    assert!(!sanitized.ends_with('_'));

    // If the original input was valid, ensure it matches the sanitized version
    if try_from_string_result.is_ok() {
        assert_eq!(input.prefix, sanitized);
    }
});