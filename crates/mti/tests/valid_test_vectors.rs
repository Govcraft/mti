//! Tests for handling valid `TypeID` string formats and values.
//!
//! This module uses a set of predefined valid test vectors to ensure
//! that the parsing and validation logic correctly accepts and processes
//! well-formed `TypeID` strings.
use mti::prelude::*;

macro_rules! create_test_vector {
    ($name:ident, $prefix:expr, $typeid:expr, $uuid:expr) => {
        #[test]
        fn $name() {

            // Test decoding
            assert_eq!($typeid.uuid_str().unwrap(), $uuid, "Decoding failed");
            assert_eq!($typeid.prefix_str().unwrap(), $prefix, "Decoding failed");

            // Test encoding

            // the quick way (infallable)
            // let type_id = $typeid.mti();
            // assert_eq!(*(type_id.prefix()), $prefix);

            //the manual way
            // here we create the prefix
            // With typeid_prefix@1.0.5, empty prefixes are correctly rejected
            // Use TypeIdPrefix::default() for empty prefixes
            let prefix = if $prefix.is_empty() {
                TypeIdPrefix::default()
            } else {
                TypeIdPrefix::try_from($prefix).unwrap()
            };
            
            // For empty prefixes, we can't directly compare with the empty string
            // as TypeIdPrefix::default() doesn't equal an empty string
            if !$prefix.is_empty() {
                assert_eq!(prefix, $prefix, "Encoding prefix failed");
            }

            // here we create the suffix
            let uuid = uuid::Uuid::parse_str($uuid).unwrap();
            let suffix : TypeIdSuffix = uuid.into();
            assert_eq!(suffix.uuid_str().unwrap(), uuid.to_string(), "Encoding suffix failed");

            // here we combine the two to create the full type_id
            let type_id = MagicTypeId::new(prefix, suffix);
            assert_eq!(type_id, $typeid, "Encoding failed");
        }
    };
}

// Now, use the macro to create individual tests for each vector
create_test_vector!(test_nil, "", "00000000000000000000000000", "00000000-0000-0000-0000-000000000000");
create_test_vector!(test_one, "", "00000000000000000000000001", "00000000-0000-0000-0000-000000000001");
create_test_vector!(test_ten, "", "0000000000000000000000000a", "00000000-0000-0000-0000-00000000000a");
create_test_vector!(test_sixteen, "", "0000000000000000000000000g", "00000000-0000-0000-0000-000000000010");
create_test_vector!(test_thirty_two, "", "00000000000000000000000010", "00000000-0000-0000-0000-000000000020");
create_test_vector!(test_max_valid, "", "7zzzzzzzzzzzzzzzzzzzzzzzzz", "ffffffff-ffff-ffff-ffff-ffffffffffff");
create_test_vector!(test_valid_alphabet, "prefix", "prefix_0123456789abcdefghjkmnpqrs", "0110c853-1d09-52d8-d73e-1194e95b5f19");
create_test_vector!(test_valid_uuidv7, "prefix", "prefix_01h455vb4pex5vsknk084sn02q", "01890a5d-ac96-774b-bcce-b302099a8057");
create_test_vector!(test_prefix_underscore, "pre_fix", "pre_fix_00000000000000000000000000", "00000000-0000-0000-0000-000000000000");