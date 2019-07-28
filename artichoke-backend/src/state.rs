use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::rc::Rc;

use crate::class;
use crate::def::{EnclosingRubyScope, Free};
use crate::eval::EvalContext;
use crate::fs::MrbFilesystem;
use crate::module;
use crate::sys::{self, DescribeState};

// NOTE: MrbState assumes that it it is stored in `mrb_state->ud` wrapped in a
// [`Rc`] with type [`Mrb`] as created by [`crate::interpreter`].
pub struct State {
    pub mrb: *mut sys::mrb_state,
    pub ctx: *mut sys::mrbc_context,
    classes: HashMap<TypeId, Rc<RefCell<class::Spec>>>,
    modules: HashMap<TypeId, Rc<RefCell<module::Spec>>>,
    pub vfs: MrbFilesystem,
    pub(crate) context_stack: Vec<EvalContext>,
    pub num_set_regexp_capture_globals: usize,
    symbol_cache: HashMap<String, sys::mrb_sym>,
}

impl State {
    /// Create a new [`State`] from a [`sys::mrb_state`] and
    /// [`sys::mrbc_context`] with an
    /// [in memory virtual filesystem](MrbFilesystem).
    pub fn new(mrb: *mut sys::mrb_state, ctx: *mut sys::mrbc_context, vfs: MrbFilesystem) -> Self {
        Self {
            mrb,
            ctx,
            classes: HashMap::default(),
            modules: HashMap::default(),
            vfs,
            context_stack: vec![],
            num_set_regexp_capture_globals: 0,
            symbol_cache: HashMap::default(),
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
    /// or an enclosing scope for a class or a module. To mutate the class spec,
    /// call `borrow_mut` on the return value of this method to get a mutable
    /// reference to the class spec.
    ///
    /// Class specs can also be retrieved from the state after creation with
    /// [`State::class_spec`].
    ///
    /// The recommended pattern for using `def_class` looks like this:
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate artichoke_backend;
    ///
    /// use artichoke_backend::convert::FromMrb;
    /// use artichoke_backend::def::{ClassLike, Define};
    /// use artichoke_backend::sys;
    /// use artichoke_backend::value::Value;
    ///
    /// extern "C" fn value(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value
    /// {
    ///     let interp = unsafe { unwrap_interpreter!(mrb) };
    ///     Value::from_mrb(&interp, 29).inner()
    /// }
    ///
    /// fn main() {
    ///     let interp = artichoke_backend::interpreter().expect("mrb init");
    ///     let spec = {
    ///         let mut api = interp.borrow_mut();
    ///         let spec = api.def_class::<()>("Container", None, None);
    ///         spec.borrow_mut().add_method("value", value, sys::mrb_args_none());
    ///         spec.borrow_mut().add_self_method("value", value, sys::mrb_args_none());
    ///         spec.borrow_mut().mrb_value_is_rust_backed(true);
    ///         spec
    ///     };
    ///     spec.borrow().define(&interp).expect("class install");
    /// }
    /// ```
    pub fn def_class<T: Any>(
        &mut self,
        name: &str,
        enclosing_scope: Option<EnclosingRubyScope>,
        free: Option<Free>,
    ) -> Rc<RefCell<class::Spec>> {
        let spec = class::Spec::new(name, enclosing_scope, free);
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
    /// or an enclosing scope for a class or a module. To mutate the class spec,
    /// call `borrow_mut` on the return value of this method to get a mutable
    /// reference to the class spec.
    pub fn class_spec<T: Any>(&self) -> Option<Rc<RefCell<class::Spec>>> {
        self.classes.get(&TypeId::of::<T>()).map(Rc::clone)
    }

    /// Create a module definition bound to a Rust type `T`. Module definitions
    /// have the same lifetime as the [`State`]. Module defs are stored by
    /// [`TypeId`] of `T`.
    ///
    /// Internally, [`module::Spec`]s are stored in an `Rc<RefCell<_>>` which
    /// allows module specs to have multiple owners, such as being an enclosing
    /// scope for a class or a module. To mutate the module spec, call
    /// `borrow_mut` on the return value of this method to get a mutable
    /// reference to the module spec.
    ///
    /// Module specs can also be retrieved from the state after creation with
    /// [`State::module_spec`].
    ///
    /// The recommended pattern for using `def_module` looks like this:
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate artichoke_backend;
    ///
    /// use artichoke_backend::convert::FromMrb;
    /// use artichoke_backend::def::{ClassLike, Define};
    /// use artichoke_backend::sys;
    /// use artichoke_backend::value::Value;
    ///
    /// extern "C" fn value(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value
    /// {
    ///     let interp = unsafe { unwrap_interpreter!(mrb) };
    ///     Value::from_mrb(&interp, 29).inner()
    /// }
    ///
    /// fn main() {
    ///     let interp = artichoke_backend::interpreter().expect("mrb init");
    ///     let spec = {
    ///         let mut api = interp.borrow_mut();
    ///         let spec = api.def_module::<()>("Container", None);
    ///         spec.borrow_mut().add_method("value", value, sys::mrb_args_none());
    ///         spec.borrow_mut().add_self_method("value", value, sys::mrb_args_none());
    ///         spec
    ///     };
    ///     spec.borrow().define(&interp).expect("class install");
    /// }
    /// ```
    pub fn def_module<T: Any>(
        &mut self,
        name: &str,
        enclosing_scope: Option<EnclosingRubyScope>,
    ) -> Rc<RefCell<module::Spec>> {
        let spec = module::Spec::new(name, enclosing_scope);
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
    /// allows module specs to have multiple owners, such as being an enclosing
    /// scope for a class or a module. To mutate the module spec, call
    /// `borrow_mut` on the return value of this method to get a mutable
    /// reference to the module spec.
    pub fn module_spec<T: Any>(&self) -> Option<Rc<RefCell<module::Spec>>> {
        self.modules.get(&TypeId::of::<T>()).map(Rc::clone)
    }

    pub fn sym_intern(&mut self, sym: &str) -> sys::mrb_sym {
        let mrb = self.mrb;
        let interned = self
            .symbol_cache
            .entry(sym.to_owned())
            .or_insert_with(|| unsafe {
                sys::mrb_intern(mrb, sym.as_ptr() as *const i8, sym.len())
            });
        *interned
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            // At this point, the only ref to the smart poitner wrapping the
            // state is stored in the `mrb_state->ud` pointer. Rematerialize the
            // `Rc`, set the userdata pointer to null, and drop the `Rc` to
            // ensure no memory leaks. After this operation, `Rc::strong_count`
            // will be 0 and the `Rc`, `RefCell`, and `State` will be
            // deallocated.
            let ptr = (*self.mrb).ud;
            if !ptr.is_null() {
                let ud = Rc::from_raw(ptr as *const RefCell<Self>);
                // cleanup pointers
                (*self.mrb).ud = std::ptr::null_mut();
                mem::drop(ud);
            }

            // Free mrb data structures
            sys::mrbc_context_free(self.mrb, self.ctx);
            sys::mrb_close(self.mrb);
            // Cleanup dangling pointers
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
