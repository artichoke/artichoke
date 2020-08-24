//! Secure random number generator interface.
//!
//! This module implements the [`SecureRandom`] package from the Ruby Standard
//! Library. It is an interface to secure random number generators which are
//! suitable for generating session keys in HTTP cookies, etc.
//!
//! You can use this library in your application by requiring it:
//!
//! ```ruby
//! require 'securerandom'
//! ```
//!
//! This implementation of `SecureRandom` supports the system RNG via the
//! [`getrandom`] crate. This implementation does not depend on OpenSSL.
//!
//! [`SecureRandom`]: https://ruby-doc.org/stdlib-2.6.3/libdoc/securerandom/rdoc/SecureRandom.html
//! [`getrandom`]: https://crates.io/crates/getrandom

use crate::extn::core::exception as exc;
use crate::extn::prelude::*;

pub mod mruby;
pub mod trampoline;

#[doc(inline)]
pub use spinoso_securerandom::{
    alphanumeric, base64, hex, random_bytes, random_number, urlsafe_base64, uuid, ArgumentError,
    DomainError, Error as SecureRandomError, Max, RandomBytesError, RandomNumber, SecureRandom,
};

impl From<SecureRandomError> for Error {
    fn from(err: SecureRandomError) -> Self {
        match err {
            SecureRandomError::Argument(err) => err.into(),
            SecureRandomError::RandomBytes(err) => err.into(),
        }
    }
}

impl From<ArgumentError> for Error {
    fn from(err: ArgumentError) -> Self {
        exc::ArgumentError::from(err.message()).into()
    }
}

impl From<RandomBytesError> for Error {
    fn from(err: RandomBytesError) -> Self {
        RuntimeError::from(err.message()).into()
    }
}

impl From<DomainError> for Error {
    fn from(err: DomainError) -> Self {
        // TODO: MRI returns `Errno::EDOM` exception class.
        ArgumentError::from(err.message()).into()
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
                    let max = max.try_into(self)?;
                    Ok(Max::Integer(max))
                }
                Ruby::Float => {
                    let max = max.try_into(self)?;
                    Ok(Max::Float(max))
                }
                _ => {
                    let max = max.implicitly_convert_to_int(self).map_err(|_| {
                        let mut message = b"invalid argument - ".to_vec();
                        message.extend(max.inspect(self).as_slice());
                        exc::ArgumentError::from(message)
                    })?;
                    Ok(Max::Integer(max))
                }
            }
        } else {
            Ok(Max::None)
        }
    }
}

impl ConvertMut<RandomNumber, Value> for Artichoke {
    fn convert_mut(&mut self, from: RandomNumber) -> Value {
        match from {
            RandomNumber::Integer(num) => self.convert(num),
            RandomNumber::Float(num) => self.convert_mut(num),
        }
    }
}
