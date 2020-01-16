use crate::convert::{RustBackedValue, TryConvert};
use crate::extn::core::exception::{Fatal, RubyException, RuntimeError};
use crate::extn::core::time::backend::MakeTime;
use crate::extn::core::time::{self, Time};
use crate::value::Value;
use crate::Artichoke;

pub fn now(interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
    let now = Time(time::factory().now(interp));
    let result = now
        .try_into_ruby(&interp, None)
        .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Time with Rust Time"))?;
    Ok(result)
}

pub fn day(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let day = time.borrow().inner().day();
    let result = interp
        .try_convert(day)
        .map_err(|_| RuntimeError::new(interp, "Time day component too large"))?;
    Ok(result)
}

pub fn hour(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let hour = time.borrow().inner().hour();
    let result = interp
        .try_convert(hour)
        .map_err(|_| RuntimeError::new(interp, "Time hour component too large"))?;
    Ok(result)
}

pub fn minute(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let minute = time.borrow().inner().minute();
    let result = interp
        .try_convert(minute)
        .map_err(|_| RuntimeError::new(interp, "Time minute component too large"))?;
    Ok(result)
}

pub fn month(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let month = time.borrow().inner().month();
    let result = interp
        .try_convert(month)
        .map_err(|_| RuntimeError::new(interp, "Time month component too large"))?;
    Ok(result)
}

pub fn nanosecond(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let nanosecond = time.borrow().inner().nanosecond();
    let result = interp
        .try_convert(nanosecond)
        .map_err(|_| RuntimeError::new(interp, "Time nanosecond component too large"))?;
    Ok(result)
}

pub fn second(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let second = time.borrow().inner().second();
    let result = interp
        .try_convert(second)
        .map_err(|_| RuntimeError::new(interp, "Time second component too large"))?;
    Ok(result)
}

pub fn microsecond(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let microsecond = time.borrow().inner().microsecond();
    let result = interp
        .try_convert(microsecond)
        .map_err(|_| RuntimeError::new(interp, "Time microsecond component too large"))?;
    Ok(result)
}

pub fn weekday(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let weekday = time.borrow().inner().weekday();
    let result = interp
        .try_convert(weekday)
        .map_err(|_| RuntimeError::new(interp, "Time weekday component too large"))?;
    Ok(result)
}

pub fn year_day(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let year_day = time.borrow().inner().year_day();
    let result = interp
        .try_convert(year_day)
        .map_err(|_| RuntimeError::new(interp, "Time year_day component too large"))?;
    Ok(result)
}

pub fn year(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let year = time.borrow().inner().year();
    let result = interp
        .try_convert(year)
        .map_err(|_| RuntimeError::new(interp, "Time year component too large"))?;
    Ok(result)
}
