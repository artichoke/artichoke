use core::iter::FusedIterator;

mod directive;
mod repetition;

use directive::Directive;
use repetition::Repetition;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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
pub struct UnpackDirectiveIterator<'a> {
    format: &'a [u8],
    directive: Option<(Directive, Repetition)>,
}

impl<'a> UnpackDirectiveIterator<'a> {
    pub fn new(format: &'a [u8]) -> Self {
        Self {
            format,
            directive: None,
        }
    }
}

impl<'a> Iterator for UnpackDirectiveIterator<'a> {
    type Item = Result<Directive, RangeError>;

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(&self.directive);
        dbg!(std::str::from_utf8(self.format).unwrap());
        if let Some((directive, repetition)) = self.directive.take() {
            match repetition.next() {
                None | Some(Repetition::Finished) => {}
                Some(repetition) => {
                    self.directive = Some((directive, repetition));
                }
            }
            return Some(Ok(directive));
        }

        loop {
            dbg!(std::str::from_utf8(self.format).unwrap());
            let directive = Directive::next_from_format_bytes(&mut self.format)?;
            dbg!(std::str::from_utf8(self.format).unwrap());
            let repetition = match Repetition::next_from_format_bytes(&mut self.format) {
                Ok(repetition) => dbg!(repetition),
                Err(err) => {
                    self.format = &[];
                    return Some(Err(err));
                }
            };
            dbg!(std::str::from_utf8(self.format).unwrap());
            if let Directive::Unknown(b) = directive {
                // ```
                // [3.2.2] > "11111111111111111111111111111111".unpack('h-1')
                // <internal:pack>:20: warning: unknown unpack directive '-' in 'h-1'
                // ```
                //
                // The format string is treated as ASCII/binary encoding:
                //
                // ```
                // [3.2.2] > "11111111111111111111111111111111".unpack('h🐹1')
                // <internal:pack>:20: warning: unknown unpack directive '\xf0' in 'h🐹1'
                // <internal:pack>:20: warning: unknown unpack directive '\x9f' in 'h🐹1'
                // <internal:pack>:20: warning: unknown unpack directive '\x90' in 'h🐹1'
                // <internal:pack>:20: warning: unknown unpack directive '\xb9' in 'h🐹1'
                // ```
                todo!("emit warning");
                continue;
            }
            if matches!(repetition, Repetition::Finished) {
                continue;
            }
            if let Some(repetition) = repetition.next() {
                self.directive = Some((directive, repetition));
            }
            return Some(Ok(directive));
        }
    }
}

impl<'a> FusedIterator for UnpackDirectiveIterator<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directive::{Directive, FloatDirective, IntegerDirective, MiscDirective, StringDirective};

    #[test]
    fn test_unpack_directive_iterator_with_format_string_a4n3c() {
        let format_string = "a4n3c";
        let iterator = UnpackDirectiveIterator::new(format_string.as_bytes());

        let directives = iterator.filter_map(Result::ok).collect::<Vec<_>>();
        let expected_directives = vec![
            Directive::String(StringDirective::ArbitraryBinary),
            Directive::String(StringDirective::ArbitraryBinary),
            Directive::String(StringDirective::ArbitraryBinary),
            Directive::String(StringDirective::ArbitraryBinary),
            Directive::Integer(IntegerDirective::Unsigned16NetworkOrder),
            Directive::Integer(IntegerDirective::Unsigned16NetworkOrder),
            Directive::Integer(IntegerDirective::Unsigned16NetworkOrder),
            Directive::Integer(IntegerDirective::Signed8),
        ];

        assert_eq!(directives, expected_directives);
    }

    #[test]
    fn test_unpack_directive_iterator_with_format_string_star_yields_forever() {
        let format_string = "c*a4";
        let mut iter = UnpackDirectiveIterator::new(format_string.as_bytes());

        for _ in 0..=10240 {
            assert_eq!(iter.next().unwrap(), Ok(Directive::Integer(IntegerDirective::Signed8)));
        }
        assert!(iter.next().is_some());
    }

    #[test]
    fn test_unpack_directive_iterator_with_all_string_directives() {
        let format_string = "AaZBbHhuMmPp";
        let iterator = UnpackDirectiveIterator::new(format_string.as_bytes());

        let directives = iterator.filter_map(Result::ok).collect::<Vec<_>>();
        let expected_directives = vec![
            Directive::String(StringDirective::ArbitraryBinaryTrimmed),
            Directive::String(StringDirective::ArbitraryBinary),
            Directive::String(StringDirective::NullTerminated),
            Directive::String(StringDirective::BitStringMsbFirst),
            Directive::String(StringDirective::BitStringLsbFirst),
            Directive::String(StringDirective::HexStringHighNibbleFirst),
            Directive::String(StringDirective::HexStringLowNibbleFirst),
            Directive::String(StringDirective::UuEncoded),
            Directive::String(StringDirective::QuotedPrintable),
            Directive::String(StringDirective::Base64Encoded),
            Directive::String(StringDirective::StructurePointer),
            Directive::String(StringDirective::NullTerminatedStringPointer),
        ];

        assert_eq!(directives, expected_directives);
    }

    #[test]
    fn test_unpack_directive_iterator_with_all_string_directives_and_repetitions() {
        let format_string = "A4a2Z3B1b3H2h3u1M1m1P1p1";
        let iterator = UnpackDirectiveIterator::new(format_string.as_bytes());

        let directives = iterator.filter_map(Result::ok).collect::<Vec<_>>();
        let expected_directives = vec![
            Directive::String(StringDirective::ArbitraryBinaryTrimmed),
            Directive::String(StringDirective::ArbitraryBinaryTrimmed),
            Directive::String(StringDirective::ArbitraryBinaryTrimmed),
            Directive::String(StringDirective::ArbitraryBinaryTrimmed),
            Directive::String(StringDirective::ArbitraryBinary),
            Directive::String(StringDirective::ArbitraryBinary),
            Directive::String(StringDirective::NullTerminated),
            Directive::String(StringDirective::NullTerminated),
            Directive::String(StringDirective::NullTerminated),
            Directive::String(StringDirective::BitStringMsbFirst),
            Directive::String(StringDirective::BitStringLsbFirst),
            Directive::String(StringDirective::BitStringLsbFirst),
            Directive::String(StringDirective::BitStringLsbFirst),
            Directive::String(StringDirective::HexStringHighNibbleFirst),
            Directive::String(StringDirective::HexStringHighNibbleFirst),
            Directive::String(StringDirective::HexStringLowNibbleFirst),
            Directive::String(StringDirective::HexStringLowNibbleFirst),
            Directive::String(StringDirective::HexStringLowNibbleFirst),
            Directive::String(StringDirective::UuEncoded),
            Directive::String(StringDirective::QuotedPrintable),
            Directive::String(StringDirective::Base64Encoded),
            Directive::String(StringDirective::StructurePointer),
            Directive::String(StringDirective::NullTerminatedStringPointer),
        ];

        assert_eq!(directives, expected_directives);
    }

    #[test]
    fn test_unpack_directive_iterator_with_all_float_directives() {
        let format_string = "DdFfEeGg";
        let iterator = UnpackDirectiveIterator::new(format_string.as_bytes());

        let directives = iterator.filter_map(Result::ok).collect::<Vec<_>>();
        let expected_directives = vec![
            Directive::Float(FloatDirective::DoubleNativeEndian),
            Directive::Float(FloatDirective::DoubleNativeEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
            Directive::Float(FloatDirective::DoubleLittleEndian),
            Directive::Float(FloatDirective::SingleLittleEndian),
            Directive::Float(FloatDirective::DoubleBigEndian),
            Directive::Float(FloatDirective::SingleBigEndian),
        ];

        assert_eq!(directives, expected_directives);
    }

    #[test]
    fn test_unpack_directive_iterator_with_all_float_directives_and_repetitions() {
        let format_string = "D3d2E1e4G2g3F4f3";
        let iterator = UnpackDirectiveIterator::new(format_string.as_bytes());

        let directives = iterator.filter_map(Result::ok).collect::<Vec<_>>();
        let expected_directives = vec![
            Directive::Float(FloatDirective::DoubleNativeEndian),
            Directive::Float(FloatDirective::DoubleNativeEndian),
            Directive::Float(FloatDirective::DoubleNativeEndian),
            Directive::Float(FloatDirective::DoubleNativeEndian),
            Directive::Float(FloatDirective::DoubleNativeEndian),
            Directive::Float(FloatDirective::DoubleLittleEndian),
            Directive::Float(FloatDirective::SingleLittleEndian),
            Directive::Float(FloatDirective::SingleLittleEndian),
            Directive::Float(FloatDirective::SingleLittleEndian),
            Directive::Float(FloatDirective::SingleLittleEndian),
            Directive::Float(FloatDirective::DoubleBigEndian),
            Directive::Float(FloatDirective::DoubleBigEndian),
            Directive::Float(FloatDirective::SingleBigEndian),
            Directive::Float(FloatDirective::SingleBigEndian),
            Directive::Float(FloatDirective::SingleBigEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
            Directive::Float(FloatDirective::SingleNativeEndian),
        ];

        assert_eq!(directives, expected_directives);
    }

    #[test]
    fn test_unpack_directive_iterator_with_repetition_1024() {
        let format_string = "D1024";
        let iterator = UnpackDirectiveIterator::new(format_string.as_bytes());

        let directives: Vec<Directive> = iterator.filter_map(Result::ok).collect();
        let expected_directive = Directive::Float(FloatDirective::DoubleNativeEndian);

        assert_eq!(directives.len(), 1024);
        assert!(directives.into_iter().all(|directive| directive == expected_directive));
    }

    #[test]
    fn test_unpack_directive_iterator_with_large_count_directive() {
        // ```
        // [3.2.2] > "".unpack "D18446744073709551616Z2"
        // <internal:pack>:20:in `unpack': pack length too big (RangeError)
        // ```
        let format_string = "D18446744073709551616Z2";
        let mut iterator = UnpackDirectiveIterator::new(format_string.as_bytes());

        assert_eq!(
            iterator.next().unwrap(),
            Err(RangeError::with_message("pack length too big"))
        );

        assert!(iterator.next().is_none());
        assert!(iterator.next().is_none());
        assert!(iterator.next().is_none());
        assert!(iterator.next().is_none());
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_unpack_directive_iterator_with_asterisk_repetition_and_specifiers() {
        let format_string = "A2B*C3";
        let iterator = UnpackDirectiveIterator::new(format_string.as_bytes());

        let expected_directive = Directive::String(StringDirective::BitStringMsbFirst);
        let mut counter = 0;

        for directive in iterator {
            let directive = directive.unwrap();
            assert_eq!(directive, expected_directive);
            counter += 1;
            if counter >= 1000 {
                // Breaking the loop after 1000 iterations to prevent infinite loops
                // and ensure the iterator runs for a reasonable number of iterations.
                break;
            }
        }

        assert!(
            counter >= 1000,
            "Iterator did not run for a sufficient number of iterations (1000)"
        );
    }
}
