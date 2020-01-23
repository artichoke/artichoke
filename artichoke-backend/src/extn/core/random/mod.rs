use artichoke_core::value::Value as _;
use rand::{self, Rng, RngCore};
use std::convert::TryFrom;
use std::ptr;

use crate::extn::prelude::*;

pub mod backend;
pub mod mruby;

#[must_use]
pub fn new(seed: Option<u64>) -> Random {
    Random(backend::rand::new(seed))
}

pub struct Random(Box<dyn backend::Rand>);

impl Default for Random {
    #[must_use]
    fn default() -> Self {
        Self(backend::default::new())
    }
}

impl Random {
    #[inline]
    fn inner(&self) -> &dyn backend::Rand {
        self.0.as_ref()
    }

    #[inline]
    fn inner_mut(&mut self) -> &mut dyn backend::Rand {
        self.0.as_mut()
    }
}

impl RustBackedValue for Random {
    #[must_use]
    fn ruby_type_name() -> &'static str {
        "Random"
    }
}

pub fn initialize(
    interp: &mut Artichoke,
    seed: Option<Value>,
    into: Option<sys::mrb_value>,
) -> Result<Value, Exception> {
    let rand = if let Some(seed) = seed {
        let seed = seed.implicitly_convert_to_int(interp)?;
        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        Random(backend::rand::new(Some(seed as u64)))
    } else {
        Random(backend::rand::new(None))
    };
    let result = rand
        .try_into_ruby(interp, into)
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Random with Rust Random"))?;
    Ok(result)
}

pub fn eql(interp: &mut Artichoke, rand: Value, other: Value) -> Result<Value, Exception> {
    if let Ok(rand) = unsafe { Random::try_from_ruby(interp, &rand) } {
        if let Ok(other) = unsafe { Random::try_from_ruby(interp, &other) } {
            if ptr::eq(rand.as_ref(), other.as_ref()) {
                Ok(interp.convert(true))
            } else {
                let this_seed = rand.borrow().inner().seed(interp)?;
                let other_seed = other.borrow().inner().seed(interp)?;
                Ok(interp.convert(this_seed == other_seed))
            }
        } else {
            Ok(interp.convert(false))
        }
    } else {
        Err(Exception::from(Fatal::new(
            interp,
            "Failed to extract Rust Random from Ruby Random receiver",
        )))
    }
}

pub fn bytes(interp: &mut Artichoke, rand: Value, size: Value) -> Result<Value, Exception> {
    let rand = if let Ok(rand) = unsafe { Random::try_from_ruby(interp, &rand) } {
        rand
    } else {
        return Err(Exception::from(Fatal::new(
            interp,
            "Failed to extract Rust Random from Ruby Random receiver",
        )));
    };
    let size = size.implicitly_convert_to_int(interp)?;
    if let Ok(size) = usize::try_from(size) {
        let mut buf = vec![0; size];
        let mut borrow = rand.borrow_mut();
        borrow.inner_mut().bytes(interp, buf.as_mut_slice())?;
        Ok(interp.convert(buf))
    } else {
        Err(Exception::from(ArgumentError::new(
            interp,
            "negative string size (or size too big)",
        )))
    }
}

pub fn rand(interp: &mut Artichoke, rand: Value, max: Option<Value>) -> Result<Value, Exception> {
    #[derive(Debug, Clone, Copy)]
    enum Max {
        Float(Float),
        Int(Int),
        None,
    }
    let rand = if let Ok(rand) = unsafe { Random::try_from_ruby(interp, &rand) } {
        rand
    } else {
        return Err(Exception::from(Fatal::new(
            interp,
            "Failed to extract Rust Random from Ruby Random receiver",
        )));
    };
    let max = if let Some(max) = max {
        if let Ok(max) = max.try_into::<Int>(interp) {
            Max::Int(max)
        } else if let Ok(max) = max.try_into::<Float>(interp) {
            Max::Float(max)
        } else {
            Max::Int(max.implicitly_convert_to_int(interp)?)
        }
    } else {
        Max::None
    };
    match max {
        Max::Float(max) if max < 0.0 => Err(Exception::from(ArgumentError::new(
            interp,
            format!("invalid argument - {}", max),
        ))),
        Max::Float(max) if max == 0.0 => {
            let mut borrow = rand.borrow_mut();
            let number = borrow.inner_mut().rand_float(interp, None)?;
            Ok(interp.convert(number))
        }
        Max::Float(max) => {
            let mut borrow = rand.borrow_mut();
            let number = borrow.inner_mut().rand_float(interp, Some(max))?;
            Ok(interp.convert(number))
        }
        Max::Int(max) if max < 1 => Err(Exception::from(ArgumentError::new(
            interp,
            format!("invalid argument - {}", max),
        ))),
        Max::Int(max) => {
            let mut borrow = rand.borrow_mut();
            let number = borrow.inner_mut().rand_int(interp, max)?;
            Ok(interp.convert(number))
        }
        Max::None => {
            let mut borrow = rand.borrow_mut();
            let number = borrow.inner_mut().rand_float(interp, None)?;
            Ok(interp.convert(number))
        }
    }
}

pub fn seed(interp: &mut Artichoke, rand: Value) -> Result<Value, Exception> {
    if let Ok(rand) = unsafe { Random::try_from_ruby(interp, &rand) } {
        let borrow = rand.borrow();
        let seed = borrow.inner().seed(interp)?;
        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        Ok(interp.convert(seed as Int))
    } else {
        Err(Exception::from(Fatal::new(
            interp,
            "Failed to extract Rust Random from Ruby Random receiver",
        )))
    }
}

pub fn new_seed(interp: &mut Artichoke) -> Result<Value, Exception> {
    let mut rng = rand::thread_rng();
    let result = rng.gen::<Int>();
    Ok(interp.convert(result))
}

pub fn srand(interp: &mut Artichoke, number: Option<Value>) -> Result<Value, Exception> {
    let _ = number;
    let new_seed = if let Some(number) = number {
        let new_seed = number.implicitly_convert_to_int(interp)?;
        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        Some(new_seed as u64)
    } else {
        None
    };
    let prng = interp.state_mut().prng_mut();
    let old_seed = prng.inner().seed(interp)?;
    prng.0 = backend::rand::new(new_seed);
    #[allow(clippy::cast_possible_wrap)]
    Ok(interp.convert(old_seed as Int))
}

pub fn urandom(interp: &mut Artichoke, size: Value) -> Result<Value, Exception> {
    let size = size.implicitly_convert_to_int(interp)?;
    let size = usize::try_from(size)
        .map_err(|_| ArgumentError::new(interp, "negative string size (or size too big)"))?;
    let mut bytes = vec![0; size];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(bytes.as_mut_slice());
    Ok(interp.convert(bytes))
}
