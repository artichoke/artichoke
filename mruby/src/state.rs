use mruby_vfs::{FakeFileSystem, FileSystem};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fmt;
use std::mem;
use std::rc::Rc;

use crate::class;
use crate::def::{Free, Parent};
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
    classes: HashMap<TypeId, Rc<RefCell<class::Spec>>>,
    modules: HashMap<TypeId, Rc<RefCell<module::Spec>>>,
    // TODO: Make this private
    pub(crate) vfs: FakeFileSystem<VfsMetadata>,
    // TODO: make this private
    pub(crate) context_stack: Vec<EvalContext>,
}

impl State {
    /// Create a new [`State`] from a [`sys::mrb_state`] and
    /// [`sys::mrbc_context`] with a fake in memory virtual filesystem
    /// ([`FakeFileSystem`]).
    ///
    /// This constructor creates the directory `source_dir` in the VFS to act as
    /// the source path for new Ruby files. See
    /// [`MrbLoadSources::def_rb_source_file`].
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

    /// Close a [`State`] and free underlying mruby structs and memory.
    pub fn close(self) {
        drop(self)
    }

    /// Create a class definition bound to a Rust type `T`. Class definitions
    /// have the same lifetime as the [`State`] because the class def owns the
    /// `mrb_data_type` for the type, which must be long-lived. Class defs are
    /// stored by [`TypeId`] of `T`.
    ///
    /// Internally, [`class::Spec`]s are stored in an `Rc<RefCell<_>>` which
    /// allows class specs to have multiple owners, such as being a super class
    /// or a parent for a class or a module. To mutate the class spec, call
    /// `borrow_mut` on the return value of this method to get a mutable
    /// reference to the class spec.
    ///
    /// Class specs can also be retrieved from the state after creation with
    /// [`State::class_spec`].
    pub fn def_class<T: Any>(
        &mut self,
        name: &str,
        parent: Option<Parent>,
        free: Option<Free>,
    ) -> Rc<RefCell<class::Spec>> {
        let spec = class::Spec::new(name, parent, free);
        let spec = Rc::new(RefCell::new(spec));
        self.classes.insert(TypeId::of::<T>(), Rc::clone(&spec));
        spec
    }

    /// Retrieve a class definition from the state bound to Rust type `T`.
    ///
    /// This function panics if type `T` has not had a class spec registered
    /// for it using [`State::def_class`].
    ///
    /// Internally, [`class::Spec`]s are stored in an `Rc<RefCell<_>>` which
    /// allows class specs to have multiple owners, such as being a super class
    /// or a parent for a class or a module. To mutate the class spec, call
    /// `borrow_mut` on the return value of this method to get a mutable
    /// reference to the class spec.
    pub fn class_spec<T: Any>(&self) -> Rc<RefCell<class::Spec>> {
        let spec = self.classes.get(&TypeId::of::<T>()).expect("class spec");
        Rc::clone(spec)
    }

    /// Create a module definition bound to a Rust type `T`. Module definitions
    /// have the same lifetime as the [`State`]. Module defs are stored by
    /// [`TypeId`] of `T`.
    ///
    /// Internally, [`module::Spec`]s are stored in an `Rc<RefCell<_>>` which
    /// allows module specs to have multiple owners, such as being a parent for
    /// a class or a module. To mutate the module spec, call `borrow_mut` on the
    /// return value of this method to get a mutable reference to the module
    /// spec.
    ///
    /// Class specs can also be retrieved from the state after creation with
    /// [`State::class_spec`].
    pub fn def_module<T: Any>(
        &mut self,
        name: &str,
        parent: Option<Parent>,
    ) -> Rc<RefCell<module::Spec>> {
        let spec = module::Spec::new(name, parent);
        let spec = Rc::new(RefCell::new(spec));
        self.modules.insert(TypeId::of::<T>(), Rc::clone(&spec));
        spec
    }

    /// Retrieve a module definition from the state bound to Rust type `T`.
    ///
    /// This function panics if type `T` has not had a class spec registered
    /// for it using [`State::def_module`].
    ///
    /// Internally, [`module::Spec`]s are stored in an `Rc<RefCell<_>>` which
    /// allows module specs to have multiple owners, such as being a parent for
    /// a class or a module. To mutate the module spec, call `borrow_mut` on the
    /// return value of this method to get a mutable reference to the module
    /// spec.
    pub fn module_spec<T: Any>(&self) -> Rc<RefCell<module::Spec>> {
        let spec = self.modules.get(&TypeId::of::<T>()).expect("module spec");
        Rc::clone(spec)
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
