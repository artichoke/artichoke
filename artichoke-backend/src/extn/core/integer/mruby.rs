use crate::extn::core::integer::trampoline;
use crate::extn::core::integer::Integer;
use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Integer>() {
        return Ok(());
    }
    let spec = class::Spec::new("Integer", None, None)?;
    class::Builder::for_spec(interp, &spec)
        .add_method("chr", artichoke_integer_chr, sys::mrb_args_opt(1))?
        .add_method(
            "[]",
            artichoke_integer_element_reference,
            sys::mrb_args_req(1),
        )?
        .add_method("/", artichoke_integer_div, sys::mrb_args_req(1))?
        .add_method("size", artichoke_integer_size, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<Integer>(spec)?;
    let _ = interp.eval(&include_bytes!("integer.rb")[..])?;
    trace!("Patched Integer onto interpreter");
    Ok(())
}

unsafe extern "C" fn artichoke_integer_chr(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let encoding = mrb_get_args!(mrb, optional = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(slf);
    let encoding = encoding.map(Value::from);
    let result = trampoline::chr(&mut guard, value, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_integer_element_reference(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let bit = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(slf);
    let bit = Value::from(bit);
    let result = trampoline::element_reference(&mut guard, value, bit);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_integer_div(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let denominator = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(slf);
    let denominator = Value::from(denominator);
    let result = trampoline::div(&mut guard, value, denominator);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_integer_size(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let guard = Guard::new(&mut interp);
    let result = trampoline::size(&guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}
