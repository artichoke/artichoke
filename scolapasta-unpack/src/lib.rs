/// Enum representing different directives used for parsing format strings in
/// Ruby's [`String#unpack`].
///
/// [`String#unpack`]: https://ruby-doc.org/core-3.1.2/String.html#method-i-unpack
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directive {
    /// Integer directives
    ///
    /// Consists of various unsigned and signed integer directives.
    ///
    /// The corresponding characters for each directive are as follows:
    ///
    /// | Integer Directive | Returns  | Meaning                                                                      |
    /// |-------------------|----------|------------------------------------------------------------------------------|
    /// | `C`               | Integer  | 8-bit unsigned (`unsigned char`)                                             |
    /// | `S`               | Integer  | 16-bit unsigned, native endian (`uint16_t`)                                  |
    /// | `L`               | Integer  | 32-bit unsigned, native endian (`uint32_t`)                                  |
    /// | `Q`               | Integer  | 64-bit unsigned, native endian (`uint64_t`)                                  |
    /// | `J`               | Integer  | pointer width unsigned, native endian (`uintptr_t`)                          |
    /// | `c`               | Integer  | 8-bit signed (`signed char`)                                                 |
    /// | `s`               | Integer  | 16-bit signed, native endian (`int16_t`)                                     |
    /// | `l`               | Integer  | 32-bit signed, native endian (`int32_t`)                                     |
    /// | `q`               | Integer  | 64-bit signed, native endian (`int64_t`)                                     |
    /// | `j`               | Integer  | pointer width signed, native endian (`intptr_t`)                             |
    /// | `S_`, `S!`        | Integer  | `unsigned short`, native endian                                              |
    /// | `I`, `I_`, `I!`   | Integer  | `unsigned int`, native endian                                                |
    /// | `L_`, `L!`        | Integer  | `unsigned long`, native endian                                               |
    /// | `Q_`, `Q!`        | Integer  | `unsigned long long`, native endian (`ArgumentError` if no `long long` type) |
    /// | `J!`              | Integer  | `uintptr_t`, native endian (same as `J`)                                     |
    /// | `s_`, `s!`        | Integer  | `signed short`, native endian                                                |
    /// | `i`, `i_`, `i!`   | Integer  | `signed int`, native endian                                                  |
    /// | `l_`, `l!`        | Integer  | `signed long`, native endian                                                 |
    /// | `q_`, `q!`        | Integer  | `signed long long`, native endian (ArgumentError if no long long type)       |
    /// | `j!`              | Integer  | `intptr_t`, native endian (same as `j`)                                      |
    /// | `S>`, `s>`, `S!>`, `s!>`  | Integer  | same as directives without ">" except big endian. `S>` is the same as `n` |
    /// | `L>`, `l>`, `L!>`, `l!>`  | Integer  | same as directives without ">" except big endian. `L>` is the same as `N` |
    /// | `I!>`, `i!>`              | Integer  | same as directives without ">" except big endian                     |
    /// | `Q>`, `q>`, `Q!>`, `q!>`  | Integer  | same as directives without ">" except big endian                     |
    /// | `J>`, `j>`, `J!>`, `j!>`  | Integer  | same as directives without ">" except big endian                     |
    /// | `S<`, `s<`, `S!<`, `s!<`  | Integer  | same as directives without "<" except little endian. `S<` is the same as `v` |
    /// | `L<`, `l<`, `L!<`, `l!<`  | Integer  | same as directives without "<" except little endian. `L<` is the same as `V` |
    /// | `I!<`, `i!<`              | Integer  | same as directives without "<" except little endian                  |
    /// | `Q<`, `q<`, `Q!<`, `q!<`  | Integer  | same as directives without "<" except little endian                  |
    /// | `J<`, `j<`, `J!<`, `j!<`  | Integer  | same as directives without "<" except little endian                  |
    /// | `n`               | Integer  | 16-bit unsigned, network (big-endian) byte order                             |
    /// | `N`               | Integer  | 32-bit unsigned, network (big-endian) byte order                             |
    /// | `v`               | Integer  | 16-bit unsigned, VAX (little-endian) byte order                              |
    /// | `V`               | Integer  | 32-bit unsigned, VAX (little-endian) byte order                              |
    /// | `U`               | Integer  | UTF-8 character                                                              |
    /// | `w`               | Integer  | BER-compressed integer (see [`Array#pack`])                                  |
    ///
    /// [`Array#pack`]: https://ruby-doc.org/core-3.1.2/Array.html#method-i-pack
    Integer(IntegerDirective),

    /// Float directives
    ///
    /// Consists of various float directives.
    ///
    /// The corresponding characters for each directive are as follows:
    ///
    /// | Float Directive | Returns  | Meaning                                                           |
    /// |-----------------|----------|-------------------------------------------------------------------|
    /// | `D`, `d`        | Float    | double-precision, native format                                   |
    /// | `F`, `f`        | Float    | single-precision, native format                                   |
    /// | `E`             | Float    | double-precision, little-endian byte order                        |
    /// | `e`             | Float    | single-precision, little-endian byte order                        |
    /// | `G`             | Float    | double-precision, network (big-endian) byte order                 |
    /// | `g`             | Float    | single-precision, network (big-endian) byte order                 |
    Float(FloatDirective),

    /// String directives
    ///
    /// Consists of various string directives.
    ///
    /// The corresponding characters for each directive are as follows:
    ///
    /// | String Directive | Returns | Meaning                                                          |
    /// |------------------|---------|------------------------------------------------------------------|
    /// | `A`              | String  | arbitrary binary string (remove trailing nulls and ASCII spaces) |
    /// | `a`              | String  | arbitrary binary string                                          |
    /// | `Z`              | String  | null-terminated string                                           |
    /// | `B`              | String  | bit string (MSB first)                                           |
    /// | `b`              | String  | bit string (LSB first)                                           |
    /// | `H`              | String  | hex string (high nibble first)                                   |
    /// | `h`              | String  | hex string (low nibble first)                                    |
    /// | `u`              | String  | UU-encoded string                                                |
    /// | `M`              | String  | quoted-printable, MIME encoding (see RFC2045)                    |
    /// | `m`              | String  | base64 encoded string (RFC 2045) (default)                       |
    /// | `m0`             | String  | base64 encoded string (RFC 4648) if followed by 0                |
    /// | `P`              | String  | pointer to a structure (fixed-length string)                     |
    /// | `p`              | String  | pointer to a null-terminated string                              |
    String(StringDirective),

    /// Miscellaneous directives
    ///
    /// Consists of various miscellaneous directives.
    ///
    /// The corresponding characters for each directive are as follows:
    ///
    /// | Directive | Returns | Meaning                                         |
    /// |-----------|---------|-------------------------------------------------|
    /// | `@`       | ---     | skip to the offset given by the length argument |
    /// | `X`       | ---     | skip backward one byte                          |
    /// | `x`       | ---     | skip forward one byte                           |
    Miscellaneous(MiscellaneousDirective),

    /// Unknown directives
    ///
    /// Unpacking cannot fail. All unknown directives are ignored, but parsed as
    /// valid directives.
    ///
    /// ```console
    /// [3.2.2] > "aa".unpack('b-10b')
    /// <internal:pack>:20: warning: unknown unpack directive '-' in 'b-10b'
    /// => ["1", "1"]
    /// ```
    ///
    /// All unknown directives print a warning:
    ///
    /// ```console
    /// [3.2.2] > "aa".unpack('b-10{b')
    /// <internal:pack>:20: warning: unknown unpack directive '-' in 'b-10{b'
    /// <internal:pack>:20: warning: unknown unpack directive '{' in 'b-10{b'
    /// => ["1", "1"]
    /// ```
    Unknown(u8),
}

impl From<u8> for Directive {
    fn from(value: u8) -> Self {
        if let Ok(directive) = IntegerDirective::try_from(value) {
            return Self::Integer(directive);
        }
        if let Ok(directive) = FloatDirective::try_from(value) {
            return Self::Float(directive);
        }
        if let Ok(directive) = StringDirective::try_from(value) {
            return Self::String(directive);
        }
        if let Ok(directive) = MiscellaneousDirective::try_from(value) {
            return Self::Miscellaneous(directive);
        }
        Self::Unknown(value)
    }
}

/// Integer directives
///
/// Consists of various unsigned and signed integer directives.
///
/// The corresponding characters for each directive are as follows:
///
/// | Integer Directive | Returns  | Meaning                                                                      |
/// |-------------------|----------|------------------------------------------------------------------------------|
/// | `C`               | Integer  | 8-bit unsigned (`unsigned char`)                                             |
/// | `S`               | Integer  | 16-bit unsigned, native endian (`uint16_t`)                                  |
/// | `L`               | Integer  | 32-bit unsigned, native endian (`uint32_t`)                                  |
/// | `Q`               | Integer  | 64-bit unsigned, native endian (`uint64_t`)                                  |
/// | `J`               | Integer  | pointer width unsigned, native endian (`uintptr_t`)                          |
/// | `c`               | Integer  | 8-bit signed (`signed char`)                                                 |
/// | `s`               | Integer  | 16-bit signed, native endian (`int16_t`)                                     |
/// | `l`               | Integer  | 32-bit signed, native endian (`int32_t`)                                     |
/// | `q`               | Integer  | 64-bit signed, native endian (`int64_t`)                                     |
/// | `j`               | Integer  | pointer width signed, native endian (`intptr_t`)                             |
/// | `S_`, `S!`        | Integer  | `unsigned short`, native endian                                              |
/// | `I`, `I_`, `I!`   | Integer  | `unsigned int`, native endian                                                |
/// | `L_`, `L!`        | Integer  | `unsigned long`, native endian                                               |
/// | `Q_`, `Q!`        | Integer  | `unsigned long long`, native endian (`ArgumentError` if no `long long` type) |
/// | `J!`              | Integer  | `uintptr_t`, native endian (same as `J`)                                     |
/// | `s_`, `s!`        | Integer  | `signed short`, native endian                                                |
/// | `i`, `i_`, `i!`   | Integer  | `signed int`, native endian                                                  |
/// | `l_`, `l!`        | Integer  | `signed long`, native endian                                                 |
/// | `q_`, `q!`        | Integer  | `signed long long`, native endian (ArgumentError if no long long type)       |
/// | `j!`              | Integer  | `intptr_t`, native endian (same as `j`)                                      |
/// | `S>`, `s>`, `S!>`, `s!>`  | Integer  | same as directives without ">" except big endian. `S>` is the same as `n` |
/// | `L>`, `l>`, `L!>`, `l!>`  | Integer  | same as directives without ">" except big endian. `L>` is the same as `N` |
/// | `I!>`, `i!>`              | Integer  | same as directives without ">" except big endian                     |
/// | `Q>`, `q>`, `Q!>`, `q!>`  | Integer  | same as directives without ">" except big endian                     |
/// | `J>`, `j>`, `J!>`, `j!>`  | Integer  | same as directives without ">" except big endian                     |
/// | `S<`, `s<`, `S!<`, `s!<`  | Integer  | same as directives without "<" except little endian. `S<` is the same as `v` |
/// | `L<`, `l<`, `L!<`, `l!<`  | Integer  | same as directives without "<" except little endian. `L<` is the same as `V` |
/// | `I!<`, `i!<`              | Integer  | same as directives without "<" except little endian                  |
/// | `Q<`, `q<`, `Q!<`, `q!<`  | Integer  | same as directives without "<" except little endian                  |
/// | `J<`, `j<`, `J!<`, `j!<`  | Integer  | same as directives without "<" except little endian                  |
/// | `n`               | Integer  | 16-bit unsigned, network (big-endian) byte order                             |
/// | `N`               | Integer  | 32-bit unsigned, network (big-endian) byte order                             |
/// | `v`               | Integer  | 16-bit unsigned, VAX (little-endian) byte order                              |
/// | `V`               | Integer  | 32-bit unsigned, VAX (little-endian) byte order                              |
/// | `U`               | Integer  | UTF-8 character                                                              |
/// | `w`               | Integer  | BER-compressed integer (see [`Array#pack`])                                  |
///
/// [`Array#pack`]: https://ruby-doc.org/core-3.1.2/Array.html#method-i-pack
///
/// These directives represent various integer types with different sizes, endianness,
/// and additional miscellaneous types.
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
    ///
    /// Also known as `J!`.
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

    /// 16-bit unsigned, big endian (`S>`)
    Unsigned16BigEndian,

    /// 32-bit unsigned, big endian (`L>`)
    Unsigned32BigEndian,

    /// 64-bit unsigned, big endian (`Q>`)
    Unsigned64BigEndian,

    /// Pointer width unsigned, big endian (`J>`)
    UnsignedPointerWidthBigEndian,

    /// 16-bit signed, big endian (`s>`)
    Signed16BigEndian,

    /// 32-bit signed, big endian (`l>`)
    Signed32BigEndian,

    /// 64-bit signed, big endian (`q>`)
    Signed64BigEndian,

    /// Pointer width signed, big endian (`j>`)
    SignedPointerWidthBigEndian,

    /// 16-bit unsigned, little endian (`S<`)
    Unsigned16LittleEndian,

    /// 32-bit unsigned, little endian (`L<`)
    Unsigned32LittleEndian,

    /// 64-bit unsigned, little endian (`Q<`)
    Unsigned64BigEndian,

    /// Pointer width unsigned, little endian (`J<`)
    UnsignedPointerWidthLittleEndian,

    /// 16-bit signed, little endian (`s<`)
    Signed16LittleEndian,

    /// 32-bit signed, little endian (`l<`)
    Signed32LittleEndian,

    /// 64-bit signed, little endian (`q<`)
    Signed64LittleEndian,

    /// Pointer width signed, little endian (`j<`)
    SignedPointerWidthLittleEndian,

    /// `unsigned short`, native endian (`S_`, `S!`)
    UnsignedShortNativeEndian,

    /// `unsigned int`, native endian (`I`, `I_`, `I!`)
    UnsignedIntNativeEndian,

    /// `unsigned long`, native endian (`L_`, `L!`)
    UnsignedLongNativeEndian,

    /// `unsigned long long`, native endian (`Q_`, `Q!`)
    ///
    /// `ArgumentError` if the platform has no `long long` type.
    UnsignedLongLongNativeEndian,

    /// `signed short`, native endian (`s_`, `s!`)
    SignedShortNativeEndian,

    /// `signed int`, native endian (`i`, `i_`, `i!`)
    SignedIntNativeEndian,

    /// `signed long`, native endian (`l_`, `l!`)
    SignedLongNativeEndian,

    /// `signed long long`, native endian (`q_`, `q!`)
    ///
    /// `ArgumentError` if the platform has no `long long` type.
    SignedLongLongNativeEndian,

    /// `unsigned short`, big endian (`S!>`)
    ///
    /// Also known as `n`.
    UnsignedShortBigEndian,

    /// `unsigned int`, big endian (`I!>`)
    UnsignedIntBigEndian,

    /// `unsigned long`, big endian (`L!>`)
    ///
    /// Also known as `N`.
    UnsignedLongBigEndian,

    /// `unsigned long long`, big endian (`Q!>`)
    ///
    /// `ArgumentError` if the platform has no `long long` type.
    UnsignedLongLongBigEndian,

    /// `signed short`, big endian (`s!>`)
    SignedShortBigEndian,

    /// `signed int`, big endian (`i!>`)
    SignedIntBigEndian,

    /// `signed long`, big endian (`l!>`)
    SignedLongBigEndian,

    /// `signed long long`, big endian (`q!>`)
    ///
    /// `ArgumentError` if the platform has no `long long` type.
    SignedLongLongBigEndian,

    /// `unsigned short`, little endian (`S!<`)
    ///
    /// Also known as `v`.
    UnsignedShortLittleEndian,

    /// `unsigned int`, little endian (`I!<`)
    UnsignedIntLittleEndian,

    /// `unsigned long`, little endian (`L!<`)
    ///
    /// Also known as `V`.
    UnsignedLongLittleEndian,

    /// `unsigned long long`, little endian (`Q!<`)
    ///
    /// `ArgumentError` if the platform has no `long long` type.
    UnsignedLongLongLittleEndian,

    /// `signed short`, little endian (`s!<`)
    SignedShortLittleEndian,

    /// `signed int`, little endian (`i!<`)
    SignedIntLittleEndian,

    /// `signed long`, little endian (`l!<`)
    SignedLongLittleEndian,

    /// `signed long long`, little endian (`q!<`)
    ///
    /// `ArgumentError` if the platform has no `long long` type.
    SignedLongLongLittleEndian,

    /// 16-bit unsigned, network (big-endian) byte order (`n`)
    ///
    /// Also known as `S>`.
    Unsigned16BigEndian,

    /// 32-bit unsigned, network (big-endian) byte order (`N`)
    ///
    /// Also known as `L>`.
    Unsigned32BigEndian,

    /// 16-bit unsigned, VAX (little-endian) byte order (`v`)
    ///
    /// Also known as `S<`.
    Unsigned16LittleEndian,

    /// 32-bit unsigned, VAX (little-endian) byte order (`V`)
    ///
    /// Also known as `L<`.
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
            b'I' => Ok(Self::UnsignedIntNativeEndian),
            b'i' => Ok(Self::SignedIntNativeEndian),
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
/// Consists of various float directives.
///
/// The corresponding characters for each directive are as follows:
///
/// | Float Directive | Returns  | Meaning                                                           |
/// |-----------------|----------|-------------------------------------------------------------------|
/// | `D`, `d`        | Float    | double-precision, native format                                   |
/// | `F`, `f`        | Float    | single-precision, native format                                   |
/// | `E`             | Float    | double-precision, little-endian byte order                        |
/// | `e`             | Float    | single-precision, little-endian byte order                        |
/// | `G`             | Float    | double-precision, network (big-endian) byte order                 |
/// | `g`             | Float    | single-precision, network (big-endian) byte order                 |
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
/// Consists of various string directives.
///
/// The corresponding characters for each directive are as follows:
///
/// | String Directive | Returns | Meaning                                                          |
/// |------------------|---------|------------------------------------------------------------------|
/// | `A`              | String  | arbitrary binary string (remove trailing nulls and ASCII spaces) |
/// | `a`              | String  | arbitrary binary string                                          |
/// | `Z`              | String  | null-terminated string                                           |
/// | `B`              | String  | bit string (MSB first)                                           |
/// | `b`              | String  | bit string (LSB first)                                           |
/// | `H`              | String  | hex string (high nibble first)                                   |
/// | `h`              | String  | hex string (low nibble first)                                    |
/// | `u`              | String  | UU-encoded string                                                |
/// | `M`              | String  | quoted-printable, MIME encoding (see RFC2045)                    |
/// | `m`              | String  | base64 encoded string (RFC 2045) (default)                       |
/// | `m0`             | String  | base64 encoded string (RFC 4648) if followed by 0                |
/// | `P`              | String  | pointer to a structure (fixed-length string)                     |
/// | `p`              | String  | pointer to a null-terminated string                              |
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
/// Consists of various miscellaneous directives.
///
/// The corresponding characters for each directive are as follows:
///
/// | Directive | Returns | Meaning                                         |
/// |-----------|---------|-------------------------------------------------|
/// | `@`       | ---     | skip to the offset given by the length argument |
/// | `X`       | ---     | skip backward one byte                          |
/// | `x`       | ---     | skip forward one byte                           |
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

/// Represents various  directives used in a format string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnpackAmount {
    /// Specifies the number of times to repeat the preceding directive.
    ///
    /// An repeat count of `0`, consumes 0 bytes from the string being unpacked.
    Repeat(usize),

    /// Consume to end.
    ///
    /// An asterisk (`*`) will use up all remaining elements.
    ConsumeToEnd,

    /// Native size (underscore).
    ///
    /// The underscore (`_`) indicates that the underlying platform's native
    /// size should be used for the specified type. If not present, it uses a
    /// platform-independent consistent size.
    ///
    /// This unpack amount is only valid for the `sSiIlL` directives.
    PlatformNativeSizeUnderscore,

    /// Native size (bang).
    ///
    /// The exclamation mark (`!`) indicates that the underlying platform's
    /// native size should be used for the specified type. If not present, it
    /// uses a platform-independent consistent size.
    ///
    /// This unpack amount is only valid for the `sSiIlL` directives.
    PlatformNativeSizeBang,
}

impl Default for UnpackAmount {
    fn default() -> Self {
        Self::Repeat(1)
    }
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
            IntegerDirective::try_from(b'I'),
            Ok(IntegerDirective::UnsignedIntNativeEndian)
        );
        assert_eq!(
            IntegerDirective::try_from(b'i'),
            Ok(IntegerDirective::SignedIntNativeEndian)
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
            IntegerDirective::try_from(directive).unwrap();

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
            FloatDirective::try_from(directive).unwrap();

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
            StringDirective::try_from(directive).unwrap();

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
            MiscellaneousDirective::try_from(directive).unwrap();

            // Try parsing as IntegerDirective
            IntegerDirective::try_from(directive).unwrap_err();

            // Try parsing as FloatDirective
            FloatDirective::try_from(directive).unwrap_err();

            // Try parsing as StringDirective
            StringDirective::try_from(directive).unwrap_err();
        }
    }

    #[test]
    fn test_parsing_all_directives() {
        for byte in u8::MIN..=u8::MAX {
            match Directive::from(byte) {
                Directive::Integer(inner) if Ok(inner) == IntegerDirective::try_from(byte) => {}
                Directive::Integer(inner) => {
                    panic!("{byte} parsed to Directive::Integer({inner:?}) but failed to parse as integer directive");
                }
                Directive::Float(inner) if Ok(inner) == FloatDirective::try_from(byte) => {}
                Directive::Float(inner) => {
                    panic!("{byte} parsed to Directive::Float({inner:?}) but failed to parse as float directive");
                }
                Directive::String(inner) if Ok(inner) == StringDirective::try_from(byte) => {}
                Directive::String(inner) => {
                    panic!("{byte} parsed to Directive::String({inner:?}) but failed to parse as string directive");
                }
                Directive::Miscellaneous(inner) if Ok(inner) == MiscellaneousDirective::try_from(byte) => {}
                Directive::Miscellaneous(inner) => {
                    panic!("{byte} parsed to Directive::Miscellaneous({inner:?}) but failed to parse as miscellaneous directive");
                }
                Directive::Unknown(..) => {}
            }
        }
    }
}
