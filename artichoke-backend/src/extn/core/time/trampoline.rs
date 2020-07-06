use crate::extn::core::time::Time;
use crate::extn::prelude::*;

pub fn now(interp: &mut Artichoke) -> Result<Value, Exception> {
    let now = Time::now();
    let result = Time::alloc_value(now, interp)?;
    Ok(result)
}

pub fn day(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let day = time.inner().day();
    let result = interp.convert(day);
    Ok(result)
}

pub fn hour(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let hour = time.inner().hour();
    let result = interp.convert(hour);
    Ok(result)
}

pub fn minute(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let minute = time.inner().minute();
    let result = interp.convert(minute);
    Ok(result)
}

pub fn month(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let month = time.inner().month();
    let result = interp.convert(month);
    Ok(result)
}

pub fn nanosecond(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let nanosecond = time.inner().nanosecond();
    let result = interp.convert(nanosecond);
    Ok(result)
}

pub fn second(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let second = time.inner().second();
    let result = interp.convert(second);
    Ok(result)
}

pub fn microsecond(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let microsecond = time.inner().microsecond();
    let result = interp.convert(microsecond);
    Ok(result)
}

pub fn weekday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let weekday = time.inner().weekday();
    let result = interp.convert(weekday);
    Ok(result)
}

pub fn year_day(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year_day = time.inner().year_day();
    let result = interp.convert(year_day);
    Ok(result)
}

pub fn year(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().year();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_sunday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_sunday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_monday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_monday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_tuesday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_tuesday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_wednesday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_wednesday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_thursday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_thursday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_friday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_friday();
    let result = interp.convert(year);
    Ok(result)
}

pub fn is_saturday(interp: &mut Artichoke, mut time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::unbox_from_value(&mut time, interp)? };
    let year = time.inner().is_saturday();
    let result = interp.convert(year);
    Ok(result)
}
