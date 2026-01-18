#![allow(clippy::cast_possible_truncation)]

use crate::errors::{DecodeError, InvalidSuffixReason};

// The base32 encoding alphabet as specified in the `TypeId`specification.
// This is the same as Crockford's base32 encoding, but `TypeId`uses it in a strict manner:
// always lowercase, no hyphens, and no decoding of multiple ambiguous characters to the same value.
const ENCODE_TABLE: &[u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";

// A lookup table for decoding base32 characters back to their 5-bit values.
// This is the inverse of ENCODE_TABLE, where the index is the ASCII value of the character,
// and the value is the 5-bit integer it represents. 0xFF is used for invalid characters.
pub const DECODE_TABLE: [u8; 256] = {
    let mut table = [0xFF; 256];
    let mut i = 0;
    while i < 32 {
        table[ENCODE_TABLE[i] as usize] = i as u8;
        i += 1;
    }
    table
};

// Encodes a 16-byte UUID into a 26-character base32 string as per the `TypeId`specification.
pub fn encode_base32(uuid: &[u8; 16]) -> [u8; 26] {
    // Convert the 16-byte UUID to a 128-bit integer in big-endian order
    let mut uuid_int = u128::from_be_bytes(*uuid);
    let mut encoded_output = [0u8; 26];

    // Encode each 5-bit chunk of the 128-bit integer into a base32 character,
    // iterating in reverse because we're processing from least significant to most significant bits
    for index in (0..26).rev() {
        // Extract the least significant 5 bits and use them as an index into the ENCODE_TABLE
        encoded_output[index] = ENCODE_TABLE[(uuid_int & 0x1F) as usize];
        // Shift right by 5 bits to process the next chunk
        uuid_int >>= 5;
    }

    // The resulting output is a 26-character base32 encoded string
    encoded_output
}

// Decodes a 26-character base32 string back into a 16-byte UUID as per the `TypeId`specification.
pub fn decode_base32(encoded: &[u8; 26]) -> Result<[u8; 16], DecodeError> {
    let mut uuid_int = 0u128;

    // Iterate over each character in the encoded input
    for &character in encoded {
        // Look up the 5-bit value corresponding to this character
        let value = DECODE_TABLE[character as usize];
        // If the character is invalid (not part of the base32 alphabet), return an error
        if value == 0xFF {
            return Err(DecodeError::InvalidSuffix(
                InvalidSuffixReason::InvalidCharacter,
            ));
        }
        // Shift the existing number left by 5 bits and add the new 5-bit value
        uuid_int = (uuid_int << 5) | u128::from(value);
    }

    // Convert the resulting 128-bit integer back to a 16-byte array in big-endian order
    Ok(uuid_int.to_be_bytes())
}
