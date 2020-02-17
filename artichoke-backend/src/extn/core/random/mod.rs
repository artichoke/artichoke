use rand::{self, Rng, RngCore};
use std::convert::TryFrom;
use std::fmt;
use std::ptr;

use rand::rngs::SmallRng;

use crate::extn::prelude::*;

pub mod backend;
pub mod mruby;

use backend::rand::Rand;

pub struct Random(Box<dyn backend::RandType>);

impl Random {
    #[must_use]
    pub fn new(seed: Option<u64>) -> Self {
        Self(backend::rand::new(seed))
    }

    #[must_use]
    pub fn interpreter_prng_delegate() -> Self {
        Self(Box::new(backend::default::Default::default()))
    }

    fn inner(&self) -> &dyn backend::RandType {
        self.0.as_ref()
    }

    fn inner_mut(&mut self) -> &mut dyn backend::RandType {
        self.0.as_mut()
    }
}

impl RustBackedValue for Random {
    fn ruby_type_name() -> &'static str {
        "Random"
    }
}

impl fmt::Debug for Random {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(inner) = self.inner().downcast_ref::<Rand<SmallRng>>() {
            f.debug_struct("Random")
                .field("backend_type", &"Rand<SmallRng>")
                .field("seed", &inner.seed())
                .finish()
        } else {
            f.debug_struct("Random")
                .field("backend_type", &"unknown")
                .finish()
        }
    }
}

pub fn initialize(
    interp: &Artichoke,
    seed: Option<Value>,
    into: Option<sys::mrb_value>,
) -> Result<Value, Exception> {
    let rand = if let Some(seed) = seed {
        let seed = seed.implicitly_convert_to_int()?;
        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        Random(backend::rand::new(Some(seed as u64)))
    } else {
        Random(backend::rand::new(None))
    };
    let result = rand.try_into_ruby(&interp, into)?;
    Ok(result)
}

pub fn eql(interp: &Artichoke, rand: Value, other: Value) -> Result<Value, Exception> {
    let rand = unsafe { Random::try_from_ruby(interp, &rand) }?;
    let other = unsafe { Random::try_from_ruby(interp, &other) }?;
    if ptr::eq(rand.as_ref(), other.as_ref()) {
        Ok(interp.convert(true))
    } else {
        let this_seed = rand.borrow().inner().seed(interp);
        let other_seed = other.borrow().inner().seed(interp);
        Ok(interp.convert(this_seed == other_seed))
    }
}

pub fn bytes(interp: &mut Artichoke, rand: Value, size: Value) -> Result<Value, Exception> {
    let rand = unsafe { Random::try_from_ruby(interp, &rand) }?;
    let size = size.implicitly_convert_to_int()?;
    if let Ok(size) = usize::try_from(size) {
        let mut buf = vec![0; size];
        let mut borrow = rand.borrow_mut();
        borrow.inner_mut().bytes(interp, buf.as_mut_slice());
        Ok(interp.convert_mut(buf))
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
    let rand = unsafe { Random::try_from_ruby(interp, &rand) }?;
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
        Max::Float(max) if max < 0.0 => Err(Exception::from(ArgumentError::new(
            interp,
            format!("invalid argument - {}", max),
        ))),
        Max::Float(max) if max == 0.0 => {
            let mut borrow = rand.borrow_mut();
            let number = borrow.inner_mut().rand_float(interp, None);
            Ok(interp.convert_mut(number))
        }
        Max::Float(max) => {
            let mut borrow = rand.borrow_mut();
            let number = borrow.inner_mut().rand_float(interp, Some(max));
            Ok(interp.convert_mut(number))
        }
        Max::Int(max) if max < 1 => Err(Exception::from(ArgumentError::new(
            interp,
            format!("invalid argument - {}", max),
        ))),
        Max::Int(max) => {
            let mut borrow = rand.borrow_mut();
            let number = borrow.inner_mut().rand_int(interp, max);
            Ok(interp.convert(number))
        }
        Max::None => {
            let mut borrow = rand.borrow_mut();
            let number = borrow.inner_mut().rand_float(interp, None);
            Ok(interp.convert_mut(number))
        }
    }
}

pub fn seed(interp: &Artichoke, rand: Value) -> Result<Value, Exception> {
    let rand = unsafe { Random::try_from_ruby(interp, &rand) }?;
    let borrow = rand.borrow();
    let seed = borrow.inner().seed(interp);
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    Ok(interp.convert(seed as Int))
}

pub fn new_seed(interp: &Artichoke) -> Result<Value, Exception> {
    let mut rng = rand::thread_rng();
    let result = rng.gen::<Int>();
    Ok(interp.convert(result))
}

pub fn srand(interp: &Artichoke, number: Option<Value>) -> Result<Value, Exception> {
    let new_seed = if let Some(number) = number {
        let new_seed = number.implicitly_convert_to_int()?;
        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        Some(new_seed as u64)
    } else {
        None
    };
    let mut borrow = interp.0.borrow_mut();
    let old_seed = borrow.prng.seed();
    borrow.prng.reseed(new_seed);
    #[allow(clippy::cast_possible_wrap)]
    Ok(interp.convert(old_seed as Int))
}

pub fn urandom(interp: &mut Artichoke, size: Value) -> Result<Value, Exception> {
    let size = size.implicitly_convert_to_int()?;
    let size = usize::try_from(size)
        .map_err(|_| ArgumentError::new(interp, "negative string size (or size too big)"))?;
    let mut bytes = vec![0; size];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(bytes.as_mut_slice());
    Ok(interp.convert_mut(bytes))
}
