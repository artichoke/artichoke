use crate::sys;
use crate::value::Value;
use crate::Artichoke;

/// Arena savepoint that can be restored to ensure mruby objects are reaped.
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
pub struct ArenaIndex<'a> {
    index: i32,
    interp: &'a mut Artichoke,
}

impl<'a> ArenaIndex<'a> {
    /// Restore the arena stack pointer to its prior index.
    pub fn restore(self) {
        drop(self);
    }

    #[inline]
    pub fn interp(&mut self) -> &mut Artichoke {
        self.interp
    }
}

impl<'a> Drop for ArenaIndex<'a> {
    fn drop(&mut self) {
        unsafe {
            let mrb = self.interp.mrb.as_mut();
            sys::mrb_sys_gc_arena_restore(mrb, self.index);
        }
    }
}

/// Garbage collection primitives for an mruby interpreter.
pub trait MrbGarbageCollection {
    /// Create a savepoint in the GC arena which will allow mruby to deallocate
    /// all of the objects created via the C API.
    ///
    /// Normally objects created via the C API are marked as permanently alive
    /// ("white" GC color) with a call to
    /// [`mrb_gc_protect`](sys::mrb_gc_protect).
    ///
    /// The returned [`ArenaIndex`] implements [`Drop`], so it is sufficient to
    /// let it go out of scope to ensure objects are eventually collected.
    fn create_arena_savepoint(&mut self) -> ArenaIndex<'_>;

    /// Retrieve the number of live objects on the interpreter heap.
    ///
    /// A live object is reachable via top self, the stack, or the arena.
    fn live_object_count(&mut self) -> i32;

    /// Mark a [`Value`] as reachable in the mruby garbage collector.
    fn mark_value(&mut self, value: &Value);

    /// Perform an incremental garbage collection.
    ///
    /// An incremental GC is less computationally expensive than a
    /// [full GC](MrbGarbageCollection::full_gc), but does not guarantee that
    /// all dead objects will be reaped. You may wish to use an incremental GC
    /// if you are operating with an interpreter in a loop.
    fn incremental_gc(&mut self);

    /// Perform a full garbage collection.
    ///
    /// A full GC guarantees that all dead objects will be reaped, so it is more
    /// expensive than an
    /// [incremental GC](MrbGarbageCollection::incremental_gc). You may wish to
    /// use a full GC if you are memory constrained.
    fn full_gc(&mut self);

    /// Enable garbage collection.
    ///
    /// Returns the prior GC enabled state.
    fn enable_gc(&mut self) -> State;

    /// Disable garbage collection.
    ///
    /// Returns the prior GC enabled state.
    fn disable_gc(&mut self) -> State;
}

impl MrbGarbageCollection for Artichoke {
    fn create_arena_savepoint(&mut self) -> ArenaIndex<'_> {
        let index = unsafe {
            self.with_ffi_boundary(|mrb| sys::mrb_sys_gc_arena_save(mrb))
                .unwrap_or_default()
        };
        ArenaIndex {
            index,
            interp: self,
        }
    }

    fn live_object_count(&mut self) -> i32 {
        unsafe {
            self.with_ffi_boundary(|mrb| sys::mrb_sys_gc_live_objects(mrb))
                .unwrap_or_default()
        }
    }

    fn mark_value(&mut self, value: &Value) {
        unsafe {
            let _ = self.with_ffi_boundary(|mrb| sys::mrb_sys_safe_gc_mark(mrb, value.inner()));
        }
    }

    fn incremental_gc(&mut self) {
        unsafe {
            let _ = self.with_ffi_boundary(|mrb| {
                sys::mrb_incremental_gc(mrb);
            });
        }
    }

    fn full_gc(&mut self) {
        unsafe {
            let _ = self.with_ffi_boundary(|mrb| {
                sys::mrb_full_gc(mrb);
            });
        }
    }

    fn enable_gc(&mut self) -> State {
        unsafe {
            self.with_ffi_boundary(|mrb| {
                if sys::mrb_sys_gc_enable(mrb) {
                    State::Enabled
                } else {
                    State::Disabled
                }
            })
            .unwrap_or(State::Disabled)
        }
    }

    fn disable_gc(&mut self) -> State {
        unsafe {
            self.with_ffi_boundary(|mrb| {
                if sys::mrb_sys_gc_disable(mrb) {
                    State::Enabled
                } else {
                    State::Disabled
                }
            })
            .unwrap_or(State::Disabled)
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum State {
    Disabled,
    Enabled,
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn arena_restore_on_explicit_restore() {
        let mut interp = crate::interpreter().unwrap();
        let baseline_object_count = interp.live_object_count();
        let mut arena = interp.create_arena_savepoint();
        for _ in 0..2000 {
            let value = arena.interp().eval(b"'a'").unwrap();
            let _ = value.to_s(arena.interp());
        }
        arena.restore();
        interp.full_gc();
        assert_eq!(
            interp.live_object_count(),
            // plus 1 because stack keep is enabled in eval which marks the last
            // returned value as live.
            baseline_object_count + 1,
            "Arena restore + full GC should free unreachable objects",
        );
    }

    #[test]
    fn arena_restore_on_drop() {
        let mut interp = crate::interpreter().unwrap();
        let baseline_object_count = interp.live_object_count();
        {
            let mut arena = interp.create_arena_savepoint();
            for _ in 0..2000 {
                let value = arena.interp().eval(b"'a'").unwrap();
                let _ = value.to_s(arena.interp());
            }
        }
        interp.full_gc();
        assert_eq!(
            interp.live_object_count(),
            // plus 1 because stack keep is enabled in eval which marks the last
            // returned value as live.
            baseline_object_count + 1,
            "Arena restore + full GC should free unreachable objects",
        );
    }

    #[test]
    fn enable_disable_gc() {
        let mut interp = crate::interpreter().unwrap();
        interp.disable_gc();
        let mut arena = interp.create_arena_savepoint();
        let _ = arena
            .interp()
            .eval(
                br#"
                # this value will be garbage collected because it is eventually
                # shadowed and becomes unreachable
                a = []
                # this value will not be garbage collected because it is a local
                # variable in top self
                a = []
                # this value will not be garbage collected because it is a local
                # variable in top self
                b = []
                # this value will not be garbage collected because the last value
                # returned by eval is retained with "stack keep"
                []
                "#,
            )
            .unwrap();
        let live = arena.interp().live_object_count();
        arena.interp().full_gc();
        assert_eq!(
            arena.interp().live_object_count(),
            live,
            "GC is disabled. No objects should be collected"
        );
        arena.restore();
        interp.enable_gc();
        interp.full_gc();
        assert_eq!(
            interp.live_object_count(),
            live - 2,
            "Arrays should be collected after enabling GC and running a full GC"
        );
    }

    #[test]
    fn gc_after_empty_eval() {
        let mut interp = crate::interpreter().unwrap();
        let mut arena = interp.create_arena_savepoint();
        let baseline_object_count = arena.interp().live_object_count();
        drop(arena.interp().eval(b"").unwrap());
        arena.restore();
        interp.full_gc();
        assert_eq!(interp.live_object_count(), baseline_object_count);
    }

    #[test]
    fn gc_functional_test() {
        let mut interp = crate::interpreter().unwrap();
        let baseline_object_count = interp.live_object_count();
        let mut initial_arena = interp.create_arena_savepoint();
        for _ in 0..2000 {
            let mut arena = initial_arena.interp().create_arena_savepoint();
            let result = arena.interp().eval(b"'gc test'");
            let value = result.unwrap();
            assert!(!value.is_dead(arena.interp()));
            arena.restore();
            initial_arena.interp().incremental_gc();
        }
        initial_arena.restore();
        interp.full_gc();
        assert_eq!(
            interp.live_object_count(),
            // plus 1 because stack keep is enabled in eval which marks the
            // last returned value as live.
            baseline_object_count + 1,
            "Started with {} live objects, ended with {}. Potential memory leak!",
            baseline_object_count,
            interp.live_object_count()
        );
    }
}
