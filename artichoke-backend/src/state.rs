use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;

use crate::class;
use crate::def::{EnclosingRubyScope, Free};
use crate::eval::Context;
use crate::fs::Filesystem;
use crate::module;
use crate::sys::{self, DescribeState};

// NOTE: ArtichokeState assumes that it it is stored in `mrb_state->ud` wrapped in a
// [`Rc`] with type [`Artichoke`] as created by [`crate::interpreter`].
pub struct State {
    pub mrb: *mut sys::mrb_state,
    pub ctx: *mut sys::mrbc_context,
    classes: HashMap<TypeId, Rc<RefCell<class::Spec>>>,
    modules: HashMap<TypeId, Rc<RefCell<module::Spec>>>,
    pub vfs: Filesystem,
    pub(crate) context_stack: Vec<Context>,
    pub active_regexp_globals: usize,
    symbol_cache: HashMap<Cow<'static, [u8]>, sys::mrb_sym>,
    captured_output: Option<String>,
}

impl State {
    /// Create a new [`State`] from a [`sys::mrb_state`] and
    /// [`sys::mrbc_context`] with an
    /// [in memory virtual filesystem](Filesystem).
    pub fn new(mrb: *mut sys::mrb_state, ctx: *mut sys::mrbc_context, vfs: Filesystem) -> Self {
        Self {
            mrb,
            ctx,
            classes: HashMap::default(),
            modules: HashMap::default(),
            vfs,
            context_stack: vec![],
            active_regexp_globals: 0,
            symbol_cache: HashMap::default(),
            captured_output: None,
        }
    }

    pub fn capture_output(&mut self) {
        self.captured_output = Some(String::default());
    }

    pub fn get_and_clear_captured_output(&mut self) -> String {
        self.captured_output
            .replace(String::default())
            .unwrap_or_default()
    }

    pub fn print(&mut self, s: &str) {
        if let Some(ref mut captured_output) = self.captured_output {
            captured_output.push_str(s);
        } else {
            print!("{}", s);
            let _ = io::stdout().flush();
        }
    }

    pub fn puts(&mut self, s: &str) {
        if let Some(ref mut captured_output) = self.captured_output {
            captured_output.push_str(s);
            captured_output.push('\n');
        } else {
            println!("{}", s);
        }
    }

    /// Close a [`State`] and free underlying mruby structs and memory.
    pub fn close(&mut self) {
        unsafe {
            // At this point, the only refs to the smart poitner wrapping the
            // state are stored in the `mrb_state->ud` pointer and any
            // `MRB_TT_DATA` objects in the mruby heap.
            //
            // To clean up:
            //
            // - Save the raw pointer to the `Artichoke` from the user data.
            // - Free the mrb context.
            // - Close the interpreter which frees every object in the heap and
            //   drops the strong count on the Rc to 1.
            // - Rematerialize the `Rc`.
            // - Drop the `Rc` which drops the strong count to 0 and frees the
            //   state.
            // - Set the userdata pointer to null.
            // - Set context and mrb properties to null.
            if self.mrb.is_null() {
                return;
            }
            let ptr = (*self.mrb).ud;
            if ptr.is_null() {
                return;
            }
            // Free mrb data structures
            sys::mrbc_context_free(self.mrb, self.ctx);
            sys::mrb_close(self.mrb);
            // Cleanup dangling pointers
            self.ctx = std::ptr::null_mut();
            self.mrb = std::ptr::null_mut();
        };
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
    /// use artichoke_backend::convert::Convert;
    /// use artichoke_backend::def::{ClassLike, Define};
    /// use artichoke_backend::sys;
    /// use artichoke_backend::value::Value;
    ///
    /// extern "C" fn value(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value
    /// {
    ///     let interp = unsafe { unwrap_interpreter!(mrb) };
    ///     let result: Value = interp.convert(29);
    ///     result.inner()
    /// }
    ///
    /// fn main() {
    ///     let interp = artichoke_backend::interpreter().expect("init");
    ///     let spec = {
    ///         let mut api = interp.0.borrow_mut();
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
    /// use artichoke_backend::convert::Convert;
    /// use artichoke_backend::def::{ClassLike, Define};
    /// use artichoke_backend::sys;
    /// use artichoke_backend::value::Value;
    ///
    /// extern "C" fn value(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value
    /// {
    ///     let interp = unsafe { unwrap_interpreter!(mrb) };
    ///     let result: Value = interp.convert(29);
    ///     result.inner()
    /// }
    ///
    /// fn main() {
    ///     let interp = artichoke_backend::interpreter().expect("init");
    ///     let spec = {
    ///         let mut api = interp.0.borrow_mut();
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

    pub fn sym_intern<T>(&mut self, sym: T) -> sys::mrb_sym
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let mrb = self.mrb;
        let sym = sym.into();
        let ptr = sym.as_ref().as_ptr();
        let len = sym.as_ref().len();
        let interned = self
            .symbol_cache
            .entry(sym)
            .or_insert_with(|| unsafe { sys::mrb_intern(mrb, ptr as *const i8, len) });
        *interned
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
