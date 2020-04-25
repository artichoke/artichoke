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
    pub fn close(mut self, mrb: &mut sys::mrb_state) {
        self.parser.close(mrb);
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = f.debug_struct("State");
        fmt.field("parser", &self.parser)
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
