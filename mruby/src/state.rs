use mruby_vfs::{FakeFileSystem, FileSystem};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ffi::c_void;
use std::fmt;
use std::mem;
use std::rc::Rc;

use crate::class;
use crate::def::{ClassLike, Free, Parent};
use crate::eval::EvalContext;
use crate::interpreter::Mrb;
use crate::module;
use crate::sys::{self, DescribeState};

#[derive(Clone, Debug)]
pub struct VfsMetadata {
    pub require: Option<fn(Mrb)>,
    already_required: bool,
}

impl VfsMetadata {
    pub fn new(require: Option<fn(Mrb)>) -> Self {
        Self {
            require,
            already_required: false,
        }
    }

    pub fn mark_required(&self) -> Self {
        Self {
            require: self.require,
            already_required: true,
        }
    }

    pub fn is_already_required(&self) -> bool {
        self.already_required
    }
}

// NOTE: MrbState assumes that it it is stored in `mrb_state->ud` wrapped in a
// [`Rc`] with type [`Mrb`] as created by [`Interpreter::create`].
pub struct State {
    // TODO: Make this private
    pub mrb: *mut sys::mrb_state,
    // TODO: Make this private
    pub ctx: *mut sys::mrbc_context,
    classes: HashMap<TypeId, Rc<class::Spec>>,
    modules: HashMap<TypeId, Rc<module::Spec>>,
    // TODO: Make this private
    pub(crate) vfs: FakeFileSystem<VfsMetadata>,
    // TODO: make this private
    pub(crate) context_stack: Vec<EvalContext>,
}

impl State {
    pub fn new(mrb: *mut sys::mrb_state, ctx: *mut sys::mrbc_context, source_dir: &str) -> Self {
        let vfs = FakeFileSystem::new();
        vfs.create_dir_all(source_dir).expect("vfs init");
        Self {
            mrb,
            ctx,
            classes: HashMap::new(),
            modules: HashMap::new(),
            vfs,
            context_stack: vec![],
        }
    }

    pub fn close(self) {
        drop(self)
    }

    pub fn def_class<T: Any>(&mut self, name: &str, parent: Option<Parent>, free: Option<Free>) {
        let spec = class::Spec::new(name, parent, free);
        self.classes.insert(TypeId::of::<T>(), Rc::new(spec));
    }

    // NOTE: This function must return a reference with a smart pointer. `Class`
    // specs are bound to the lifetime of the `Mrb` interpreter because if the
    // sys pointers are deallocated, mruby may segfault.
    pub fn class_spec<T: Any>(&self) -> Rc<class::Spec> {
        let spec = self.classes.get(&TypeId::of::<T>()).expect("class spec");
        Rc::clone(spec)
    }

    // NOTE: This function will panic if there is more than one `Rc` pointing
    // to the `class::Spec` backing the type `T`. There may be more than one
    // `Rc` if a class has been used as a parent for another `Module` or
    // `Class`. In practice, this means that `mruby` classes are closed once
    // they are `define`d on an `Mrb` interpreter.
    pub fn class_spec_mut<T: Any>(&mut self) -> &mut class::Spec {
        let spec = self
            .classes
            .get_mut(&TypeId::of::<T>())
            .expect("class spec");
        let name = spec.name().to_owned();
        Rc::get_mut(spec).unwrap_or_else(|| panic!("mutable class spec for {}", name))
    }

    pub fn def_module<T: Any>(&mut self, name: &str, parent: Option<Parent>) {
        let spec = module::Spec::new(name, parent);
        self.modules.insert(TypeId::of::<T>(), Rc::new(spec));
    }

    // NOTE: This function must return a reference with a smart pointer.
    // `Module` specs are bound to the lifetime of the `Mrb` interpreter because
    // if the sys pointers are deallocated, mruby may segfault.
    pub fn module_spec<T: Any>(&self) -> Rc<module::Spec> {
        let spec = self.modules.get(&TypeId::of::<T>()).expect("module spec");
        Rc::clone(spec)
    }

    // NOTE: This function will panic if there is more than one `Rc` pointing
    // to the `module::Spec` backing the type `T`. There may be more than one
    // `Rc` if a module has been used as a parent for another `Module` or
    // `Class`. In practice, this means that `mruby` modules are closed once
    // they are `define`d on an `Mrb` interpreter.
    pub fn module_spec_mut<T: Any>(&mut self) -> &mut module::Spec {
        let spec = self
            .modules
            .get_mut(&TypeId::of::<T>())
            .expect("module spec");
        let name = spec.name().to_owned();
        Rc::get_mut(spec).unwrap_or_else(|| panic!("mutable module spec for {}", name))
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            // At this point, the only ref to the smart poitner wrapping the
            // state is stored in the `mrb_state->ud` pointer. Rematerialize the
            // `Rc`, set the userdata pointer to null, and drop the `Rc` to
            // ensure no memory leaks. After this operation, `Rc::strong_count`
            // will be 0 and the `Rc`, `RefCell`, and `MrbState` will be
            // deallocated.
            let ptr = (*self.mrb).ud;
            if !ptr.is_null() {
                let ud = mem::transmute::<*mut c_void, Mrb>(ptr);
                // cleanup pointers
                (*self.mrb).ud = std::ptr::null_mut();
                mem::drop(ud);
            }

            // Free mrb data structures
            sys::mrbc_context_free(self.mrb, self.ctx);
            sys::mrb_close(self.mrb);
            // Cleanup dangling pointers in `MrbApi`
            self.ctx = std::ptr::null_mut();
            self.mrb = std::ptr::null_mut();
        };
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mrb.debug())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mrb.info())
    }
}
