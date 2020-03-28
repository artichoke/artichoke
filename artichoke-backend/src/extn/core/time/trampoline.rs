use crate::extn::core::time::backend::MakeTime;
use crate::extn::core::time::{self, Time};
use crate::extn::prelude::*;

pub fn now(interp: &Artichoke) -> Result<Value, Exception> {
    let now = Time(Box::new(time::factory().now(interp)));
    let result = now.try_into_ruby(&interp, None)?;
    Ok(result)
}

pub fn day(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let day = time.borrow().inner().day();
    let result = interp.convert(day);
    Ok(result)
}

pub fn hour(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let hour = time.borrow().inner().hour();
    let result = interp.convert(hour);
    Ok(result)
}

pub fn minute(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let minute = time.borrow().inner().minute();
    let result = interp.convert(minute);
    Ok(result)
}

pub fn month(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let month = time.borrow().inner().month();
    let result = interp.convert(month);
    Ok(result)
}

pub fn nanosecond(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let nanosecond = time.borrow().inner().nanosecond();
    let result = interp.convert(nanosecond);
    Ok(result)
}

pub fn second(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let second = time.borrow().inner().second();
    let result = interp.convert(second);
    Ok(result)
}

pub fn microsecond(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let microsecond = time.borrow().inner().microsecond();
    let result = interp.convert(microsecond);
    Ok(result)
}

pub fn weekday(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let weekday = time.borrow().inner().weekday();
    let result = interp.convert(weekday);
    Ok(result)
}

pub fn year_day(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let year_day = time.borrow().inner().year_day();
    let result = interp.convert(year_day);
    Ok(result)
}

pub fn year(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let year = time.borrow().inner().year();
    let result = interp.convert(year);
    Ok(result)
}
