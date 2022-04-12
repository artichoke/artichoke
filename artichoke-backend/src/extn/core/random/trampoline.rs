//! Glue between mruby FFI and `ENV` Rust implementation.

use super::{Random, Rng, Seed};
use crate::convert::implicitly_convert_to_int;
use crate::extn::prelude::*;

pub fn initialize(interp: &mut Artichoke, seed: Option<Value>, into: Value) -> Result<Value, Error> {
    let seed: Seed = interp.try_convert_mut(seed)?;
    let random = Random::with_array_seed(seed.to_mt_seed())?;
    let random = Rng::Value(Box::new(random));
    let random = Rng::box_into_value(random, into, interp)?;
    Ok(random)
}

pub fn equal(interp: &mut Artichoke, mut rand: Value, mut other: Value) -> Result<Value, Error> {
    let random = unsafe { Rng::unbox_from_value(&mut rand, interp)? };
    let other = unsafe { Rng::unbox_from_value(&mut other, interp)? };
    let eql = *random == *other;
    Ok(interp.convert(eql))
}

pub fn bytes(interp: &mut Artichoke, mut rand: Value, size: Value) -> Result<Value, Error> {
    let mut random = unsafe { Rng::unbox_from_value(&mut rand, interp)? };
    let size = implicitly_convert_to_int(interp, size)?;
    let buf = match &mut *random {
        Rng::Global => interp.prng_mut()?.bytes(size)?,
        Rng::Value(random) => random.bytes(size)?,
    };
    interp.try_convert_mut(buf)
}

pub fn rand(interp: &mut Artichoke, mut rand: Value, max: Option<Value>) -> Result<Value, Error> {
    let mut random = unsafe { Rng::unbox_from_value(&mut rand, interp)? };
    let max = interp.try_convert_mut(max)?;
    let num = match &mut *random {
        Rng::Global => interp.prng_mut()?.rand(max)?,
        Rng::Value(random) => random.rand(max)?,
    };
    Ok(interp.convert_mut(num))
}

pub fn seed(interp: &mut Artichoke, mut rand: Value) -> Result<Value, Error> {
    let random = unsafe { Rng::unbox_from_value(&mut rand, interp)? };
    let seed = match &*random {
        Rng::Global => interp.prng()?.seed(),
        Rng::Value(random) => random.seed(),
    };
    Ok(interp.convert(seed))
}

pub fn new_seed(interp: &mut Artichoke) -> Result<Value, Error> {
    let seed = super::new_seed()?;
    Ok(interp.convert(seed))
}

pub fn srand(interp: &mut Artichoke, seed: Option<Value>) -> Result<Value, Error> {
    let seed = interp.try_convert_mut(seed)?;
    let old_seed = super::srand(interp, seed)?;
    Ok(interp.convert(old_seed))
}

pub fn urandom(interp: &mut Artichoke, size: Value) -> Result<Value, Error> {
    let size = implicitly_convert_to_int(interp, size)?;
    let buf = super::urandom(size)?;
    interp.try_convert_mut(buf)
}
