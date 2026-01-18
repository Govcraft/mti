//! Specification test vectors for `TypeIdSuffix`.
//!
//! These tests verify that `TypeIdSuffix` correctly implements the `TypeID` specification
//! by testing against known test vectors. Each test vector consists of a `TypeID` suffix
//! and its corresponding UUID representation.

use typeid_suffix::prelude::*;
use uuid::Uuid;
use std::str::FromStr;
macro_rules! create_test_vector {
    ($name:ident, $typeid:expr, $uuid:expr) => {
        #[test]
        fn $name() {
            let typeid = $typeid;
            let uuid_str = $uuid;

            // Remove the prefix if present
            let suffix = typeid.split('_').last().unwrap();

            // Test decoding
            let decoded = TypeIdSuffix::from_str(suffix)
                .unwrap_or_else(|e| panic!("Failed to decode `TypeId`suffix: {:?}", e));
            let uuid = Uuid::parse_str(uuid_str)
                .unwrap_or_else(|e| panic!("Failed to parse UUID: {:?}", e));
            assert_eq!(Uuid::try_from(&decoded).unwrap(), uuid, "Decoding failed");

            // Test encoding
            let encoded : TypeIdSuffix = uuid.into();
            assert_eq!(encoded, decoded, "Encoding failed");
        }
    };
}

// Now, use the macro to create individual tests for each vector
create_test_vector!(test_nil, "00000000000000000000000000", "00000000-0000-0000-0000-000000000000");
create_test_vector!(test_one, "00000000000000000000000001", "00000000-0000-0000-0000-000000000001");
create_test_vector!(test_ten, "0000000000000000000000000a", "00000000-0000-0000-0000-00000000000a");
create_test_vector!(test_sixteen, "0000000000000000000000000g", "00000000-0000-0000-0000-000000000010");
create_test_vector!(test_thirty_two, "00000000000000000000000010", "00000000-0000-0000-0000-000000000020");
create_test_vector!(test_max_valid, "7zzzzzzzzzzzzzzzzzzzzzzzzz", "ffffffff-ffff-ffff-ffff-ffffffffffff");
create_test_vector!(test_valid_alphabet, "prefix_0123456789abcdefghjkmnpqrs", "0110c853-1d09-52d8-d73e-1194e95b5f19");
create_test_vector!(test_valid_uuidv7, "prefix_01h455vb4pex5vsknk084sn02q", "01890a5d-ac96-774b-bcce-b302099a8057");
create_test_vector!(test_prefix_underscore, "pre_fix_00000000000000000000000000", "00000000-0000-0000-0000-000000000000");