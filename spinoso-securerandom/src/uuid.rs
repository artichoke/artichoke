//! Generator for Version 4 UUIDs.
//!
//! See [RFC 4122], Section 4.4.
//!
//! [RFC 4122]: https://tools.ietf.org/html/rfc4122#section-4.4

use rand::rngs::OsRng;
use rand::{CryptoRng, RngCore};
use scolapasta_hex as hex;

use crate::{Error, RandomBytesError};

/// The UUID format is 16 octets.
///
/// See [RFC 4122, Section 4.1].
///
/// [RFC 4122, Section 4.1]: https://tools.ietf.org/html/rfc4122#section-4.1
const OCTETS: usize = 16;

// See the BNF from JDK 8 that confirms stringified UUIDs are 36 characters
// long:
//
// https://docs.oracle.com/javase/8/docs/api/java/util/UUID.html#toString--
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

    let mut iter = bytes.iter().copied();
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
    debug_assert!(buf.len() == ENCODED_LENGTH);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::v4 as uuid;

    const ITERATIONS: usize = 1 << 12;
    const PATTERN: &str = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx";

    #[test]
    fn harness() {
        validate(PATTERN);
    }

    #[test]
    fn check() {
        for _ in 0..ITERATIONS {
            let gen = uuid().unwrap();
            validate(gen.as_str());
            let uuid_only_contains_chars_in_alphabet = gen.chars().all(|ch| matches!(ch, 'a'..='f' | '0'..='9' | '-'));
            assert!(
                uuid_only_contains_chars_in_alphabet,
                "Expected alphabet 'a'..='f', '0'..='9', '-', found '{}'",
                gen
            );
        }
    }

    fn validate(pattern: &str) {
        assert_eq!(pattern.len(), 36);
        assert!(pattern.is_ascii());
        assert_eq!(&pattern[8..9], "-");
        assert_eq!(&pattern[13..14], "-");
        assert_eq!(&pattern[14..15], "4");
        assert_eq!(&pattern[18..19], "-");
        assert!(matches!(&pattern[19..20], "8" | "9" | "a" | "b" | "y"));
        assert_eq!(&pattern[23..24], "-");
    }
}
