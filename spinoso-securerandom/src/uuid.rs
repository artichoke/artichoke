//! Generator for Version 4 UUIDs.
//!
//! See [RFC 4122], Section 4.4.
//!
//! [RFC 4122]: https://tools.ietf.org/html/rfc4122#section-4.4

use rand::rngs::OsRng;
use rand::{CryptoRng, RngCore};
use scolapasta_hex as hex;

use crate::{Error, RandomBytesError};

/// The number of octets (bytes) in a UUID, as defined in [RFC4122].
///
/// According to RFC4122, a UUID consists of 16 octets (128 bits) and is
/// represented as a hexadecimal string of 32 characters, typically separated by
/// hyphens into five groups: 8-4-4-4-12.
///
/// [RFC4122]: https://tools.ietf.org/html/rfc4122#section-4.1
const OCTETS: usize = 16;

/// The length of an encoded UUID string, including hyphens, as defined in
/// [RFC4122].
///
/// According to RFC4122, an encoded UUID string consists of 36 characters,
/// which includes the hexadecimal digits and four hyphens in the format
/// 8-4-4-4-12.
///
/// [RFC4122]: https://tools.ietf.org/html/rfc4122
const ENCODED_LENGTH: usize = 36;

#[inline]
pub fn v4() -> Result<String, Error> {
    fn get_random_bytes<T: RngCore + CryptoRng>(mut rng: T, slice: &mut [u8]) -> Result<(), RandomBytesError> {
        rng.try_fill_bytes(slice)?;
        Ok(())
    }

    let mut bytes = [0; OCTETS];
    get_random_bytes(OsRng, &mut bytes)?;

    // Per RFC 4122, Section 4.4, set bits for version and `clock_seq_hi_and_reserved`.
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    let mut buf = String::new();
    buf.try_reserve(ENCODED_LENGTH)?;

    let mut iter = bytes.into_iter();
    for byte in iter.by_ref().take(4) {
        let escaped = hex::escape_byte(byte);
        buf.push_str(escaped);
    }
    buf.push('-');
    for byte in iter.by_ref().take(2) {
        let escaped = hex::escape_byte(byte);
        buf.push_str(escaped);
    }
    buf.push('-');
    for byte in iter.by_ref().take(2) {
        let escaped = hex::escape_byte(byte);
        buf.push_str(escaped);
    }
    buf.push('-');
    for byte in iter.by_ref().take(2) {
        let escaped = hex::escape_byte(byte);
        buf.push_str(escaped);
    }
    buf.push('-');
    for byte in iter {
        let escaped = hex::escape_byte(byte);
        buf.push_str(escaped);
    }
    debug_assert_eq!(buf.len(), ENCODED_LENGTH, "UUID had unexpected length");
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    // Number of iterations for UUID generation tests.
    //
    // Chosen to provide a high level of confidence in the correctness and
    // uniqueness of the UUID generation function, striking a balance between
    // test coverage and reasonable execution time. Can be adjusted based on
    // specific application requirements.
    const ITERATIONS: usize = 10240;

    #[test]
    fn test_v4_returns_valid_uuid() {
        for _ in 0..ITERATIONS {
            let uuid = v4().unwrap();
            // Validate the UUID format
            assert_eq!(uuid.len(), 36, "UUID length should be 36 characters");
            assert_eq!(&uuid[8..9], "-", "Invalid UUID format");
            assert_eq!(&uuid[13..14], "-", "Invalid UUID format");
            assert_eq!(&uuid[18..19], "-", "Invalid UUID format");
            assert_eq!(&uuid[23..24], "-", "Invalid UUID format");

            // Validate that the UUID is version 4
            assert_eq!(&uuid[14..15], "4", "Invalid UUID version");

            // Validate that the non-hyphen positions are lowercase ASCII alphanumeric characters
            for (idx, c) in uuid.chars().enumerate() {
                if matches!(idx, 8 | 13 | 18 | 23) {
                    assert_eq!(c, '-', "Expected hyphen at position {idx}");
                } else {
                    assert!(
                        matches!(c, '0'..='9' | 'a'..='f'),
                        "Character at position {idx} should match ASCII numeric and lowercase hex"
                    );
                }
            }
        }
    }

    #[test]
    fn test_v4_generated_uuids_are_unique() {
        let mut generated_uuids = HashSet::with_capacity(ITERATIONS);

        for _ in 0..ITERATIONS {
            let uuid = v4().unwrap();

            // Ensure uniqueness of generated UUIDs
            assert!(
                generated_uuids.insert(uuid.clone()),
                "Generated UUID is not unique: {uuid}"
            );
        }
    }

    #[test]
    fn test_v4_generated_uuids_are_ascii_only() {
        for _ in 0..ITERATIONS {
            let uuid = v4().unwrap();
            assert!(uuid.is_ascii(), "UUID should consist of only ASCII characters: {uuid}");
        }
    }

    #[test]
    fn test_v4_clock_seq_hi_and_reserved() {
        for _ in 0..ITERATIONS {
            let uuid = v4().unwrap();

            // Extract the relevant portion of the generated UUID for comparison
            //
            // Per the RFC, `clock_seq_hi_and_reserved` is octet 8 (zero indexed).
            // Additionally: the two most significant bits (bits 6 and 7) are set
            // to zero and one, respectively.
            let clock_seq_hi_and_reserved = u8::from_str_radix(&uuid[14..16], 16).unwrap();

            // Assert that the two most significant bits are correct
            assert_eq!(
                clock_seq_hi_and_reserved & 0b1100_0000,
                0b0100_0000,
                "Incorrect clock_seq_hi_and_reserved bits in v4 UUID"
            );
        }
    }
}
