use std::rc::Rc;

use crate::sys;
use crate::Mrb;

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
#[derive(Debug, Clone)]
pub struct ArenaIndex {
    index: i32,
    interp: Mrb,
}

impl ArenaIndex {
    /// Restore the arena stack pointer to its prior index.
    pub fn restore(self) {
        drop(self);
    }
}

impl Drop for ArenaIndex {
    fn drop(&mut self) {
        unsafe { sys::mrb_sys_gc_arena_restore(self.interp.borrow().mrb, self.index) };
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
    fn create_arena_savepoint(&self) -> ArenaIndex;

    /// Retrieve the number of live objects on the interpreter heap.
    ///
    /// A live object is reachable via top self, the stack, or the arena.
    fn live_object_count(&self) -> i32;

    /// Perform an incremental garbage collection.
    ///
    /// An incremental GC is less computationally expensive than a
    /// [full GC](MrbGarbageCollection::full_gc), but does not guarantee that
    /// all dead objects will be reaped. You may wish to use an incremental GC
    /// if you are operating with an interpreter in a loop.
    fn incremental_gc(&self);

    /// Perform a full garbage collection.
    ///
    /// A full GC guarantees that all dead objects will be reaped, so it is more
    /// expensive than an
    /// [incremental GC](MrbGarbageCollection::incremental_gc). You may wish to
    /// use a full GC if you are memory constrained.
    fn full_gc(&self);

    /// Enable garbage collection.
    ///
    /// Returns the prior GC enabled state.
    fn enable_gc(&self) -> bool;

    /// Disable garbage collection.
    ///
    /// Returns the prior GC enabled state.
    fn disable_gc(&self) -> bool;
}

impl MrbGarbageCollection for Mrb {
    fn create_arena_savepoint(&self) -> ArenaIndex {
        ArenaIndex {
            index: unsafe { sys::mrb_sys_gc_arena_save(self.borrow().mrb) },
            interp: Rc::clone(self),
        }
    }

    fn live_object_count(&self) -> i32 {
        unsafe { sys::mrb_sys_gc_live_objects(self.borrow().mrb) }
    }

    fn incremental_gc(&self) {
        unsafe { sys::mrb_incremental_gc(self.borrow().mrb) };
    }

    fn full_gc(&self) {
        unsafe { sys::mrb_full_gc(self.borrow().mrb) };
    }

    fn enable_gc(&self) -> bool {
        unsafe { sys::mrb_sys_gc_enable(self.borrow().mrb) }
    }

    fn disable_gc(&self) -> bool {
        unsafe { sys::mrb_sys_gc_disable(self.borrow().mrb) }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::Eval;
    use crate::gc::MrbGarbageCollection;

    #[test]
    fn arena_restore_on_explicit_restore() {
        let interp = crate::interpreter().expect("mrb init");
        let baseline_object_count = interp.live_object_count();
        let arena = interp.create_arena_savepoint();
        for _ in 0..2000 {
            let value = interp.eval("'a'").expect("value");
            let _ = value.to_s();
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
        let interp = crate::interpreter().expect("mrb init");
        let baseline_object_count = interp.live_object_count();
        {
            let _arena = interp.create_arena_savepoint();
            for _ in 0..2000 {
                let value = interp.eval("'a'").expect("value");
                let _ = value.to_s();
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
    fn arena_clone() {
        let interp = crate::interpreter().expect("mrb init");
        let baseline_object_count = interp.live_object_count();
        let arena = interp.create_arena_savepoint();
        let arena_clone = arena.clone();
        // restore original before any objects have been allocated
        arena.restore();
        for _ in 0..2000 {
            let value = interp.eval("'a'").expect("value");
            let _ = value.to_s();
        }
        arena_clone.restore();
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
        let interp = crate::interpreter().expect("mrb init");
        interp.disable_gc();
        let arena = interp.create_arena_savepoint();
        interp
            .eval(
                r#"
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
            .expect("eval");
        let live = interp.live_object_count();
        interp.full_gc();
        assert_eq!(
            interp.live_object_count(),
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
        let interp = crate::interpreter().expect("mrb init");
        let arena = interp.create_arena_savepoint();
        let baseline_object_count = interp.live_object_count();
        drop(interp.eval("").expect("eval"));
        arena.restore();
        interp.full_gc();
        assert_eq!(interp.live_object_count(), baseline_object_count);
    }

    #[test]
    fn gc_functional_test() {
        let interp = crate::interpreter().expect("mrb init");
        let baseline_object_count = interp.live_object_count();
        let initial_arena = interp.create_arena_savepoint();
        for _ in 0..2000 {
            let arena = interp.create_arena_savepoint();
            let result = interp.eval("'gc test'");
            let value = result.unwrap();
            assert!(!value.is_dead());
            arena.restore();
            interp.incremental_gc();
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
