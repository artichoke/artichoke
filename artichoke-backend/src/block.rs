use std::borrow::Cow;
use std::convert::TryFrom;
use std::error;
use std::fmt;

use crate::class_registry::ClassRegistry;
use crate::core::ConvertMut;
use crate::exception::{Exception, RubyException};
use crate::exception_handler;
use crate::extn::core::exception::{Fatal, TypeError};
use crate::gc::MrbGarbageCollection;
use crate::sys::{self, protect};
use crate::types::{self, Ruby};
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoBlockGiven(Ruby);

impl fmt::Display for NoBlockGiven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("no block given")
    }
}

impl error::Error for NoBlockGiven {}

impl RubyException for NoBlockGiven {
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(b"no block given")
    }

    fn name(&self) -> Cow<'_, str> {
        "TypeError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let value = interp
            .new_instance::<TypeError>(&[message])
            .ok()
            .flatten()?;
        Some(value.inner())
    }
}

impl From<NoBlockGiven> for Exception {
    fn from(exception: NoBlockGiven) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<NoBlockGiven>> for Exception {
    fn from(exception: Box<NoBlockGiven>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<NoBlockGiven> for Box<dyn RubyException> {
    fn from(exception: NoBlockGiven) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<NoBlockGiven>> for Box<dyn RubyException> {
    fn from(exception: Box<NoBlockGiven>) -> Box<dyn RubyException> {
        exception
    }
}

impl From<Value> for NoBlockGiven {
    fn from(value: Value) -> Self {
        Self(value.ruby_type())
    }
}

impl From<sys::mrb_value> for NoBlockGiven {
    fn from(value: sys::mrb_value) -> Self {
        Self(types::ruby_from_mrb_value(value))
    }
}

impl From<Ruby> for NoBlockGiven {
    fn from(ruby_type: Ruby) -> Self {
        Self(ruby_type)
    }
}

impl Default for NoBlockGiven {
    fn default() -> Self {
        Self::new()
    }
}

impl NoBlockGiven {
    /// Construce a new, empty no block given error.
    ///
    /// The inner Ruby type is `nil`.
    #[must_use]
    pub const fn new() -> Self {
        Self(Ruby::Nil)
    }

    /// Return the [`Ruby`] type of the object given instead of a block.
    #[must_use]
    pub const fn ruby_type(self) -> Ruby {
        self.0
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Block(sys::mrb_value);

impl From<sys::mrb_value> for Option<Block> {
    fn from(value: sys::mrb_value) -> Self {
        if let Ruby::Nil = types::ruby_from_mrb_value(value) {
            None
        } else {
            Some(Block(value))
        }
    }
}

impl TryFrom<sys::mrb_value> for Block {
    type Error = NoBlockGiven;

    fn try_from(value: sys::mrb_value) -> Result<Self, Self::Error> {
        if let Some(block) = value.into() {
            Ok(block)
        } else {
            Err(NoBlockGiven::from(value))
        }
    }
}

impl Block {
    /// Construct a `Block` from a Ruby value.
    #[must_use]
    pub fn new(block: sys::mrb_value) -> Option<Self> {
        if let Ruby::Nil = types::ruby_from_mrb_value(block) {
            None
        } else {
            Some(Self(block))
        }
    }

    /// Construct a `Block` from a Ruby value.
    ///
    /// # Safety
    ///
    /// The block must not be `nil`.
    #[must_use]
    pub unsafe fn new_unchecked(block: sys::mrb_value) -> Self {
        Self(block)
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> sys::mrb_value {
        self.0
    }

    pub fn yield_arg(&self, interp: &mut Artichoke, arg: &Value) -> Result<Value, Exception> {
        let mut arena = interp.create_arena_savepoint()?;

        let result = unsafe {
            arena
                .interp()
                .with_ffi_boundary(|mrb| protect::block_yield(mrb, self.inner(), arg.inner()))?
        };
        match result {
            Ok(value) => {
                let value = Value::from(value);
                if value.is_unreachable() {
                    // Unreachable values are internal to the mruby interpreter
                    // and interacting with them via the C API is unspecified
                    // and may result in a segfault.
                    //
                    // See: https://github.com/mruby/mruby/issues/4460
                    Err(Fatal::from("Unreachable Ruby value").into())
                } else {
                    Ok(value)
                }
            }
            Err(exception) => {
                let exception = Value::from(exception);
                Err(exception_handler::last_error(&mut arena, exception)?)
            }
        }
    }
}
