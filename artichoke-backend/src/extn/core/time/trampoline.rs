use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
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

pub fn minute(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let minute = time.borrow().inner().minute();
    Ok(interp.convert(minute))
}

pub fn month(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let month = time.borrow().inner().month();
    Ok(interp.convert(month))
}

pub fn nanosecond(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let nanosecond = time.borrow().inner().nanosecond();
    Ok(interp.convert(nanosecond))
}

pub fn second(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let second = time.borrow().inner().second();
    Ok(interp.convert(second))
}

pub fn microsecond(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let microsecond = time.borrow().inner().microsecond();
    Ok(interp.convert(microsecond))
}

pub fn weekday(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let weekday = time.borrow().inner().weekday();
    Ok(interp.convert(weekday))
}

pub fn year_day(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let year_day = time.borrow().inner().year_day();
    Ok(interp.convert(year_day))
}

pub fn year(interp: &Artichoke, time: Value) -> Result<Value, Box<dyn RubyException>> {
    let time = unsafe { Time::try_from_ruby(interp, &time) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Time from Ruby Time receiver",
        )
    })?;
    let year = time.borrow().inner().year();
    Ok(interp.convert(year))
}
