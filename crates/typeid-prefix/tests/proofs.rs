#![doc(hidden)]

#[cfg(kani)]
mod verification {
    use std::convert::TryFrom;
    use typeid_prefix::{TypeIdPrefix, PrefixFactory};
    use kani::Arbitrary;

    #[derive(Debug)]
    struct TypeIdPrefixInput {
        input: String,
    }

    impl Arbitrary for TypeIdPrefixInput {
        fn any() -> Self {
            // Generate a random length for the string (between 0 and 100 to test beyond the 63 character limit)
            let len: usize = kani::any();
            kani::assume(len <= 100);

            // Generate arbitrary ASCII characters (including non-lowercase and non-underscore)
            let input: String = (0..len)
                .map(|_| {
                    let c: u8 = kani::any();
                    kani::assume(c.is_ascii());
                    c as char
                })
                .collect();
            TypeIdPrefixInput { input }
        }
    }

    #[kani::proof]
    fn verify_typeidprefix_try_from_and_sanitize() {
        let input: TypeIdPrefixInput = kani::any();
        let try_from_result = TypeIdPrefix::try_from(input.input.clone());
        let sanitized = input.input.create_prefix_sanitized();

        // Verify try_from behavior
        if input.input.len() > 63 {
            kani::assert(try_from_result.is_err(), "Input length exceeds 63 characters, should be error.");
        } else if input.input.is_empty() {
            kani::assert(try_from_result.is_ok(), "Empty input should be ok.");
        } else {
            let is_ascii = input.input.is_ascii();
            let starts_with_valid_char = input.input.chars().next().map_or(false, |c| c.is_ascii_lowercase());
            let ends_with_valid_char = input.input.chars().last().map_or(false, |c| c.is_ascii_lowercase());
            let contains_only_valid_chars = input.input.chars().all(|c| c.is_ascii_lowercase() || c == '_');

            if !is_ascii || !starts_with_valid_char || !ends_with_valid_char || !contains_only_valid_chars {
                kani::assert(try_from_result.is_err(), "Invalid input should be error.");
            } else {
                kani::assert(try_from_result.is_ok(), "Valid input should be ok.");
            }
        }

        // Verify sanitized output
        kani::assert(sanitized.len() <= 63, "Sanitized output should not exceed 63 characters.");
        kani::assert(sanitized.chars().all(|c| c.is_ascii_lowercase() || c == '_'), "Sanitized output should only contain lowercase ASCII or underscore.");
        kani::assert(!sanitized.starts_with('_'), "Sanitized output should not start with underscore.");
        kani::assert(!sanitized.ends_with('_'), "Sanitized output should not end with underscore.");

        // Ensure sanitized version is always valid
        kani::assert(TypeIdPrefix::try_from(sanitized.as_str()).is_ok(), "Sanitized output should always be valid.");

        // If the original input was valid, ensure it matches the sanitized version
        if try_from_result.is_ok() {
            kani::assert(input.input == sanitized, "Valid input should match its sanitized version.");
        }
    }

    #[kani::proof]
    fn verify_typeidprefix_try_from_str_and_sanitize() {
        let input: TypeIdPrefixInput = kani::any();
        let try_from_result = TypeIdPrefix::try_from(input.input.as_str());
        let sanitized = input.input.create_prefix_sanitized();

        // Verify try_from behavior (same as in the previous proof)
        if input.input.len() > 63 {
            kani::assert(try_from_result.is_err(), "Input length exceeds 63 characters, should be error.");
        } else if input.input.is_empty() {
            kani::assert(try_from_result.is_ok(), "Empty input should be ok.");
        } else {
            let is_ascii = input.input.is_ascii();
            let starts_with_valid_char = input.input.chars().next().map_or(false, |c| c.is_ascii_lowercase());
            let ends_with_valid_char = input.input.chars().last().map_or(false, |c| c.is_ascii_lowercase());
            let contains_only_valid_chars = input.input.chars().all(|c| c.is_ascii_lowercase() || c == '_');

            if !is_ascii || !starts_with_valid_char || !ends_with_valid_char || !contains_only_valid_chars {
                kani::assert(try_from_result.is_err(), "Invalid input should be error.");
            } else {
                kani::assert(try_from_result.is_ok(), "Valid input should be ok.");
            }
        }

        // Verify sanitized output (same as in the previous proof)
        kani::assert(sanitized.len() <= 63, "Sanitized output should not exceed 63 characters.");
        kani::assert(sanitized.chars().all(|c| c.is_ascii_lowercase() || c == '_'), "Sanitized output should only contain lowercase ASCII or underscore.");
        kani::assert(!sanitized.starts_with('_'), "Sanitized output should not start with underscore.");
        kani::assert(!sanitized.ends_with('_'), "Sanitized output should not end with underscore.");

        // Ensure sanitized version is always valid
        kani::assert(TypeIdPrefix::try_from(sanitized.as_str()).is_ok(), "Sanitized output should always be valid.");

        // If the original input was valid, ensure it matches the sanitized version
        if try_from_result.is_ok() {
            kani::assert(input.input == sanitized, "Valid input should match its sanitized version.");
        }
    }
}