use bstr::ByteSlice;
use std::fmt;

pub fn escape_unicode(mut f: impl fmt::Write, string: &[u8]) -> fmt::Result {
    let buf = bstr::B(string);
    for (s, e, ch) in buf.char_indices() {
        if ch == '\u{FFFD}' {
            for &b in buf[s..e].as_bytes() {
                write!(f, r"\x{:X}", b)?;
            }
        } else {
            write!(f, "{}", ch.escape_debug())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::escape_unicode;

    #[test]
    fn invalid_utf8() {
        let mut buf = String::new();
        escape_unicode(&mut buf, &b"abc\xFF"[..]).unwrap();
        assert_eq!(r"abc\xFF", buf.as_str());
    }

    #[test]
    fn ascii() {
        let mut buf = String::new();
        escape_unicode(&mut buf, &b"abc"[..]).unwrap();
        assert_eq!(r"abc", buf.as_str());
    }

    #[test]
    fn emoji() {
        let mut buf = String::new();
        escape_unicode(&mut buf, "Ruby 💎".as_bytes()).unwrap();
        assert_eq!(r"Ruby 💎", buf.as_str());
    }

    #[test]
    fn escaped() {
        let mut buf = String::new();
        escape_unicode(&mut buf, "\n".as_bytes()).unwrap();
        assert_eq!(r"\n", buf.as_str());
    }
}
