use artichoke_core::value::Value as _;
use rand::{self, Rng, RngCore};
use std::convert::TryFrom;
use std::ptr;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{ArgumentError, Fatal, RubyException};
use crate::sys;
use crate::types::{Float, Int};
use crate::value::Value;
use crate::Artichoke;

pub mod backend;
pub mod mruby;

pub fn new(seed: Option<u64>) -> Random {
    Random(backend::rand::new(seed))
}

pub fn default() -> Random {
    Random(backend::default::new())
}

pub struct Random(Box<dyn backend::Rand>);

impl Random {
    fn inner(&self) -> &dyn backend::Rand {
        self.0.as_ref()
    }

    fn inner_mut(&mut self) -> &mut dyn backend::Rand {
        self.0.as_mut()
    }
}

impl RustBackedValue for Random {
    fn ruby_type_name() -> &'static str {
        "Random"
    }
}

pub fn initialize(
    interp: &Artichoke,
    seed: Option<Value>,
    into: Option<sys::mrb_value>,
) -> Result<Value, Box<dyn RubyException>> {
    let rand = if let Some(seed) = seed {
        let seed = seed.implicitly_convert_to_int()?;
        Random(backend::rand::new(Some(seed as u64)))
    } else {
        Random(backend::rand::new(None))
    };
    let result = unsafe { rand.try_into_ruby(&interp, into) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Random with Rust Random"))?;
    Ok(result)
}

pub fn eql(interp: &Artichoke, rand: Value, other: Value) -> Result<Value, Box<dyn RubyException>> {
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
        Err(Box::new(Fatal::new(
            interp,
            "Failed to extract Rust Random from Ruby Random receiver",
        )))
    }
}

pub fn bytes(
    interp: &Artichoke,
    rand: Value,
    size: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let rand = if let Ok(rand) = unsafe { Random::try_from_ruby(interp, &rand) } {
        rand
    } else {
        return Err(Box::new(Fatal::new(
            interp,
            "Failed to extract Rust Random from Ruby Random receiver",
        )));
    };
    let size = size.implicitly_convert_to_int()?;
    if let Ok(size) = usize::try_from(size) {
        let mut buf = vec![0; size];
        let mut borrow = rand.borrow_mut();
        borrow.inner_mut().bytes(interp, buf.as_mut_slice())?;
        Ok(interp.convert(buf))
    } else {
        Err(Box::new(ArgumentError::new(
            interp,
            "negative string size (or size too big)",
        )))
    }
}

pub fn rand(
    interp: &Artichoke,
    rand: Value,
    max: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    #[derive(Debug, Clone, Copy)]
    enum Max {
        Float(Float),
        Int(Int),
        None,
    }
    let rand = if let Ok(rand) = unsafe { Random::try_from_ruby(interp, &rand) } {
        rand
    } else {
        return Err(Box::new(Fatal::new(
            interp,
            "Failed to extract Rust Random from Ruby Random receiver",
        )));
    };
    let max = if let Some(max) = max {
        if let Ok(max) = max.clone().try_into::<Int>() {
            Max::Int(max)
        } else if let Ok(max) = max.clone().try_into::<Float>() {
            Max::Float(max)
        } else {
            Max::Int(max.implicitly_convert_to_int()?)
        }
    } else {
        Max::None
    };
    match max {
        Max::Float(max) => {
            if max < 0.0 {
                return Err(Box::new(ArgumentError::new(
                    interp,
                    format!("invalid argument - {}", max),
                )));
            } else if max == 0.0 {
                let mut borrow = rand.borrow_mut();
                let rand = borrow.inner_mut().rand_float(interp, None)?;
                Ok(interp.convert(rand))
            } else {
                let mut borrow = rand.borrow_mut();
                let rand = borrow.inner_mut().rand_float(interp, Some(max))?;
                Ok(interp.convert(rand))
            }
        }
        Max::Int(max) => {
            if max < 1 {
                return Err(Box::new(ArgumentError::new(
                    interp,
                    format!("invalid argument - {}", max),
                )));
            } else {
                let mut borrow = rand.borrow_mut();
                let rand = borrow.inner_mut().rand_int(interp, max)?;
                Ok(interp.convert(rand))
            }
        }
        Max::None => {
            let mut borrow = rand.borrow_mut();
            let rand = borrow.inner_mut().rand_float(interp, None)?;
            Ok(interp.convert(rand))
        }
    }
}

pub fn seed(interp: &Artichoke, rand: Value) -> Result<Value, Box<dyn RubyException>> {
    if let Ok(rand) = unsafe { Random::try_from_ruby(interp, &rand) } {
        let borrow = rand.borrow();
        let seed = borrow.inner().seed(interp)?;
        Ok(interp.convert(seed as Int))
    } else {
        Err(Box::new(Fatal::new(
            interp,
            "Failed to extract Rust Random from Ruby Random receiver",
        )))
    }
}

pub fn new_seed(interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
    let mut rng = rand::thread_rng();
    let result = rng.gen::<Int>();
    Ok(interp.convert(result))
}

pub fn srand(interp: &Artichoke, number: Option<Value>) -> Result<Value, Box<dyn RubyException>> {
    let _ = number;
    let new_seed = if let Some(number) = number {
        let new_seed = number.implicitly_convert_to_int()?;
        Some(new_seed as u64)
    } else {
        None
    };
    let mut borrow = interp.0.borrow_mut();
    let prng = borrow.prng_mut();
    let old_seed = prng.inner().seed(interp)?;
    prng.0 = backend::rand::new(new_seed);
    Ok(interp.convert(old_seed as Int))
}

pub fn urandom(interp: &Artichoke, size: Value) -> Result<Value, Box<dyn RubyException>> {
    let size = size.implicitly_convert_to_int()?;
    let size = usize::try_from(size)
        .map_err(|_| ArgumentError::new(interp, "negative string size (or size too big)"))?;
    let mut bytes = vec![0; size];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(bytes.as_mut_slice());
    Ok(interp.convert(bytes))
}
