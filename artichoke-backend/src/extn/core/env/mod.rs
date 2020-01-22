use std::collections::HashMap;

use crate::extn::prelude::*;

pub mod backend;
pub mod mruby;

pub trait Env {
    fn get(&self, interp: &Artichoke, name: &[u8]) -> Result<Value, Exception>;
    fn put(
        &mut self,
        interp: &Artichoke,
        name: &[u8],
        value: Option<&[u8]>,
    ) -> Result<(), Exception>;
    fn as_map(&self, interp: &Artichoke) -> Result<HashMap<Vec<u8>, Vec<u8>>, Exception>;
}

pub struct Environ(Box<dyn Env>);

impl RustBackedValue for Environ {
    #[must_use]
    fn ruby_type_name() -> &'static str {
        "Artichoke::Environ"
    }
}

#[cfg(feature = "artichoke-system-environ")]
pub fn initialize(
    interp: &mut Artichoke,
    into: Option<sys::mrb_value>,
) -> Result<Value, Exception> {
    let obj = Environ(Box::new(backend::system::System::new()));
    let result = obj
        .try_into_ruby(interp, into)
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby ENV with Rust ENV"))?;
    Ok(result)
}

#[cfg(not(feature = "artichoke-system-environ"))]
pub fn initialize(
    interp: &mut Artichoke,
    into: Option<sys::mrb_value>,
) -> Result<Value, Exception> {
    let obj = Environ(Box::new(backend::memory::Memory::new()));
    let result = obj
        .try_into_ruby(interp, into)
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby ENV with Rust ENV"))?;
    Ok(result)
}

pub fn element_reference(
    interp: &mut Artichoke,
    obj: Value,
    name: &Value,
) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let ruby_type = name.pretty_name(interp);
    let name = if let Ok(name) = name.try_into::<&[u8]>(interp) {
        name
    } else if let Ok(name) = name.funcall::<&[u8]>(interp, "to_str", &[], None) {
        name
    } else {
        return Err(Exception::from(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", ruby_type),
        )));
    };
    let result = obj.borrow().0.get(interp, name)?;
    Ok(result)
}

pub fn element_assignment(
    interp: &mut Artichoke,
    obj: Value,
    name: &Value,
    value: Value,
) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let name_type_name = name.pretty_name(interp);
    let name = if let Ok(name) = name.try_into::<&[u8]>(interp) {
        name
    } else if let Ok(name) = name.funcall::<&[u8]>(interp, "to_str", &[], None) {
        name
    } else {
        return Err(Exception::from(TypeError::new(
            interp,
            format!("no implicit conversion of {} into String", name_type_name),
        )));
    };
    let value_type_name = value.pretty_name(interp);
    let value = if let Ok(value) = value.try_into::<Option<&[u8]>>(interp) {
        value
    } else if let Ok(value) = value.clone().funcall::<&[u8]>(interp, "to_str", &[], None) {
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

pub fn to_h(interp: &mut Artichoke, obj: Value) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }
        .map_err(|_| Fatal::new(interp, "Unable to extract Rust ENV from Ruby ENV receiver"))?;
    let result = obj.borrow().0.as_map(interp)?;
    Ok(interp.convert(result))
}
