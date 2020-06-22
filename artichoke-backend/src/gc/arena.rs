//! Garbage collection arenas for native code.

use std::ops::{Deref, DerefMut};

use crate::sys;
use crate::Artichoke;

/// Interpreter guard that acts as a GC arena savepoint.
///
/// Arena savepoints ensure mruby objects are reaped even when allocated with
/// the C API.
///
/// mruby manages objects created via the C API in a memory construct called
/// the
/// [arena](https://github.com/mruby/mruby/blob/master/doc/guides/gc-arena-howto.md).
/// The arena is a stack and objects stored there are permanently alive to avoid
/// having to track lifetimes externally to the interperter.
///
/// An [`ArenaIndex`] is an index to some position of the stack. When restoring
/// an `ArenaIndex`, the stack pointer is moved. All objects beyond the pointer
/// are no longer live and are eligible to be collected at the next GC.
///
/// `ArenaIndex` implements [`Drop`], so letting it go out of scope is
/// sufficient to ensure objects get collected eventually.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ArenaIndex<'a> {
    index: i32,
    interp: &'a mut Artichoke,
}

impl<'a> ArenaIndex<'a> {
    /// Create a new arena savepoint.
    pub fn new(interp: &'a mut Artichoke) -> Self {
        let index = unsafe {
            interp
                .with_ffi_boundary(|mrb| sys::mrb_sys_gc_arena_save(mrb))
                .unwrap_or_default()
        };
        Self { index, interp }
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

impl<'a> AsRef<Artichoke> for ArenaIndex<'a> {
    #[inline]
    fn as_ref(&self) -> &Artichoke {
        &*self.interp
    }
}

impl<'a> AsMut<Artichoke> for ArenaIndex<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Artichoke {
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
        let _ = unsafe {
            self.interp
                .with_ffi_boundary(|mrb| sys::mrb_sys_gc_arena_restore(mrb, idx))
        };
    }
}
