mod float;
mod integer;
mod misc;
mod string;

pub use float::Directive as FloatDirective;
pub use integer::Directive as IntegerDirective;
pub use misc::Directive as MiscDirective;
pub use string::Directive as StringDirective;

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
    Misc(MiscDirective),

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

impl Directive {
    pub fn next_from_format_bytes(format: &mut &[u8]) -> Option<Self> {
        let (&first, tail) = format.split_first()?;

        let mut directive = Directive::from(first);

        *format = tail;

        if let Directive::Integer(ref mut directive) = directive {
            directive.update_from_modifiers(format);
        }
        Some(directive)
    }
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
        if let Ok(directive) = MiscDirective::try_from(value) {
            return Self::Misc(directive);
        }
        Self::Unknown(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert!(FloatDirective::try_from(directive).is_err());

            // Try parsing as StringDirective
            assert!(StringDirective::try_from(directive).is_err());

            // Try parsing as MiscDirective
            assert!(MiscDirective::try_from(directive).is_err());
        }
    }

    #[test]
    fn test_float_directives_cannot_parse_as_other_types() {
        let float_directives = [b'F', b'D', b'E', b'G', b'f', b'd', b'e', b'g'];

        for directive in float_directives {
            FloatDirective::try_from(directive).unwrap();

            // Try parsing as IntegerDirective
            assert!(IntegerDirective::try_from(directive).is_err());

            // Try parsing as StringDirective
            assert!(StringDirective::try_from(directive).is_err());

            // Try parsing as MiscDirective
            assert!(MiscDirective::try_from(directive).is_err());
        }
    }

    #[test]
    fn test_string_directives_cannot_parse_as_other_types() {
        let string_directives = [b'A', b'a', b'Z', b'B', b'b', b'H', b'h', b'u', b'M', b'm', b'P', b'p'];

        for directive in string_directives {
            StringDirective::try_from(directive).unwrap();

            // Try parsing as IntegerDirective
            assert!(IntegerDirective::try_from(directive).is_err());

            // Try parsing as FloatDirective
            assert!(FloatDirective::try_from(directive).is_err());

            // Try parsing as MiscDirective
            assert!(MiscDirective::try_from(directive).is_err());
        }
    }

    #[test]
    fn test_misc_directives_cannot_parse_as_other_types() {
        let misc_directives = [b'@', b'X', b'x'];

        for directive in misc_directives {
            MiscDirective::try_from(directive).unwrap();

            // Try parsing as IntegerDirective
            assert!(IntegerDirective::try_from(directive).is_err());

            // Try parsing as FloatDirective
            assert!(FloatDirective::try_from(directive).is_err());

            // Try parsing as StringDirective
            assert!(StringDirective::try_from(directive).is_err());
        }
    }

    #[test]
    fn test_try_from_integer_directive() {
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
            let result = Directive::from(directive);
            assert_eq!(result, Directive::Integer(directive.try_into().unwrap()));
        }
    }

    #[test]
    fn test_try_from_float_directive() {
        let float_directives = [b'D', b'd', b'F', b'f', b'E', b'e', b'G', b'g'];

        for directive in float_directives {
            let result = Directive::from(directive);
            assert_eq!(result, Directive::Float(directive.try_into().unwrap()));
        }
    }

    #[test]
    fn test_try_from_string_directive() {
        let string_directives = [
            b'A', b'a', b'Z', b'B', b'b', b'H', b'h', b'u', b'M', b'm', b'm', b'P', b'p',
        ];

        for directive in string_directives {
            let result = Directive::from(directive);
            assert_eq!(result, Directive::String(directive.try_into().unwrap()));
        }
    }

    #[test]
    fn test_try_from_misc_directive() {
        let misc_directives = [b'@', b'X', b'x'];

        for directive in misc_directives {
            let result = Directive::from(directive);
            assert_eq!(result, Directive::Misc(directive.try_into().unwrap()));
        }
    }

    #[test]
    fn test_try_from_unknown_directive() {
        for directive in 0..=u8::MAX {
            let result = Directive::from(directive);

            if !b"CSLQJcslqjIinNvVUwDdFfEeGgAaZBbHhuMmPp@Xx".contains(&directive) {
                assert_eq!(result, Directive::Unknown(directive));
            } else {
                assert!(!matches!(result, Directive::Unknown(_)));
            }
        }
    }
}
