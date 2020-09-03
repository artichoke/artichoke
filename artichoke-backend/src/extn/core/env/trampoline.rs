//! Glue between mruby FFI and `ENV` Rust implementation.

use std::borrow::Cow;

use crate::extn::core::env::Environ;
use crate::extn::prelude::*;

pub fn initialize(interp: &mut Artichoke, into: Value) -> Result<Value, Error> {
    let environ = Environ::new();
    let result = Environ::box_into_value(environ, into, interp)?;
    Ok(result)
}

pub fn element_reference(
    interp: &mut Artichoke,
    mut environ: Value,
    mut name: Value,
) -> Result<Value, Error> {
    let environ = unsafe { Environ::unbox_from_value(&mut environ, interp) }?;
    let name = name.implicitly_convert_to_string(interp)?;
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
    let name = name.implicitly_convert_to_string(interp)?;
    let env_value = value.implicitly_convert_to_nilable_string(interp)?;
    environ.put(name, env_value)?;
    // Return original object, even if we converted it to a `String`.
    Ok(value)
}

pub fn to_h(interp: &mut Artichoke, mut environ: Value) -> Result<Value, Error> {
    let environ = unsafe { Environ::unbox_from_value(&mut environ, interp) }?;
    let result = environ.to_map()?;
    Ok(interp.convert_mut(result))
}
