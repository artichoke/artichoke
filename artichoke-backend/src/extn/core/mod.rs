#![allow(clippy::too_many_lines)]

use crate::extn::prelude::*;

pub mod array;
pub mod artichoke;
pub mod comparable;
pub mod enumerable;
pub mod enumerator;
#[cfg(feature = "core-env")]
pub mod env;
pub mod exception;
pub mod float;
pub mod hash;
pub mod integer;
pub mod kernel;
pub mod matchdata;
pub mod math;
pub mod method;
pub mod module;
pub mod numeric;
pub mod object;
pub mod proc;
#[cfg(feature = "core-random")]
pub mod random;
pub mod range;
pub mod regexp;
pub mod string;
pub mod symbol;
pub mod thread;
#[cfg(feature = "core-time")]
pub mod time;
pub mod warning;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    // These core classes are ordered according to the dependency DAG between
    // them.
    let _ = interp.eval(&include_bytes!("object.rb")[..])?;
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
    numeric::init(interp)?;
    integer::mruby::init(interp)?;
    float::init(interp)?;
    kernel::mruby::init(interp)?;
    matchdata::mruby::init(interp)?;
    math::mruby::init(interp)?;
    method::init(interp)?;
    module::init(interp)?;
    object::init(interp)?;
    proc::init(interp)?;
    #[cfg(feature = "core-random")]
    random::mruby::init(interp)?;
    range::init(interp)?;
    regexp::mruby::init(interp)?;
    string::mruby::init(interp)?;
    thread::init(interp)?;
    #[cfg(feature = "core-time")]
    time::mruby::init(interp)?;
    warning::init(interp)?;
    Ok(())
}
