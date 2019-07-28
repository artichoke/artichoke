//! [`Regexp::new`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-new)
//! and
//! [`Regexp::compile`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-c-compile)

use std::mem;

use crate::convert::RustBackedValue;
use crate::extn::core::regexp::enc::{self, Encoding};
use crate::extn::core::regexp::opts::{self, Options};
use crate::extn::core::regexp::Regexp;
use crate::sys;
use crate::value::Value;
use crate::warn::MrbWarn;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    NoImplicitConversionToString,
    Syntax,
    Unicode,
}

#[derive(Debug)]
pub struct Args {
    pub pattern: Value,
    pub options: Option<Options>,
    pub encoding: Option<Encoding>,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o|o?o?\0";

    pub unsafe fn extract(interp: &Mrb) -> Result<Self, Error> {
        let mut pattern = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut opts = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut has_opts = <mem::MaybeUninit<sys::mrb_bool>>::uninit();
        let mut enc = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut has_enc = <mem::MaybeUninit<sys::mrb_bool>>::uninit();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            pattern.as_mut_ptr(),
            opts.as_mut_ptr(),
            has_opts.as_mut_ptr(),
            enc.as_mut_ptr(),
            has_enc.as_mut_ptr(),
        );
        let pattern = pattern.assume_init();
        let has_opts = has_opts.assume_init() != 0;
        let has_enc = has_enc.assume_init() != 0;
        let pattern = Value::new(&interp, pattern);
        let options = if has_opts {
            Some(opts::parse(&Value::new(interp, opts.assume_init())))
        } else {
            None
        };
        let encoding = if has_enc {
            let encoding = Value::new(interp, enc.assume_init());
            match enc::parse(&encoding) {
                Ok(encoding) => Some(encoding),
                Err(enc::Error::InvalidEncoding) => {
                    let warning = format!("encoding option is ignored -- {}", encoding.to_s());
                    interp.warn(warning.as_str()).map_err(|_| Error::Fatal)?;
                    None
                }
            }
        } else if has_opts {
            let encoding = Value::new(interp, opts.assume_init());
            match enc::parse(&encoding) {
                Ok(encoding) => Some(encoding),
                Err(enc::Error::InvalidEncoding) => {
                    let warning = format!("encoding option is ignored -- {}", encoding.to_s());
                    interp.warn(warning.as_str()).map_err(|_| Error::Fatal)?;
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            pattern,
            options,
            encoding,
        })
    }
}

pub fn method(interp: &Mrb, args: Args, slf: sys::mrb_value) -> Result<Value, Error> {
    let mut literal_options = args.options.unwrap_or_default();
    let literal_pattern =
        if let Ok(regexp) = unsafe { Regexp::try_from_ruby(interp, &args.pattern) } {
            interp
                .warn("flags ignored when initializing from Regexp")
                .map_err(|_| Error::Fatal)?;
            let borrow = regexp.borrow();
            literal_options = borrow.options;
            borrow.literal_pattern.clone()
        } else {
            let bytes = args
                .pattern
                .try_into::<Vec<u8>>()
                .map_err(|_| Error::NoImplicitConversionToString)?;
            String::from_utf8(bytes).map_err(|_| Error::Unicode)?
        };
    let (pattern, options) = opts::parse_pattern(literal_pattern.as_str(), literal_options);
    if let Some(data) = Regexp::new(
        literal_pattern,
        pattern,
        literal_options,
        options,
        args.encoding.unwrap_or_default(),
    ) {
        unsafe {
            data.try_into_ruby(interp, Some(slf))
                .map_err(|_| Error::Fatal)
        }
    } else {
        // Regexp is invalid.
        Err(Error::Syntax)
    }
}
