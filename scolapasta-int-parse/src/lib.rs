#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![cfg_attr(test, allow(clippy::non_ascii_literal))]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Parse a given byte string and optional [`Radix`] into an [`i64`].
//!
//! [`parse`] wraps [`i64::from_str_radix`] by performing normalizations on the
//! input byte string:
//!
//! - Assert the byte string is ASCII and does not contain NUL bytes.
//! - Trim leading whitespace.
//! - Accept a single, optional `+` or `-` sign byte.
//! - Parse a literal radix out of the string from one of `0b`, `0B`, `0o`,
//!   `0O`, `0d`, `0D`, `0x`, or `0X`. If the given radix is `Some(_)`, the
//!   radix must match the embedded radix literal. A `0` prefix of arbitrary
//!   length is interpreted as an octal literal.
//! - Remove ("squeeze") leading zeros.
//! - Collect ASCII alphanumeric bytes and filter out underscores.
//!
//! The functions and types in this crate can be used to implement
//! [`Kernel#Integer`] in Ruby.
//!
//! [`Kernel#Integer`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-Integer

#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod error;
mod parser;
mod radix;
mod subject;

pub use error::{ArgumentError, Error, InvalidRadixError, InvalidRadixExceptionKind};
use parser::{Sign, State as ParseState};
pub use radix::Radix;
use radix::RADIX_TABLE;
use subject::IntegerString;

/// Parse a given byte string and optional [`Radix`] into an [`i64`].
///
/// This function wraps [`i64::from_str_radix`] by performing normalizations on
/// the input byte string:
///
/// - Assert the byte string is ASCII and does not contain NUL bytes.
/// - Parse the radix to ensure it is in range and valid for the given input
///   byte string.
/// - Trim leading whitespace.
/// - Accept a single, optional `+` or `-` sign byte.
/// - Parse a literal radix out of the string from one of `0b`, `0B`, `0o`,
///   `0O`, `0d`, `0D`, `0x`, or `0X`. If the given radix is `Some(_)`, the
///   radix must match the embedded radix literal. A `0` prefix of arbitrary
///   length is interpreted as an octal literal.
/// - Remove ("squeeze") leading zeros.
/// - Collect ASCII alphanumeric bytes and filter out underscores.
/// - Pass the collected ASCII alphanumeric bytes to [`i64::from_str_radix`].
///
/// If the given radix argument is [`None`] the input byte string is either
/// parsed with the radix embedded within it (e.g. `0x...` is base 16) or
/// defaults to base 10.
///
/// # Errors
///
/// This function can return an error in the following circumstances:
///
/// - The input byte string has non-ASCII bytes.
/// - The input byte string contains a NUL byte.
/// - The input byte string is the empty byte slice.
/// - The input byte string only contains +/- signs.
/// - The given radix does not match a `0x`-style prefix.
/// - Invalid or duplicate +/- signs are in the input.
/// - Consecutive underscores are present in the input.
/// - Leading or trailing underscores are present in the input.
/// - The input contains ASCII alphanumeric bytes that are invalid for the
///   computed radix.
/// - The input radix is out of range of [`i32`].
/// - The input radix is negative (if the input byte string does not have an
///   `0x`-style prefix) and out of range `-36..=-2`.
/// - The input raidx is out of range of `2..=36`.
///
/// See [`ArgumentError`] and [`InvalidRadixError`] for more details.
///
/// # Examples
///
/// ```
/// # use scolapasta_int_parse::{Error, parse};
/// # fn example() -> Result<(), Error<'static>> {
/// let int_max = parse("9_223_372_036_854_775_807", None)?;
/// assert_eq!(int_max, i64::MAX);
///
/// let deadbeef = parse("                       0x000000000deadbeef", None)?;
/// assert_eq!(deadbeef, 3_735_928_559);
///
/// let octal = parse("000123", None)?;
/// assert_eq!(octal, 83);
///
/// let negative = parse("-199", None)?;
/// assert_eq!(negative, -199);
///
/// let positive = parse("+199", None)?;
/// assert_eq!(positive, 199);
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// If a `Some(_)` radix is given, that radix is used:
///
/// ```
/// # use scolapasta_int_parse::{Error, parse};
/// # fn example() -> Result<(), Error<'static>> {
/// let num = parse("32xyz", Some(36))?;
/// assert_eq!(num, 5_176_187);
///
/// let binary = parse("1100_0011", Some(2))?;
/// assert_eq!(binary, 195);
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// If a `Some(_)` radix is given and it does not match the embedded radix, an
/// error is returned:
///
/// ```
/// # use scolapasta_int_parse::parse;
/// let result = parse("0b1100_0011", Some(12));
/// assert!(result.is_err());
/// ```
pub fn parse<T>(subject: &T, radix: Option<i64>) -> Result<i64, Error<'_>>
where
    T: AsRef<[u8]> + ?Sized,
{
    let subject = subject.as_ref();
    parse_inner(subject, radix)
}

fn parse_inner(subject: &[u8], radix: Option<i64>) -> Result<i64, Error<'_>> {
    // Phase 1: Ensure ASCII, ensure no NUL bytes.
    let subject = IntegerString::try_from(subject)?;
    // Phase 2: Parse radix
    let radix = if let Some(radix) = radix {
        Radix::try_base_from_str_and_i64(subject, radix)?
    } else {
        None
    };
    let mut state = ParseState::new(subject);

    // Phase 3: Trim leading whitespace.
    let mut chars = subject
        .as_bytes()
        .iter()
        .copied()
        .skip_while(u8::is_ascii_whitespace)
        .peekable();

    // Phase 4: Set sign.
    match chars.peek() {
        Some(b'+') => {
            state = state.set_sign(Sign::Positive)?;
            chars.next();
        }
        Some(b'-') => {
            state = state.set_sign(Sign::Negative)?;
            chars.next();
        }
        Some(_) => {}
        None => return Err(subject.into()),
    }

    // Phase 5: Determine radix.
    let radix = match chars.peek() {
        // https://github.com/ruby/ruby/blob/v3_1_2/bignum.c#L4094-L4115
        Some(b'0') => {
            chars.next();
            match (chars.peek(), radix) {
                (Some(b'b' | b'B'), None) => {
                    chars.next();
                    2
                }
                (Some(b'b' | b'B'), Some(radix)) if radix == 2 => {
                    chars.next();
                    2
                }
                (Some(b'o' | b'O'), None) => {
                    chars.next();
                    8
                }
                (Some(b'o' | b'O'), Some(radix)) if radix == 8 => {
                    chars.next();
                    8
                }
                (Some(b'd' | b'D'), None) => {
                    chars.next();
                    10
                }
                (Some(b'd' | b'D'), Some(radix)) if radix == 10 => {
                    chars.next();
                    10
                }
                (Some(b'x' | b'X'), None) => {
                    chars.next();
                    16
                }
                (Some(b'x' | b'X'), Some(radix)) if radix == 16 => {
                    chars.next();
                    16
                }
                (Some(b'b' | b'B' | b'o' | b'O' | b'd' | b'D' | b'x' | b'X'), Some(_)) => return Err(subject.into()),
                (Some(_) | None, None) => 8,
                (Some(_) | None, Some(radix)) => radix,
            }
        }
        Some(_) => radix.unwrap_or(10),
        None => return Err(subject.into()),
    };

    // Phase 6: Squeeze leading zeros, reject invalid underscore sequences.
    loop {
        if chars.next_if_eq(&b'0').is_some() {
            if chars.next_if_eq(&b'_').is_some() {
                match chars.peek() {
                    None | Some(b'_') => return Err(subject.into()),
                    Some(_) => {}
                }
            }
        } else if let Some(b'_') = chars.peek() {
            return Err(subject.into());
        } else {
            break;
        }
    }

    // Phase 7: Collect ASCII alphanumeric digits, reject invalid underscore
    // sequences.
    loop {
        match chars.next() {
            Some(b'_') => match chars.peek() {
                None | Some(b'_') => return Err(subject.into()),
                Some(_) => {}
            },
            Some(b) if RADIX_TABLE[usize::from(b)] <= radix => {
                state = state.collect_digit(b);
            }
            Some(_) => return Err(subject.into()),
            None => break,
        }
    }

    // Phase 8: Parse (signed) ASCII alphanumeric string to an `i64`.
    let src = state.into_numeric_string()?;
    i64::from_str_radix(&*src, radix).map_err(|_| subject.into())
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn parse_int_max() {
        let result = parse("9_223_372_036_854_775_807", None);
        assert_eq!(result.unwrap(), i64::MAX);
        let result = parse("+9_223_372_036_854_775_807", None);
        assert_eq!(result.unwrap(), i64::MAX);
    }

    #[test]
    fn parse_int_min() {
        let result = parse("-9_223_372_036_854_775_808", None);
        assert_eq!(result.unwrap(), i64::MIN);
    }

    #[test]
    fn leading_zero_does_not_imply_octal_when_given_radix() {
        // ```
        // [3.1.2] > parse('017', 12)
        // => 19
        // [3.1.2] > parse('-017', 12)
        // => -19
        // ```
        let result = parse("017", Some(12));
        assert_eq!(result.unwrap(), 19);
        let result = parse("-017", Some(12));
        assert_eq!(result.unwrap(), -19);
    }

    #[test]
    fn squeeze_leading_zeros() {
        let result = parse("0x0000000000000011", Some(16));
        assert_eq!(result.unwrap(), 17);
        let result = parse("-0x0000000000000011", Some(16));
        assert_eq!(result.unwrap(), -17);

        let result = parse("0x00_00000000000011", Some(16));
        assert_eq!(result.unwrap(), 17);
        let result = parse("-0x00_00000000000011", Some(16));
        assert_eq!(result.unwrap(), -17);

        let result = parse("0x0_0_0_11", Some(16));
        assert_eq!(result.unwrap(), 17);
        let result = parse("-0x0_0_0_11", Some(16));
        assert_eq!(result.unwrap(), -17);

        let result = parse("-0x00000_15", Some(16));
        assert_eq!(result.unwrap(), -21);
    }

    #[test]
    fn squeeze_leading_zeros_is_octal_when_octal_digits() {
        let result = parse("000000000000000000000000000000000000000123", None);
        assert_eq!(result.unwrap(), 83);
    }

    #[test]
    fn squeeze_leading_is_invalid_when_non_octal_digits() {
        parse("000000000000000000000000000000000000000987", None).unwrap_err();
    }

    #[test]
    fn squeeze_leading_zeros_enforces_no_double_underscore() {
        parse("0x___11", Some(16)).unwrap_err();
        parse("-0x___11", Some(16)).unwrap_err();
        parse("0x0___11", Some(16)).unwrap_err();
        parse("-0x0___11", Some(16)).unwrap_err();
        parse("0x_0__11", Some(16)).unwrap_err();
        parse("-0x_0__11", Some(16)).unwrap_err();
        parse("0x_00__11", Some(16)).unwrap_err();
        parse("-0x_00__11", Some(16)).unwrap_err();
    }

    #[test]
    fn no_digits_with_base_prefix() {
        parse("0x", None).unwrap_err();
        parse("0b", None).unwrap_err();
        parse("0o", None).unwrap_err();
        parse("o", None).unwrap_err();
        parse("0d", None).unwrap_err();
        parse("0X", None).unwrap_err();
        parse("0B", None).unwrap_err();
        parse("0O", None).unwrap_err();
        parse("O", None).unwrap_err();
        parse("0D", None).unwrap_err();
    }

    #[test]
    fn no_digits_with_base_prefix_neg() {
        parse("-0x", None).unwrap_err();
        parse("-0b", None).unwrap_err();
        parse("-0o", None).unwrap_err();
        parse("-o", None).unwrap_err();
        parse("-0d", None).unwrap_err();
        parse("-0X", None).unwrap_err();
        parse("-0B", None).unwrap_err();
        parse("-0O", None).unwrap_err();
        parse("-O", None).unwrap_err();
        parse("-0D", None).unwrap_err();
    }

    #[test]
    fn no_digits_with_invalid_base_prefix() {
        parse("0z", None).unwrap_err();
        parse("0z", Some(12)).unwrap_err();
    }

    #[test]
    fn no_digits_with_invalid_base_prefix_neg() {
        parse("-0z", None).unwrap_err();
        parse("-0z", Some(12)).unwrap_err();
    }

    #[test]
    fn binary_alpha_requires_zero_prefix() {
        parse("B1", None).unwrap_err();
        parse("b1", None).unwrap_err();
    }

    #[test]
    fn binary_parses() {
        let result = parse("0B1111", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("0b1111", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0B1111", None);
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0b1111", None);
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn binary_with_given_2_radix_parses() {
        let result = parse("0B1111", Some(2));
        assert_eq!(result.unwrap(), 15);
        let result = parse("0b1111", Some(2));
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0B1111", Some(2));
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0b1111", Some(2));
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn binary_with_mismatched_radix_is_err() {
        parse("0B1111", Some(24)).unwrap_err();
        parse("0b1111", Some(24)).unwrap_err();
        parse("-0B1111", Some(24)).unwrap_err();
        parse("-0b1111", Some(24)).unwrap_err();
    }

    #[test]
    fn binary_with_digits_out_of_radix_is_err() {
        parse("0B1111AH", None).unwrap_err();
        parse("0b1111ah", None).unwrap_err();
    }

    #[test]
    fn octal_alpha_requires_zero_prefix() {
        parse("O7", None).unwrap_err();
        parse("o7", None).unwrap_err();
    }

    #[test]
    fn octal_parses() {
        let result = parse("0O17", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("0o17", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0O17", None);
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0o17", None);
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn octal_with_given_8_radix_parses() {
        let result = parse("0O17", Some(8));
        assert_eq!(result.unwrap(), 15);
        let result = parse("0o17", Some(8));
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0O17", Some(8));
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0o17", Some(8));
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn octal_no_alpha_parses() {
        let result = parse("017", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("-017", None);
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn octal_no_alpha_with_given_8_radix_parses() {
        let result = parse("017", Some(8));
        assert_eq!(result.unwrap(), 15);
        let result = parse("-017", Some(8));
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn octal_with_mismatched_radix_is_err() {
        parse("0O17", Some(24)).unwrap_err();
        parse("0o17", Some(24)).unwrap_err();
        parse("-0O17", Some(24)).unwrap_err();
        parse("-0o17", Some(24)).unwrap_err();
    }

    #[test]
    fn octal_with_digits_out_of_radix_is_err() {
        parse("0O17AH", None).unwrap_err();
        parse("0o17ah", None).unwrap_err();
    }

    #[test]
    fn decimal_alpha_requires_zero_prefix() {
        parse("D9", None).unwrap_err();
        parse("d9", None).unwrap_err();
    }

    #[test]
    fn decimal_parses() {
        let result = parse("0D15", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("0d15", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0D15", None);
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0d15", None);
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn decimal_with_given_10_radix_parses() {
        let result = parse("0D15", Some(10));
        assert_eq!(result.unwrap(), 15);
        let result = parse("0d15", Some(10));
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0D15", Some(10));
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0d15", Some(10));
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn decimal_with_mismatched_radix_is_err() {
        parse("0D15", Some(24)).unwrap_err();
        parse("0d15", Some(24)).unwrap_err();
        parse("-0D15", Some(24)).unwrap_err();
        parse("-0d15", Some(24)).unwrap_err();
    }

    #[test]
    fn decimal_with_digits_out_of_radix_is_err() {
        parse("0D15AH", None).unwrap_err();
        parse("0d15ah", None).unwrap_err();
    }

    #[test]
    fn hex_alpha_requires_zero_prefix() {
        parse("XF", None).unwrap_err();
        parse("xF", None).unwrap_err();
        parse("Xf", None).unwrap_err();
        parse("xf", None).unwrap_err();
    }

    #[test]
    fn hex_parses() {
        let result = parse("0XF", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("0xF", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0XF", None);
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0xF", None);
        assert_eq!(result.unwrap(), -15);
        let result = parse("0Xf", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("0xf", None);
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0Xf", None);
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0xf", None);
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn hex_with_given_16_radix_parses() {
        let result = parse("0XF", Some(16));
        assert_eq!(result.unwrap(), 15);
        let result = parse("0xF", Some(16));
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0XF", Some(16));
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0xF", Some(16));
        assert_eq!(result.unwrap(), -15);
        let result = parse("0Xf", Some(16));
        assert_eq!(result.unwrap(), 15);
        let result = parse("0xf", Some(16));
        assert_eq!(result.unwrap(), 15);
        let result = parse("-0Xf", Some(16));
        assert_eq!(result.unwrap(), -15);
        let result = parse("-0xf", Some(16));
        assert_eq!(result.unwrap(), -15);
    }

    #[test]
    fn hex_with_mismatched_radix_is_err() {
        parse("0XF", Some(24)).unwrap_err();
        parse("0xF", Some(24)).unwrap_err();
        parse("0Xf", Some(24)).unwrap_err();
        parse("0xf", Some(24)).unwrap_err();
        parse("-0XF", Some(24)).unwrap_err();
        parse("-0xF", Some(24)).unwrap_err();
        parse("-0Xf", Some(24)).unwrap_err();
        parse("-0xf", Some(24)).unwrap_err();
    }

    #[test]
    fn hex_with_digits_out_of_radix_is_err() {
        parse("0XFAH", None).unwrap_err();
        parse("0xFah", None).unwrap_err();
        parse("0XfAH", None).unwrap_err();
        parse("0xfah", None).unwrap_err();
    }

    #[test]
    fn digits_out_of_radix_is_err() {
        parse("17AH", Some(12)).unwrap_err();
        parse("17ah", Some(12)).unwrap_err();
        parse("17AH", None).unwrap_err();
        parse("17ah", None).unwrap_err();
    }

    #[test]
    fn parsing_is_case_insensitive() {
        // ```
        // [3.1.2] > parse('abcdefgxyz', 36)
        // => 1047601316316923
        // [3.1.2] > parse('abcdefgxyz'.upcase, 36)
        // => 1047601316316923
        // ```
        let result = parse("abcdefgxyz", Some(36));
        assert_eq!(result.unwrap(), 1_047_601_316_316_923);
        let result = parse("ABCDEFGXYZ", Some(36));
        assert_eq!(result.unwrap(), 1_047_601_316_316_923);
    }

    #[test]
    fn leading_underscore_is_err() {
        parse("0x_0000001234567", None).unwrap_err();
        parse("0_x0000001234567", None).unwrap_err();
        parse("___0x0000001234567", None).unwrap_err();
    }

    #[test]
    fn double_underscore_is_err() {
        parse("0x111__11", None).unwrap_err();
    }

    #[test]
    fn trailing_underscore_is_err() {
        parse("0x111_11_", None).unwrap_err();
        parse("0x00000_", None).unwrap_err();
    }

    #[test]
    fn all_spaces_is_err() {
        parse("    ", None).unwrap_err();
    }

    #[test]
    fn empty_is_err() {
        parse("", None).unwrap_err();
    }

    #[test]
    fn more_than_one_sign_is_err() {
        parse("++12", None).unwrap_err();
        parse("+-12", None).unwrap_err();
        parse("-+12", None).unwrap_err();
        parse("--12", None).unwrap_err();
    }

    #[test]
    fn zero_radix_is_default() {
        // ```
        // [3.1.2] > Integer "0x111", 0
        // => 273
        // [3.1.2] > Integer "111", 0
        // => 111
        // ```
        let result = parse("0x111", Some(0));
        assert_eq!(result.unwrap(), 273);
        let result = parse("111", Some(0));
        assert_eq!(result.unwrap(), 111);
    }

    #[test]
    fn negative_one_radix_is_default() {
        // ```
        // [3.1.2] > Integer('0x123f'.upcase, -1)
        // => 4671
        // [3.1.2] > Integer('0x123f'.upcase, 16)
        // => 4671
        // [3.1.2] > Integer "111", -1
        // => 111
        // ```
        let result = parse("0x123f", Some(-1));
        assert_eq!(result.unwrap(), 4671);
        let result = parse("111", Some(-1));
        assert_eq!(result.unwrap(), 111);
    }

    #[test]
    fn one_radix_is_err() {
        parse("0x123f", Some(1)).unwrap_err();
        parse("111", Some(1)).unwrap_err();
    }

    #[test]
    fn out_of_range_radix_is_err() {
        parse("0x123f", Some(1200)).unwrap_err();
        parse("123", Some(1200)).unwrap_err();
        parse("123", Some(-1200)).unwrap_err();
    }

    #[test]
    fn literals_with_negative_out_of_range_radix_ignore_radix() {
        let result = parse("0x123f", Some(-1200));
        assert_eq!(result.unwrap(), 4671);
    }

    #[test]
    fn negative_radix_in_valid_range_is_parsed() {
        // ```
        // [3.1.2] > Integer "111", -2
        // => 7
        // [3.1.2] > Integer "111", -10
        // => 111
        // [3.1.2] > Integer "111", -36
        // => 1333
        // ```
        let result = parse("111", Some(-2));
        assert_eq!(result.unwrap(), 7);
        let result = parse("111", Some(-10));
        assert_eq!(result.unwrap(), 111);
        let result = parse("111", Some(-36));
        assert_eq!(result.unwrap(), 1333);
    }

    #[test]
    fn all_valid_radixes() {
        // ```
        // (2..36).each {|r| puts "(\"111\", #{r}, #{Integer "111", r}),"; nil }.to_a.uniq
        // (2..36).each {|r| puts "(\"111\", #{-r}, #{Integer "111", -r}),"; nil }.to_a.uniq
        // ```
        let test_cases = [
            ("111", 2, 7),
            ("111", 3, 13),
            ("111", 4, 21),
            ("111", 5, 31),
            ("111", 6, 43),
            ("111", 7, 57),
            ("111", 8, 73),
            ("111", 9, 91),
            ("111", 10, 111),
            ("111", 11, 133),
            ("111", 12, 157),
            ("111", 13, 183),
            ("111", 14, 211),
            ("111", 15, 241),
            ("111", 16, 273),
            ("111", 17, 307),
            ("111", 18, 343),
            ("111", 19, 381),
            ("111", 20, 421),
            ("111", 21, 463),
            ("111", 22, 507),
            ("111", 23, 553),
            ("111", 24, 601),
            ("111", 25, 651),
            ("111", 26, 703),
            ("111", 27, 757),
            ("111", 28, 813),
            ("111", 29, 871),
            ("111", 30, 931),
            ("111", 31, 993),
            ("111", 32, 1057),
            ("111", 33, 1123),
            ("111", 34, 1191),
            ("111", 35, 1261),
            ("111", 36, 1333),
            ("111", -2, 7),
            ("111", -3, 13),
            ("111", -4, 21),
            ("111", -5, 31),
            ("111", -6, 43),
            ("111", -7, 57),
            ("111", -8, 73),
            ("111", -9, 91),
            ("111", -10, 111),
            ("111", -11, 133),
            ("111", -12, 157),
            ("111", -13, 183),
            ("111", -14, 211),
            ("111", -15, 241),
            ("111", -16, 273),
            ("111", -17, 307),
            ("111", -18, 343),
            ("111", -19, 381),
            ("111", -20, 421),
            ("111", -21, 463),
            ("111", -22, 507),
            ("111", -23, 553),
            ("111", -24, 601),
            ("111", -25, 651),
            ("111", -26, 703),
            ("111", -27, 757),
            ("111", -28, 813),
            ("111", -29, 871),
            ("111", -30, 931),
            ("111", -31, 993),
            ("111", -32, 1057),
            ("111", -33, 1123),
            ("111", -34, 1191),
            ("111", -35, 1261),
            ("111", -36, 1333),
        ];
        for (subject, radix, output) in test_cases {
            let result = parse(subject, Some(radix));
            assert_eq!(
                result.unwrap(),
                output,
                "Mismatched output for test case ({subject}, {radix}, {output})"
            );
        }
    }

    #[test]
    fn int_max_radix_does_not_panic() {
        parse("111", Some(i64::MAX)).unwrap_err();
    }

    #[test]
    fn int_min_radix_does_not_panic() {
        parse("111", Some(i64::MIN)).unwrap_err();
    }
}
