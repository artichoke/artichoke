use rand::distributions::Alphanumeric;
use rand::{self, Rng, RngCore};
use std::convert::TryFrom;
use uuid::Uuid;

use crate::extn::prelude::*;

pub mod mruby;
pub mod trampoline;

const DEFAULT_REQUESTED_BYTES: usize = 16;

#[cfg(test)]
mod tests {
    use rand::CryptoRng;

    fn rng_must_be_cryptographically_secure<T: CryptoRng>(_rng: T) {}

    #[test]
    fn rand_thread_rng_must_be_cryptographically_secure() {
        rng_must_be_cryptographically_secure(rand::thread_rng())
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecureRandom {
    _private: (),
}

impl SecureRandom {
    /// Constructs a new, default `SecureRandom`.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

pub fn random_bytes(len: Option<Int>) -> Result<Vec<u8>, Exception> {
    let len = if let Some(len) = len {
        match usize::try_from(len) {
            Ok(0) => return Ok(Vec::new()),
            Ok(len) => len,
            Err(_) => {
                return Err(ArgumentError::from("negative string size (or size too big)").into())
            }
        }
    } else {
        DEFAULT_REQUESTED_BYTES
    };
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0; len];
    rng.try_fill_bytes(&mut bytes)
        .map_err(|err| RuntimeError::from(err.to_string()))?;
    Ok(bytes)
}

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
                        message.extend(max.inspect(self).as_slice());
                        ArgumentError::from(message)
                    })?;
                    Ok(RandomNumberMax::Integer(max))
                }
            }
        } else {
            Ok(RandomNumberMax::None)
        }
    }
}

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

pub fn random_number(max: RandomNumberMax) -> Result<RandomNumber, ArgumentError> {
    let mut rng = rand::thread_rng();
    match max {
        RandomNumberMax::Float(max) if !max.is_finite() => {
            // NOTE: MRI returns `Errno::EDOM` exception class.
            Err(ArgumentError::from("Numerical argument out of domain"))
        }
        RandomNumberMax::Float(max) if max <= 0.0 => {
            let number = rng.gen_range(0.0, 1.0);
            Ok(RandomNumber::Float(number))
        }
        RandomNumberMax::Float(max) => {
            let number = rng.gen_range(0.0, max);
            Ok(RandomNumber::Float(number))
        }
        RandomNumberMax::Integer(max) if max <= 0 => {
            let number = rng.gen_range(0.0, 1.0);
            Ok(RandomNumber::Float(number))
        }
        RandomNumberMax::Integer(max) => {
            let number = rng.gen_range(0, max);
            Ok(RandomNumber::Integer(number))
        }
        RandomNumberMax::None => {
            let number = rng.gen_range(0.0, 1.0);
            Ok(RandomNumber::Float(number))
        }
    }
}

#[inline]
pub fn hex(len: Option<Int>) -> Result<String, Exception> {
    let bytes = random_bytes(len)?;
    Ok(hex::encode(bytes))
}

#[inline]
pub fn base64(len: Option<Int>) -> Result<String, Exception> {
    let bytes = random_bytes(len)?;
    Ok(base64::encode(bytes))
}

pub fn alphanumeric(len: Option<Int>) -> Result<String, Exception> {
    let len = if let Some(len) = len {
        match usize::try_from(len) {
            Ok(0) => return Ok(String::new()),
            Ok(len) => len,
            Err(_) => {
                return Err(ArgumentError::from("negative string size (or size too big)").into())
            }
        }
    } else {
        DEFAULT_REQUESTED_BYTES
    };
    let rng = rand::thread_rng();
    let string = rng.sample_iter(Alphanumeric).take(len).collect();
    Ok(string)
}

#[must_use]
pub fn uuid() -> String {
    let uuid = Uuid::new_v4();
    let mut buf = Uuid::encode_buffer();
    let enc = uuid.to_hyphenated().encode_lower(&mut buf);
    String::from(enc)
}
