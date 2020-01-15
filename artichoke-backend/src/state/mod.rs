use intaglio::bytes::SymbolTable;

use crate::class;
use crate::fs::{self, Filesystem};
use crate::module;
use crate::sys;

pub mod output;
pub mod parser;
#[cfg(feature = "core-random")]
pub mod prng;
pub mod regexp;

#[cfg(feature = "core-random")]
use prng::Prng;

/// Container for domain-specific interpreter state.
#[derive(Default, Debug)]
pub struct State {
    pub parser: Option<parser::State>,
    pub classes: class::Registry,
    pub modules: module::Registry,
    pub vfs: Box<dyn Filesystem>,
    pub regexp: regexp::State,
    pub symbols: SymbolTable,
    pub output: output::Strategy,
    #[cfg(feature = "core-random")]
    pub prng: Prng,
}

impl State {
    /// Create a new `State`.
    ///
    /// The state is comprised of several components:
    ///
    /// - [`Class`](crate::class_registry::ClassRegistry) and
    ///   [`Module`](crate::module_registry::ModuleRegistry) registries.
    /// - `Regexp` [global state](regexp::State).
    /// - [In-memory virtual filesystem](fs).
    /// - [Ruby parser and file context](parser::State).
    /// - [Intepreter-level PRNG](Prng) (behind the `core-random` feature).
    /// - [IO capturing](output::Strategy) strategy.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parser: None,
            classes: class::Registry::new(),
            modules: module::Registry::new(),
            vfs: fs::filesystem(),
            regexp: regexp::State::new(),
            symbols: SymbolTable::new(),
            output: output::Strategy::new(),
            #[cfg(feature = "core-random")]
            prng: Prng::new(),
        }
    }

    /// Create a new [`parser::State`] from a [`sys::mrb_state`].
    pub fn try_init_parser(&mut self, mrb: &mut sys::mrb_state) {
        if let Some(parser) = parser::State::new(mrb) {
            if let Some(old_parser) = self.parser.replace(parser) {
                old_parser.close(mrb);
            }
        }
    }
}
