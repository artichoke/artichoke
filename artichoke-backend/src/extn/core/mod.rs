#![allow(clippy::too_many_lines)]

use crate::extn::prelude::*;

pub(crate) mod array;
pub(crate) mod artichoke;
pub(crate) mod basicobject;
pub(crate) mod comparable;
pub(crate) mod encoding;
pub(crate) mod enumerable;
pub(crate) mod enumerator;
#[cfg(feature = "core-env")]
pub(crate) mod env;
pub(crate) mod exception;
pub(crate) mod falseclass;
pub(crate) mod float;
pub(crate) mod hash;
pub(crate) mod integer;
pub(crate) mod kernel;
#[cfg(feature = "core-regexp")]
pub(crate) mod matchdata;
#[cfg(feature = "core-math")]
pub(crate) mod math;
pub(crate) mod method;
pub(crate) mod module;
pub(crate) mod nilclass;
pub(crate) mod numeric;
pub(crate) mod object;
pub(crate) mod proc;
#[cfg(feature = "core-random")]
pub(crate) mod random;
pub(crate) mod range;
#[cfg(feature = "core-regexp")]
pub(crate) mod regexp;
pub(crate) mod string;
pub(crate) mod symbol;
pub(crate) mod thread;
#[cfg(feature = "core-time")]
pub(crate) mod time;
pub(crate) mod trueclass;
pub(crate) mod warning;

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
    // `String` is reliant on `Encoding`
    encoding::mruby::init(interp)?;
    string::mruby::init(interp)?;
    thread::init(interp)?;
    #[cfg(feature = "core-time")]
    time::mruby::init(interp)?;
    warning::init(interp)?;
    Ok(())
}
