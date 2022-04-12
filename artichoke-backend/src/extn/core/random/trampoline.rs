//! Glue between mruby FFI and `ENV` Rust implementation.

use super::{Random, Rng, Seed};
use crate::convert::implicitly_convert_to_int;
use crate::extn::prelude::*;

pub fn initialize(interp: &mut Artichoke, seed: Option<Value>, into: Value) -> Result<Value, Error> {
    let seed: Seed = interp.try_convert_mut(seed)?;
    let random = if let Some(seed) = seed.to_mt_seed() {
        Random::with_array_seed(seed)
    } else {
        Random::new()?
    };
    let random = Rng::Instance(Box::new(random));
    let random = Rng::box_into_value(random, into, interp)?;
    Ok(random)
}

pub fn equal(interp: &mut Artichoke, mut rand: Value, mut other: Value) -> Result<Value, Error> {
    let random = unsafe { Rng::unbox_from_value(&mut rand, interp)? };
    let other = unsafe { Rng::unbox_from_value(&mut other, interp)? };
    let eql = random.as_ref() == other.as_ref();
    Ok(interp.convert(eql))
}

pub fn bytes(interp: &mut Artichoke, mut rand: Value, size: Value) -> Result<Value, Error> {
    let mut random = unsafe { Rng::unbox_from_value(&mut rand, interp)? };
    let size = implicitly_convert_to_int(interp, size)?;
    let mut buf = match usize::try_from(size) {
        Ok(0) => return interp.try_convert_mut(Vec::<u8>::new()),
        Ok(len) => vec![0; len],
        Err(_) => return Err(ArgumentError::with_message("negative string size (or size too big)").into()),
    };
    match &mut *random {
        Rng::Global => interp.prng_mut()?.fill_bytes(&mut buf),
        Rng::Instance(random) => random.fill_bytes(&mut buf),
    }
    interp.try_convert_mut(buf)
}

pub fn rand(interp: &mut Artichoke, mut rand: Value, max: Option<Value>) -> Result<Value, Error> {
    let mut random = unsafe { Rng::unbox_from_value(&mut rand, interp)? };
    let max = interp.try_convert_mut(max)?;
    let num = match &mut *random {
        Rng::Global => spinoso_random::rand(interp.prng_mut()?, max)?,
        Rng::Instance(random) => spinoso_random::rand(random, max)?,
    };
    Ok(interp.convert_mut(num))
}

pub fn seed(interp: &mut Artichoke, mut rand: Value) -> Result<Value, Error> {
    let random = unsafe { Rng::unbox_from_value(&mut rand, interp)? };
    let seed = match random.as_ref() {
        Rng::Global => interp.prng()?.seed(),
        Rng::Instance(random) => random.seed(),
    };
    interp.try_convert(Seed::from_mt_seed_lossy(seed))
}

pub fn new_seed(interp: &mut Artichoke) -> Result<Value, Error> {
    let seed = spinoso_random::new_seed()?;
    let seed = Seed::from_mt_seed_lossy(seed);
    interp.try_convert(seed)
}

pub fn srand(interp: &mut Artichoke, seed: Option<Value>) -> Result<Value, Error> {
    let seed: Seed = interp.try_convert_mut(seed)?;
    let old_seed = Seed::from_mt_seed_lossy(interp.prng()?.seed());

    let new_random = if let Some(seed) = seed.to_mt_seed() {
        Random::with_array_seed(seed)
    } else {
        Random::new()?
    };
    // "Reseed" by replacing the RNG with a newly seeded one.
    let prng = interp.prng_mut()?;
    *prng = new_random;

    interp.try_convert(old_seed)
}

pub fn urandom(interp: &mut Artichoke, size: Value) -> Result<Value, Error> {
    let size = implicitly_convert_to_int(interp, size)?;
    let mut buf = match usize::try_from(size) {
        Ok(0) => return interp.try_convert_mut(Vec::<u8>::new()),
        Ok(len) => vec![0; len],
        Err(_) => return Err(ArgumentError::with_message("negative string size (or size too big)").into()),
    };
    spinoso_random::urandom(&mut buf)?;
    interp.try_convert_mut(buf)
}
