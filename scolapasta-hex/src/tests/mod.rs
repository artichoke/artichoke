use super::*;

#[cfg(feature = "alloc")]
mod alloc;
#[cfg(feature = "std")]
mod std;

#[test]
fn test_literal_exhaustive() {
    for byte in 0..=255 {
        let mut lit = EscapedByte::from(byte);
        let left = lit.next().unwrap();
        let top = byte >> 4;
        match top {
            0x0 => assert_eq!(left, '0', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x1 => assert_eq!(left, '1', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x2 => assert_eq!(left, '2', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x3 => assert_eq!(left, '3', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x4 => assert_eq!(left, '4', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x5 => assert_eq!(left, '5', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x6 => assert_eq!(left, '6', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x7 => assert_eq!(left, '7', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x8 => assert_eq!(left, '8', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0x9 => assert_eq!(left, '9', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0xA => assert_eq!(left, 'a', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0xB => assert_eq!(left, 'b', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0xC => assert_eq!(left, 'c', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0xD => assert_eq!(left, 'd', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0xE => assert_eq!(left, 'e', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            0xF => assert_eq!(left, 'f', "for byte {byte} ({byte:02x}) got incorrect left hex digit"),
            tuple => panic!("unknown top 16th: {tuple}, from byte: {byte} ({byte:02x})"),
        }

        let right = lit.next().unwrap();
        let bottom = byte & 0xF;
        match bottom {
            0x0 => assert_eq!(right, '0', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x1 => assert_eq!(right, '1', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x2 => assert_eq!(right, '2', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x3 => assert_eq!(right, '3', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x4 => assert_eq!(right, '4', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x5 => assert_eq!(right, '5', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x6 => assert_eq!(right, '6', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x7 => assert_eq!(right, '7', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x8 => assert_eq!(right, '8', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0x9 => assert_eq!(right, '9', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0xA => assert_eq!(right, 'a', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0xB => assert_eq!(right, 'b', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0xC => assert_eq!(right, 'c', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0xD => assert_eq!(right, 'd', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0xE => assert_eq!(right, 'e', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            0xF => assert_eq!(right, 'f', "for byte {byte} ({byte:02x}) got incorrect right hex digit"),
            tuple => panic!("unknown bottom 16th: {tuple}, from byte: {byte} ({byte:02x})"),
        }
        assert!(
            lit.next().is_none(),
            "literal must only expand to two ASCII chracters, found 3+"
        );
    }
}

#[test]
fn test_escape_byte() {
    // Expected hex escape codes for all possible u8 values
    #[rustfmt::skip]
        const EXPECTED: [&str; 256] = [
            "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0a", "0b", "0c", "0d", "0e", "0f",
            "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1a", "1b", "1c", "1d", "1e", "1f",
            "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2a", "2b", "2c", "2d", "2e", "2f",
            "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3a", "3b", "3c", "3d", "3e", "3f",
            "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4a", "4b", "4c", "4d", "4e", "4f",
            "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5a", "5b", "5c", "5d", "5e", "5f",
            "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6a", "6b", "6c", "6d", "6e", "6f",
            "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7a", "7b", "7c", "7d", "7e", "7f",
            "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8a", "8b", "8c", "8d", "8e", "8f",
            "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9a", "9b", "9c", "9d", "9e", "9f",
            "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af",
            "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf",
            "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf",
            "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "da", "db", "dc", "dd", "de", "df",
            "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef",
            "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "fa", "fb", "fc", "fd", "fe", "ff",
        ];

    // Iterate over all possible u8 values
    for byte in 0..=255 {
        let expected = EXPECTED[byte as usize];
        assert_eq!(EscapedByte::from(byte).as_str(), expected);
        assert_eq!(EscapedByte::hex_escape(byte), expected);
        assert_eq!(escape_byte(byte), expected);
    }
}

#[test]
fn test_escaped_byte_as_str() {
    let escaped_byte = EscapedByte::from(b'H');
    assert_eq!(escaped_byte.as_str(), "48");
}

#[test]
fn test_escaped_byte_len() {
    let escaped_byte = EscapedByte::from(b'H');
    assert_eq!(escaped_byte.len(), 2);
}

#[test]
fn test_escaped_byte_is_empty() {
    let escaped_byte = EscapedByte::from(b'\x00');
    assert!(!escaped_byte.is_empty());

    let escaped_byte = EscapedByte::from(b'H');
    assert!(!escaped_byte.is_empty());
}

#[test]
fn test_escaped_byte_double_ended_iterator() {
    let mut iter = EscapedByte::from(b'H');

    assert_eq!(iter.next(), Some('4'));
    assert_eq!(iter.next_back(), Some('8'));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn test_escaped_byte_is_empty_with_partial_consumption() {
    // Partial consumption of iterator
    let mut escaped_byte = EscapedByte::from(b'H');
    assert!(!escaped_byte.is_empty());

    // Consume one character
    escaped_byte.next();
    assert!(!escaped_byte.is_empty());

    // Fully consume the iterator
    escaped_byte.next();
    assert!(escaped_byte.is_empty());
}

#[test]
fn test_escaped_byte_len_with_partial_consumption() {
    // Partial consumption of iterator
    let mut escaped_byte = EscapedByte::from(b'H');
    assert_eq!(escaped_byte.len(), 2);

    // Consume one character
    escaped_byte.next();
    assert_eq!(escaped_byte.len(), 1);

    // Fully consume the iterator
    escaped_byte.next();
    assert_eq!(escaped_byte.len(), 0);
}

#[test]
fn test_hex_iterator_with_remaining_escaped_byte() {
    let hex_str = &[0x41, 0x42, 0x1B, 0x43];
    let mut hex_iter = Hex::from(hex_str);

    // Consume the first three bytes
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, '3');
    assert!(hex_iter.next().is_none());
}

#[test]
fn test_hex_iterator_empty_after_exhausted() {
    let hex_str = &[0x41, 0x42];
    let mut hex_iter = Hex::from(hex_str);

    // Consume all bytes
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();

    assert!(hex_iter.is_empty());
    assert!(hex_iter.next().is_none());
}

#[test]
fn test_hex_iterator_not_empty_with_remaining_escaped_byte() {
    let hex_str = &[0x41, 0x42, 0x1B, 0x43];
    let mut hex_iter = Hex::from(hex_str);

    // Consume the first three bytes
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();

    assert!(!hex_iter.is_empty());
    assert!(hex_iter.next().is_some());
}

#[test]
fn test_hex_iterator_not_empty_with_remaining_byte() {
    let hex_str = &[0x41, 0x42, 0x43];
    let mut hex_iter = Hex::from(hex_str);

    // Consume the first two bytes
    hex_iter.next().unwrap();
    hex_iter.next().unwrap();

    assert!(!hex_iter.is_empty());
}

#[test]
fn test_hex_iterator_empty() {
    let hex_str = b"";
    let hex_iter = Hex::from(hex_str);

    assert!(hex_iter.is_empty());
}

#[test]
fn test_hex_iterator_with_single_escaped_byte() {
    let hex_str = &[0x1B];
    let mut hex_iter = Hex::from(hex_str);

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');
    assert!(hex_iter.is_empty());
}

#[test]
fn test_hex_iterator_with_multiple_escaped_bytes() {
    let hex_str = &[0x1B, 0x1B, 0x1B];
    let mut hex_iter = Hex::from(hex_str);

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    assert!(hex_iter.next().is_none());
    assert!(hex_iter.is_empty());
}

#[test]
fn test_hex_iterator_with_invalid_escape_sequence() {
    let hex_str = &[0x41, 0x1B, 0x42];
    let mut hex_iter = Hex::from(hex_str);

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, '2');

    assert!(hex_iter.next().is_none());
    assert!(hex_iter.is_empty());
}

#[test]
fn test_hex_iterator_with_no_remaining_bytes() {
    let hex_str = &[0x41, 0x42, 0x43];
    let mut hex_iter = Hex::from(hex_str);

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, '2');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');
    let result = hex_iter.next().unwrap();
    assert_eq!(result, '3');

    assert!(hex_iter.next().is_none());
    assert!(hex_iter.is_empty());
}

#[test]
fn test_hex_iterator_with_emoji_sequence() {
    // ```
    // >>> binascii.hexlify(bytes("Hello, 😃!", "utf-8"))
    // b'48656c6c6f2c20f09f988321'
    // ```
    let mut hex_iter = Hex::from("Hello, 😃!");

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '8');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '6');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '5');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '6');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'c');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '6');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'c');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '6');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'f');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '2');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'c');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '2');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '0');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'f');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '0');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '9');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'f');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '9');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '8');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '8');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '3');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '2');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');

    assert!(hex_iter.next().is_none());
    assert!(hex_iter.is_empty());
}

#[test]
fn test_hex_iterator_with_chinese_hanzi_sequence() {
    // ```
    // >>> binascii.hexlify(bytes("你好，世界！", "utf-8"))
    // b'e4bda0e5a5bdefbc8ce4b896e7958cefbc81'
    // ```
    let mut hex_iter = Hex::from("你好，世界！");

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'e');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'd');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'a');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '0');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'e');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '5');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'a');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '5');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'd');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'e');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'f');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'c');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '8');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'c');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'e');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '4');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '8');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '9');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '6');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'e');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '7');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '9');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '5');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '8');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'c');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'e');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'f');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'b');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, 'c');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '8');

    let result = hex_iter.next().unwrap();
    assert_eq!(result, '1');

    assert!(hex_iter.next().is_none());
    assert!(hex_iter.is_empty());
}

#[test]
fn test_hex_iterator_with_byte_array_exhaustive() {
    let mut hex_str = [0u8; 256];

    for (i, byte) in hex_str.iter_mut().enumerate() {
        *byte = i.try_into().unwrap();
    }

    let mut hex_iter = Hex::from(&hex_str);

    for byte in &hex_str {
        let expected_chars = [
            char::from_digit(u32::from(byte >> 4), 16).unwrap(),
            char::from_digit(u32::from(byte & 0x0F), 16).unwrap(),
        ];
        for expected_char in expected_chars {
            let result = hex_iter.next().unwrap();
            assert_eq!(result, expected_char);
        }
    }

    assert!(hex_iter.next().is_none());
    assert!(hex_iter.is_empty());
}

#[test]
fn test_hex_exact_size_iterator() {
    // ```
    // >>> import binascii
    // >>> binascii.hexlify(bytes("Hello", "utf-8"))
    // b'48656c6c6f'
    // ```
    let mut iter = Hex::from("Hello");
    assert_eq!(iter.len(), 10);
    assert_eq!(iter.size_hint(), (10, Some(10)));

    assert_eq!(iter.next(), Some('4'));
    assert_eq!(iter.len(), 9);
    assert_eq!(iter.size_hint(), (9, Some(9)));

    assert_eq!(iter.next(), Some('8'));
    assert_eq!(iter.next(), Some('6'));
    assert_eq!(iter.len(), 7);
    assert_eq!(iter.size_hint(), (7, Some(7)));

    assert_eq!(iter.next(), Some('5'));
    assert_eq!(iter.next(), Some('6'));
    assert_eq!(iter.len(), 5);
    assert_eq!(iter.size_hint(), (5, Some(5)));

    assert_eq!(iter.next(), Some('c'));
    assert_eq!(iter.next(), Some('6'));
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));

    assert_eq!(iter.next(), Some('c'));
    assert_eq!(iter.next(), Some('6'));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.size_hint(), (1, Some(1)));

    assert_eq!(iter.next(), Some('f'));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.size_hint(), (0, Some(0)));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.size_hint(), (0, Some(0)));
}

#[test]
fn test_hex_is_empty() {
    let iter = Hex::from("Hello");
    assert!(!iter.is_empty());

    let iter = Hex::from("");
    assert!(iter.is_empty());

    let mut iter = Hex::from("W");
    assert!(!iter.is_empty());
    assert_eq!(iter.next(), Some('5'));
    assert!(!iter.is_empty());
    assert_eq!(iter.next(), Some('7'));
    assert!(iter.is_empty());
    assert_eq!(iter.next(), None);
    assert!(iter.is_empty());
}

#[test]
fn test_hex_last() {
    let iter = Hex::from("");
    assert_eq!(iter.last(), None);

    let iter = Hex::from("Hello");
    assert_eq!(iter.last(), Some('f'));

    let mut iter = Hex::from("World");
    assert_eq!(iter.next(), Some('5'));
    assert_eq!(iter.last(), Some('4'));

    let iter = Hex::from("123456");
    assert_eq!(iter.last(), Some('6'));

    let iter = Hex::from("A");
    assert_eq!(iter.last(), Some('1'));
}
