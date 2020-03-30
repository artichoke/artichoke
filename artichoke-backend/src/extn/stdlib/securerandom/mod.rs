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
    fn rng_must_be_cryptographically_secure<T>(_rng: T)
    where
        T: rand::CryptoRng,
    {
    }

    #[test]
    fn rand_thread_rng_must_be_cryptographically_secure() {
        rng_must_be_cryptographically_secure(rand::thread_rng())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SecureRandom;

pub fn random_bytes(interp: &mut Artichoke, len: Option<Value>) -> Result<Vec<u8>, Exception> {
    let len = if let Some(len) = len {
        let len = len.implicitly_convert_to_int()?;
        match usize::try_from(len) {
            Ok(0) => return Ok(Vec::new()),
            Ok(len) => len,
            Err(_) => {
                return Err(Exception::from(ArgumentError::new(
                    interp,
                    "negative string size (or size too big)",
                )))
            }
        }
    } else {
        DEFAULT_REQUESTED_BYTES
    };
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0; len];
    rng.try_fill_bytes(&mut bytes)
        .map_err(|err| RuntimeError::new(interp, err.to_string()))?;
    Ok(bytes)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RandomNumberMax {
    Float(Float),
    Integer(Int),
    None,
}

impl TryConvert<Value, RandomNumberMax> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, max: Value) -> Result<RandomNumberMax, Self::Error> {
        let optional: Option<Value> = self.try_convert(max)?;
        self.try_convert(optional)
    }
}

impl TryConvert<Option<Value>, RandomNumberMax> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, max: Option<Value>) -> Result<RandomNumberMax, Self::Error> {
        if let Some(max) = max {
            match max.ruby_type() {
                Ruby::Fixnum => {
                    let max = max.try_into()?;
                    Ok(RandomNumberMax::Integer(max))
                }
                Ruby::Float => {
                    let max = max.try_into()?;
                    Ok(RandomNumberMax::Float(max))
                }
                _ => {
                    let max = max.implicitly_convert_to_int().map_err(|_| {
                        let mut message = b"invalid argument - ".to_vec();
                        message.extend(max.inspect().as_slice());
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RandomNumber {
    Integer(Int),
    Float(Float),
}

impl ConvertMut<RandomNumber, Value> for Artichoke {
    fn convert_mut(&mut self, from: RandomNumber) -> Value {
        match from {
            RandomNumber::Integer(num) => self.convert(num),
            RandomNumber::Float(num) => self.convert_mut(num),
        }
    }
}

pub fn random_number(
    interp: &mut Artichoke,
    max: RandomNumberMax,
) -> Result<RandomNumber, ArgumentError> {
    let mut rng = rand::thread_rng();
    match max {
        RandomNumberMax::Float(max) if !max.is_finite() => {
            // NOTE: MRI returns `Errno::EDOM` exception class.
            Err(ArgumentError::new(
                interp,
                "Numerical argument out of domain",
            ))
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
pub fn hex(interp: &mut Artichoke, len: Option<Value>) -> Result<String, Exception> {
    let bytes = random_bytes(interp, len)?;
    Ok(hex::encode(bytes))
}

#[inline]
pub fn base64(interp: &mut Artichoke, len: Option<Value>) -> Result<String, Exception> {
    let bytes = random_bytes(interp, len)?;
    Ok(base64::encode(bytes))
}

pub fn alphanumeric(interp: &mut Artichoke, len: Option<Value>) -> Result<String, Exception> {
    let len = if let Some(len) = len {
        let len = len.implicitly_convert_to_int()?;
        match usize::try_from(len) {
            Ok(0) => return Ok(String::new()),
            Ok(len) => len,
            Err(_) => {
                return Err(Exception::from(ArgumentError::new(
                    interp,
                    "negative string size (or size too big)",
                )))
            }
        }
    } else {
        DEFAULT_REQUESTED_BYTES
    };
    let rng = rand::thread_rng();
    let string = rng.sample_iter(Alphanumeric).take(len).collect();
    Ok(string)
}

pub fn uuid(interp: &mut Artichoke) -> String {
    let _ = interp;
    let uuid = Uuid::new_v4();
    let mut buf = Uuid::encode_buffer();
    let enc = uuid.to_hyphenated().encode_lower(&mut buf);
    enc.to_owned()
}
