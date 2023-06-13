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
}
