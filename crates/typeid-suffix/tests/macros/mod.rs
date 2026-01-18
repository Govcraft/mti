#[macro_export]
macro_rules! create_test_vectors {
    () => {
        const TEST_VECTORS: &str = include_str!("valid.json");

        fn run_test_vector(name: &str, typeid: &str, uuid_str: &str) {
            // Remove the prefix if present
            let suffix = typeid.split('_').last().unwrap();

            // Test decoding
            let decoded = TypeIdSuffix::from_str(suffix)
                .unwrap_or_else(|e| panic!("Failed to decode `TypeId`suffix for '{}': {:?}", name, e));
            let uuid = Uuid::parse_str(uuid_str)
                .unwrap_or_else(|e| panic!("Failed to parse UUID for '{}': {:?}", name, e));
            assert_eq!(Uuid::try_from(&decoded).unwrap(), uuid, "Decoding failed for '{}'", name);

            // Test encoding
            let encoded = TypeIdSuffix::new(uuid)
                .unwrap_or_else(|e| panic!("Failed to create `TypeId`suffix for '{}': {:?}", name, e));
            assert_eq!(encoded.as_str(), suffix, "Encoding failed for '{}'", name);
        }

        fn generate_tests() {
            let test_vectors: Value = serde_json::from_str(TEST_VECTORS).expect("Failed to parse JSON");

            for test_case in test_vectors.as_array().unwrap() {
                let name = test_case["name"].as_str().unwrap();
                let typeid = test_case["typeid"].as_str().unwrap();
                let uuid = test_case["uuid"].as_str().unwrap();

                run_test_vector(name, typeid, uuid);
            }
        }

        // #[cfg(test)]
        // mod tests {
        //     use super::*;
        //     use crate::create_test_vectors;
        //
        //     #[test]
        //     fn run_all_test_vectors() {
        //         create_test_vectors::generate_tests();
        //     }
        // }
    };
}
