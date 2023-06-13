use core::fmt;

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
pub enum Directive {
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
    Unsigned64LittleEndian,

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
    Unsigned16NetworkOrder,

    /// 32-bit unsigned, network (big-endian) byte order (`N`)
    ///
    /// Also known as `L>`.
    Unsigned32NetworkOrder,

    /// 16-bit unsigned, VAX (little-endian) byte order (`v`)
    ///
    /// Also known as `S<`.
    Unsigned16VaxOrder,

    /// 32-bit unsigned, VAX (little-endian) byte order (`V`)
    ///
    /// Also known as `L<`.
    Unsigned32VaxOrder,

    /// UTF-8 character (`U`)
    Utf8Character,

    /// BER-compressed integer (`w`)
    BerCompressedInteger,
}

impl TryFrom<u8> for Directive {
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
            b'n' => Ok(Self::Unsigned16NetworkOrder),
            b'N' => Ok(Self::Unsigned32NetworkOrder),
            b'v' => Ok(Self::Unsigned16VaxOrder),
            b'V' => Ok(Self::Unsigned32VaxOrder),
            b'U' => Ok(Self::Utf8Character),
            b'w' => Ok(Self::BerCompressedInteger),
            _ => Err(()),
        }
    }
}

impl Directive {
    pub fn next_from_format_bytes(bytes: &mut &[u8]) -> Option<Self> {
        let (&first, tail) = bytes.split_first()?;

        let mut directive = Directive::try_from(first).ok()?;

        *bytes = tail;

        directive.update_from_modifiers(bytes);
        Some(directive)
    }

    pub fn update_from_modifiers(&mut self, format: &mut &[u8]) {
        let mut chomp = 1_usize;
        match (*self, *format) {
            (_, []) => return,
            (
                Self::Unsigned16NativeEndian
                | Self::UnsignedIntNativeEndian
                | Self::Unsigned32NativeEndian
                | Self::Unsigned64NativeEndian
                | Self::UnsignedPointerWidthNativeEndian,
                [b'!', b'>', ..],
            ) => {
                self.modify_platform_specific();
                self.modify_big_endian();
                chomp = 2;
            }
            (
                Self::Unsigned16NativeEndian
                | Self::UnsignedIntNativeEndian
                | Self::Unsigned32NativeEndian
                | Self::Unsigned64NativeEndian
                | Self::UnsignedPointerWidthNativeEndian,
                [b'!', b'<', ..],
            ) => {
                self.modify_platform_specific();
                self.modify_little_endian();
                chomp = 2;
            }
            (
                Self::Unsigned16NativeEndian
                | Self::UnsignedIntNativeEndian
                | Self::Unsigned32NativeEndian
                | Self::Unsigned64NativeEndian
                | Self::Signed16NativeEndian
                | Self::SignedIntNativeEndian
                | Self::Signed32NativeEndian
                | Self::Signed64NativeEndian,
                [b'_' | b'!', ..],
            ) => {
                self.modify_platform_specific();
            }
            (Self::UnsignedPointerWidthNativeEndian | Self::SignedPointerWidthNativeEndian, [b'!', ..]) => {
                self.modify_platform_specific();
            }
            (
                Self::Unsigned16NativeEndian
                | Self::UnsignedIntNativeEndian
                | Self::Unsigned32NativeEndian
                | Self::Unsigned64NativeEndian
                | Self::UnsignedPointerWidthNativeEndian,
                [b'>', ..],
            ) => {
                self.modify_big_endian();
            }
            (
                Self::Unsigned16NativeEndian
                | Self::UnsignedIntNativeEndian
                | Self::Unsigned32NativeEndian
                | Self::Unsigned64NativeEndian
                | Self::UnsignedPointerWidthNativeEndian,
                [b'<', ..],
            ) => {
                self.modify_little_endian();
            }
            _ => {
                chomp = 0;
            }
        }
        *format = &format[chomp..];
    }

    fn modify_little_endian(&mut self) {
        match self {
            Self::Unsigned16NativeEndian => *self = Self::Unsigned16LittleEndian,
            Self::Unsigned32NativeEndian => *self = Self::Unsigned32LittleEndian,
            Self::Unsigned64NativeEndian => *self = Self::Unsigned64LittleEndian,
            Self::Signed16NativeEndian => *self = Self::Signed16LittleEndian,
            Self::Signed32NativeEndian => *self = Self::Signed32LittleEndian,
            Self::Signed64NativeEndian => *self = Self::Signed64LittleEndian,
            Self::UnsignedShortNativeEndian => *self = Self::UnsignedShortLittleEndian,
            Self::UnsignedIntNativeEndian => *self = Self::UnsignedIntLittleEndian,
            Self::UnsignedLongNativeEndian => *self = Self::UnsignedLongLittleEndian,
            Self::UnsignedLongLongNativeEndian => *self = Self::UnsignedLongLongLittleEndian,
            Self::SignedShortNativeEndian => *self = Self::SignedShortLittleEndian,
            Self::SignedIntNativeEndian => *self = Self::SignedIntLittleEndian,
            Self::SignedLongNativeEndian => *self = Self::SignedLongLittleEndian,
            Self::SignedLongLongNativeEndian => *self = Self::SignedLongLongLittleEndian,
            _ => {} // No modification needed for other variants
        }
    }

    fn modify_big_endian(&mut self) {
        match self {
            Self::Unsigned16NativeEndian => *self = Self::Unsigned16BigEndian,
            Self::Unsigned32NativeEndian => *self = Self::Unsigned32BigEndian,
            Self::Unsigned64NativeEndian => *self = Self::Unsigned64BigEndian,
            Self::Signed16NativeEndian => *self = Self::Signed16BigEndian,
            Self::Signed32NativeEndian => *self = Self::Signed32BigEndian,
            Self::Signed64NativeEndian => *self = Self::Signed64BigEndian,
            Self::UnsignedShortNativeEndian => *self = Self::UnsignedShortBigEndian,
            Self::UnsignedIntNativeEndian => *self = Self::UnsignedIntBigEndian,
            Self::UnsignedLongNativeEndian => *self = Self::UnsignedLongBigEndian,
            Self::UnsignedLongLongNativeEndian => *self = Self::UnsignedLongLongBigEndian,
            Self::SignedShortNativeEndian => *self = Self::SignedShortBigEndian,
            Self::SignedIntNativeEndian => *self = Self::SignedIntBigEndian,
            Self::SignedLongNativeEndian => *self = Self::SignedLongBigEndian,
            Self::SignedLongLongNativeEndian => *self = Self::SignedLongLongBigEndian,
            _ => {} // No modification needed for other variants
        }
    }

    fn modify_platform_specific(&mut self) {
        match self {
            Self::Unsigned16NativeEndian => *self = Self::UnsignedShortNativeEndian,
            Self::Unsigned32NativeEndian => *self = Self::UnsignedLongNativeEndian,
            Self::Unsigned64NativeEndian => *self = Self::UnsignedLongLongNativeEndian,
            Self::Signed16NativeEndian => *self = Self::SignedShortNativeEndian,
            Self::Signed32NativeEndian => *self = Self::SignedLongNativeEndian,
            Self::Signed64NativeEndian => *self = Self::SignedLongLongNativeEndian,
            Self::Unsigned16BigEndian => *self = Self::UnsignedShortBigEndian,
            Self::Unsigned32BigEndian => *self = Self::UnsignedLongBigEndian,
            Self::Unsigned64BigEndian => *self = Self::UnsignedLongLongBigEndian,
            Self::Signed16BigEndian => *self = Self::SignedShortBigEndian,
            Self::Signed32BigEndian => *self = Self::SignedLongBigEndian,
            Self::Signed64BigEndian => *self = Self::SignedLongLongBigEndian,
            Self::Unsigned16LittleEndian => *self = Self::UnsignedShortLittleEndian,
            Self::Unsigned32LittleEndian => *self = Self::UnsignedLongLittleEndian,
            Self::Unsigned64LittleEndian => *self = Self::UnsignedLongLongLittleEndian,
            Self::Signed16LittleEndian => *self = Self::SignedShortLittleEndian,
            Self::Signed32LittleEndian => *self = Self::SignedLongLittleEndian,
            Self::Signed64LittleEndian => *self = Self::SignedLongLongLittleEndian,
            _ => {} // No modification needed for other variants
        }
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let directive = match self {
            Self::Unsigned8 => "C",
            Self::Unsigned16NativeEndian => "S",
            Self::Unsigned32NativeEndian => "L",
            Self::Unsigned64NativeEndian => "Q",
            Self::UnsignedPointerWidthNativeEndian => "J",
            Self::Signed8 => "c",
            Self::Signed16NativeEndian => "s",
            Self::Signed32NativeEndian => "l",
            Self::Signed64NativeEndian => "q",
            Self::SignedPointerWidthNativeEndian => "j",
            Self::Unsigned16BigEndian => "S>",
            Self::Unsigned32BigEndian => "L>",
            Self::Unsigned64BigEndian => "Q>",
            Self::UnsignedPointerWidthBigEndian => "J>",
            Self::Signed16BigEndian => "s>",
            Self::Signed32BigEndian => "l>",
            Self::Signed64BigEndian => "q>",
            Self::SignedPointerWidthBigEndian => "j>",
            Self::Unsigned16LittleEndian => "S<",
            Self::Unsigned32LittleEndian => "L<",
            Self::Unsigned64LittleEndian => "Q<",
            Self::UnsignedPointerWidthLittleEndian => "J<",
            Self::Signed16LittleEndian => "s<",
            Self::Signed32LittleEndian => "l<",
            Self::Signed64LittleEndian => "q<",
            Self::SignedPointerWidthLittleEndian => "j<",
            Self::UnsignedShortNativeEndian => "S_",
            Self::UnsignedIntNativeEndian => "I",
            Self::UnsignedLongNativeEndian => "L_",
            Self::UnsignedLongLongNativeEndian => "Q_",
            Self::SignedShortNativeEndian => "s_",
            Self::SignedIntNativeEndian => "i",
            Self::SignedLongNativeEndian => "l_",
            Self::SignedLongLongNativeEndian => "q_",
            Self::UnsignedShortBigEndian => "S!>",
            Self::UnsignedIntBigEndian => "I!>",
            Self::UnsignedLongBigEndian => "L!>",
            Self::UnsignedLongLongBigEndian => "Q!>",
            Self::SignedShortBigEndian => "s!>",
            Self::SignedIntBigEndian => "i!>",
            Self::SignedLongBigEndian => "l!>",
            Self::SignedLongLongBigEndian => "q!>",
            Self::UnsignedShortLittleEndian => "S!<",
            Self::UnsignedIntLittleEndian => "I!<",
            Self::UnsignedLongLittleEndian => "L!<",
            Self::UnsignedLongLongLittleEndian => "Q!<",
            Self::SignedShortLittleEndian => "s!<",
            Self::SignedIntLittleEndian => "i!<",
            Self::SignedLongLittleEndian => "l!<",
            Self::SignedLongLongLittleEndian => "q!<",
            Self::Unsigned16NetworkOrder => "n",
            Self::Unsigned32NetworkOrder => "N",
            Self::Unsigned16VaxOrder => "v",
            Self::Unsigned32VaxOrder => "V",
            Self::Utf8Character => "U",
            Self::BerCompressedInteger => "w",
        };
        f.write_str(directive)
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write as _;

    use super::*;

    #[test]
    fn test_directive_try_from_valid() {
        // Positive tests for valid directives
        assert_eq!(Directive::try_from(b'C'), Ok(Directive::Unsigned8));
        assert_eq!(Directive::try_from(b'S'), Ok(Directive::Unsigned16NativeEndian));
        assert_eq!(Directive::try_from(b'L'), Ok(Directive::Unsigned32NativeEndian));
        assert_eq!(Directive::try_from(b'Q'), Ok(Directive::Unsigned64NativeEndian));
        assert_eq!(
            Directive::try_from(b'J'),
            Ok(Directive::UnsignedPointerWidthNativeEndian)
        );
        assert_eq!(Directive::try_from(b'c'), Ok(Directive::Signed8));
        assert_eq!(Directive::try_from(b's'), Ok(Directive::Signed16NativeEndian));
        assert_eq!(Directive::try_from(b'l'), Ok(Directive::Signed32NativeEndian));
        assert_eq!(Directive::try_from(b'q'), Ok(Directive::Signed64NativeEndian));
        assert_eq!(Directive::try_from(b'j'), Ok(Directive::SignedPointerWidthNativeEndian));
        assert_eq!(Directive::try_from(b'I'), Ok(Directive::UnsignedIntNativeEndian));
        assert_eq!(Directive::try_from(b'i'), Ok(Directive::SignedIntNativeEndian));
        assert_eq!(Directive::try_from(b'n'), Ok(Directive::Unsigned16NetworkOrder));
        assert_eq!(Directive::try_from(b'N'), Ok(Directive::Unsigned32NetworkOrder));
        assert_eq!(Directive::try_from(b'v'), Ok(Directive::Unsigned16VaxOrder));
        assert_eq!(Directive::try_from(b'V'), Ok(Directive::Unsigned32VaxOrder));
        assert_eq!(Directive::try_from(b'U'), Ok(Directive::Utf8Character));
        assert_eq!(Directive::try_from(b'w'), Ok(Directive::BerCompressedInteger));
    }

    #[test]
    fn test_directive_try_from_invalid() {
        // Negative tests for non-directive characters
        assert_eq!(Directive::try_from(b'X'), Err(()));
        assert_eq!(Directive::try_from(b'Y'), Err(()));
        assert_eq!(Directive::try_from(b'Z'), Err(()));
        assert_eq!(Directive::try_from(b'x'), Err(()));
        assert_eq!(Directive::try_from(b'y'), Err(()));
        assert_eq!(Directive::try_from(b'z'), Err(()));

        // Negative tests for ASCII control characters
        assert_eq!(Directive::try_from(0x00), Err(())); // NUL
        assert_eq!(Directive::try_from(0x07), Err(())); // BEL
        assert_eq!(Directive::try_from(0x1B), Err(())); // ESC
        assert_eq!(Directive::try_from(0x7F), Err(())); // DEL

        // Negative tests for bytes outside the ASCII range
        assert_eq!(Directive::try_from(0x80), Err(()));
        assert_eq!(Directive::try_from(0xA5), Err(()));
        assert_eq!(Directive::try_from(0xFF), Err(()));
    }

    #[test]
    fn test_integer_directive_display() {
        let test_cases = [
            Directive::Unsigned8,
            Directive::Unsigned16NativeEndian,
            Directive::Unsigned32NativeEndian,
            Directive::Unsigned64NativeEndian,
            Directive::UnsignedPointerWidthNativeEndian,
            Directive::Signed8,
            Directive::Signed16NativeEndian,
            Directive::Signed32NativeEndian,
            Directive::Signed64NativeEndian,
            Directive::SignedPointerWidthNativeEndian,
            Directive::Unsigned16BigEndian,
            Directive::Unsigned32BigEndian,
            Directive::Unsigned64BigEndian,
            Directive::UnsignedPointerWidthBigEndian,
            Directive::Signed16BigEndian,
            Directive::Signed32BigEndian,
            Directive::Signed64BigEndian,
            Directive::SignedPointerWidthBigEndian,
            Directive::Unsigned16LittleEndian,
            Directive::Unsigned32LittleEndian,
            Directive::Unsigned64LittleEndian,
            Directive::UnsignedPointerWidthLittleEndian,
            Directive::Signed16LittleEndian,
            Directive::Signed32LittleEndian,
            Directive::Signed64LittleEndian,
            Directive::SignedPointerWidthLittleEndian,
            Directive::UnsignedShortNativeEndian,
            Directive::UnsignedIntNativeEndian,
            Directive::UnsignedLongNativeEndian,
            Directive::UnsignedLongLongNativeEndian,
            Directive::SignedShortNativeEndian,
            Directive::SignedIntNativeEndian,
            Directive::SignedLongNativeEndian,
            Directive::SignedLongLongNativeEndian,
            Directive::UnsignedShortBigEndian,
            Directive::UnsignedIntBigEndian,
            Directive::UnsignedLongBigEndian,
            Directive::UnsignedLongLongBigEndian,
            Directive::SignedShortBigEndian,
            Directive::SignedIntBigEndian,
            Directive::SignedLongBigEndian,
            Directive::SignedLongLongBigEndian,
            Directive::UnsignedShortLittleEndian,
            Directive::UnsignedIntLittleEndian,
            Directive::UnsignedLongLittleEndian,
            Directive::UnsignedLongLongLittleEndian,
            Directive::SignedShortLittleEndian,
            Directive::SignedIntLittleEndian,
            Directive::SignedLongLittleEndian,
            Directive::SignedLongLongLittleEndian,
            Directive::Unsigned16NetworkOrder,
            Directive::Unsigned32NetworkOrder,
            Directive::Unsigned16VaxOrder,
            Directive::Unsigned32VaxOrder,
            Directive::Utf8Character,
            Directive::BerCompressedInteger,
        ];

        for directive in test_cases {
            let mut buf = String::new();
            write!(&mut buf, "{directive}").unwrap();
            assert!(
                !buf.is_empty(),
                "Formatted string is empty for directive: {directive:?}",
            );
        }
    }

    #[test]
    fn test_parse_integer_directive_with_repetition() {
        let mut format: &[u8] = b"n3c*";
        assert_eq!(
            Directive::next_from_format_bytes(&mut format).unwrap(),
            Directive::Unsigned16NetworkOrder,
        );
        assert_eq!(format, &b"3c*"[..]);
    }
}
