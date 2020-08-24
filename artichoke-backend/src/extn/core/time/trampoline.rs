use crate::extn::core::time::Time;
use crate::extn::prelude::*;

// Constructor

pub fn now(interp: &mut Artichoke) -> Result<Value, Error> {
    let now = Time::now();
    let result = Time::alloc_value(now, interp)?;
    Ok(result)
}

pub fn at<T>(interp: &mut Artichoke, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let _ = interp;
    let _ = args;
    Err(NotImplementedError::new().into())
}

pub fn mkutc<T>(interp: &mut Artichoke, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let _ = interp;
    let _ = args;
    Err(NotImplementedError::new().into())
}

pub fn mktime<T>(interp: &mut Artichoke, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let _ = interp;
    let _ = args;
    Err(NotImplementedError::new().into())
}

// Core

pub fn to_int(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

pub fn to_float(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

pub fn to_rational(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    // Requires `Rational`
    Err(NotImplementedError::new().into())
}

pub fn cmp(interp: &mut Artichoke, time: Value, other: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = other;
    Err(NotImplementedError::new().into())
}

pub fn eql(interp: &mut Artichoke, time: Value, other: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = other;
    Err(NotImplementedError::new().into())
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
    let _ = args;
    Err(NotImplementedError::new().into())
}

pub fn initialize_copy(interp: &mut Artichoke, time: Value, from: Value) -> Result<Value, Error> {
    // Time does not support clone. This is what the implementation of this
    // function should be:
    //
    // let from = unsafe { Time::unbox_from_value(&mut from, interp)? };
    // let result = from.clone();
    // Time::box_into_value(result, time, interp)

    let _ = interp;
    let _ = time;
    let _ = from;
    Err(NotImplementedError::new().into())
}

// Mutators and converters

pub fn mutate_to_local(
    interp: &mut Artichoke,
    time: Value,
    offset: Option<Value>,
) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = offset;
    Err(NotImplementedError::new().into())
}

pub fn mutate_to_utc(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

pub fn as_local(
    interp: &mut Artichoke,
    time: Value,
    offset: Option<Value>,
) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = offset;
    Err(NotImplementedError::new().into())
}

pub fn as_utc(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

// Inspect

pub fn asctime(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

pub fn to_string(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

pub fn to_array(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

// Math

pub fn plus(interp: &mut Artichoke, time: Value, other: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = other;
    Err(NotImplementedError::new().into())
}

pub fn minus(interp: &mut Artichoke, time: Value, other: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = other;
    Err(NotImplementedError::new().into())
}

// Coarse math

pub fn succ(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

pub fn round(
    interp: &mut Artichoke,
    time: Value,
    num_digits: Option<Value>,
) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = num_digits;
    Err(NotImplementedError::new().into())
}

// Datetime

pub fn second(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let second = time.inner().second();
    let result = interp.convert(second);
    Ok(result)
}

pub fn minute(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let minute = time.inner().minute();
    let result = interp.convert(minute);
    Ok(result)
}

pub fn hour(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let hour = time.inner().hour();
    let result = interp.convert(hour);
    Ok(result)
}

pub fn day(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let day = time.inner().day();
    let result = interp.convert(day);
    Ok(result)
}

pub fn month(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let month = time.inner().month();
    let result = interp.convert(month);
    Ok(result)
}

pub fn year(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().year();
    let result = interp.convert(year);
    Ok(result)
}

pub fn weekday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let weekday = time.inner().weekday();
    let result = interp.convert(weekday);
    Ok(result)
}

pub fn year_day(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year_day = time.inner().year_day();
    let result = interp.convert(year_day);
    Ok(result)
}

pub fn is_dst(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
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

pub fn is_utc(interp: &mut Artichoke, time: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    Err(NotImplementedError::new().into())
}

// Day of week

pub fn is_sunday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_sunday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_monday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_monday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_tuesday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_tuesday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_wednesday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_wednesday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_thursday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_thursday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_friday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_friday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_saturday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_saturday();
    let result = interp.convert(year);
    Ok(result)
}

// Unix time value

pub fn microsecond(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let microsecond = time.inner().microsecond();
    let result = interp.convert(microsecond);
    Ok(result)
}

pub fn nanosecond(interp: &mut Artichoke, mut time: Value) -> Result<Value, Error> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let nanosecond = time.inner().nanosecond();
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

pub fn strftime(interp: &mut Artichoke, time: Value, format: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = time;
    let _ = format;
    // Requires a parser.
    Err(NotImplementedError::new().into())
}
