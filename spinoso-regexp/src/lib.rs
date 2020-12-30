#[macro_use]
extern crate bitflags;

use core::num::NonZeroUsize;
use std::borrow::Cow;

pub mod debug;
mod encoding;
mod error;
mod options;
mod regexp;

#[doc(inline)]
pub use debug::Debug;
pub use encoding::{Encoding, InvalidEncodingError};
pub use error::{ArgumentError, Error, RegexpError, SyntaxError};
pub use options::{Options, RegexpOption};

bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const IGNORECASE      = 0b00000001;
        const EXTENDED        = 0b00000010;
        const MULTILINE       = 0b00000100;
        const ALL_REGEXP_OPTS = Self::IGNORECASE.bits | Self::EXTENDED.bits | Self::MULTILINE.bits;

        const FIXEDENCODING   = 0b00010000;
        const NOENCODING      = 0b00100000;

        const LITERAL         = 0b10000000;
    }
}

/// The string matched by the last successful match.
pub const LAST_MATCHED_STRING: &[u8] = b"$&";

/// The string to the left of the last successful match.
pub const STRING_LEFT_OF_MATCH: &[u8] = b"$`";

/// The string to the right of the last successful match.
pub const STRING_RIGHT_OF_MATCH: &[u8] = b"$'";

/// The highest group matched by the last successful match.
// TODO: implement this.
pub const HIGHEST_MATCH_GROUP: &[u8] = b"$+";

/// The information about the last match in the current scope.
pub const LAST_MATCH: &[u8] = b"$~";

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config {
    pattern: Vec<u8>,
    options: Options,
}

/// Global variable name for the nth capture group from a `Regexp` match.
#[inline]
#[must_use]
pub fn nth_match_group(group: NonZeroUsize) -> Cow<'static, [u8]> {
    match group.get() {
        1 => Cow::Borrowed(b"$1"),
        2 => Cow::Borrowed(b"$2"),
        3 => Cow::Borrowed(b"$3"),
        4 => Cow::Borrowed(b"$4"),
        5 => Cow::Borrowed(b"$5"),
        6 => Cow::Borrowed(b"$6"),
        7 => Cow::Borrowed(b"$7"),
        8 => Cow::Borrowed(b"$8"),
        9 => Cow::Borrowed(b"$9"),
        10 => Cow::Borrowed(b"$10"),
        11 => Cow::Borrowed(b"$11"),
        12 => Cow::Borrowed(b"$12"),
        13 => Cow::Borrowed(b"$13"),
        14 => Cow::Borrowed(b"$14"),
        15 => Cow::Borrowed(b"$15"),
        16 => Cow::Borrowed(b"$16"),
        17 => Cow::Borrowed(b"$17"),
        18 => Cow::Borrowed(b"$18"),
        19 => Cow::Borrowed(b"$19"),
        20 => Cow::Borrowed(b"$20"),
        num => {
            let mut buf = String::from("$");
            // Suppress fmt errors because this function is infallible.
            //
            // In practice `itoa::fmt` will never error because the `fmt::Write`
            // impl for `String` never panics.
            let _ = itoa::fmt(&mut buf, num);
            Cow::Owned(buf.into_bytes())
        }
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroUsize;
    use std::borrow::Cow;

    use super::nth_match_group;

    #[test]
    fn match_group_symbol() {
        for num in 1..=1024 {
            let num = NonZeroUsize::new(num).unwrap();
            let sym = nth_match_group(num);
            let num = format!("{}", num);
            assert!(sym.len() > 1);
            assert_eq!(sym[0..1], *b"$");
            assert_eq!(sym[1..], *num.as_bytes());
        }
    }

    #[test]
    fn some_globals_are_static_slices() {
        for num in 1..=20 {
            let num = NonZeroUsize::new(num).unwrap();
            let sym = nth_match_group(num);
            assert!(matches!(sym, Cow::Borrowed(_)));
        }
        for num in 21..=1024 {
            let num = NonZeroUsize::new(num).unwrap();
            let sym = nth_match_group(num);
            assert!(matches!(sym, Cow::Owned(_)));
        }
    }
}
