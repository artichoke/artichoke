use crate::extn::core::random::{self, Random};
use crate::extn::prelude::*;

pub fn initialize(
    interp: &mut Artichoke,
    seed: Option<Value>,
    into: Value,
) -> Result<Value, Exception> {
    let seed = interp.try_convert_mut(seed)?;
    let rand = Random::initialize(interp, seed)?;
    let rand = Random::box_into_value(rand, into, interp)?;
    Ok(rand)
}

pub fn equal(interp: &mut Artichoke, mut rand: Value, other: Value) -> Result<Value, Exception> {
    let rand = unsafe { Random::unbox_from_value(&mut rand, interp)? };
    let eql = rand.eql(interp, other)?;
    Ok(interp.convert(eql))
}

pub fn bytes(interp: &mut Artichoke, mut rand: Value, size: Value) -> Result<Value, Exception> {
    let mut rand = unsafe { Random::unbox_from_value(&mut rand, interp)? };
    let size = size.implicitly_convert_to_int(interp)?;
    let buf = rand.bytes(interp, size)?;
    Ok(interp.convert_mut(buf))
}

pub fn rand(
    interp: &mut Artichoke,
    mut rand: Value,
    max: Option<Value>,
) -> Result<Value, Exception> {
    let mut rand = unsafe { Random::unbox_from_value(&mut rand, interp)? };
    let max = interp.try_convert_mut(max)?;
    let num = rand.rand(interp, max)?;
    Ok(interp.convert_mut(num))
}

pub fn seed(interp: &mut Artichoke, mut rand: Value) -> Result<Value, Exception> {
    let rand = unsafe { Random::unbox_from_value(&mut rand, interp)? };
    let seed = rand.seed(interp)?;
    Ok(interp.convert(seed))
}

pub fn new_seed(interp: &mut Artichoke) -> Result<Value, Exception> {
    let seed = Random::new_seed();
    Ok(interp.convert(seed))
}

pub fn srand(interp: &mut Artichoke, seed: Option<Value>) -> Result<Value, Exception> {
    let seed = interp.try_convert_mut(seed)?;
    let old_seed = random::srand(interp, seed)?;
    Ok(interp.convert(old_seed))
}

pub fn urandom(interp: &mut Artichoke, size: Value) -> Result<Value, Exception> {
    let size = size.implicitly_convert_to_int(interp)?;
    let buf = random::urandom(interp, size)?;
    Ok(interp.convert_mut(buf))
}
