mod directive;
mod repetition;

use directive::{Directive, MiscDirective};
use repetition::Repetition;

pub struct RangeError {
    message: &'static str,
}

impl RangeError {
    pub fn with_message(message: &'static str) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}

/// Iterator that parses a format string into directives.
pub struct FormatStringIterator<'a> {
    bytes: &'a [u8],
    idx: usize,
    last_directive: Option<(Directive, Repetition)>,
}

impl<'a> FormatStringIterator<'a> {
    pub fn new(format_string: &'a [u8]) -> Self {
        FormatStringIterator {
            bytes: format_string,
            idx: 0,
            last_directive: None,
        }
    }
}

impl<'a> Iterator for FormatStringIterator<'a> {
    type Item = Result<Directive, RangeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((directive, repetition)) = self.last_directive.take() {
            match repetition.next() {
                None | Some(Repetition::Finished) => {}
                Some(next_repetition) => {
                    self.last_directive = Some((directive, next_repetition));
                }
            }
            return Some(Ok(directive));
        }

        let misc = None;
        loop {
            let s = match misc {
                Some((_, Repetition::ConsumeToEnd)) => {
                    self.idx = usize::MAX;
                    &[]
                }
                Some((MiscDirective::SkipToOffset, Repetition::Finished)) => {
                    self.idx = 0;
                    self.bytes
                }
                Some((MiscDirective::SkipToOffset, Repetition::Repeat(idx))) => {
                    self.idx = idx.get();
                    match self.bytes.get(self.idx..) {
                        Some(s) => s,
                        None => return Some(Err(RangeError::with_message("@ outside of string"))),
                    }
                }
                Some((MiscDirective::SkipBackward | MiscDirective::SkipForward, Repetition::Finished)) => self.bytes,
                Some((MiscDirective::SkipBackward, Repetition::Repeat(idx))) => {
                    self.idx = idx.get();
                    match self.bytes.get(self.idx..) {
                        Some(s) => s,
                        None => return Some(Err(RangeError::with_message("@ outside of string"))),
                    }
                }
                Some((MiscDirective::SkipForward, Repetition::Repeat(idx))) => {
                    self.idx = idx.get();
                    match self.bytes.get(self.idx..) {
                        Some(s) => s,
                        None => return Some(Err(RangeError::with_message("@ outside of string"))),
                    }
                }
                None => self.bytes.get(self.idx..)?,
            };

            let mut format = s;
            let directive = Directive::next_from_format_bytes(&mut format);
            let amount = match Repetition::next_from_format_bytes(&mut format) {
                Ok(amount) => amount,
                Err(err) => {
                    self.idx = usize::MAX;
                    return Some(Err(err));
                }
            };
        }
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
            Ok(IntegerDirective::Unsigned16NetworkOrder)
        );
        assert_eq!(
            IntegerDirective::try_from(b'N'),
            Ok(IntegerDirective::Unsigned32NetworkOrder)
        );
        assert_eq!(
            IntegerDirective::try_from(b'v'),
            Ok(IntegerDirective::Unsigned16VaxOrder)
        );
        assert_eq!(
            IntegerDirective::try_from(b'V'),
            Ok(IntegerDirective::Unsigned32VaxOrder)
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
