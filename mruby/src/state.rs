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
use crate::MrbError;

#[derive(Clone, Debug)]
pub struct VfsMetadata {
    pub require: Option<fn(Mrb) -> Result<(), MrbError>>,
    already_required: bool,
}

impl VfsMetadata {
    pub fn new() -> Self {
        Self::default()
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

impl Default for VfsMetadata {
    fn default() -> Self {
        Self {
            require: None,
            already_required: false,
        }
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
    /// [`MrbLoadSources::def_rb_source_file`](crate::load::MrbLoadSources::def_rb_source_file).
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
    ///
    /// The recommended pattern for using `def_class` looks like this:
    ///
    /// ```rust
    /// use mruby::def::{ClassLike, Define};
    /// use mruby::interpreter::{Interpreter, MrbApi};
    /// use mruby::interpreter_or_raise;
    /// use mruby::sys;
    ///
    /// extern "C" fn value(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value
    /// {
    ///     let interp = unsafe { interpreter_or_raise!(mrb) };
    ///     interp.fixnum(29).inner()
    /// }
    ///
    /// let interp = Interpreter::create().expect("mrb init");
    /// let spec = {
    ///     let mut api = interp.borrow_mut();
    ///     let spec = api.def_class::<()>("Container", None, None);
    ///     spec.borrow_mut().add_method("value", value, sys::mrb_args_none());
    ///     spec.borrow_mut().add_self_method("value", value, sys::mrb_args_none());
    ///     spec.borrow_mut().mrb_value_is_rust_backed(true);
    ///     spec
    /// };
    /// spec.borrow().define(&interp).expect("class install");
    /// ```
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
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`State::def_class`].
    ///
    /// Internally, [`class::Spec`]s are stored in an `Rc<RefCell<_>>` which
    /// allows class specs to have multiple owners, such as being a super class
    /// or a parent for a class or a module. To mutate the class spec, call
    /// `borrow_mut` on the return value of this method to get a mutable
    /// reference to the class spec.
    pub fn class_spec<T: Any>(&self) -> Option<Rc<RefCell<class::Spec>>> {
        self.classes.get(&TypeId::of::<T>()).map(Rc::clone)
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
    ///
    /// The recommended pattern for using `def_class` looks like this:
    ///
    /// ```rust
    /// use mruby::def::{ClassLike, Define};
    /// use mruby::interpreter::{Interpreter, MrbApi};
    /// use mruby::interpreter_or_raise;
    /// use mruby::sys;
    ///
    /// extern "C" fn value(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value
    /// {
    ///     let interp = unsafe { interpreter_or_raise!(mrb) };
    ///     interp.fixnum(29).inner()
    /// }
    ///
    /// let interp = Interpreter::create().expect("mrb init");
    /// let spec = {
    ///     let mut api = interp.borrow_mut();
    ///     let spec = api.def_module::<()>("Container", None);
    ///     spec.borrow_mut().add_method("value", value, sys::mrb_args_none());
    ///     spec.borrow_mut().add_self_method("value", value, sys::mrb_args_none());
    ///     spec
    /// };
    /// spec.borrow().define(&interp).expect("class install");
    /// ```
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
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`State::def_module`].
    ///
    /// Internally, [`module::Spec`]s are stored in an `Rc<RefCell<_>>` which
    /// allows module specs to have multiple owners, such as being a parent for
    /// a class or a module. To mutate the module spec, call `borrow_mut` on the
    /// return value of this method to get a mutable reference to the module
    /// spec.
    pub fn module_spec<T: Any>(&self) -> Option<Rc<RefCell<module::Spec>>> {
        self.modules.get(&TypeId::of::<T>()).map(Rc::clone)
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
