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

/// Container for domain-specific interpreter state.
#[derive(Debug)]
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
    /// - [`Class`](crate::class_registry::ClassRegistry) and
    ///   [`Module`](crate::module_registry::ModuleRegistry) registries.
    /// - `Regexp` [global state](regexp::State).
    /// - [In-memory virtual filesystem](fs::Virtual).
    /// - [Ruby parser and file context](parser::State).
    /// - [Intepreter-level PRNG](Prng) (behind the `core-random` feature).
    /// - [IO capturing](output::Strategy) strategy.
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
}
