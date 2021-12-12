#![allow(clippy::too_many_lines)]

use crate::extn::prelude::*;

pub mod array;
pub mod artichoke;
pub mod basicobject;
pub mod comparable;
pub mod enumerable;
pub mod enumerator;
#[cfg(feature = "core-env")]
pub mod env;
pub mod exception;
pub mod falseclass;
pub mod float;
pub mod hash;
pub mod integer;
pub mod kernel;
#[cfg(feature = "core-regexp")]
pub mod matchdata;
#[cfg(feature = "core-math")]
pub mod math;
pub mod method;
pub mod module;
pub mod nilclass;
pub mod numeric;
pub mod object;
pub mod proc;
#[cfg(feature = "core-random")]
pub mod random;
pub mod range;
#[cfg(feature = "core-regexp")]
pub mod regexp;
pub mod string;
pub mod symbol;
pub mod thread;
#[cfg(feature = "core-time")]
pub mod time;
pub mod trueclass;
pub mod warning;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    // These core classes are ordered according to the dependency DAG between
    // them.
    enumerable::init(interp)?;
    // `Array` depends on: `Enumerable`
    array::mruby::init(interp)?;
    module::init(interp)?;
    // Some `Exception`s depend on: `attr_accessor` (defined in `Module`)
    exception::mruby::init(interp)?;
    comparable::init(interp)?;
    symbol::mruby::init(interp)?;
    artichoke::init(interp)?;
    enumerator::init(interp)?;
    #[cfg(feature = "core-env")]
    env::mruby::init(interp)?;
    hash::init(interp)?;
    numeric::mruby::init(interp)?;
    integer::mruby::init(interp)?;
    float::mruby::init(interp)?;
    kernel::mruby::init(interp)?;
    #[cfg(feature = "core-regexp")]
    matchdata::mruby::init(interp)?;
    #[cfg(feature = "core-math")]
    math::mruby::init(interp)?;
    method::init(interp)?;
    module::init(interp)?;
    object::init(interp)?;
    proc::init(interp)?;
    trueclass::init(interp)?;
    falseclass::init(interp)?;
    nilclass::init(interp)?;
    basicobject::init(interp)?;
    #[cfg(feature = "core-random")]
    random::mruby::init(interp)?;
    range::init(interp)?;
    #[cfg(feature = "core-regexp")]
    regexp::mruby::init(interp)?;
    string::mruby::init(interp)?;
    thread::init(interp)?;
    #[cfg(feature = "core-time")]
    time::mruby::init(interp)?;
    warning::init(interp)?;
    Ok(())
}
