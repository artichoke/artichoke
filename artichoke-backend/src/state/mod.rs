use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::num::NonZeroUsize;
use std::ptr::{self, NonNull};

use crate::class;
use crate::fs;
use crate::module;
use crate::sys;

pub mod output;
pub mod parser;
#[cfg(feature = "artichoke-random")]
pub mod prng;

// NOTE: `State` assumes that it it is stored in `mrb_state->ud` wrapped in a
// [`Rc`] with type [`Artichoke`] as created by [`crate::interpreter`].
pub struct State {
    pub mrb: *mut sys::mrb_state,
    pub parser: parser::State,
    classes: HashMap<TypeId, Box<class::Spec>>,
    modules: HashMap<TypeId, Box<module::Spec>>,
    pub vfs: fs::Virtual,
    pub active_regexp_globals: Option<NonZeroUsize>,
    pub output: Box<dyn output::Output>,
    #[cfg(feature = "artichoke-random")]
    pub prng: prng::Prng,
}

impl State {
    /// Create a new [`State`] from a [`sys::mrb_state`].
    ///
    /// The state is comprised of several components:
    ///
    /// - `Class` and `Module` registries.
    /// - `Regexp` global state.
    /// - [In memory virtual filesystem](fs::Virtual).
    /// - Ruby parser and file context.
    /// - [Intepreter-level PRNG](prng::Prng) (behind the `artichoke-random`
    ///   feature).
    /// - IO capturing strategy.
    pub fn new(mrb: &mut sys::mrb_state) -> Option<Self> {
        let parser = parser::State::new(mrb)?;
        let state = Self {
            mrb,
            parser,
            classes: HashMap::default(),
            modules: HashMap::default(),
            vfs: fs::Virtual::new(),
            active_regexp_globals: None,
            output: Box::new(output::Process::new()),
            #[cfg(feature = "artichoke-random")]
            prng: prng::Prng::default(),
        };
        Some(state)
    }

    /// Close a [`State`] and free underlying mruby structs and memory.
    // TODO: this should take self by value but can't because self is stored in
    // a `RefCell`.
    pub fn close(&mut self) {
        // At this point, the only refs to the smart poitner wrapping the state
        // are stored in the `mrb_state->ud` pointer and any `MRB_TT_DATA`
        // objects in the mruby heap.
        //
        // To clean up:
        //
        // - Save the raw pointer to the `Artichoke` from the user data.
        // - Free the mrb context.
        // - Close the interpreter which frees every object in the heap and
        // drops the strong count on the Rc to 1.
        // - Rematerialize the `Rc`.
        // - Drop the `Rc` which drops the strong count to 0 and frees the
        //   state.
        // - Set the userdata pointer to null.
        // - Set context and mrb properties to null.
        if let Some(mut mrb) = NonNull::new(self.mrb) {
            let parser = mem::replace(&mut self.parser, unsafe { parser::State::uninit() });
            parser.close(unsafe { mrb.as_mut() });
            if let Some(_userdata) = NonNull::new(unsafe { mrb.as_ref().ud }) {
                unsafe {
                    sys::mrb_close(mrb.as_mut());
                }
            }
        }
        // Cleanup dangling pointers
        self.mrb = ptr::null_mut();
    }

    /// Create a class definition bound to a Rust type `T`. Class definitions
    /// have the same lifetime as the [`State`] because the class def owns the
    /// `mrb_data_type` for the type, which must be long-lived. Class defs are
    /// stored by [`TypeId`] of `T`.
    pub fn def_class<T>(&mut self, spec: class::Spec)
    where
        T: Any,
    {
        self.classes.insert(TypeId::of::<T>(), Box::new(spec));
    }

    /// Retrieve a class definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`State::def_class`].
    pub fn class_spec<T>(&self) -> Option<&class::Spec>
    where
        T: Any,
    {
        self.classes.get(&TypeId::of::<T>()).map(Box::as_ref)
    }

    /// Create a module definition bound to a Rust type `T`. Module definitions
    /// have the same lifetime as the [`State`]. Module defs are stored by
    /// [`TypeId`] of `T`.
    pub fn def_module<T>(&mut self, spec: module::Spec)
    where
        T: Any,
    {
        self.modules.insert(TypeId::of::<T>(), Box::new(spec));
    }

    /// Retrieve a module definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`State::def_module`].
    pub fn module_spec<T>(&self) -> Option<&module::Spec>
    where
        T: Any,
    {
        self.modules.get(&TypeId::of::<T>()).map(Box::as_ref)
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = f.debug_struct("State");
        fmt.field("mrb", &sys::mrb_sys_state_debug(self.mrb))
            .field("parser", &self.parser)
            .field("classes", &self.classes)
            .field("modules", &self.modules)
            .field("vfs", &self.vfs)
            .field("active_regexp_globals", &self.active_regexp_globals)
            .field("output", &self.output.backend_name());
        #[cfg(feature = "artichoke-random")]
        fmt.field("prng", &self.prng);
        fmt.finish()
    }
}
