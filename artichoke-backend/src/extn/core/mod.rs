use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub mod array;
pub mod comparable;
pub mod enumerable;
pub mod env;
pub mod exception;
pub mod float;
pub mod hash;
pub mod integer;
pub mod kernel;
pub mod matchdata;
pub mod module;
pub mod numeric;
pub mod object;
pub mod proc;
pub mod range;
pub mod regexp;
pub mod string;
pub mod symbol;
pub mod thread;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    // These core classes are ordered according to the dependency DAG between
    // them.
    enumerable::init(interp)?;
    module::init(interp)?;
    // Some `Exception`s depend on: `attr_accessor` (defined in `Module`)
    exception::init(interp)?;
    interp.eval(include_str!("object.rb"))?;
    // `Array` depends on: `Enumerable`, `attr_accessor` (defined in `Module`)
    array::init(interp)?;
    comparable::init(interp)?;
    env::init(interp)?;
    hash::init(interp)?;
    numeric::init(interp)?;
    integer::init(interp)?;
    float::init(interp)?;
    kernel::init(interp)?;
    matchdata::init(interp)?;
    module::init(interp)?;
    object::init(interp)?;
    proc::init(interp)?;
    range::init(interp)?;
    regexp::init(interp)?;
    string::init(interp)?;
    symbol::init(interp)?;
    thread::init(interp)?;
    Ok(())
}
