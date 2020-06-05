use rand::{self, Rng, RngCore};
use std::convert::TryFrom;
use std::fmt;

use crate::extn::prelude::*;

pub mod backend;
pub mod mruby;
pub mod trampoline;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Seed {
    New(Int),
    None,
}

impl Seed {
    fn to_reseed(self) -> Option<u64> {
        if let Self::New(seed) = self {
            #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
            Some(seed as u64)
        } else {
            None
        }
    }
}

impl TryConvertMut<Value, Seed> for Artichoke {
    type Error = TypeError;

    fn try_convert_mut(&mut self, value: Value) -> Result<Seed, Self::Error> {
        let optional: Option<Value> = self.convert(value);
        self.try_convert_mut(optional)
    }
}

impl TryConvertMut<Option<Value>, Seed> for Artichoke {
    type Error = TypeError;

    fn try_convert_mut(&mut self, value: Option<Value>) -> Result<Seed, Self::Error> {
        if let Some(value) = value {
            let seed = value.implicitly_convert_to_int(self)?;
            Ok(Seed::New(seed))
        } else {
            Ok(Seed::None)
        }
    }
}

pub fn srand(interp: &mut Artichoke, seed: Seed) -> Result<Int, Exception> {
    let old_seed = interp.prng_seed()?;
    interp.prng_reseed(seed.to_reseed())?;
    #[allow(clippy::cast_possible_wrap)]
    Ok(old_seed as Int)
}

pub fn urandom(interp: &mut Artichoke, size: Int) -> Result<Vec<u8>, Exception> {
    match usize::try_from(size) {
        Ok(0) => Ok(Vec::new()),
        Ok(len) => {
            let mut buf = vec![0; len];
            let mut rng = rand::thread_rng();
            rng.try_fill_bytes(&mut buf)
                .map_err(|err| RuntimeError::new(interp, err.to_string()))?;
            Ok(buf)
        }
        Err(_) => Err(Exception::from(ArgumentError::new(
            interp,
            "negative string size (or size too big)",
        ))),
    }
}

pub struct Random(Box<dyn backend::RandType>);

impl fmt::Debug for Random {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Random")
            .field("backend", self.0.as_debug())
            .finish()
    }
}

impl Random {
    #[must_use]
    pub fn new(seed: Option<u64>) -> Self {
        Self(backend::rand::new(seed))
    }

    #[must_use]
    pub fn interpreter_prng_delegate() -> Self {
        Self(Box::new(backend::default::Default::default()))
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &dyn backend::RandType {
        self.0.as_ref()
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut dyn backend::RandType {
        self.0.as_mut()
    }

    #[inline]
    pub fn initialize(interp: &mut Artichoke, seed: Seed) -> Result<Self, Exception> {
        let _ = interp;
        Ok(Self(backend::rand::new(seed.to_reseed())))
    }

    pub fn eql(&self, interp: &mut Artichoke, other: Value) -> Result<bool, Exception> {
        if let Ok(other) = unsafe { Random::try_from_ruby(interp, &other) } {
            let this_seed = self.inner().seed(interp)?;
            let other_seed = other.borrow().inner().seed(interp)?;
            Ok(this_seed == other_seed)
        } else {
            Ok(false)
        }
    }

    pub fn bytes(&mut self, interp: &mut Artichoke, size: Int) -> Result<Vec<u8>, Exception> {
        match usize::try_from(size) {
            Ok(0) => Ok(Vec::new()),
            Ok(len) => {
                let mut buf = vec![0; len];
                self.inner_mut().bytes(interp, &mut buf)?;
                Ok(buf)
            }
            Err(_) => Err(Exception::from(ArgumentError::new(
                interp,
                "negative string size (or size too big)",
            ))),
        }
    }

    pub fn rand(
        &mut self,
        interp: &mut Artichoke,
        max: RandomNumberMax,
    ) -> Result<RandomNumber, Exception> {
        match max {
            RandomNumberMax::Float(max) if !max.is_finite() => {
                // NOTE: MRI returns `Errno::EDOM` exception class.
                Err(Exception::from(ArgumentError::new(
                    interp,
                    "Numerical argument out of domain",
                )))
            }
            RandomNumberMax::Float(max) if max < 0.0 => {
                let mut message = b"invalid argument - ".to_vec();
                string::write_float_into(&mut message, max)?;
                Err(Exception::from(ArgumentError::new_raw(interp, message)))
            }
            RandomNumberMax::Float(max) if max == 0.0 => {
                let number = self.inner_mut().rand_float(interp, None)?;
                Ok(RandomNumber::Float(number))
            }
            RandomNumberMax::Float(max) => {
                let number = self.inner_mut().rand_float(interp, Some(max))?;
                Ok(RandomNumber::Float(number))
            }
            RandomNumberMax::Integer(max) if max < 1 => {
                let mut message = String::from("invalid argument - ");
                string::format_int_into(&mut message, max)?;
                Err(Exception::from(ArgumentError::new(interp, message)))
            }
            RandomNumberMax::Integer(max) => {
                let number = self.inner_mut().rand_int(interp, max)?;
                Ok(RandomNumber::Integer(number))
            }
            RandomNumberMax::None => {
                let number = self.inner_mut().rand_float(interp, None)?;
                Ok(RandomNumber::Float(number))
            }
        }
    }

    #[inline]
    pub fn seed(&self, interp: &mut Artichoke) -> Result<Int, Exception> {
        let seed = self.inner().seed(interp)?;
        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        let seed = seed as Int;
        Ok(seed)
    }

    #[must_use]
    pub fn new_seed() -> Int {
        let mut rng = rand::thread_rng();
        rng.gen::<Int>()
    }
}

impl RustBackedValue for Random {
    fn ruby_type_name() -> &'static str {
        "Random"
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RandomNumberMax {
    Float(Fp),
    Integer(Int),
    None,
}

impl TryConvertMut<Value, RandomNumberMax> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, max: Value) -> Result<RandomNumberMax, Self::Error> {
        let optional: Option<Value> = self.try_convert(max)?;
        self.try_convert_mut(optional)
    }
}

impl TryConvertMut<Option<Value>, RandomNumberMax> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, max: Option<Value>) -> Result<RandomNumberMax, Self::Error> {
        if let Some(max) = max {
            match max.ruby_type() {
                Ruby::Fixnum => {
                    let max = max.try_into(self)?;
                    Ok(RandomNumberMax::Integer(max))
                }
                Ruby::Float => {
                    let max = max.try_into(self)?;
                    Ok(RandomNumberMax::Float(max))
                }
                _ => {
                    let max = max.implicitly_convert_to_int(self).map_err(|_| {
                        let mut message = b"invalid argument - ".to_vec();
                        message.extend(max.inspect(self));
                        ArgumentError::new_raw(self, message)
                    })?;
                    Ok(RandomNumberMax::Integer(max))
                }
            }
        } else {
            Ok(RandomNumberMax::None)
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RandomNumber {
    Float(Fp),
    Integer(Int),
}

impl ConvertMut<RandomNumber, Value> for Artichoke {
    fn convert_mut(&mut self, from: RandomNumber) -> Value {
        match from {
            RandomNumber::Integer(num) => self.convert(num),
            RandomNumber::Float(num) => self.convert_mut(num),
        }
    }
}
