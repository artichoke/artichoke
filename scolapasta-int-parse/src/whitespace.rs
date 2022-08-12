pub fn trim_leading(bytes: &[u8]) -> &[u8] {
    if let Some(idx) = bytes.iter().position(|&b| !b.is_ascii_whitespace()) {
        &bytes[idx..]
    } else {
        bytes
    }
}

pub fn trim_trailing(bytes: &[u8]) -> &[u8] {
    if let Some(idx) = bytes.iter().rev().position(|&b| !b.is_ascii_whitespace()) {
        &bytes[..bytes.len() - idx]
    } else {
        bytes
    }
}

pub fn trim(bytes: &[u8]) -> &[u8] {
    let bytes = trim_leading(bytes);
    trim_trailing(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_leading_trims_leading_whitespace() {
        assert_eq!(trim_leading(b"abc"), b"abc");
        assert_eq!(trim_leading(b" abc"), b"abc");
        assert_eq!(trim_leading(b"      abc"), b"abc");
        assert_eq!(trim_leading(b"\tabc"), b"abc");
        assert_eq!(trim_leading(b"\nabc"), b"abc");
        assert_eq!(trim_leading(b"\x0Aabc"), b"abc");
        assert_eq!(trim_leading(b"\x0Cabc"), b"abc");
        assert_eq!(trim_leading(b" \t\n\x0A\x0Cabc"), b"abc");
        assert_eq!(
            trim_leading(b" \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0Cabc"),
            b"abc"
        );
    }

    #[test]
    fn trim_leading_preserves_trailing_whitespace() {
        assert_eq!(trim_leading(b"abc "), b"abc ");
        assert_eq!(trim_leading(b" abc "), b"abc ");
        assert_eq!(trim_leading(b"      abc      "), b"abc      ");
        assert_eq!(trim_leading(b"\tabc\t"), b"abc\t");
        assert_eq!(trim_leading(b"\nabc\n"), b"abc\n");
        assert_eq!(trim_leading(b"\x0Aabc\x0A"), b"abc\x0A");
        assert_eq!(trim_leading(b"\x0Cabc\x0C"), b"abc\x0C");
        assert_eq!(trim_leading(b" \t\n\x0A\x0Cabc \t\n\x0A\x0C"), b"abc \t\n\x0A\x0C");
        assert_eq!(
            trim_leading(b" \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0Cabc \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C"),
            b"abc \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C"
        );
    }

    #[test]
    fn trim_trailing_trims_trailing_whitespace() {
        assert_eq!(trim_trailing(b"abc"), b"abc");
        assert_eq!(trim_trailing(b"abc "), b"abc");
        assert_eq!(trim_trailing(b"abc      "), b"abc");
        assert_eq!(trim_trailing(b"abc\t"), b"abc");
        assert_eq!(trim_trailing(b"abc\n"), b"abc");
        assert_eq!(trim_trailing(b"abc\x0A"), b"abc");
        assert_eq!(trim_trailing(b"abc\x0C"), b"abc");
        assert_eq!(trim_trailing(b"abc \t\n\x0A\x0C"), b"abc");
        assert_eq!(
            trim_trailing(b"abc \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C"),
            b"abc"
        );
    }

    #[test]
    fn trim_leading_preserves_leading_whitespace() {
        assert_eq!(trim_trailing(b" abc"), b" abc");
        assert_eq!(trim_trailing(b" abc "), b" abc");
        assert_eq!(trim_trailing(b"      abc      "), b"      abc");
        assert_eq!(trim_trailing(b"\tabc\t"), b"\tabc");
        assert_eq!(trim_trailing(b"\nabc\n"), b"\nabc");
        assert_eq!(trim_trailing(b"\x0Aabc\x0A"), b"\x0Aabc");
        assert_eq!(trim_trailing(b"\x0Cabc\x0C"), b"\x0Cabc");
        assert_eq!(trim_trailing(b" \t\n\x0A\x0Cabc \t\n\x0A\x0C"), b" \t\n\x0A\x0Cabc");
        assert_eq!(
            trim_trailing(b" \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0Cabc \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C"),
            b" \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0Cabc"
        );
    }

    #[test]
    fn trim_trims_all_whitespace() {
        assert_eq!(trim(b" abc "), b"abc");
        assert_eq!(trim(b" abc "), b"abc");
        assert_eq!(trim(b"      abc      "), b"abc");
        assert_eq!(trim(b"\tabc\t"), b"abc");
        assert_eq!(trim(b"\nabc\n"), b"abc");
        assert_eq!(trim(b"\x0Aabc\x0A"), b"abc");
        assert_eq!(trim(b"\x0Cabc\x0C"), b"abc");
        assert_eq!(trim(b" \t\n\x0A\x0Cabc \t\n\x0A\x0C"), b"abc");
        assert_eq!(
            trim(b" \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0Cabc \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C \t\n\x0A\x0C"),
            b"abc"
        );
    }
}
