#[doc(inline)]
pub use core::char::REPLACEMENT_CHARACTER;

/// The UTF-8 encoded byte representation of `U+FFFD REPLACEMENT CHARACTER` (�).
///
/// # Examples
///
/// ```ruby
/// [2.6.3] > "\u{FFFD}"
/// => "�"
/// [2.6.3] > "\u{FFFD}".bytes
/// => [239, 191, 189]
/// [2.6.3] > "\u{FFFD}".ord.to_s(16)
/// => "fffd"
/// [2.6.3] > '�'.ord
/// => 65533
/// [2.6.3] > '�'.ord.to_s(16)
/// => "fffd"
/// [2.6.3] > '�'.bytes
/// => [239, 191, 189]
/// ```
pub const REPLACEMENT_CHARACTER_BYTES: [u8; 3] = [239, 191, 189];

#[cfg(test)]
mod tests {
    use super::{REPLACEMENT_CHARACTER, REPLACEMENT_CHARACTER_BYTES};
    use core::str;

    #[test]
    fn unicode_replacement_char_to_bytes() {
        let mut enc = [0; 4];
        let utf8_bytes = REPLACEMENT_CHARACTER.encode_utf8(&mut enc).as_bytes();
        assert_eq!(utf8_bytes.len(), 3);
        assert_eq!(utf8_bytes, REPLACEMENT_CHARACTER_BYTES);
    }

    #[test]
    fn unicode_replacement_bytes_to_char() {
        let enc = REPLACEMENT_CHARACTER_BYTES;
        let string = str::from_utf8(&enc).unwrap();
        assert_eq!(string.chars().count(), 1);
        assert_eq!(string.chars().next().unwrap(), REPLACEMENT_CHARACTER);
    }
}
