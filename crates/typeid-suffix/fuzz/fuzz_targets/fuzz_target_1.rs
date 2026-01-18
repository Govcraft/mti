#![no_main]

use libfuzzer_sys::fuzz_target;
use typeid_suffix::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

fuzz_target!(|data: &[u8]| {
    if data.len() < 16 {
        return;
    }

    // Test ``UUIDv7``
    let uuid = Uuid::from_bytes(data[..16].try_into().unwrap());
    if let Ok(suffix) = TypeIdSuffix::new(uuid) {
        // Test encoding
        assert_eq!(suffix.as_str().len(), 26);

        // Test decoding
        let decoded: Result<Uuid, _> = suffix.clone().try_into();
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap(), uuid);

        // Test FromStr
        let from_str = TypeIdSuffix::from_str(suffix.as_str());
        assert!(from_str.is_ok());
        assert_eq!(from_str.unwrap().as_str(), suffix.as_str());
    }

    // Test other UUID versions
    let suffix = TypeIdSuffix::new(uuid);
    if suffix.is_ok() {
        let suffix = suffix.unwrap();

        // Test encoding
        assert_eq!(suffix.as_str().len(), 26);

        // Test decoding
        let decoded: Result<Uuid, _> = suffix.clone().try_into();
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap(), uuid);

        // Test FromStr
        let from_str = TypeIdSuffix::from_str(suffix.as_str());
        assert!(from_str.is_ok());
        assert_eq!(from_str.unwrap().as_str(), suffix.as_str());
    }

    // Test invalid inputs
    if data.len() >= 26 {
        let invalid_str = std::str::from_utf8(&data[..26]);
        if let Ok(invalid_str) = invalid_str {
            let _ = TypeIdSuffix::from_str(invalid_str);
            let _ = TypeIdSuffix::from_str(invalid_str);
        }
    }
});