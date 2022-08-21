//! FFI glue between the Rust trampolines and the mruby C interpreter.

use std::ffi::CStr;

use crate::extn::core::time::{self, trampoline};
use crate::extn::prelude::*;

const TIME_CSTR: &CStr = qed::const_cstr_from_str!("Time\0");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<time::Time>() {
        return Ok(());
    }

    let spec = class::Spec::new("Time", TIME_CSTR, None, Some(def::box_unbox_free::<time::Time>))?;
    // NOTE: The ordering of method declarations in the builder below is the
    // same as in `Init_Time` in MRI `time.c`.
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        // Constructor
        .add_self_method("now", time_self_now, sys::mrb_args_none())?
        .add_self_method("at", time_self_at, sys::mrb_args_req_and_opt(1, 3))?
        .add_self_method("utc", time_self_mkutc, sys::mrb_args_any())?
        .add_self_method("gm", time_self_mkutc, sys::mrb_args_any())?
        .add_self_method("local", time_self_mktime, sys::mrb_args_any())?
        .add_self_method("mktime", time_self_mktime, sys::mrb_args_any())?
        // Core
        .add_method("to_i", time_to_int, sys::mrb_args_none())?
        .add_method("to_f", time_to_float, sys::mrb_args_none())?
        .add_method("to_r", time_to_rational, sys::mrb_args_none())?
        .add_method("<=>", time_cmp, sys::mrb_args_req(1))?
        .add_method("eql?", time_eql, sys::mrb_args_none())?
        .add_method("hash", time_hash, sys::mrb_args_none())?
        .add_method("initialize", time_initialize, sys::mrb_args_any())?
        .add_method(
            "initialize_copy",
            time_initialize_copy,
            sys::mrb_args_req(1),
        )?
        // Mutators and converters
        .add_method("localtime", time_mutate_to_local, sys::mrb_args_opt(1))?
        .add_method("gmtime", time_mutate_to_utc, sys::mrb_args_none())?
        .add_method("utc", time_mutate_to_utc, sys::mrb_args_none())?
        .add_method("getlocal", time_as_local, sys::mrb_args_opt(1))?
        .add_method("getgm", time_as_utc, sys::mrb_args_none())?
        .add_method("getutc", time_as_utc, sys::mrb_args_none())?
        // Inspect
        .add_method("ctime", time_asctime, sys::mrb_args_none())?
        .add_method("asctime", time_asctime, sys::mrb_args_none())?
        .add_method("to_s", time_to_string, sys::mrb_args_none())?
        .add_method("inspect", time_to_string, sys::mrb_args_none())?
        .add_method("to_a", time_to_array, sys::mrb_args_none())?
        // Math
        .add_method("+", time_plus, sys::mrb_args_req(1))?
        .add_method("-", time_minus, sys::mrb_args_req(1))?
        // Coarse math
        .add_method("succ", time_succ, sys::mrb_args_none())?
        .add_method("round", time_round, sys::mrb_args_req(1))?
        // Datetime
        .add_method("sec", time_second, sys::mrb_args_none())?
        .add_method("min", time_minute, sys::mrb_args_none())?
        .add_method("hour", time_hour, sys::mrb_args_none())?
        .add_method("mday", time_day, sys::mrb_args_none())?
        .add_method("day", time_day, sys::mrb_args_none())?
        .add_method("mon", time_month, sys::mrb_args_none())?
        .add_method("month", time_month, sys::mrb_args_none())?
        .add_method("year", time_year, sys::mrb_args_none())?
        .add_method("wday", time_weekday, sys::mrb_args_none())?
        .add_method("yday", time_year_day, sys::mrb_args_none())?
        .add_method("isdst", time_is_dst, sys::mrb_args_none())?
        .add_method("dst?", time_is_dst, sys::mrb_args_none())?
        .add_method("zone", time_zone, sys::mrb_args_none())?
        .add_method("gmtoff", time_utc_offset, sys::mrb_args_none())?
        .add_method("gmt_offset", time_utc_offset, sys::mrb_args_none())?
        .add_method("utc_offset", time_utc_offset, sys::mrb_args_none())?
        // Timezone mode
        .add_method("gmt?", time_is_utc, sys::mrb_args_none())?
        .add_method("utc?", time_is_utc, sys::mrb_args_none())?
        // Day of week
        .add_method("sunday?", time_is_sunday, sys::mrb_args_none())?
        .add_method("monday?", time_is_monday, sys::mrb_args_none())?
        .add_method("tuesday?", time_is_tuesday, sys::mrb_args_none())?
        .add_method("wednesday?", time_is_wednesday, sys::mrb_args_none())?
        .add_method("thursday?", time_is_thursday, sys::mrb_args_none())?
        .add_method("friday?", time_is_friday, sys::mrb_args_none())?
        .add_method("saturday?", time_is_saturday, sys::mrb_args_none())?
        // Unix time value
        .add_method("tv_sec", time_to_int, sys::mrb_args_none())?
        .add_method("tv_usec", time_microsecond, sys::mrb_args_none())?
        .add_method("usec", time_microsecond, sys::mrb_args_none())?
        .add_method("tv_nsec", time_nanosecond, sys::mrb_args_none())?
        .add_method("nsec", time_nanosecond, sys::mrb_args_none())?
        .add_method("subsec", time_subsec, sys::mrb_args_none())?
        // Time format
        .add_method("strftime", time_strftime, sys::mrb_args_req(1))?
        .define()?;
    interp.def_class::<time::Time>(spec)?;

    Ok(())
}

// Constructor

unsafe extern "C" fn time_self_now(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::now(&mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_self_at(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let (seconds, opt1, opt2, opt3) = mrb_get_args!(mrb, required = 1, optional = 3);
    unwrap_interpreter!(mrb, to => guard);
    let seconds = Value::from(seconds);

    let opt1 = opt1.map(Value::from);
    let opt2 = opt2.map(Value::from);
    let opt3 = opt3.map(Value::from);

    let result = trampoline::at(&mut guard, seconds, opt1, opt2, opt3);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_self_mkutc(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let args = args.iter().copied().map(Value::from);
    let result = trampoline::mkutc(&mut guard, args);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_self_mktime(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let args = args.iter().copied().map(Value::from);
    let result = trampoline::mktime(&mut guard, args);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Core

unsafe extern "C" fn time_to_int(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::to_int(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_to_float(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::to_float(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_to_rational(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::to_rational(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_cmp(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::cmp(&mut guard, time, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_eql(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::eql(&mut guard, time, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_hash(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::hash(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let args = args.iter().copied().map(Value::from);
    let result = trampoline::initialize(&mut guard, time, args);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_initialize_copy(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::initialize_copy(&mut guard, time, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Mutators and converters

unsafe extern "C" fn time_mutate_to_local(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let utc_offset = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let utc_offset = utc_offset.map(Value::from);
    let result = trampoline::mutate_to_local(&mut guard, time, utc_offset);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_mutate_to_utc(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::mutate_to_utc(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_as_local(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let utc_offset = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let utc_offset = utc_offset.map(Value::from);
    let result = trampoline::as_local(&mut guard, time, utc_offset);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_as_utc(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::as_utc(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Inspect

unsafe extern "C" fn time_asctime(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::asctime(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_to_string(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::to_string(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_to_array(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::to_array(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Math

unsafe extern "C" fn time_plus(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::plus(&mut guard, time, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_minus(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::minus(&mut guard, time, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Coarse math

unsafe extern "C" fn time_succ(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::succ(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_round(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let num_digits = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let num_digits = num_digits.map(Value::from);
    let result = trampoline::round(&mut guard, time, num_digits);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Datetime

unsafe extern "C" fn time_second(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::second(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_minute(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::minute(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_hour(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::hour(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_day(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::day(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_month(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::month(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_year(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::year(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_weekday(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::weekday(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_year_day(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::year_day(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_is_dst(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_dst(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_zone(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::timezone(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_utc_offset(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::utc_offset(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Timezone mode

unsafe extern "C" fn time_is_utc(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_utc(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Day of week

unsafe extern "C" fn time_is_sunday(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_sunday(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_is_monday(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_monday(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_is_tuesday(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_tuesday(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_is_wednesday(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_wednesday(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_is_thursday(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_thursday(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_is_friday(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_friday(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_is_saturday(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::is_saturday(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Unix time value

unsafe extern "C" fn time_microsecond(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::microsecond(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_nanosecond(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::nanosecond(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn time_subsec(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let result = trampoline::subsec(&mut guard, time);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// Time format

unsafe extern "C" fn time_strftime(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let format = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let time = Value::from(slf);
    let format = Value::from(format);
    let result = trampoline::strftime(&mut guard, time, format);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
