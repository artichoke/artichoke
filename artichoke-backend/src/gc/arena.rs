//! Garbage collection arenas for native code.

use std::borrow::Cow;
use std::error;
use std::fmt;
use std::ops::{Deref, DerefMut};

use spinoso_exception::Fatal;

use crate::core::{ClassRegistry, TryConvertMut};
use crate::error::{Error, RubyException};
use crate::sys;
use crate::Artichoke;

/// Failed to create a new GC arena savepoint.
///
/// This error is returned by [`ArenaIndex::new`].
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArenaSavepointError {
    _private: (),
}

impl ArenaSavepointError {
    /// Constructs a new, default `ArenaSavepointError`.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl fmt::Display for ArenaSavepointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Failed to create internal garbage collection savepoint")
    }
}

impl error::Error for ArenaSavepointError {}

impl RubyException for ArenaSavepointError {
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(b"Failed to create internal garbage collection savepoint")
    }

    fn name(&self) -> Cow<'_, str> {
        "fatal".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.message()).ok()?;
        let value = interp.new_instance::<Fatal>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<ArenaSavepointError> for Error {
    fn from(exception: ArenaSavepointError) -> Self {
        let err: Box<dyn RubyException> = Box::new(exception);
        Self::from(err)
    }
}

/// Interpreter guard that acts as a GC arena savepoint.
///
/// Arena savepoints ensure mruby objects are reaped even when allocated with
/// the C API.
///
/// mruby manages objects created via the C API in a memory construct called
/// the [arena]. The arena is a stack and objects stored there are permanently
/// alive to avoid having to track lifetimes externally to the interpreter.
///
/// An [`ArenaIndex`] is an index to some position of the stack. When restoring
/// an `ArenaIndex`, the stack pointer is moved. All objects beyond the pointer
/// are no longer live and are eligible to be collected at the next GC.
///
/// `ArenaIndex` implements [`Drop`], so letting it go out of scope is
/// sufficient to ensure objects get collected eventually.
///
/// [arena]: https://github.com/mruby/mruby/blob/master/doc/guides/gc-arena-howto.md
#[derive(Debug)]
pub struct ArenaIndex<'a> {
    index: i32,
    interp: &'a mut Artichoke,
}

impl<'a> ArenaIndex<'a> {
    /// Create a new arena savepoint.
    pub fn new(interp: &'a mut Artichoke) -> Result<Self, ArenaSavepointError> {
        unsafe {
            interp
                .with_ffi_boundary(|mrb| sys::mrb_sys_gc_arena_save(mrb))
                .map(move |index| Self { index, interp })
                .map_err(|_| ArenaSavepointError::new())
        }
    }

    /// Restore the arena stack pointer to its prior index.
    pub fn restore(self) {
        drop(self);
    }

    /// Access the inner guarded interpreter.
    ///
    /// The interpreter is also accessible via [`Deref`], [`DerefMut`],
    /// [`AsRef`], and [`AsMut`].
    #[inline]
    pub fn interp(&mut self) -> &mut Artichoke {
        self.interp
    }
}

impl<'a> Deref for ArenaIndex<'a> {
    type Target = Artichoke;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.interp
    }
}

impl<'a> DerefMut for ArenaIndex<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.interp
    }
}

impl<'a> Drop for ArenaIndex<'a> {
    fn drop(&mut self) {
        let idx = self.index;
        // We can't panic in a drop impl, so ignore errors when crossing the
        // FFI boundary.
        let _ignored = unsafe {
            self.interp
                .with_ffi_boundary(|mrb| sys::mrb_sys_gc_arena_restore(mrb, idx))
        };
    }
}
