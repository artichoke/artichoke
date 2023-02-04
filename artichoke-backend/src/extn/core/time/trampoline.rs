//! Glue between mruby FFI and `Time` Rust implementation.

use spinoso_time::strftime::{
    Error::{FormattedStringTooLarge, InvalidFormatString, WriteZero},
    ASCTIME_FORMAT_STRING,
};

use crate::convert::implicitly_convert_to_int;
use crate::convert::to_str;
use crate::extn::core::string::{Encoding, String};
use crate::extn::core::time::{args::Args, subsec::Subsec, Offset, Time};
use crate::extn::prelude::*;

// Constructor
pub fn now(interp: &mut Artichoke) -> Result<Value, Error> {
    let now = Time::now()?;
    let result = Time::alloc_value(now, interp)?;
    Ok(result)
}

pub fn at(
    interp: &mut Artichoke,
    seconds: Value,
    first: Option<Value>,
    second: Option<Value>,
    third: Option<Value>,
) -> Result<Value, Error> {
    // Coerce the params to the correct place. Specifically:
    // - the options hash might not always be provided as the last argument.
    // - subseconds can be provided with an optional symbol for the type of subsec.
    //
    // ```console
    // [3.1.2] > Time.at(1)
    // => 1970-01-01 01:00:01 +0100
    // [3.1.2] > Time.at(1, 1)
    // => 1970-01-01 01:00:01.000001 +0100
    // [3.1.2] > Time.at(1, 1, :nsec)
    // => 1970-01-01 01:00:01.000000001 +0100
    // [3.1.2] > Time.at(1, in: "A")
    // => 1970-01-01 01:00:01 +0100
    // [3.1.2] > Time.at(1, 1, in: "A")
    // => 1970-01-01 01:00:01.000001 +0100
    // [3.1.2] > Time.at(1, 1, :nsec)
    // => 1970-01-01 01:00:01.000000001 +0100
    // [3.1.2] > Time.at(1, 1, :nsec, in: "A")
    // => 1970-01-01 01:00:01.000000001 +0100
    // ```

    let mut subsec = first;
    let mut subsec_unit = second;
    let mut options = third;

    // Re-position the options hash under the `options` if it exists. Calling
    // `Time.at` without the optional parameters will end up placing the
    // options hash in the incorrect parameter position.
    //
    // ```console
    // Time.at(0, in: "A")
    // #          ^--first
    // Time.at(0, 1, in: "A")
    // #             ^-- second
    // Time.at(0, 1, :nsec, in: "A")
    // #                    ^-- third
    // ```
    //
    // The below logic:
    // - ensures the third parameter is a Ruby::Hash if provided.
    // - if third param is not options, check the second parameter, if it is a
    //   Ruby::Hash then assume this is the options hash, and clear out the
    //   second parameter.
    // - if second param is not options, check the first param, if it is a
    //   Ruby::Hash then assume this is the options hash, and clear out the
    //   first parameter.
    if let Some(third_param) = third {
        if third_param.ruby_type() != Ruby::Hash {
            return Err(ArgumentError::with_message("invalid offset options").into());
        }
    } else {
        options = if let Some(second_param) = second {
            if second_param.ruby_type() == Ruby::Hash {
                subsec_unit = None;
                Some(second_param)
            } else if let Some(first_param) = first {
                if first_param.ruby_type() == Ruby::Hash {
                    subsec = None;
                    Some(first_param)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    let subsec: Subsec = interp.try_convert_mut((subsec, subsec_unit))?;
    let (subsec_secs, subsec_nanos) = subsec.to_tuple();

    let seconds = implicitly_convert_to_int(interp, seconds)?
        .checked_add(subsec_secs)
        .ok_or(ArgumentError::with_message("Time too large"))?;

    let offset: Offset = if let Some(options) = options {
        let offset: Option<Offset> = interp.try_convert_mut(options)?;
        offset.unwrap_or_else(Offset::local)
    } else {
        Offset::local()
    };

    let time = Time::with_timespec_and_offset(seconds, subsec_nanos, offset)?;

    Time::alloc_value(time, interp)
}

pub fn mkutc(interp: &mut Artichoke, args: &[Value]) -> Result<Value, Error> {
    let args: Args = interp.try_convert_mut(args)?;

    let time = Time::utc(
        args.year()?,
        args.month()?,
        args.day()?,
        args.hour()?,
        args.minute()?,
        args.second()?,
        args.nanoseconds()?,
    )?;

    Time::alloc_value(time, interp)
}

pub fn mktime<T>(interp: &mut Artichoke, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let _ = interp;
    let _ignored_while_unimplemented = args.into_iter();
    Err(NotImplementedError::new().into())
}

// Core

pub fn to_int(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let timestamp = time.to_int();
    Ok(interp.convert(timestamp))
}

pub fn to_float(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let duration = time.to_float();
    Ok(interp.convert_mut(duration))
}

pub fn to_rational(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    // Requires `Rational`
    Err(NotImplementedError::new().into())
}

pub fn cmp(interp: &mut Artichoke, mut time: Value, mut other: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    if let Ok(other) = unsafe { Time::unbox_from_value(&mut other, interp) } {
        let cmp = time.cmp(&other);
        Ok(interp.convert(cmp as i32))
    } else {
        let mut message = b"comparison of Time with ".to_vec();
        message.extend_from_slice(interp.inspect_type_name_for_value(other).as_bytes());
        message.extend_from_slice(b" failed");
        Err(ArgumentError::from(message).into())
    }
}

pub fn eql(interp: &mut Artichoke, mut time: Value, mut other: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    if let Ok(other) = unsafe { Time::unbox_from_value(&mut other, interp) } {
        let cmp = time.eq(&other);
        Ok(interp.convert(cmp))
    } else {
        Ok(interp.convert(false))
    }
}

pub fn hash(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

pub fn initialize<T>(interp: &mut Artichoke, time: Value, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let _ = interp;
    let _ = time;
    let _ignored_while_unimplemented = args.into_iter();
    Err(NotImplementedError::new().into())
}

pub fn initialize_copy(interp: &mut Artichoke, time: Value, mut from: Value) -> Result<Value, Error> {
    let from = unsafe { Time::unbox_from_value(&mut from, interp)? };
    let result = *from;
    Time::box_into_value(result, time, interp)
}

// Mutators and converters

pub fn mutate_to_local(interp: &mut Artichoke, time: Value, offset: Option<Value>) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = offset;
    Err(NotImplementedError::new().into())
}

pub fn mutate_to_utc(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let mut obj = unsafe { Time::unbox_from_value(&mut time, interp)? };
    obj.set_utc()?;
    Ok(time)
}

pub fn as_local(interp: &mut Artichoke, time: Value, offset: Option<Value>) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = offset;
    Err(NotImplementedError::new().into())
}

pub fn as_utc(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let utc = time.to_utc()?;
    Time::alloc_value(utc, interp)
}

// Inspect

pub fn asctime(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    strftime_with_encoding(interp, time, ASCTIME_FORMAT_STRING.as_bytes(), Encoding::Utf8)
}

pub fn to_string(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    // %z will always display a +/-HHMM value, however it's expected that UTC
    // is shown if it is UTC Time.
    let format = if unsafe { Time::unbox_from_value(&mut time, interp)? }.is_utc() {
        "%Y-%m-%d %H:%M:%S UTC"
    } else {
        "%Y-%m-%d %H:%M:%S %z"
    };

    strftime_with_encoding(interp, time, format.as_bytes(), Encoding::Utf8)
}

pub fn to_array(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    // Need to implement `Convert` for timezone offset.
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

// Math

pub fn plus(interp: &mut Artichoke, mut time: Value, mut other: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    if unsafe { Time::unbox_from_value(&mut other, interp) }.is_ok() {
        // ```console
        // [3.1.2] > Time.now + Time.now
        // (irb):15:in `+': time + time? (TypeError)
        //         from (irb):15:in `<main>'
        //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
        // ```
        Err(TypeError::with_message("time + time?").into())
    } else if let Ok(other) = other.try_convert_into::<f64>(interp) {
        let result = time.checked_add_f64(other)?;

        Time::alloc_value(result, interp)
    } else if let Ok(other) = implicitly_convert_to_int(interp, other) {
        let result = time.checked_add_i64(other)?;

        Time::alloc_value(result, interp)
    } else {
        Err(TypeError::with_message("can't convert into an exact number").into())
    }
}

pub fn minus(interp: &mut Artichoke, mut time: Value, mut other: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    if let Ok(other) = unsafe { Time::unbox_from_value(&mut other, interp) } {
        let result: Value = interp.convert_mut(time.to_float() - other.to_float());
        Ok(result)
    } else if let Ok(other) = implicitly_convert_to_int(interp, other) {
        let result = time.checked_sub_i64(other)?;

        Time::alloc_value(result, interp)
    } else if let Ok(other) = other.try_convert_into::<f64>(interp) {
        let result = time.checked_sub_f64(other)?;

        Time::alloc_value(result, interp)
    } else {
        Err(TypeError::with_message("can't convert into an exact number").into())
    }
}

// Coarse math

pub fn succ(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    interp.warn(b"warning: Time#succ is obsolete; use time + 1")?;
    plus(interp, time, interp.convert(1))
}

pub fn round(interp: &mut Artichoke, time: Value, num_digits: Option<Value>) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = num_digits;
    Err(NotImplementedError::new().into())
}

// Datetime

pub fn second(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let second = time.second();
    let result = interp.convert(second);
    Ok(result)
}

pub fn minute(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let minute = time.minute();
    let result = interp.convert(minute);
    Ok(result)
}

pub fn hour(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let hour = time.hour();
    let result = interp.convert(hour);
    Ok(result)
}

pub fn day(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let day = time.day();
    let result = interp.convert(day);
    Ok(result)
}

pub fn month(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let month = time.month();
    let result = interp.convert(month);
    Ok(result)
}

pub fn year(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.year();
    let result = interp.convert(year);
    Ok(result)
}

pub fn weekday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let weekday = time.day_of_week();
    let result = interp.convert(weekday);
    Ok(result)
}

pub fn year_day(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year_day = time.day_of_year();
    let result = interp.convert(year_day);
    Ok(result)
}

pub fn is_dst(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_dst = time.is_dst();
    Ok(interp.convert(is_dst))
}

pub fn timezone(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

pub fn utc_offset(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

// Timezone mode

pub fn is_utc(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_utc = time.is_utc();
    Ok(interp.convert(is_utc))
}

// Day of week

pub fn is_sunday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_sunday = time.is_sunday();
    let result = interp.convert(is_sunday);
    Ok(result)
}

pub fn is_monday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_monday = time.is_monday();
    let result = interp.convert(is_monday);
    Ok(result)
}

pub fn is_tuesday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_tuesday = time.is_tuesday();
    let result = interp.convert(is_tuesday);
    Ok(result)
}

pub fn is_wednesday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_wednesday = time.is_wednesday();
    let result = interp.convert(is_wednesday);
    Ok(result)
}

pub fn is_thursday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_thursday = time.is_thursday();
    let result = interp.convert(is_thursday);
    Ok(result)
}

pub fn is_friday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_friday = time.is_friday();
    let result = interp.convert(is_friday);
    Ok(result)
}

pub fn is_saturday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let is_saturday = time.is_saturday();
    let result = interp.convert(is_saturday);
    Ok(result)
}

// Unix time value

pub fn microsecond(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let microsecond = time.microseconds();
    let result = interp.convert(microsecond);
    Ok(result)
}

pub fn nanosecond(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let nanosecond = time.nanoseconds();
    let result = interp.convert(nanosecond);
    Ok(result)
}

pub fn subsec(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    // Requires `Rational`
    Err(NotImplementedError::new().into())
}

// Time format
fn strftime_with_encoding(
    interp: &mut Artichoke,
    mut time: Value,
    format: &[u8],
    encoding: Encoding,
) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };

    let bytes: Vec<u8> = time.strftime(format).map_err(|e| {
        // InvalidFormatString is the only true ArgumentError, where as the
        // rest that can be thrown from strftime are runtime failures.
        //
        // ```console
        // [3.1.2]> Time.now.strftime("%")
        // (irb):1:in `strftime': invalid format: % (ArgumentError)
        //      from (irb):1:in `<main>'
        // 	    from /home/ben/.rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
        // 	    from /home/ben/.rbenv/versions/3.1.2/bin/irb:25:in `load'
        // 	    from /home/ben/.rbenv/versions/3.1.2/bin/irb:25:in `<main>'
        // ```
        //
        // Note: The errors which are re-thrown as RuntimeError include (but is
        // not limited to: `InvalidTime`, `FmtError(Error)`,
        // `OutOfMemory(TryReserveError)`
        #[allow(clippy::match_same_arms)]
        match e {
            InvalidFormatString => {
                let mut message = br#"invalid format: "#.to_vec();
                message.extend_from_slice(format);
                Error::from(ArgumentError::from(message))
            }
            FormattedStringTooLarge => {
                // TODO: This should be an `Errno::ERANGE` not an ArgumentError
                //
                // ```console
                // [3.1.2] > Time.now.strftime "%4718593m"
                // (irb):28:in `strftime': Result too large - %4718593m (Errno::ERANGE)
                //      from (irb):28:in `<main>'
                //      from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
                //      from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
                //      from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
                // ```
                let mut message = br#"Result too large - "#.to_vec();
                message.extend_from_slice(format);
                Error::from(ArgumentError::from(message))
            }
            WriteZero => {
                // TODO: This should be an `Errno::ERANGE` not an ArgumentError
                //
                // ```console
                // [3.1.2] > Time.now.strftime "%2147483647m"
                // (irb):28:in `strftime': Result too large - %2147483647m (Errno::ERANGE)
                //      from (irb):29:in `<main>'
                //      from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
                //      from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
                //      from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
                // ```
                let mut message = br#"Result too large - "#.to_vec();
                message.extend_from_slice(format);
                Error::from(ArgumentError::from(message))
            }
            _ => Error::from(RuntimeError::with_message("Unexpected failure")),
        }
    })?;

    let result = String::with_bytes_and_encoding(bytes, encoding);

    String::alloc_value(result, interp)
}

pub fn strftime(interp: &mut Artichoke, time: Value, format: Value) -> Result<Value, Error> {
    let mut format = to_str(interp, format)?;

    let format = unsafe { String::unbox_from_value(&mut format, interp)? };

    strftime_with_encoding(interp, time, &format, format.encoding())
}
