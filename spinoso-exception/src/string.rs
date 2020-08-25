use bstr::ByteSlice;
use core::fmt::{self, Write};

use crate::literal::Literal;
use crate::unicode::{REPLACEMENT_CHARACTER, REPLACEMENT_CHARACTER_BYTES};

pub fn format_into<T, W>(message: T, mut dest: W) -> fmt::Result
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
            // exception message character.
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
                    let lit = Literal::from(byte);
                    for part in lit {
                        let part = part.encode_utf8(&mut enc);
                        dest.write_str(part)?;
                    }
                }
            }
            ch if Literal::is_ascii_char_with_escape(ch) => {
                let bytes = (ch as u32).to_le_bytes();
                let iter = Literal::from(bytes[0]);
                for part in iter {
                    let part = part.encode_utf8(&mut enc);
                    dest.write_str(part)?;
                }
            }
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
    use super::format_into;
    use alloc::string::String;

    #[test]
    fn format_ascii_message() {
        let message = "Spinoso Exception";
        let mut dest = String::new();
        format_into(message, &mut dest).unwrap();
        assert_eq!(dest, "Spinoso Exception");
    }

    #[test]
    fn format_unicode_message() {
        let message = "Spinoso Exception 💎🦀";
        let mut dest = String::new();
        format_into(message, &mut dest).unwrap();
        assert_eq!(dest, "Spinoso Exception 💎🦀");
    }

    #[test]
    fn format_invalid_utf8_message() {
        let message = b"oh no! \xFF";
        let mut dest = String::new();
        format_into(message, &mut dest).unwrap();
        assert_eq!(dest, r"oh no! \xFF");
    }

    #[test]
    fn format_escape_code_message() {
        let message = "yes to symbolic \t\n\x7F";
        let mut dest = String::new();
        format_into(message, &mut dest).unwrap();
        assert_eq!(dest, r"yes to symbolic \t\n\x7F");
    }

    #[test]
    fn replacement_character() {
        let message = "This is the replacement character: \u{FFFD}";
        let mut dest = String::new();
        format_into(message, &mut dest).unwrap();
        assert_eq!(dest, "This is the replacement character: \u{FFFD}");
    }
}
