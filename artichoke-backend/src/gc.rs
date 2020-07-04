use crate::gc::arena::IndexError;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

pub mod arena;

use arena::ArenaIndex;

/// Garbage collection primitives for an mruby interpreter.
pub trait MrbGarbageCollection {
    /// Create a savepoint in the GC arena.
    ///
    /// Savepoints allow mruby to deallocate all of the objects created via the
    /// C API.
    ///
    /// Normally objects created via the C API are marked as permanently alive
    /// ("white" GC color) with a call to
    /// [`mrb_gc_protect`](sys::mrb_gc_protect).
    ///
    /// The returned [`ArenaIndex`] implements [`Drop`], so it is sufficient to
    /// let it go out of scope to ensure objects are eventually collected.
    fn create_arena_savepoint(&mut self) -> Result<ArenaIndex<'_>, IndexError>;

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
    fn create_arena_savepoint(&mut self) -> Result<ArenaIndex<'_>, IndexError> {
        ArenaIndex::new(self)
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
        let mut arena = interp.create_arena_savepoint().unwrap();
        for _ in 0..2000 {
            let value = arena.eval(b"'a'").unwrap();
            let _ = value.to_s(&mut arena);
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
            let mut arena = interp.create_arena_savepoint().unwrap();
            for _ in 0..2000 {
                let value = arena.eval(b"'a'").unwrap();
                let _ = value.to_s(&mut arena);
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
        let mut arena = interp.create_arena_savepoint().unwrap();
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
        let live = arena.live_object_count();
        arena.full_gc();
        assert_eq!(
            arena.live_object_count(),
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
        let mut arena = interp.create_arena_savepoint().unwrap();
        let baseline_object_count = arena.live_object_count();
        drop(&mut arena.eval(b"").unwrap());
        arena.restore();
        interp.full_gc();
        assert_eq!(interp.live_object_count(), baseline_object_count);
    }

    #[test]
    fn gc_functional_test() {
        let mut interp = crate::interpreter().unwrap();
        let baseline_object_count = interp.live_object_count();
        let mut initial_arena = interp.create_arena_savepoint().unwrap();
        for _ in 0..2000 {
            let mut arena = initial_arena.create_arena_savepoint().unwrap();
            let result = arena.eval(b"'gc test'");
            let value = result.unwrap();
            assert!(!value.is_dead(&mut arena));
            arena.restore();
            initial_arena.incremental_gc();
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
