//! Encoding represents a character encoding usable in Ruby
//!
//! This module implements the [`Encoding`] class from Ruby Core.
//!
//! You can use this class in your application by accessing it directly. As a
//! Core class, it is globally available:
//!
//! ```ruby
//! Encoding.list
//! ```
//!
//! [`Encoding`]: https://ruby-doc.org/3.1.2/Encoding.html

mod backend;
pub(in crate::extn) mod mruby;
pub(super) mod trampoline;

pub const RUBY_TYPE: &str = "Encoding";
pub use backend::spinoso::Encoding;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Encoding";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("encoding_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
