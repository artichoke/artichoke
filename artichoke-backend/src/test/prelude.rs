//! A "prelude" for writing tests in the `artichoke-backend` crate.
//!
//! This prelude is similar to the standard library's prelude in that you'll
//! almost always want to import its entire contents, but unlike the standard
//! library's prelude, you'll have to do so manually:
//!
//! ```
//! use artichoke_backend::test::prelude::*;
//! ```
//!
//! The prelude may grow over time as additional items see ubiquitous use.

pub use crate::block::Block;
pub use crate::class;
pub use crate::convert::{BoxUnboxVmValue, HeapAllocatedData};
pub use crate::core::{Regexp as _, Value as _, *};
pub use crate::def::{self, EnclosingRubyScope, NotDefinedError};
pub use crate::error;
pub use crate::ffi::InterpreterExtractError;
pub use crate::module;
pub use crate::prelude::*;
pub use crate::state::parser::Context;
pub use crate::string::{format_unicode_debug_into, WriteError};
pub use crate::sys;
pub use crate::value::Value;

// This type has a custom `Drop` implementation that automatically closes the
// `Artichoke` interpreter in tests.
//
// See https://github.com/artichoke/artichoke/issues/930 for rationale of this
// type.
pub struct AutoDropArtichoke(Option<Artichoke>);

impl std::ops::Deref for AutoDropArtichoke {
    type Target = Artichoke;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl std::ops::DerefMut for AutoDropArtichoke {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

impl Drop for AutoDropArtichoke {
    fn drop(&mut self) {
        if let Some(interp) = self.0.take() {
            interp.close();
        }
    }
}

// This function returns a wrapper around the `Artichoke` interpreter that has a
// custom `Drop` implementation that automatically closes the `Artichoke`
// interpreter in tests.
//
// See https://github.com/artichoke/artichoke/issues/930 for rationale of this
// constructor.
pub fn interpreter() -> Result<AutoDropArtichoke, Error> {
    let interp = crate::interpreter()?;
    Ok(AutoDropArtichoke(Some(interp)))
}
