//! The Ruby Math module.
//!
//! The Math module contains module functions for basic trigonometric and
//! transcendental functions. See class [`Float`] for a list of constants that
//! define Ruby's floating point accuracy.
//!
//! You can use the `Math` module by accessing it in the interpreter. `Math` is
//! globally available in the root namespace.
//!
//! ```ruby
//! Math.hypot(3, 4)
//! ```
//!
//! This module implements the core math module with [`spinoso-math`] and
//! re-exports some of its internals.
//!
//! [`Float`]: https://ruby-doc.org/core-3.1.2/Float.html
//! [`spinoso-math`]: spinoso_math

use std::borrow::Cow;

use crate::extn::prelude::*;

pub mod mruby;
pub mod trampoline;

#[doc(inline)]
pub use spinoso_math::{DomainError, Math, E, PI};
use spinoso_math::{Error as MathError, NotImplementedError as MathNotImplementedError};

impl RubyException for DomainError {
    fn message(&self) -> Cow<'_, [u8]> {
        let message = DomainError::message(*self);
        Cow::Borrowed(message.as_bytes())
    }

    fn name(&self) -> Cow<'_, str> {
        "DomainError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.message()).ok()?;
        let value = interp.new_instance::<Self>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<DomainError> for Error {
    fn from(exception: DomainError) -> Self {
        let err: Box<dyn RubyException> = Box::new(exception);
        Self::from(err)
    }
}

impl From<MathNotImplementedError> for Error {
    fn from(err: MathNotImplementedError) -> Self {
        let exc = NotImplementedError::from(err.message());
        exc.into()
    }
}

impl From<MathError> for Error {
    fn from(err: MathError) -> Self {
        match err {
            MathError::Domain(err) => err.into(),
            MathError::NotImplemented(err) => err.into(),
        }
    }
}
