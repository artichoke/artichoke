use crate::extn::core::time::{self, trampoline};
use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().class_spec::<time::Time>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Time", None, Some(def::rust_data_free::<time::Time>))?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_self_method("now", artichoke_time_self_now, sys::mrb_args_none())?
        .add_method("day", artichoke_time_day, sys::mrb_args_none())?
        .add_method("hour", artichoke_time_hour, sys::mrb_args_none())?
        .add_method("min", artichoke_time_minute, sys::mrb_args_none())?
        .add_method("mon", artichoke_time_month, sys::mrb_args_none())?
        .add_method("month", artichoke_time_month, sys::mrb_args_none())?
        .add_method("nsec", artichoke_time_nanosecond, sys::mrb_args_none())?
        .add_method("sec", artichoke_time_second, sys::mrb_args_none())?
        .add_method("tv_nsec", artichoke_time_nanosecond, sys::mrb_args_none())?
        .add_method("tv_sec", artichoke_time_second, sys::mrb_args_none())?
        .add_method("tv_usec", artichoke_time_microsecond, sys::mrb_args_none())?
        .add_method("usec", artichoke_time_microsecond, sys::mrb_args_none())?
        .add_method("wday", artichoke_time_weekday, sys::mrb_args_none())?
        .add_method("yday", artichoke_time_year_day, sys::mrb_args_none())?
        .add_method("year", artichoke_time_year, sys::mrb_args_none())?
        .define()?;
    interp.state_mut().def_class::<time::Time>(spec);

    let _ = interp.eval(&include_bytes!("time.rb")[..])?;
    trace!("Patched Random onto interpreter");
    Ok(())
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_self_now(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let result = trampoline::now(&interp);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_day(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::day(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_hour(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::hour(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_minute(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::minute(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_month(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::month(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_nanosecond(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::nanosecond(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_second(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::second(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_microsecond(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::microsecond(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_weekday(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::weekday(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_year_day(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::year_day(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_time_year(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let time = Value::new(&interp, slf);
    let result = trampoline::year(&interp, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}
