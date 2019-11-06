use artichoke_core::value::Value as _;
use std::collections::HashMap;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

pub mod backend;
pub mod mruby;

pub trait Env {
    fn get(&self, interp: &Artichoke, name: &[u8]) -> Result<Value, Box<dyn RubyException>>;
    fn put(
        &mut self,
        interp: &Artichoke,
        name: &[u8],
        value: Option<&[u8]>,
    ) -> Result<Value, Box<dyn RubyException>>;
    fn as_map(
        &self,
        interp: &Artichoke,
    ) -> Result<HashMap<Vec<u8>, Vec<u8>>, Box<dyn RubyException>>;
}

pub struct ENV(Box<dyn Env>);

impl RustBackedValue for ENV {
    fn ruby_type_name() -> &'static str {
        "EnvClass"
    }
}

pub fn initialize(
    interp: &Artichoke,
    into: Option<sys::mrb_value>,
) -> Result<Value, Box<dyn RubyException>> {
    let obj = ENV(Box::new(backend::system::System::new()));
    let result = unsafe { obj.try_into_ruby(&interp, into) }
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby ENV with Rust ENV"))?;
    Ok(result)
}

pub fn element_reference(
    interp: &Artichoke,
    obj: Value,
    name: &Value,
) -> Result<Value, Box<dyn RubyException>> {
    let obj = unsafe { ENV::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let ruby_type = name.pretty_name();
    let name = if let Ok(name) = name.clone().try_into::<&[u8]>() {
        name
    } else if let Ok(name) = name.funcall::<&[u8]>("to_str", &[], None) {
        name
    } else {
        return Err(Box::new(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", ruby_type),
        )));
    };
    let result = obj.borrow().0.get(interp, name)?;
    Ok(result)
}

pub fn element_assignment(
    interp: &Artichoke,
    obj: Value,
    name: &Value,
    value: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let obj = unsafe { ENV::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let name_type_name = name.pretty_name();
    let name = if let Ok(name) = name.clone().try_into::<&[u8]>() {
        name
    } else if let Ok(name) = name.funcall::<&[u8]>("to_str", &[], None) {
        name
    } else {
        return Err(Box::new(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", name_type_name),
        )));
    };
    let value_type_name = value.pretty_name();
    let value = if let Ok(value) = value.clone().try_into::<Option<&[u8]>>() {
        value
    } else if let Ok(value) = value.clone().funcall::<&[u8]>("to_str", &[], None) {
        Some(value)
    } else {
        return Err(Box::new(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", value_type_name),
        )));
    };
    obj.borrow_mut().0.put(interp, name, value)?;
    // Return original object, even if we converted it to a `String`.
    Ok(interp.convert(value))
}

pub fn to_h(interp: &Artichoke, obj: Value) -> Result<Value, Box<dyn RubyException>> {
    let obj = unsafe { ENV::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let result = obj.borrow().0.as_map(interp)?;
    Ok(interp.convert(result))
}
