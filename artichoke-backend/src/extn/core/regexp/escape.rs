//! [`Regexp::escape`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-escape)
//! and
//! [`Regexp::quote`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-quote)

use artichoke_core::value::Value as ValueLike;

use crate::convert::Convert;
use crate::extn::core::exception::{RubyException, TypeError};
use crate::extn::core::regexp::syntax;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy)]
pub struct Args<'a> {
    pub pattern: &'a str,
}

impl<'a> Args<'a> {
    pub fn extract(interp: &Artichoke, pattern: Value) -> Result<Self, Box<dyn RubyException>> {
        if let Ok(pattern) = pattern.clone().try_into::<&str>(interp) {
            Ok(Self { pattern })
        } else if let Ok(pattern) = pattern.funcall::<&str>(interp, "to_str", &[], None) {
            Ok(Self { pattern })
        } else {
            Err(Box::new(TypeError::new(
                interp,
                "No implicit conversion into String",
            )))
        }
    }
}

pub fn method(interp: &Artichoke, args: &Args) -> Result<Value, Box<dyn RubyException>> {
    Ok(interp.convert(syntax::escape(args.pattern)))
}
