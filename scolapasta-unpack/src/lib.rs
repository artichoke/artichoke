#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directive {
    /// Integer directives
    ///
    /// Consists of various unsigned and signed integer directives.
    ///
    /// The corresponding characters for each directive are as follows:
    ///
    /// Unsigned directives:
    /// - C: Unsigned 8-bit integer (unsigned char)
    /// - S: Unsigned 16-bit integer, native endian (uint16_t)
    /// - L: Unsigned 32-bit integer, native endian (uint32_t)
    /// - Q: Unsigned 64-bit integer, native endian (uint64_t)
    /// - J: Unsigned pointer-width integer, native endian (uintptr_t)
    /// - S_, S!, S>, S!>: Unsigned short, native endian
    /// - I, I_, I!, I!>: Unsigned int, native endian
    /// - L_, L!, L>, L!>: Unsigned long, native endian
    /// - Q_, Q!, Q>, Q!>: Unsigned long long, native endian (ArgumentError if the platform has no long long type)
    /// - J!, J>!, J!>, J!>>: uintptr_t, native endian (same as J)
    /// - n: 16-bit unsigned integer, network (big-endian) byte order
    /// - N: 32-bit unsigned integer, network (big-endian) byte order
    /// - v: 16-bit unsigned integer, VAX (little-endian) byte order
    /// - V: 32-bit unsigned integer, VAX (little-endian) byte order
    ///
    /// Signed directives:
    /// - c: Signed 8-bit integer (signed char)
    /// - s: Signed 16-bit integer, native endian (int16_t)
    /// - l: Signed 32-bit integer, native endian (int32_t)
    /// - q: Signed 64-bit integer, native endian (int64_t)
    /// - j: Signed pointer-width integer, native endian (intptr_t)
    /// - s_, s!, s<, s!<: Signed short, native endian
    /// - i, i_, i!, i!>: Signed int, native endian
    /// - l_, l!, l<, l!<: Signed long, native endian
    /// - q_, q!, q<, q!<: Signed long long, native endian (ArgumentError if the platform has no long long type)
    /// - j!, j>!, j<, j!<: intptr_t, native endian (same as j)
    ///
    /// Miscellaneous directives:
    /// - U: UTF-8 character
    /// - w: BER-compressed integer (see Array#pack)
    ///
    /// These directives represent various integer types with different sizes, endianness,
    /// and additional miscellaneous types.
    Integer(IntegerDirective),

    /// Float directives
    ///
    /// Represents various floating-point directives.
    ///
    /// The corresponding characters for each directive are as follows:
    ///
    /// - D, d: Double, native format
    /// - F, f: Single, native format
    /// - E: Double, little-endian byte order
    /// - e: Single, little-endian byte order
    /// - G: Double, network (big-endian) byte order
    /// - g: Single, network (big-endian) byte order
    Float(FloatDirective),

    /// String directives
    ///
    /// Represents various string directives.
    ///
    /// The corresponding characters for each directive are as follows:
    ///
    /// - A: Arbitrary binary
    /// - a: Arbitrary binary
    /// - Z: Null-terminated string
    /// - B: Bit string (MSB first)
    /// - b: Bit string (LSB first)
    /// - H: Hex string (high nibble first)
    /// - h: Hex string (low nibble first)
    /// - u: UU-encoded string
    /// - M: Quoted-printable, MIME encoding
    /// - m: Base64-encoded string (RFC 2045)
    ///   - m0: Base64-encoded string (RFC 4648)
    /// - P: Pointer to a structure (fixed-length string)
    /// - p: Pointer to a null-terminated string
    String(StringDirective),

    /// Miscellaneous directives
    ///
    /// Represents various miscellaneous directives.
    ///
    /// The corresponding characters for each directive are as follows:
    ///
    /// - @: Skip to offset
    /// - X: Skip backward
    /// - x: Skip forward
    Miscellaneous(MiscellaneousDirective),
}

impl TryFrom<u8> for Directive {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Ok(directive) = IntegerDirective::try_from(value) {
            return Ok(Self::Integer(directive));
        }
        if let Ok(directive) = FloatDirective::try_from(value) {
            return Ok(Self::Float(directive));
        }
        if let Ok(directive) = StringDirective::try_from(value) {
            return Ok(Self::String(directive));
        }
        if let Ok(directive) = MiscellaneousDirective::try_from(value) {
            return Ok(Self::Miscellaneous(directive));
        }
        Err(())
    }
}

/// Integer directives
///
/// C (8-bit unsigned integer)
/// S (16-bit unsigned, native endian)
/// L (32-bit unsigned, native endian)
/// Q (64-bit unsigned, native endian)
/// J (Pointer width unsigned, native endian)
///
/// c (8-bit signed integer)
/// s (16-bit signed, native endian)
/// l (32-bit signed, native endian)
/// q (64-bit signed, native endian)
/// j (Pointer width signed, native endian)
///
/// S_, S! (Unsigned short, native endian)
/// I, I_, I! (Unsigned int, native endian)
/// L_, L! (Unsigned long, native endian)
/// Q_, Q! (Unsigned long long, native endian)
///
/// J! (uintptr_t, native endian)
///
/// s_, s! (Signed short, native endian)
/// i, i_, i! (Signed int, native endian)
/// l_, l! (Signed long, native endian)
/// q_, q! (Signed long long, native endian)
///
/// j! (intptr_t, native endian)
///
/// S>, s>, S!>, s!> (Same as the directives without ">", big endian)
/// L>, l>, L!>, l!> (Same as the directives without ">", big endian)
/// I!> (Same as "I", big endian)
/// Q>, q>, Q!>, q!> (Same as the directives without ">", big endian)
/// J>, j>, J!>, j!> (Same as the directives without ">", big endian)
///
/// S<, s<, S!<, s!< (Same as the directives without "<", little endian)
/// L<, l<, L!<, l!< (Same as the directives without "<", little endian)
/// I!< (Same as "I", little endian)
/// Q<, q<, Q!<, q!< (Same as the directives without "<", little endian)
/// J<, j<, J!<, j!< (Same as the directives without "<", little endian)
///
/// n (16-bit unsigned, network (big-endian) byte order)
/// N (32-bit unsigned, network (big-endian) byte order)
/// v (16-bit unsigned, VAX (little-endian) byte order)
/// V (32-bit unsigned, VAX (little-endian) byte order)
///
/// U (UTF-8 character)
/// w (BER-compressed integer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerDirective {
    /// 8-bit unsigned integer (`C`)
    Unsigned8,

    /// 16-bit unsigned, native endian (`S`)
    Unsigned16NativeEndian,

    /// 32-bit unsigned, native endian (`L`)
    Unsigned32NativeEndian,

    /// 64-bit unsigned, native endian (`Q`)
    Unsigned64NativeEndian,

    /// Pointer width unsigned, native endian (`J`)
    UnsignedPointerWidthNativeEndian,

    /// 8-bit signed integer (`c`)
    Signed8,

    /// 16-bit signed, native endian (`s`)
    Signed16NativeEndian,

    /// 32-bit signed, native endian (`l`)
    Signed32NativeEndian,

    /// 64-bit signed, native endian (`q`)
    Signed64NativeEndian,

    /// Pointer width signed, native endian (`j`)
    SignedPointerWidthNativeEndian,

    /// 16-bit unsigned, network (big-endian) byte order (`n`)
    Unsigned16BigEndian,

    /// 32-bit unsigned, network (big-endian) byte order (`N`)
    Unsigned32BigEndian,

    /// 16-bit unsigned, VAX (little-endian) byte order (`v`)
    Unsigned16LittleEndian,

    /// 32-bit unsigned, VAX (little-endian) byte order (`V`)
    Unsigned32LittleEndian,

    /// UTF-8 character (`U`)
    Utf8Character,

    /// BER-compressed integer (`w`)
    BerCompressedInteger,
}

impl TryFrom<u8> for IntegerDirective {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'C' => Ok(Self::Unsigned8),
            b'S' => Ok(Self::Unsigned16NativeEndian),
            b'L' => Ok(Self::Unsigned32NativeEndian),
            b'Q' => Ok(Self::Unsigned64NativeEndian),
            b'J' => Ok(Self::UnsignedPointerWidthNativeEndian),
            b'c' => Ok(Self::Signed8),
            b's' => Ok(Self::Signed16NativeEndian),
            b'l' => Ok(Self::Signed32NativeEndian),
            b'q' => Ok(Self::Signed64NativeEndian),
            b'j' => Ok(Self::SignedPointerWidthNativeEndian),
            b'n' => Ok(Self::Unsigned16BigEndian),
            b'N' => Ok(Self::Unsigned32BigEndian),
            b'v' => Ok(Self::Unsigned16LittleEndian),
            b'V' => Ok(Self::Unsigned32LittleEndian),
            b'U' => Ok(Self::Utf8Character),
            b'w' => Ok(Self::BerCompressedInteger),
            _ => Err(()),
        }
    }
}

/// Float directives
///
/// D, d (Double-precision, native format)
/// F, f (Single-precision, native format)
/// E (Double-precision, little-endian byte order)
/// e (Single-precision, little-endian byte order)
/// G (Double-precision, network (big-endian) byte order)
/// g (Single-precision, network (big-endian) byte order)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatDirective {
    /// Double-precision, native format (`D`, `d`)
    DoubleNativeEndian,

    /// Single-precision, native format (`F`, `f`)
    SingleNativeEndian,

    /// Double-precision, little-endian byte order (`E`)
    DoubleLittleEndian,

    /// Single-precision, little-endian byte order (`e`)
    SingleLittleEndian,

    /// Double-precision, network (big-endian) byte order (`G`)
    DoubleBigEndian,

    /// Single-precision, network (big-endian) byte order (`g`)
    SingleBigEndian,
}

impl TryFrom<u8> for FloatDirective {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'D' | b'd' => Ok(Self::DoubleNativeEndian),
            b'F' | b'f' => Ok(Self::SingleNativeEndian),
            b'E' => Ok(Self::DoubleLittleEndian),
            b'e' => Ok(Self::SingleLittleEndian),
            b'G' => Ok(Self::DoubleBigEndian),
            b'g' => Ok(Self::SingleBigEndian),
            _ => Err(()),
        }
    }
}

/// String directives
///
/// A (Arbitrary binary string, remove trailing nulls and ASCII spaces)
/// a (Arbitrary binary string)
/// Z (Null-terminated string)
/// B (Bit string, MSB first)
/// b (Bit string, LSB first)
/// H (Hex string, high nibble first)
/// h (Hex string, low nibble first)
/// U (UU-encoded string)
/// M (Quoted-printable, MIME encoding - see RFC2045)
/// m (Base64 encoded string - RFC 2045 default, RFC 4648 if followed by 0)
/// P (Pointer to a structure, fixed-length string)
/// p (Pointer to a null-terminated string)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringDirective {
    /// Arbitrary binary string, remove trailing nulls and ASCII spaces (`A`)
    ArbitraryBinaryTrimmed,

    /// Arbitrary binary string (`a`)
    ArbitraryBinary,

    /// Null-terminated string (`Z`)
    NullTerminated,

    /// Bit string, MSB first (`B`)
    BitStringMsbFirst,

    /// Bit string, LSB first (`b`)
    BitStringLsbFirst,

    /// Hex string, high nibble first (`H`)
    HexStringHighNibbleFirst,

    /// Hex string, low nibble first (`h`)
    HexStringLowNibbleFirst,

    /// UU-encoded string (`U`)
    UuEncoded,

    /// Quoted-printable, MIME encoding - see RFC2045 (`M`)
    QuotedPrintable,

    /// Base64 encoded string - RFC 2045 default, RFC 4648 if followed by 0 (`m`)
    Base64Encoded,

    /// Pointer to a structure, fixed-length string (`P`)
    StructurePointer,

    /// Pointer to a null-terminated string (`p`)
    NullTerminatedStringPointer,
}

impl TryFrom<u8> for StringDirective {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(Self::ArbitraryBinaryTrimmed),
            b'a' => Ok(Self::ArbitraryBinary),
            b'Z' => Ok(Self::NullTerminated),
            b'B' => Ok(Self::BitStringMsbFirst),
            b'b' => Ok(Self::BitStringLsbFirst),
            b'H' => Ok(Self::HexStringHighNibbleFirst),
            b'h' => Ok(Self::HexStringLowNibbleFirst),
            b'u' => Ok(Self::UuEncoded),
            b'M' => Ok(Self::QuotedPrintable),
            b'm' => Ok(Self::Base64Encoded),
            b'P' => Ok(Self::StructurePointer),
            b'p' => Ok(Self::NullTerminatedStringPointer),
            _ => Err(()),
        }
    }
}

/// Miscellaneous directives
///
/// @ (Skip to offset)
/// X (Skip backward)
/// x (Skip forward)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiscellaneousDirective {
    /// Skip to offset (`@`)
    SkipToOffset,

    /// Skip backward (`X`)
    SkipBackward,

    /// Skip forward (`x`)
    SkipForward,
}

impl TryFrom<u8> for MiscellaneousDirective {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'@' => Ok(Self::SkipToOffset),
            b'X' => Ok(Self::SkipBackward),
            b'x' => Ok(Self::SkipForward),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectiveType {
    Repeat(usize),
    ConsumeToEnd,
    PlatformNativeSizeUnderscore,
    PlatformNativeSizeBang,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_integer_directive() {
        assert_eq!(IntegerDirective::try_from(b'C'), Ok(IntegerDirective::Unsigned8));
        assert_eq!(
            IntegerDirective::try_from(b'S'),
            Ok(IntegerDirective::Unsigned16NativeEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'L'),
            Ok(IntegerDirective::Unsigned32NativeEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'Q'),
            Ok(IntegerDirective::Unsigned64NativeEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'J'),
            Ok(IntegerDirective::UnsignedPointerWidthNativeEndian)
        );
        assert_eq!(IntegerDirective::try_from(b'c'), Ok(IntegerDirective::Signed8));
        assert_eq!(
            IntegerDirective::try_from(b's'),
            Ok(IntegerDirective::Signed16NativeEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'l'),
            Ok(IntegerDirective::Signed32NativeEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'q'),
            Ok(IntegerDirective::Signed64NativeEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'j'),
            Ok(IntegerDirective::SignedPointerWidthNativeEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'n'),
            Ok(IntegerDirective::Unsigned16BigEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'N'),
            Ok(IntegerDirective::Unsigned32BigEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'v'),
            Ok(IntegerDirective::Unsigned16LittleEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'V'),
            Ok(IntegerDirective::Unsigned32LittleEndian)
        );
        assert_eq!(IntegerDirective::try_from(b'U'), Ok(IntegerDirective::Utf8Character));
        assert_eq!(
            IntegerDirective::try_from(b'w'),
            Ok(IntegerDirective::BerCompressedInteger)
        );

        // Test invalid directive
        assert_eq!(IntegerDirective::try_from(b'X'), Err(()));
    }
    #[test]
    fn test_try_from_float_directive() {
        assert_eq!(FloatDirective::try_from(b'D'), Ok(FloatDirective::DoubleNativeEndian));
        assert_eq!(FloatDirective::try_from(b'd'), Ok(FloatDirective::DoubleNativeEndian));
        assert_eq!(FloatDirective::try_from(b'F'), Ok(FloatDirective::SingleNativeEndian));
        assert_eq!(FloatDirective::try_from(b'f'), Ok(FloatDirective::SingleNativeEndian));
        assert_eq!(FloatDirective::try_from(b'E'), Ok(FloatDirective::DoubleLittleEndian));
        assert_eq!(FloatDirective::try_from(b'e'), Ok(FloatDirective::SingleLittleEndian));
        assert_eq!(FloatDirective::try_from(b'G'), Ok(FloatDirective::DoubleBigEndian));
        assert_eq!(FloatDirective::try_from(b'g'), Ok(FloatDirective::SingleBigEndian));

        // Test invalid directive
        assert_eq!(FloatDirective::try_from(b'X'), Err(()));
    }

    #[test]
    fn test_try_from_string_directive() {
        assert_eq!(
            StringDirective::try_from(b'A'),
            Ok(StringDirective::ArbitraryBinaryTrimmed)
        );
        assert_eq!(StringDirective::try_from(b'a'), Ok(StringDirective::ArbitraryBinary));
        assert_eq!(StringDirective::try_from(b'Z'), Ok(StringDirective::NullTerminated));
        assert_eq!(StringDirective::try_from(b'B'), Ok(StringDirective::BitStringMsbFirst));
        assert_eq!(StringDirective::try_from(b'b'), Ok(StringDirective::BitStringLsbFirst));
        assert_eq!(
            StringDirective::try_from(b'H'),
            Ok(StringDirective::HexStringHighNibbleFirst)
        );
        assert_eq!(
            StringDirective::try_from(b'h'),
            Ok(StringDirective::HexStringLowNibbleFirst)
        );
        assert_eq!(StringDirective::try_from(b'u'), Ok(StringDirective::UuEncoded));
        assert_eq!(StringDirective::try_from(b'M'), Ok(StringDirective::QuotedPrintable));
        assert_eq!(StringDirective::try_from(b'm'), Ok(StringDirective::Base64Encoded));
        assert_eq!(StringDirective::try_from(b'P'), Ok(StringDirective::StructurePointer));
        assert_eq!(
            StringDirective::try_from(b'p'),
            Ok(StringDirective::NullTerminatedStringPointer)
        );

        // Test invalid directive
        assert_eq!(StringDirective::try_from(b'X'), Err(()));
    }

    #[test]
    fn test_try_from_miscellaneous_directive() {
        assert_eq!(
            MiscellaneousDirective::try_from(b'@'),
            Ok(MiscellaneousDirective::SkipToOffset)
        );
        assert_eq!(
            MiscellaneousDirective::try_from(b'X'),
            Ok(MiscellaneousDirective::SkipBackward)
        );
        assert_eq!(
            MiscellaneousDirective::try_from(b'x'),
            Ok(MiscellaneousDirective::SkipForward)
        );

        // Test invalid directive
        assert_eq!(MiscellaneousDirective::try_from(b'Y'), Err(()));
    }

    #[test]
    fn test_integer_directives_cannot_parse_as_other_types() {
        #[rustfmt::skip]
        let integer_directives = [
            b'C', b'S', b'L', b'Q', b'J',
            b'c', b's', b'l', b'q', b'j',
            b'S', b'I', b'L', b'Q',
            b'J',
            b's', b'i', b'l', b'q',
            b'j',
            b'n', b'N', b'v', b'V',
            b'U', b'w',
        ];

        for directive in integer_directives {
            // Try parsing as FloatDirective
            FloatDirective::try_from(directive).unwrap_err();

            // Try parsing as StringDirective
            StringDirective::try_from(directive).unwrap_err();

            // Try parsing as MiscellaneousDirective
            MiscellaneousDirective::try_from(directive).unwrap_err();
        }
    }

    #[test]
    fn test_float_directives_cannot_parse_as_other_types() {
        let float_directives = [b'D', b'd', b'F', b'f', b'E', b'e', b'G', b'g'];

        for directive in float_directives {
            // Try parsing as IntegerDirective
            IntegerDirective::try_from(directive).unwrap_err();

            // Try parsing as StringDirective
            StringDirective::try_from(directive).unwrap_err();

            // Try parsing as MiscellaneousDirective
            MiscellaneousDirective::try_from(directive).unwrap_err();
        }
    }

    #[test]
    fn test_string_directives_cannot_parse_as_other_types() {
        let string_directives = [b'A', b'a', b'Z', b'B', b'b', b'H', b'h', b'u', b'M', b'm', b'P', b'p'];

        for directive in string_directives {
            // Try parsing as IntegerDirective
            IntegerDirective::try_from(directive).unwrap_err();

            // Try parsing as FloatDirective
            FloatDirective::try_from(directive).unwrap_err();

            // Try parsing as MiscellaneousDirective
            MiscellaneousDirective::try_from(directive).unwrap_err();
        }
    }

    #[test]
    fn test_miscellaneous_directives_cannot_parse_as_other_types() {
        let miscellaneous_directives = [b'@', b'X', b'x'];

        for directive in miscellaneous_directives {
            // Try parsing as IntegerDirective
            IntegerDirective::try_from(directive).unwrap_err();

            // Try parsing as FloatDirective
            FloatDirective::try_from(directive).unwrap_err();

            // Try parsing as StringDirective
            StringDirective::try_from(directive).unwrap_err();
        }
    }
}
