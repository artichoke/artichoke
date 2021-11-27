//! Glue between mruby FFI and `securerandom` Rust implementation.

use crate::convert::implicitly_convert_to_int;
use crate::extn::prelude::*;
use crate::extn::stdlib::securerandom;

#[inline]
pub fn alphanumeric(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Error> {
    let alpha = if let Some(len) = len {
        let len = implicitly_convert_to_int(interp, len)?;
        securerandom::alphanumeric(Some(len))?
    } else {
        securerandom::alphanumeric(None)?
    };
    let alpha = spinoso_string::String::ascii(alpha);
    spinoso_string::String::alloc_value(alpha, interp)
}

#[inline]
pub fn base64(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Error> {
    let base64 = if let Some(len) = len {
        let len = implicitly_convert_to_int(interp, len)?;
        securerandom::base64(Some(len))?
    } else {
        securerandom::base64(None)?
    };
    let base64 = spinoso_string::String::ascii(base64.into_bytes());
    spinoso_string::String::alloc_value(base64, interp)
}

#[inline]
pub fn urlsafe_base64(interp: &mut Artichoke, len: Option<Value>, padding: Option<Value>) -> Result<Value, Error> {
    let padding = match padding {
        None => false,
        Some(val) if val.is_nil() => false,
        Some(padding) => {
            // All truthy values evaluate to `true` for this argument. So either
            // `padding` is a `bool` and we can extract it, or we default to
            // `true` since only `nil` (handled above) and `false` are falsy.
            padding.try_convert_into::<bool>(interp).unwrap_or(true)
        }
    };
    let base64 = if let Some(len) = len {
        let len = implicitly_convert_to_int(interp, len)?;
        securerandom::urlsafe_base64(Some(len), padding)?
    } else {
        securerandom::urlsafe_base64(None, padding)?
    };
    let base64 = spinoso_string::String::ascii(base64.into_bytes());
    spinoso_string::String::alloc_value(base64, interp)
}

#[inline]
pub fn hex(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Error> {
    let hex = if let Some(len) = len {
        let len = implicitly_convert_to_int(interp, len)?;
        securerandom::hex(Some(len))?
    } else {
        securerandom::hex(None)?
    };
    let hex = spinoso_string::String::ascii(hex.into_bytes());
    spinoso_string::String::alloc_value(hex, interp)
}

#[inline]
pub fn random_bytes(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Error> {
    let bytes = if let Some(len) = len {
        // Upstream uses `n.to_int`, which means we must implicitly convert to
        // int.
        //
        // https://github.com/ruby/ruby/blob/v2_6_3/lib/securerandom.rb#L135
        let len = implicitly_convert_to_int(interp, len)?;
        securerandom::random_bytes(Some(len))?
    } else {
        securerandom::random_bytes(None)?
    };
    let bytes = spinoso_string::String::binary(bytes);
    spinoso_string::String::alloc_value(bytes, interp)
}

#[inline]
pub fn random_number(interp: &mut Artichoke, max: Option<Value>) -> Result<Value, Error> {
    let max = interp.try_convert_mut(max)?;
    let num = securerandom::random_number(max)?;
    interp.try_convert_mut(num)
}

#[inline]
pub fn uuid(interp: &mut Artichoke) -> Result<Value, Error> {
    let uuid = securerandom::uuid()?;
    let uuid = spinoso_string::String::ascii(uuid.into_bytes());
    spinoso_string::String::alloc_value(uuid, interp)
}
