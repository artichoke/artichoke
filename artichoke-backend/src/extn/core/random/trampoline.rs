use crate::extn::core::random::{self, Random};
use crate::extn::prelude::*;

pub fn initialize(
    interp: &mut Artichoke,
    seed: Option<Value>,
    into: Value,
) -> Result<Value, Exception> {
    let seed = interp.try_convert(seed)?;
    let rand = Random::initialize(interp, seed)?;
    let rand = rand.try_into_ruby(interp, Some(into.inner()))?;
    Ok(rand)
}

pub fn equal(interp: &mut Artichoke, rand: Value, other: Value) -> Result<Value, Exception> {
    let rand = unsafe { Random::try_from_ruby(interp, &rand)? };
    let borrow = rand.borrow();
    let eql = borrow.eql(interp, other);
    Ok(interp.convert(eql))
}

pub fn bytes(interp: &mut Artichoke, rand: Value, size: Value) -> Result<Value, Exception> {
    let rand = unsafe { Random::try_from_ruby(interp, &rand)? };
    let mut borrow = rand.borrow_mut();
    let size = size.implicitly_convert_to_int()?;
    let buf = borrow.bytes(interp, size)?;
    Ok(interp.convert_mut(buf))
}

pub fn rand(interp: &mut Artichoke, rand: Value, max: Option<Value>) -> Result<Value, Exception> {
    let rand = unsafe { Random::try_from_ruby(interp, &rand)? };
    let mut borrow = rand.borrow_mut();
    let max = interp.try_convert(max)?;
    let num = borrow.rand(interp, max)?;
    Ok(interp.convert_mut(num))
}

pub fn seed(interp: &mut Artichoke, rand: Value) -> Result<Value, Exception> {
    let rand = unsafe { Random::try_from_ruby(interp, &rand)? };
    let borrow = rand.borrow();
    let seed = borrow.seed(interp);
    Ok(interp.convert(seed))
}

pub fn new_seed(interp: &mut Artichoke) -> Result<Value, Exception> {
    let seed = Random::new_seed();
    Ok(interp.convert(seed))
}

pub fn srand(interp: &mut Artichoke, seed: Option<Value>) -> Result<Value, Exception> {
    let seed = interp.try_convert(seed)?;
    let old_seed = random::srand(interp, seed)?;
    Ok(interp.convert(old_seed))
}

pub fn urandom(interp: &mut Artichoke, size: Value) -> Result<Value, Exception> {
    let size = size.implicitly_convert_to_int()?;
    let buf = random::urandom(interp, size)?;
    Ok(interp.convert_mut(buf))
}
