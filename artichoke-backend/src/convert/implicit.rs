use artichoke_core::debug::Debug as _;
use artichoke_core::value::Value as _;
use core::mem;
use spinoso_exception::TypeError;

use crate::convert::BoxUnboxVmValue;
use crate::extn::core::symbol::Symbol;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

pub fn implicitly_convert_to_int(interp: &mut Artichoke, value: Value) -> Result<Int, TypeError> {
    match value.try_into::<Option<Int>>(interp) {
        Ok(Some(num)) => return Ok(num),
        Ok(None) => return Err(TypeError::with_message("no implicit conversion from nil to integer")),
        Err(_) => {}
    }
    if let Ok(true) = value.respond_to(interp, "to_int") {
        if let Ok(maybe) = value.funcall(interp, "to_int", &[], None) {
            if let Ok(num) = maybe.try_into::<Int>(interp) {
                Ok(num)
            } else {
                let mut message = String::from("can't convert ");
                let name = interp.inspect_type_name_for_value(value);
                message.push_str(name);
                message.push_str(" to Integer (");
                message.push_str(name);
                message.push_str("#to_int gives ");
                message.push_str(interp.inspect_type_name_for_value(maybe));
                message.push(')');
                Err(TypeError::from(message))
            }
        } else {
            let mut message = String::from("no implicit conversion of ");
            message.push_str(interp.inspect_type_name_for_value(value));
            message.push_str(" into Integer");
            Err(TypeError::from(message))
        }
    } else {
        let mut message = String::from("no implicit conversion of ");
        message.push_str(interp.inspect_type_name_for_value(value));
        message.push_str(" into Integer");
        Err(TypeError::from(message))
    }
}

pub unsafe fn implicitly_convert_to_string<'a>(
    interp: &mut Artichoke,
    value: &'a mut Value,
) -> Result<&'a [u8], TypeError> {
    if let Ok(string) = value.try_into_mut::<&[u8]>(interp) {
        Ok(string)
    } else if let Ok(sym) = Symbol::unbox_from_value(value, interp) {
        let bytes = sym.bytes(interp);
        // Safety:
        //
        // Symbols are valid for the lifetime of the interpreter, which is a
        // longer lifetime than that of `value`.
        //
        // This transmute shrinks the lifetime of the interned bytes to the
        // lifetime of the given `Value`.
        Ok(mem::transmute(bytes))
    } else if let Ok(true) = value.respond_to(interp, "to_str") {
        if let Ok(maybe) = value.funcall(interp, "to_str", &[], None) {
            if let Ok(string) = maybe.try_into_mut::<&[u8]>(interp) {
                Ok(string)
            } else {
                let mut message = String::from("can't convert ");
                let name = interp.inspect_type_name_for_value(*value);
                message.push_str(name);
                message.push_str(" to String (");
                message.push_str(name);
                message.push_str("#to_str gives ");
                message.push_str(interp.inspect_type_name_for_value(maybe));
                message.push(')');
                Err(TypeError::from(message))
            }
        } else {
            let mut message = String::from("no implicit conversion of ");
            message.push_str(interp.inspect_type_name_for_value(*value));
            message.push_str(" into String");
            Err(TypeError::from(message))
        }
    } else {
        let mut message = String::from("no implicit conversion of ");
        message.push_str(interp.inspect_type_name_for_value(*value));
        message.push_str(" into String");
        Err(TypeError::from(message))
    }
}

pub unsafe fn implicitly_convert_to_nilable_string<'a>(
    interp: &mut Artichoke,
    value: &'a mut Value,
) -> Result<Option<&'a [u8]>, TypeError> {
    if value.is_nil() {
        Ok(None)
    } else {
        let string = implicitly_convert_to_string(interp, value)?;
        Ok(Some(string))
    }
}
