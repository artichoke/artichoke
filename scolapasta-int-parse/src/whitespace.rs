fn is_posix_ascii_whitespace(b: u8) -> bool {
    // From the docs for `u8::is_ascii_whitespace`:
    // https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_whitespace
    //
    // > Rust uses the WhatWG Infra Standard’s definition of ASCII whitespace.
    // > There are several other definitions in wide use. For instance, the
    // > POSIX > locale includes U+000B VERTICAL TAB as well as all the above
    // > characters [...]
    //
    // Ruby uses the POSIX standards:
    // https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap07.html
    b.is_ascii_whitespace() || matches!(b, b'\x0B')
}

pub fn trim_leading(bytes: &[u8]) -> &[u8] {
    if let Some(idx) = bytes.iter().position(|&b| !is_posix_ascii_whitespace(b)) {
        &bytes[idx..]
    } else {
        bytes
    }
}

pub fn trim_trailing(bytes: &[u8]) -> &[u8] {
    if let Some(idx) = bytes.iter().rposition(|&b| !is_posix_ascii_whitespace(b)) {
        &bytes[..=idx]
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
    fn byte_is_whitespace() {
        // ```
        // [3.1.2] > (0..127).each { |b| s = "#{b.chr}27"; puts "#{b}, #{s.inspect}" unless Integer(s, exception: false).nil? }
        // 9, "\t27"
        // 10, "\n27"
        // 11, "\v27"
        // 12, "\f27"
        // 13, "\r27"
        // 32, " 27"
        // 43, "+27"
        // 45, "-27"
        // 48, "027"
        // 49, "127"
        // 50, "227"
        // 51, "327"
        // 52, "427"
        // 53, "527"
        // 54, "627"
        // 55, "727"
        // 56, "827"
        // 57, "927"
        // ```
        const WHITESPACE_BYTES: &[u8] = &[9, 10, 11, 12, 13, 32];
        for b in u8::MIN..=u8::MAX {
            assert_eq!(is_posix_ascii_whitespace(b), WHITESPACE_BYTES.contains(&b));
        }
    }

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

    #[test]
    fn vertical_tab_is_whitespace() {
        // ```
        // [3.1.2] > Integer "    \u000B 27"
        // => 27
        // ```
        assert_eq!(trim_leading(b"\x0Babc"), b"abc");
        assert_eq!(trim_trailing(b"abc\x0B"), b"abc");
        assert_eq!(trim(b"\x0Babc\x0B"), b"abc");
    }
}
