#![allow(clippy::too_many_lines)]

use artichoke_core::eval::Eval;

use crate::{Artichoke, ArtichokeError};

pub mod array;
pub mod artichoke;
pub mod comparable;
pub mod enumerable;
pub mod enumerator;
pub mod env;
pub mod exception;
pub mod float;
pub mod hash;
pub mod integer;
pub mod kernel;
pub mod matchdata;
pub mod method;
pub mod module;
pub mod numeric;
pub mod object;
pub mod proc;
#[cfg(feature = "artichoke-random")]
pub mod random;
pub mod range;
pub mod regexp;
pub mod string;
pub mod symbol;
pub mod thread;
pub mod warning;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    // These core classes are ordered according to the dependency DAG between
    // them.
    interp.eval(&include_bytes!("object.rb")[..])?;
    enumerable::init(interp)?;
    // `Array` depends on: `Enumerable`
    array::mruby::init(interp)?;
    module::init(interp)?;
    // Some `Exception`s depend on: `attr_accessor` (defined in `Module`)
    exception::init(interp)?;
    artichoke::init(interp)?;
    comparable::init(interp)?;
    enumerator::init(interp)?;
    env::mruby::init(interp)?;
    hash::init(interp)?;
    numeric::init(interp)?;
    integer::init(interp)?;
    float::init(interp)?;
    kernel::init(interp)?;
    matchdata::init(interp)?;
    method::init(interp)?;
    module::init(interp)?;
    object::init(interp)?;
    proc::init(interp)?;
    #[cfg(feature = "artichoke-random")]
    random::mruby::init(interp)?;
    range::init(interp)?;
    regexp::mruby::init(interp)?;
    string::init(interp)?;
    symbol::init(interp)?;
    thread::init(interp)?;
    warning::init(interp)?;
    Ok(())
}
