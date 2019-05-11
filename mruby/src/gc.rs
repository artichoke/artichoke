use std::rc::Rc;

use crate::interpreter::Mrb;
use crate::sys;

#[derive(Debug)]
pub struct ArenaIndex {
    index: i32,
    interp: Mrb,
}

impl ArenaIndex {
    pub fn restore(self) {
        drop(self);
    }
}

impl Clone for ArenaIndex {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            interp: Rc::clone(&self.interp),
        }
    }
}

impl Drop for ArenaIndex {
    fn drop(&mut self) {
        unsafe { sys::mrb_sys_gc_arena_restore(self.interp.borrow().mrb, self.index) };
    }
}

pub trait GarbageCollection {
    fn create_arena_savepoint(&self) -> ArenaIndex;

    fn live_object_count(&self) -> i32;

    fn incremental_gc(&self);

    fn full_gc(&self);

    fn enable_gc(&self);

    fn disable_gc(&self);
}

impl GarbageCollection for Mrb {
    fn create_arena_savepoint(&self) -> ArenaIndex {
        // Create a savepoint in the GC arena which will allow mruby to
        // deallocate all of the objects we create via the C API. Normally
        // objects created via the C API are marked as permannently alive
        // ("white" GC color) with a call to `mrb_gc_protect`.
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

    fn enable_gc(&self) {
        unsafe { sys::mrb_sys_gc_enable(self.borrow().mrb) };
    }

    fn disable_gc(&self) {
        unsafe { sys::mrb_sys_gc_disable(self.borrow().mrb) };
    }
}

#[cfg(test)]
mod tests {
    use crate::gc::GarbageCollection;
    use crate::interpreter::{Interpreter, MrbApi};

    #[test]
    fn arena_restore_on_explicit_restore() {
        let interp = Interpreter::create().expect("mrb init");
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
        let interp = Interpreter::create().expect("mrb init");
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
        let interp = Interpreter::create().expect("mrb init");
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
        let interp = Interpreter::create().expect("mrb init");
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

    mod functional {
        use super::*;

        #[test]
        fn empty_eval() {
            let interp = Interpreter::create().expect("mrb init");
            let arena = interp.create_arena_savepoint();
            let baseline_object_count = interp.live_object_count();
            drop(interp.eval("").expect("eval"));
            arena.restore();
            interp.full_gc();
            assert_eq!(interp.live_object_count(), baseline_object_count);
        }

        #[test]
        fn gc() {
            let interp = Interpreter::create().expect("mrb init");
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
}
