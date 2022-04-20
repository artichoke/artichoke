use core::fmt::{self, Write};

use crate::literal::{is_ascii_char_with_escape, Literal};

/// Write a UTF-8 debug representation of a byte slice into the given writer.
///
/// This method encodes a bytes slice into a UTF-8 valid representation by
/// writing invalid sequences as hex escape codes (e.g. `\x00`) or C escape
/// sequences (e.g. `\a`).
///
/// This method also escapes UTF-8 valid characters like `\n` and `\t`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use scolapasta_string_escape::format_debug_escape_into;
///
/// let mut message = String::from("cannot load such file -- ");
/// let filename = b"utf8-invalid-name-\xFF";
/// format_debug_escape_into(&mut message, filename);
/// assert_eq!(r"cannot load such file -- utf8-invalid-name-\xFF", message);
/// ```
///
/// # Errors
///
/// This method only returns an error when the given writer returns an
/// error.
pub fn format_debug_escape_into<W, T>(mut dest: W, message: T) -> fmt::Result
where
    W: Write,
    T: AsRef<[u8]>,
{
    let mut buf = [0; 4];
    let mut message = message.as_ref();
    while !message.is_empty() {
        let (ch, size) = bstr::decode_utf8(message);
        match ch {
            Some(ch) if is_ascii_char_with_escape(ch) => {
                let [ascii_byte, ..] = u32::from(ch).to_le_bytes();
                let escaped = Literal::debug_escape(ascii_byte);
                dest.write_str(escaped)?;
            }
            Some(ch) => {
                let enc = ch.encode_utf8(&mut buf);
                dest.write_str(enc)?;
            }
            // Otherwise, we've gotten invalid UTF-8, which means this is not an
            // printable char.
            None => {
                for &byte in &message[..size] {
                    let escaped = Literal::debug_escape(byte);
                    dest.write_str(escaped)?;
                }
            }
        }
        message = &message[size..];
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::string::{String, ToString};

    use super::format_debug_escape_into;

    #[test]
    fn format_ascii_message() {
        let message = "Spinoso Exception";
        let mut dest = String::new();
        format_debug_escape_into(&mut dest, message).unwrap();
        assert_eq!(dest, "Spinoso Exception");
    }

    #[test]
    fn format_unicode_message() {
        let message = "Spinoso Exception 💎🦀";
        let mut dest = String::new();
        format_debug_escape_into(&mut dest, message).unwrap();
        assert_eq!(dest, "Spinoso Exception 💎🦀");
    }

    #[test]
    fn format_invalid_utf8_message() {
        let message = b"oh no! \xFF";
        let mut dest = String::new();
        format_debug_escape_into(&mut dest, message).unwrap();
        assert_eq!(dest, r"oh no! \xFF");
    }

    #[test]
    fn format_escape_code_message() {
        let message = "yes to symbolic \t\n\x7F";
        let mut dest = String::new();
        format_debug_escape_into(&mut dest, message).unwrap();
        assert_eq!(dest, r"yes to symbolic \t\n\x7F");
    }

    #[test]
    fn replacement_character() {
        let message = "This is the replacement character: \u{FFFD}";
        let mut dest = String::new();
        format_debug_escape_into(&mut dest, message).unwrap();
        assert_eq!(dest, "This is the replacement character: \u{FFFD}");
    }

    #[test]
    fn as_ref() {
        let message = b"Danger".to_vec();
        let mut dest = String::new();
        format_debug_escape_into(&mut dest, message).unwrap();
        assert_eq!(dest, "Danger");

        let message = "Danger".to_string();
        let mut dest = String::new();
        format_debug_escape_into(&mut dest, message).unwrap();
        assert_eq!(dest, "Danger");
    }
}
