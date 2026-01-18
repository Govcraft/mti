//! Tests for the `MagicTypeIdExt` trait and its implementations.
//!
//! This module focuses on verifying the extended functionalities provided
//! for `MagicTypeId` instances, such as custom formatting or
//! additional utility methods.
#[cfg(test)]
mod tests {
    use mti::prelude::*;

    #[test]
    fn test_magic_typeid_ext_capabilities() {
        // Test with a valid `MagicTypeId` string
        let valid_str = "user_01h455vb4pex5vsknk084sn02q";

        // Test prefix() method
        assert_eq!(valid_str.prefix_str().unwrap(), "user");

        // Test prefix() method
        let try_prefix = valid_str.prefix_str().unwrap();
        assert_eq!(try_prefix, "user");

        // Test try_suffix() method
        let try_suffix = valid_str.suffix_str().unwrap();
        assert_eq!(try_suffix.len(), 26);

        // Test try_uuid() method
        let try_uuid = valid_str.uuid_str().unwrap();
        assert_eq!(try_uuid.len(), 36); // UUID string length

        // Test magic_type_id() method
        let magic_id = "user".create_type_id::<V7>();
        assert_eq!(magic_id.prefix().as_str(), "user");
        assert_eq!(magic_id.suffix().to_string().len(), 26);

        // Test try_magic_type_id() method
        let try_magic_id = "user".try_create_type_id::<V4>().unwrap();
        assert_eq!(try_magic_id.prefix().as_str(), "user");
        assert_eq!(try_magic_id.suffix().to_string().len(), 26);

        // Test with a string containing only a suffix
        let suffix_only = "01h455vb4pex5vsknk084sn02q";
        assert_eq!(suffix_only.prefix_str().unwrap(), "");
        assert!(suffix_only.prefix_str().unwrap().is_empty());
        assert_eq!(suffix_only.suffix_str().unwrap().len(), 26);
        assert_eq!(suffix_only.uuid_str().unwrap().len(), 36);

        // Test with an invalid prefix
        let invalid_prefix = "Invalid Prefix_01h455vb4pex5vsknk084sn02q";
        assert_eq!(
            invalid_prefix.create_prefix_sanitized().as_str(),
            "invalidprefix_hvbpexvsknksnq"
        );
        assert!(invalid_prefix.prefix_str().is_err());
        assert_eq!(invalid_prefix.suffix_str().unwrap().len(), 26);
        assert_eq!(invalid_prefix.uuid_str().unwrap().len(), 36);

        // Test with an invalid suffix
        let invalid_suffix = "user_invalid_suffix";
        assert_eq!(invalid_suffix.prefix_str().unwrap(), "user_invalid");
        assert!(invalid_suffix.prefix_str().is_ok());
        assert!(invalid_suffix.suffix_str().is_err());
        assert!(invalid_suffix.uuid_str().is_err());

        // Test creating new `MagicTypeId` with different UUID versions
        let v4_id = "test_v4".create_type_id::<V4>();
        let v7_id = "test_v7".create_type_id::<V7>();
        assert_ne!(v4_id.suffix(), v7_id.suffix());

        // Test try_create with invalid input
        let invalid_create = "123".try_create_type_id::<V4>();
        assert!(invalid_create.is_err());
    }
}
