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
    let result = interp
        .try_convert(day)
        .map_err(|_| RuntimeError::new(interp, "Time day component too large"))?;
    Ok(result)
}

pub fn hour(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let hour = time.borrow().inner().hour();
    let result = interp
        .try_convert(hour)
        .map_err(|_| RuntimeError::new(interp, "Time hour component too large"))?;
    Ok(result)
}

pub fn minute(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let minute = time.borrow().inner().minute();
    let result = interp
        .try_convert(minute)
        .map_err(|_| RuntimeError::new(interp, "Time minute component too large"))?;
    Ok(result)
}

pub fn month(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let month = time.borrow().inner().month();
    let result = interp
        .try_convert(month)
        .map_err(|_| RuntimeError::new(interp, "Time month component too large"))?;
    Ok(result)
}

pub fn nanosecond(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let nanosecond = time.borrow().inner().nanosecond();
    let result = interp
        .try_convert(nanosecond)
        .map_err(|_| RuntimeError::new(interp, "Time nanosecond component too large"))?;
    Ok(result)
}

pub fn second(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let second = time.borrow().inner().second();
    let result = interp
        .try_convert(second)
        .map_err(|_| RuntimeError::new(interp, "Time second component too large"))?;
    Ok(result)
}

pub fn microsecond(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let microsecond = time.borrow().inner().microsecond();
    let result = interp
        .try_convert(microsecond)
        .map_err(|_| RuntimeError::new(interp, "Time microsecond component too large"))?;
    Ok(result)
}

pub fn weekday(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let weekday = time.borrow().inner().weekday();
    let result = interp
        .try_convert(weekday)
        .map_err(|_| RuntimeError::new(interp, "Time weekday component too large"))?;
    Ok(result)
}

pub fn year_day(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let year_day = time.borrow().inner().year_day();
    let result = interp
        .try_convert(year_day)
        .map_err(|_| RuntimeError::new(interp, "Time year_day component too large"))?;
    Ok(result)
}

pub fn year(interp: &Artichoke, time: Value) -> Result<Value, Exception> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }?;
    let year = time.borrow().inner().year();
    let result = interp
        .try_convert(year)
        .map_err(|_| RuntimeError::new(interp, "Time year component too large"))?;
    Ok(result)
}
