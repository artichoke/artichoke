use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::ptr::{self, NonNull};

use crate::class;
use crate::fs;
use crate::module;
use crate::sys;

pub mod output;
pub mod parser;
#[cfg(feature = "core-random")]
pub mod prng;
pub mod regexp;
pub mod type_registry;

#[cfg(feature = "core-random")]
use prng::Prng;
use type_registry::TypeRegistry;

// NOTE: `State` assumes that it it is stored in `mrb_state->ud` wrapped in a
// [`Rc`] with type [`Artichoke`] as created by [`crate::interpreter`].
pub struct State {
    pub parser: parser::State,
    pub classes: TypeRegistry<class::Spec>,
    pub modules: TypeRegistry<module::Spec>,
    pub vfs: fs::Virtual,
    pub regexp: regexp::State,
    pub output: output::Strategy,
    #[cfg(feature = "core-random")]
    pub prng: Prng,
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
    /// - [Intepreter-level PRNG](prng::Prng) (behind the `core-random`
    ///   feature).
    /// - IO capturing strategy.
    pub fn new(mrb: &mut sys::mrb_state) -> Option<Self> {
        let parser = parser::State::new(mrb)?;
        let state = Self {
            parser,
            classes: TypeRegistry::new(),
            modules: TypeRegistry::new(),
            vfs: fs::Virtual::new(),
            regexp: regexp::State::new(),
            output: output::Strategy::new(),
            #[cfg(feature = "core-random")]
            prng: Prng::default(),
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
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = f.debug_struct("State");
        fmt.field("mrb", &sys::mrb_sys_state_debug(self.mrb))
            .field("parser", &self.parser)
            .field("classes", &self.classes)
            .field("modules", &self.modules)
            .field("vfs", &self.vfs)
            .field("regexp", &self.regexp)
            .field("output", &self.output);
        #[cfg(feature = "core-random")]
        fmt.field("prng", &self.prng);
        fmt.finish()
    }
}
