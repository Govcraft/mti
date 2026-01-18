//! Tests for handling invalid `TypeID` string formats and values.
//!
//! This module uses a set of predefined invalid test vectors to ensure
//! that the parsing and validation logic correctly rejects malformed
//! or otherwise incorrect `TypeID` strings.
use mti::prelude::*;
use std::str::FromStr;
macro_rules! create_test_vector {
    ($name:ident, $prefix:expr, $typeid:expr, $uuid:expr) => {
        #[test]
        fn $name() {

            // all of these typeid's should parse into an error
            let type_id = MagicTypeId::from_str($typeid);
            assert!(type_id.is_err());

            // however some of the prefixes are not invalid. this is a good place to show
            // how sanitizing allows creating a valid TypeId from dirty prefix input
            let sanitized_prefix = $prefix.create_prefix_sanitized();
            
            // With typeid_prefix@1.0.5, empty prefixes are correctly rejected
            // Check if the sanitized prefix is empty
            if sanitized_prefix.as_str().is_empty() {
                // If empty, we expect an error when trying to create a TypeId
                let new_type_id = sanitized_prefix.try_create_type_id::<V7>();
                assert!(new_type_id.is_err());
            } else {
                // If not empty, we expect success
                let new_type_id = sanitized_prefix.try_create_type_id::<V7>();
                assert!(new_type_id.is_ok());
            }
        }
    };
}

// Now, use the macro to create individual tests for each vector
create_test_vector!(test_prefix_uppercase, "PREFIX", "PREFIX_00000000000000000000000000", "");
create_test_vector!(test_prefix_numeric, "12345", "12345_00000000000000000000000000", "");
create_test_vector!(test_prefix_period, "pre.fix", "pre.fix_00000000000000000000000000", "");
create_test_vector!(test_prefix_non_ascii, "préfix", "préfix_00000000000000000000000000", "");
create_test_vector!(test_prefix_spaces, "  prefix", "  prefix_00000000000000000000000000", "");
create_test_vector!(test_prefix_64_chars, "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl", "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl_00000000000000000000000000", "");
create_test_vector!(test_separator_empty_prefix, "", "_00000000000000000000000000", "");
create_test_vector!(test_separator_empty, "", "_", "");
create_test_vector!(test_suffix_short, "prefix", "prefix_1234567890123456789012345", "");
create_test_vector!(test_suffix_long, "prefix", "prefix_123456789012345678901234567", "");
create_test_vector!(test_suffix_spaces, "prefix", "prefix_1234567890123456789012345 ", "");
create_test_vector!(test_suffix_uppercase, "prefix", "prefix_0123456789ABCDEFGHJKMNPQRS", "");
create_test_vector!(test_suffix_hyphens, "prefix", "prefix_123456789-123456789-123456", "");
create_test_vector!(test_suffix_wrong_alphabet, "prefix", "prefix_ooooooiiiiiiuuuuuuulllllll", "");
create_test_vector!(test_suffix_ambiguous_crockford, "prefix", "prefix_i23456789ol23456789oi23456", "");
create_test_vector!(test_suffix_hyphens_crockford, "prefix", "prefix_123456789-0123456789-0123456", "");
create_test_vector!(test_suffix_overflow, "prefix", "prefix_8zzzzzzzzzzzzzzzzzzzzzzzzz", "");
create_test_vector!(test_prefix_underscore_start, "_prefix", "_prefix_00000000000000000000000000", "");
create_test_vector!(test_prefix_underscore_end, "prefix_", "prefix__00000000000000000000000000", "");