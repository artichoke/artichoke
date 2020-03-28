use crate::extn::core::integer::trampoline;
use crate::extn::core::integer::Integer;
use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().class_spec::<Integer>().is_some() {
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
    interp.0.borrow_mut().def_class::<Integer>(spec);
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
    let value = Value::new(&interp, slf);
    let encoding = encoding.map(|encoding| Value::new(&interp, encoding));
    let result = trampoline::chr(&mut interp, value, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_integer_element_reference(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let bit = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let bit = Value::new(&interp, bit);
    let result = trampoline::element_reference(&mut interp, value, bit);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_integer_div(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let denominator = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let denominator = Value::new(&interp, denominator);
    let result = trampoline::div(&mut interp, value, denominator);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_integer_size(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let result = trampoline::size(&interp);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}
