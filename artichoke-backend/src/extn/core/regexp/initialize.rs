//! [`Regexp::new`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-new)
//! and
//! [`Regexp::compile`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-compile)

use std::str;

use crate::convert::RustBackedValue;
use crate::extn::core::exception::{Fatal, RubyException, RuntimeError, SyntaxError, TypeError};
use crate::extn::core::regexp::enc::{self, Encoding};
use crate::extn::core::regexp::opts::{self, Options};
use crate::extn::core::regexp::Regexp;
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::warn::Warn;
use crate::Artichoke;

#[derive(Clone, Copy)]
pub struct Args {
    pub pattern: Value,
    pub options: Option<Options>,
    pub encoding: Option<Encoding>,
}

impl Args {
    pub fn extract(
        interp: &Artichoke,
        pattern: Value,
        options: Option<Value>,
        encoding: Option<Value>,
    ) -> Result<Self, Box<dyn RubyException>> {
        let (options, encoding) = if let Some(encoding) = encoding {
            let encoding = match enc::parse(interp, &encoding) {
                Ok(encoding) => Some(encoding),
                Err(enc::Error::InvalidEncoding) => {
                    let warning =
                        format!("encoding option is ignored -- {}", encoding.to_s(interp));
                    interp
                        .warn(warning.as_str())
                        .map_err(|_| Fatal::new(interp, "Warn for ignored encoding failed"))?;
                    None
                }
            };
            let options = options.as_ref().map(|opt| opts::parse(interp, opt));
            (options, encoding)
        } else if let Some(options) = options {
            let encoding = match enc::parse(interp, &options) {
                Ok(encoding) => Some(encoding),
                Err(enc::Error::InvalidEncoding) => {
                    let warning = format!("encoding option is ignored -- {}", options.to_s(interp));
                    interp
                        .warn(warning.as_str())
                        .map_err(|_| Fatal::new(interp, "Warn for ignored encoding failed"))?;
                    None
                }
            };
            let options = opts::parse(interp, &options);
            (Some(options), encoding)
        } else {
            (None, None)
        };

        Ok(Self {
            pattern,
            options,
            encoding,
        })
    }
}

pub fn method(
    interp: &Artichoke,
    args: Args,
    into: Option<sys::mrb_value>,
) -> Result<Value, Box<dyn RubyException>> {
    let mut literal_options = args.options.unwrap_or_default();
    let literal_pattern =
        if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &args.pattern) } {
            if args.options.is_some() || args.encoding.is_some() {
                interp
                    .warn("flags ignored when initializing from Regexp")
                    .map_err(|_| Fatal::new(interp, "Warn for ignored encoding failed"))?;
            }
            let borrow = regexp.borrow();
            literal_options = borrow.options;
            borrow.literal_pattern.to_owned()
        } else if let Ok(bytes) = args.pattern.clone().try_into::<&[u8]>(interp) {
            str::from_utf8(bytes)
                .map_err(|_| RuntimeError::new(interp, "Pattern is invalid UTF-8"))?
                // Defer allocating until the parse succeds
                .to_owned()
        } else if let Ok(bytes) = args.pattern.funcall::<&[u8]>(interp, "to_str", &[], None) {
            str::from_utf8(bytes)
                .map_err(|_| RuntimeError::new(interp, "Pattern is invalid UTF-8"))?
                // Defer allocating until the parse succeds
                .to_owned()
        } else {
            return Err(Box::new(TypeError::new(
                interp,
                "no implicit conversion into String",
            )));
        };
    let (pattern, options) = opts::parse_pattern(literal_pattern.as_str(), literal_options);
    if let Some(data) = Regexp::new(
        literal_pattern,
        pattern,
        literal_options,
        options,
        args.encoding.unwrap_or_default(),
    ) {
        let regexp = unsafe { data.try_into_ruby(interp, into) }.map_err(|_| {
            Fatal::new(
                interp,
                "Failed to initialize Regexp Ruby Value with Rust Regexp",
            )
        })?;
        Ok(regexp)
    } else {
        // Regexp is invalid.
        Err(Box::new(SyntaxError::new(
            interp,
            "Failed to parse Regexp pattern",
        )))
    }
}
