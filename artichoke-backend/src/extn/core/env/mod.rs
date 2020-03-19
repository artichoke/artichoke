use std::borrow::Cow;
use std::fmt;

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

impl fmt::Debug for Environ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environ")
            .field("backend", self.0.as_debug())
            .finish()
    }
}

#[cfg(feature = "core-env-system")]
pub fn initialize(interp: &Artichoke, into: Option<sys::mrb_value>) -> Result<Value, Exception> {
    use backend::system::System;

    let obj = Environ(Box::new(System::new()));
    let result = obj.try_into_ruby(&interp, into)?;
    Ok(result)
}

#[cfg(not(feature = "core-env-system"))]
pub fn initialize(interp: &Artichoke, into: Option<sys::mrb_value>) -> Result<Value, Exception> {
    use backend::memory::Memory;

    let obj = Environ(Box::new(Memory::new()));
    let result = obj.try_into_ruby(&interp, into)?;
    Ok(result)
}

pub fn element_reference(
    interp: &mut Artichoke,
    obj: Value,
    name: &Value,
) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }?;
    let name = name.implicitly_convert_to_string()?;
    let env = obj.borrow();
    let result = env.0.get(interp, name)?;
    let mut result = interp.convert_mut(result.as_ref().map(Cow::as_ref));
    result.freeze()?;
    Ok(result)
}

pub fn element_assignment(
    interp: &mut Artichoke,
    obj: Value,
    name: &Value,
    value: Value,
) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }?;
    let name = name.implicitly_convert_to_string()?;
    let env_value = value.implicitly_convert_to_nilable_string()?;
    obj.borrow_mut().0.put(interp, name, env_value)?;
    // Return original object, even if we converted it to a `String`.
    Ok(value)
}

pub fn to_h(interp: &mut Artichoke, obj: Value) -> Result<Value, Exception> {
    let obj = unsafe { Environ::try_from_ruby(interp, &obj) }?;
    let result = obj.borrow().0.as_map(interp);
    Ok(interp.convert_mut(result))
}
