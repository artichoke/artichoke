use std::borrow::Cow;

use crate::extn::prelude::*;

pub mod backend;
pub mod mruby;

use backend::EnvType;

pub struct Environ(Box<dyn EnvType>);

impl RustBackedValue for Environ {
    fn ruby_type_name() -> &'static str {
        "Artichoke::Environ"
    }
}

#[cfg(feature = "artichoke-system-environ")]
pub fn initialize(interp: &Artichoke, into: Option<sys::mrb_value>) -> Result<Value, Exception> {
    let obj = Environ(Box::new(backend::system::System::new()));
    let result = obj
        .try_into_ruby(&interp, into)
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby ENV with Rust ENV"))?;
    Ok(result)
}

#[cfg(not(feature = "artichoke-system-environ"))]
pub fn initialize(interp: &Artichoke, into: Option<sys::mrb_value>) -> Result<Value, Exception> {
    let obj = Environ(Box::new(backend::memory::Memory::new()));
    let result = obj
        .try_into_ruby(&interp, into)
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby ENV with Rust ENV"))?;
    Ok(result)
}

pub fn element_reference(interp: &Artichoke, obj: Value, name: &Value) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let ruby_type = name.pretty_name();
    let name = if let Ok(name) = name.clone().try_into::<&[u8]>() {
        name
    } else if let Ok(name) = name.funcall::<&[u8]>("to_str", &[], None) {
        name
    } else {
        return Err(Exception::from(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", ruby_type),
        )));
    };
    let env = obj.borrow();
    let result = env.0.get(interp, name)?;
    Ok(interp.convert(result.as_ref().map(Cow::as_ref)))
}

pub fn element_assignment(
    interp: &Artichoke,
    obj: Value,
    name: &Value,
    value: Value,
) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let name_type_name = name.pretty_name();
    let name = if let Ok(name) = name.clone().try_into::<&[u8]>() {
        name
    } else if let Ok(name) = name.funcall::<&[u8]>("to_str", &[], None) {
        name
    } else {
        return Err(Exception::from(TypeError::new(
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
        return Err(Exception::from(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", value_type_name),
        )));
    };
    obj.borrow_mut().0.put(interp, name, value)?;
    // Return original object, even if we converted it to a `String`.
    Ok(interp.convert(value))
}

pub fn to_h(interp: &Artichoke, obj: Value) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let result = obj.borrow().0.as_map(interp)?;
    Ok(interp.convert(result))
}
