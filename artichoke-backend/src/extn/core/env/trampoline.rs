//! Glue between mruby FFI and `ENV` Rust implementation.

use std::borrow::Cow;

use crate::convert::{implicitly_convert_to_nilable_string, implicitly_convert_to_string};
use crate::extn::core::env::Environ;
use crate::extn::prelude::*;

pub fn initialize(interp: &mut Artichoke, into: Value) -> Result<Value, Error> {
    let environ = Environ::new();
    let result = Environ::box_into_value(environ, into, interp)?;
    Ok(result)
}

pub fn element_reference(interp: &mut Artichoke, mut environ: Value, mut name: Value) -> Result<Value, Error> {
    let environ = unsafe { Environ::unbox_from_value(&mut environ, interp) }?;
    let name = unsafe { implicitly_convert_to_string(interp, &mut name)? };
    let result = environ.get(name)?;
    let mut result = interp.convert_mut(result.as_ref().map(Cow::as_ref));
    result.freeze(interp)?;
    Ok(result)
}

pub fn element_assignment(
    interp: &mut Artichoke,
    mut environ: Value,
    mut name: Value,
    mut value: Value,
) -> Result<Value, Error> {
    let mut environ = unsafe { Environ::unbox_from_value(&mut environ, interp) }?;
    let name = unsafe { implicitly_convert_to_string(interp, &mut name)? };
    let env_value = unsafe { implicitly_convert_to_nilable_string(interp, &mut value)? };
    environ.put(name, env_value)?;
    // Return original object, even if we converted it to a `String`.
    Ok(value)
}

pub fn to_h(interp: &mut Artichoke, mut environ: Value) -> Result<Value, Error> {
    let environ = unsafe { Environ::unbox_from_value(&mut environ, interp) }?;
    let result = environ.to_map()?;
    interp.try_convert_mut(result)
}
