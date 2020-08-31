use bstr::ByteSlice;
use core::fmt::{self, Write};

use crate::literal::{is_ascii_char_with_escape, Literal};
use crate::unicode::{REPLACEMENT_CHARACTER, REPLACEMENT_CHARACTER_BYTES};

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
/// format_debug_escape_into(filename, &mut message);
/// assert_eq!(r"cannot load such file -- utf8-invalid-name-\xFF", message);
/// ```
///
/// # Errors
///
/// This method only returns an error when the given writer returns an
/// error.
pub fn format_debug_escape_into<T, W>(message: T, mut dest: W) -> fmt::Result
where
    T: AsRef<[u8]>,
    W: Write,
{
    let mut enc = [0; 4];
    let message = message.as_ref();
    for (start, end, ch) in message.char_indices() {
        match ch {
            // `char_indices` uses the Unicode replacement character to
            // indicate the current char is invalid UTF-8. However, the
            // replacement character itself _is_ valid UTF-8 and a valid
            // unescaped String character.
            //
            // If `char_indices` yields a replacement char and the byte span
            // matches the UTF-8 encoding of the replacement char, continue.
            REPLACEMENT_CHARACTER if message[start..end] == REPLACEMENT_CHARACTER_BYTES[..] => {
                let part = REPLACEMENT_CHARACTER.encode_utf8(&mut enc);
                dest.write_str(part)?;
            }
            // Otherwise, we've gotten invalid UTF-8, which means this is not an
            // printable char.
            REPLACEMENT_CHARACTER => {
                for &byte in &message[start..end] {
                    let escaped = Literal::debug_escape(byte);
                    dest.write_str(escaped)?;
                }
            }
            // If the character is ASCII and has a non-trivial escape, retrieve
            // it and write it to the destination.
            ch if is_ascii_char_with_escape(ch) => {
                let [ascii_byte, _, _, _] = (ch as u32).to_le_bytes();
                let escaped = Literal::debug_escape(ascii_byte);
                dest.write_str(escaped)?;
            }
            // Otherwise, encode the char to a UTF-8 str and write it out.
            ch => {
                let part = ch.encode_utf8(&mut enc);
                dest.write_str(part)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::format_debug_escape_into;
    use alloc::string::String;

    #[test]
    fn format_ascii_message() {
        let message = "Spinoso Exception";
        let mut dest = String::new();
        format_debug_escape_into(message, &mut dest).unwrap();
        assert_eq!(dest, "Spinoso Exception");
    }

    #[test]
    fn format_unicode_message() {
        let message = "Spinoso Exception 💎🦀";
        let mut dest = String::new();
        format_debug_escape_into(message, &mut dest).unwrap();
        assert_eq!(dest, "Spinoso Exception 💎🦀");
    }

    #[test]
    fn format_invalid_utf8_message() {
        let message = b"oh no! \xFF";
        let mut dest = String::new();
        format_debug_escape_into(message, &mut dest).unwrap();
        assert_eq!(dest, r"oh no! \xFF");
    }

    #[test]
    fn format_escape_code_message() {
        let message = "yes to symbolic \t\n\x7F";
        let mut dest = String::new();
        format_debug_escape_into(message, &mut dest).unwrap();
        assert_eq!(dest, r"yes to symbolic \t\n\x7F");
    }

    #[test]
    fn replacement_character() {
        let message = "This is the replacement character: \u{FFFD}";
        let mut dest = String::new();
        format_debug_escape_into(message, &mut dest).unwrap();
        assert_eq!(dest, "This is the replacement character: \u{FFFD}");
    }
}
