use std::convert::TryFrom;
use std::fmt;

use crate::exception::Exception;
use crate::exception_handler;
use crate::extn::core::exception::{Fatal, TypeError};
use crate::gc::MrbGarbageCollection;
use crate::sys::{self, protect};
use crate::types::{self, Ruby};
use crate::value::Value;
use crate::Artichoke;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NoBlockGiven(Ruby);

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

impl NoBlockGiven {
    pub fn new(ruby_type: Ruby) -> Self {
        Self(ruby_type)
    }

    pub fn ruby_type(self) -> Ruby {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct Block(sys::mrb_value);

impl From<sys::mrb_value> for Option<Block> {
    fn from(value: sys::mrb_value) -> Self {
        Block::new(value)
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

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "proc")
    }
}

impl Block {
    #[must_use]
    pub fn new(block: sys::mrb_value) -> Option<Self> {
        if let Ruby::Nil = types::ruby_from_mrb_value(block) {
            None
        } else {
            Some(Self(block))
        }
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> sys::mrb_value {
        self.0
    }

    pub fn yield_arg(&self, interp: &mut Artichoke, arg: &Value) -> Result<Value, Exception> {
        let mut arena = interp.create_arena_savepoint();

        let result = unsafe {
            arena
                .interp()
                .with_ffi_boundary(|mrb| protect::block_yield(mrb, self.inner(), arg.inner()))?
        };
        match result {
            Ok(value) => {
                let value = Value::new(&arena, value);
                if value.is_unreachable() {
                    // Unreachable values are internal to the mruby interpreter
                    // and interacting with them via the C API is unspecified
                    // and may result in a segfault.
                    //
                    // See: https://github.com/mruby/mruby/issues/4460
                    Err(Exception::from(Fatal::new(
                        arena.interp(),
                        "Unreachable Ruby value",
                    )))
                } else {
                    Ok(value)
                }
            }
            Err(exception) => {
                let exception = Value::new(&arena, exception);
                Err(exception_handler::last_error(&mut arena, exception)?)
            }
        }
    }
}
