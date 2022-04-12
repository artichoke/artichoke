//! Random provides an interface to Ruby's pseudo-random number generator, or
//! PRNG. The PRNG produces a deterministic sequence of bits which approximate
//! true randomness. The sequence may be represented by integers, floats, or
//! binary strings.
//!
//! This module implements the [`Random`] singleton object from Ruby Core.
//!
//! In Artichoke, `Random` is implemented using a modified Mersenne Twister that
//! reproduces the same byte and float sequences as the MRI implementation.
//!
//! You can use this class in your application by accessing it directly. As a
//! Core API, it is globally available:
//!
//! ```ruby
//! Random::DEFAULT.bytes(16)
//! r = Random.new(33)
//! r.rand
//! ```
//!
//! [`Random`]: https://ruby-doc.org/core-2.6.3/Random.html

use core::mem;

use spinoso_random::{InitializeError, NewSeedError, UrandomError};
#[doc(inline)]
pub use spinoso_random::{Max, Rand, Random};

use crate::convert::{implicitly_convert_to_int, HeapAllocatedData};
use crate::extn::prelude::*;

pub mod mruby;
pub mod trampoline;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Rng {
    Global,
    Instance(Box<Random>),
}

impl HeapAllocatedData for Rng {
    const RUBY_TYPE: &'static str = "Random";
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Seed {
    New(i64),
    None,
}

impl Default for Seed {
    fn default() -> Self {
        Self::new()
    }
}

impl From<i64> for Seed {
    fn from(seed: i64) -> Seed {
        Seed::New(seed)
    }
}

impl Seed {
    /// Construct a an empty seed.
    #[must_use]
    pub const fn new() -> Self {
        Self::None
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    pub fn from_mt_seed_lossy(seed: [u32; 4]) -> Self {
        qed::const_assert_eq!(mem::size_of::<[u32; 4]>(), mem::size_of::<i128>());
        let seed = unsafe { mem::transmute::<_, i128>(seed) };

        // TODO: return a bignum instead of truncating.
        let seed_bytes = seed.to_ne_bytes();
        let mut buf = [0_u8; mem::size_of::<i64>()];
        buf.copy_from_slice(&seed_bytes[..mem::size_of::<i64>()]);
        let seed = i64::from_ne_bytes(buf);

        Self::New(seed)
    }

    #[must_use]
    pub fn to_mt_seed(self) -> Option<[u32; 4]> {
        if let Self::New(seed) = self {
            let seed = i128::from(seed);
            let seed = seed.to_le_bytes();
            let seed = spinoso_random::seed_to_key(seed);
            Some(seed)
        } else {
            None
        }
    }
}

impl TryConvert<Seed, Value> for Artichoke {
    type Error = Error;

    fn try_convert(&self, seed: Seed) -> Result<Value, Self::Error> {
        match seed {
            Seed::None => Ok(Value::nil()),
            Seed::New(seed) => self.try_convert(seed),
        }
    }
}

impl TryConvertMut<Value, Seed> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Seed, Self::Error> {
        let seed = implicitly_convert_to_int(self, value)?;
        Ok(Seed::New(seed))
    }
}

impl TryConvertMut<Option<Value>, Seed> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<Value>) -> Result<Seed, Self::Error> {
        if let Some(value) = value {
            let seed = self.try_convert_mut(value)?;
            Ok(seed)
        } else {
            Ok(Seed::None)
        }
    }
}

impl TryConvertMut<Value, Max> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, max: Value) -> Result<Max, Self::Error> {
        let optional: Option<Value> = self.try_convert(max)?;
        self.try_convert_mut(optional)
    }
}

impl TryConvertMut<Option<Value>, Max> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, max: Option<Value>) -> Result<Max, Self::Error> {
        if let Some(max) = max {
            match max.ruby_type() {
                Ruby::Fixnum => {
                    let max = max.try_convert_into(self)?;
                    Ok(Max::Integer(max))
                }
                Ruby::Float => {
                    let max = max.try_convert_into(self)?;
                    Ok(Max::Float(max))
                }
                _ => {
                    let max = implicitly_convert_to_int(self, max).map_err(|_| {
                        let mut message = b"invalid argument - ".to_vec();
                        message.extend(max.inspect(self));
                        ArgumentError::from(message)
                    })?;
                    Ok(Max::Integer(max))
                }
            }
        } else {
            Ok(Max::None)
        }
    }
}

impl ConvertMut<Rand, Value> for Artichoke {
    fn convert_mut(&mut self, from: Rand) -> Value {
        match from {
            Rand::Integer(num) => self.convert(num),
            Rand::Float(num) => self.convert_mut(num),
        }
    }
}

impl From<spinoso_random::ArgumentError> for Error {
    fn from(err: spinoso_random::ArgumentError) -> Self {
        // XXX: Should this be an `ArgumentError`?
        let err = RuntimeError::from(err.to_string());
        err.into()
    }
}

impl From<InitializeError> for Error {
    fn from(err: InitializeError) -> Self {
        let err = RuntimeError::from(err.message());
        err.into()
    }
}

impl From<NewSeedError> for Error {
    fn from(err: NewSeedError) -> Self {
        let err = RuntimeError::from(err.message());
        err.into()
    }
}

impl From<UrandomError> for Error {
    fn from(err: UrandomError) -> Self {
        let err = RuntimeError::from(err.message());
        err.into()
    }
}
