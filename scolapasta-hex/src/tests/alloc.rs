use alloc::string::String;
use core::fmt::Write;

use crate::{escape_byte, format_into, try_encode, try_encode_into, EscapedByte, Hex};

// https://tools.ietf.org/html/rfc4648#section-10
#[test]
fn test_rfc4648_test_vectors_encode() {
    // ```
    // BASE16("") = ""
    // ```
    assert_eq!(try_encode("").unwrap(), "");

    // ```
    // BASE16("f") = "66"
    // ```
    assert_eq!(try_encode("f").unwrap(), "66");

    // ```
    // BASE16("fo") = "666F"
    // ```
    assert_eq!(try_encode("fo").unwrap(), "666f");

    // ```
    // BASE16("foo") = "666F6F"
    // ```
    assert_eq!(try_encode("foo").unwrap(), "666f6f");

    // ```
    // BASE16("foob") = "666F6F62"
    // ```
    assert_eq!(try_encode("foob").unwrap(), "666f6f62");

    // ```
    // BASE16("fooba") = "666F6F6261"
    // ```
    assert_eq!(try_encode("fooba").unwrap(), "666f6f6261");

    // ```
    // BASE16("foobar") = "666F6F626172"
    // ```
    assert_eq!(try_encode("foobar").unwrap(), "666f6f626172");
}

// https://tools.ietf.org/html/rfc4648#section-10
#[test]
fn test_rfc4648_test_vectors_hex_iter() {
    // ```
    // BASE16("") = ""
    // ```
    assert_eq!(Hex::from("").collect::<String>(), "");

    // ```
    // BASE16("f") = "66"
    // ```
    assert_eq!(Hex::from("f").collect::<String>(), "66");

    // ```
    // BASE16("fo") = "666F"
    // ```
    assert_eq!(Hex::from("fo").collect::<String>(), "666f");

    // ```
    // BASE16("foo") = "666F6F"
    // ```
    assert_eq!(Hex::from("foo").collect::<String>(), "666f6f");

    // ```
    // BASE16("foob") = "666F6F62"
    // ```
    assert_eq!(Hex::from("foob").collect::<String>(), "666f6f62");

    // ```
    // BASE16("fooba") = "666F6F6261"
    // ```
    assert_eq!(Hex::from("fooba").collect::<String>(), "666f6f6261");

    // ```
    // BASE16("foobar") = "666F6F626172"
    // ```
    assert_eq!(Hex::from("foobar").collect::<String>(), "666f6f626172");
}

// https://tools.ietf.org/html/rfc4648#section-10
#[test]
fn test_rfc4648_test_vectors_encode_into_string() {
    // ```
    // BASE16("") = ""
    // ```
    let mut s = String::new();
    try_encode_into("", &mut s).unwrap();
    assert_eq!(s, "");
    assert_eq!(s.capacity(), 0);

    // ```
    // BASE16("f") = "66"
    // ```
    let mut s = String::new();
    try_encode_into("f", &mut s).unwrap();
    assert_eq!(s, "66");
    assert!(s.capacity() >= 2);

    // ```
    // BASE16("fo") = "666F"
    // ```
    let mut s = String::new();
    try_encode_into("fo", &mut s).unwrap();
    assert_eq!(s, "666f");
    assert!(s.capacity() >= 4);

    // ```
    // BASE16("foo") = "666F6F"
    // ```
    let mut s = String::new();
    try_encode_into("foo", &mut s).unwrap();
    assert_eq!(s, "666f6f");
    assert!(s.capacity() >= 6);

    // ```
    // BASE16("foob") = "666F6F62"
    // ```
    let mut s = String::new();
    try_encode_into("foob", &mut s).unwrap();
    assert_eq!(s, "666f6f62");
    assert!(s.capacity() >= 8);

    // ```
    // BASE16("fooba") = "666F6F6261"
    // ```
    let mut s = String::new();
    try_encode_into("fooba", &mut s).unwrap();
    assert_eq!(s, "666f6f6261");
    assert!(s.capacity() >= 10);

    // ```
    // BASE16("foobar") = "666F6F626172"
    // ```
    let mut s = String::new();
    try_encode_into("foobar", &mut s).unwrap();
    assert_eq!(s, "666f6f626172");
    assert!(s.capacity() >= 12);
}

// https://tools.ietf.org/html/rfc4648#section-10
#[test]
fn test_rfc4648_test_vectors_format_into() {
    // ```
    // BASE16("") = ""
    // ```
    let mut fmt = String::new();
    format_into("", &mut fmt).unwrap();
    assert_eq!(fmt, "");

    // ```
    // BASE16("f") = "66"
    // ```
    let mut fmt = String::new();
    format_into("f", &mut fmt).unwrap();
    assert_eq!(fmt, "66");

    // ```
    // BASE16("fo") = "666F"
    // ```
    let mut fmt = String::new();
    format_into("fo", &mut fmt).unwrap();
    assert_eq!(fmt, "666f");

    // ```
    // BASE16("foo") = "666F6F"
    // ```
    let mut fmt = String::new();
    format_into("foo", &mut fmt).unwrap();
    assert_eq!(fmt, "666f6f");

    // ```
    // BASE16("foob") = "666F6F62"
    // ```
    let mut fmt = String::new();
    format_into("foob", &mut fmt).unwrap();
    assert_eq!(fmt, "666f6f62");

    // ```
    // BASE16("fooba") = "666F6F6261"
    // ```
    let mut fmt = String::new();
    format_into("fooba", &mut fmt).unwrap();
    assert_eq!(fmt, "666f6f6261");

    // ```
    // BASE16("foobar") = "666F6F626172"
    // ```
    let mut fmt = String::new();
    format_into("foobar", &mut fmt).unwrap();
    assert_eq!(fmt, "666f6f626172");
}

#[test]
fn test_try_encode() {
    let data = b"Artichoke Ruby";
    let result = try_encode(data).unwrap();
    assert_eq!(result, "4172746963686f6b652052756279");
}

#[test]
fn test_try_encode_into() {
    let data = b"Artichoke Ruby";
    let mut buf = String::new();
    try_encode_into(data, &mut buf).unwrap();
    assert_eq!(buf, "4172746963686f6b652052756279");
}

#[test]
fn test_format_into() {
    let data = b"Artichoke Ruby";
    let mut buf = String::new();
    format_into(data, &mut buf).unwrap();
    assert_eq!(buf, "4172746963686f6b652052756279");
}

#[test]
fn test_hex_iterator() {
    let data = "Artichoke Ruby";
    let iter = Hex::from(data);
    let result = iter.collect::<String>();
    assert_eq!(result, "4172746963686f6b652052756279");
}

#[test]
fn test_hex_escape() {
    let mut buf = String::new();
    for value in 0..=255 {
        write!(&mut buf, "{value:02x}").unwrap();
        assert_eq!(EscapedByte::from(value).as_str(), buf);
        assert_eq!(EscapedByte::hex_escape(value), buf);
        assert_eq!(escape_byte(value), buf);
        buf.clear();
    }
}

#[test]
fn test_escaped_byte_iterator() {
    let escaped_byte = EscapedByte::from(b'H');

    let result = escaped_byte.collect::<String>();
    assert_eq!(result, "48");
}

#[test]
fn test_empty_input() {
    let input: &[u8] = &[];
    assert_eq!(try_encode(input).as_deref(), Ok(""));
}

#[test]
fn test_single_byte_input() {
    let input: &[u8] = &[0];
    assert_eq!(try_encode(input).as_deref(), Ok("00"));

    let input: &[u8] = &[255];
    assert_eq!(try_encode(input).as_deref(), Ok("ff"));

    let input: &[u8] = &[127];
    assert_eq!(try_encode(input).as_deref(), Ok("7f"));
}

#[test]
fn test_multi_byte_input() {
    let input: &[u8] = &[0, 1, 255];
    assert_eq!(try_encode(input).as_deref(), Ok("0001ff"));

    let input: &[u8] = &[10, 20, 30, 40, 50];
    assert_eq!(try_encode(input).as_deref(), Ok("0a141e2832"));
}

#[test]
fn test_boundary_values() {
    let input: &[u8] = &[0];
    assert_eq!(try_encode(input).as_deref(), Ok("00"));

    let input: &[u8] = &[255];
    assert_eq!(try_encode(input).as_deref(), Ok("ff"));
}

#[test]
fn test_null_byte() {
    let input: &[u8] = &[0];
    assert_eq!(try_encode(input).as_deref(), Ok("00"));
}

#[test]
fn test_special_characters() {
    let input: &[u8] = &[10, 13, 9, 92];
    assert_eq!(try_encode(input).as_deref(), Ok("0a0d095c"));
}

#[test]
fn test_repeated_patterns() {
    let input: &[u8] = &[1, 1, 1, 1, 1];
    assert_eq!(try_encode(input).as_deref(), Ok("0101010101"));
}

#[test]
fn test_random_byte_sequences() {
    let input: &[u8] = &[45, 68, 122, 200, 33, 90, 111];
    assert_eq!(try_encode(input).as_deref(), Ok("2d447ac8215a6f"));
}
