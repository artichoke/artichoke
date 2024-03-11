use core::hash::BuildHasherDefault;
use std::collections::hash_map::{HashMap, RandomState};

use intaglio::bytes::SymbolTable;
use mezzaluna_type_registry::Registry;
use rustc_hash::FxHasher;

use crate::class;
use crate::encoding_registry;
#[cfg(feature = "core-random")]
use crate::extn::core::random::Random;
use crate::interpreter::InterpreterAllocError;
use crate::load_path;
use crate::module;
use crate::sys;

pub mod output;
pub mod parser;

type SymbolTableHasher = BuildHasherDefault<FxHasher>;

/// Container for interpreter global state.
///
/// A Ruby interpreter requires various pieces of state to execute Ruby code. It
/// needs an object heap, type registry, symbol table, pseudorandom number
/// generator, and more.
///
/// This struct stores all of these components and allows them to be passed
/// around as one bundle. This is useful in FFI contexts because this `State`
/// can be [`Box`]ed and stored in a user data pointer.
#[derive(Debug)]
pub struct State {
    pub parser: Option<parser::State>,
    pub classes: Registry<class::Spec>,
    pub modules: Registry<module::Spec>,
    pub encodings: HashMap<u8, encoding_registry::Spec>,
    pub load_path_vfs: load_path::Adapter,
    #[cfg(feature = "core-regexp")]
    pub regexp: spinoso_regexp::State,
    pub symbols: SymbolTable<SymbolTableHasher>,
    pub output: output::Strategy,
    pub hash_builder: RandomState,
    #[cfg(feature = "core-random")]
    pub prng: Random,
}

impl State {
    /// Create a new `State`.
    ///
    /// The state is comprised of several components:
    ///
    /// - [`Class`] and [`Module`] registries.
    #[cfg_attr(feature = "core-regexp", doc = "- `Regexp` [global state][regexp-state].")]
    /// - [In-memory virtual file system].
    /// - [Ruby parser and file context].
    #[cfg_attr(feature = "core-random", doc = "- [Intepreter-level PRNG].")]
    /// - [IO capturing] strategy.
    ///
    /// # Errors
    ///
    /// If the `core-random` feature is enabled, this function may return an
    /// error if the interpreter-global pseudorandom number generator fails
    /// to initialize using the platform source of randomness.
    ///
    /// [`Class`]: crate::core::ClassRegistry
    /// [`Module`]: crate::core::ModuleRegistry
    #[cfg_attr(feature = "core-regexp", doc = "[regexp-state]: spinoso_regexp::State")]
    /// [In-memory virtual file system]: load_path
    /// [Ruby parser and file context]: parser::State
    #[cfg_attr(feature = "core-random", doc = "[Intepreter-level PRNG]: Random")]
    /// [IO capturing]: output::Strategy
    pub fn new() -> Result<Self, InterpreterAllocError> {
        Ok(Self {
            parser: None,
            classes: Registry::new(),
            modules: Registry::new(),
            encodings: HashMap::new(),
            load_path_vfs: load_path::Adapter::new(),
            #[cfg(feature = "core-regexp")]
            regexp: spinoso_regexp::State::new(),
            symbols: SymbolTable::with_hasher(SymbolTableHasher::default()),
            output: output::Strategy::new(),
            hash_builder: RandomState::new(),
            #[cfg(feature = "core-random")]
            prng: Random::new().map_err(|_| InterpreterAllocError::new())?,
        })
    }

    /// Create a new [`parser::State`] from a [`sys::mrb_state`].
    #[doc(hidden)]
    pub(crate) fn try_init_parser(&mut self, mrb: &mut sys::mrb_state) {
        if let Some(parser) = parser::State::new(mrb) {
            if let Some(old_parser) = self.parser.replace(parser) {
                old_parser.close(mrb);
            }
        }
    }
}
