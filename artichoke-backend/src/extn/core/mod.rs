#![allow(clippy::too_many_lines)]

use crate::extn::prelude::*;

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
pub mod time;
pub mod warning;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    // These core classes are ordered according to the dependency DAG between
    // them.
    let _ = interp.eval(&include_bytes!("object.rb")[..])?;
    println!("core");
    enumerable::init(interp)?;
    println!("core");
    // `Array` depends on: `Enumerable`
    array::mruby::init(interp)?;
    println!("core");
    module::init(interp)?;
    println!("core");
    // Some `Exception`s depend on: `attr_accessor` (defined in `Module`)
    exception::init(interp)?;
    println!("core");
    artichoke::init(interp)?;
    println!("core");
    comparable::init(interp)?;
    println!("core");
    enumerator::init(interp)?;
    println!("core");
    env::mruby::init(interp)?;
    println!("core");
    hash::init(interp)?;
    println!("core");
    numeric::init(interp)?;
    println!("core");
    integer::mruby::init(interp)?;
    println!("core");
    float::init(interp)?;
    println!("core");
    kernel::mruby::init(interp)?;
    println!("core");
    matchdata::mruby::init(interp)?;
    println!("core");
    math::mruby::init(interp)?;
    println!("core");
    method::init(interp)?;
    println!("core");
    module::init(interp)?;
    println!("core");
    object::init(interp)?;
    println!("core");
    proc::init(interp)?;
    println!("core");
    #[cfg(feature = "core-random")]
    random::mruby::init(interp)?;
    println!("core");
    range::init(interp)?;
    println!("core");
    regexp::mruby::init(interp)?;
    println!("core");
    string::mruby::init(interp)?;
    println!("core");
    symbol::init(interp)?;
    println!("core");
    thread::init(interp)?;
    println!("core");
    time::mruby::init(interp)?;
    println!("core");
    warning::init(interp)?;
    println!("core");
    Ok(())
}
