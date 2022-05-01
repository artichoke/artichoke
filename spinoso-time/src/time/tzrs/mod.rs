use core::time::Duration;
use tz::timezone::{TimeZone, TimeZoneRef};

mod offset;
mod to_a;
mod math;

pub use offset::Offset;
pub use to_a::ToA;

// Time#[-|+|hash]
#[derive(Default,Clone,Eq,PartialEq,Hash)]
pub struct Time {
    // Timestamps extend the standard Rust core::time::Duration, which uses a u64 for the number of
    // seconds, and u32 for the sub part seconds. EPOCH is considered `0x1 << 63` which enables i63
    // number of seconds to be registered (which ruby requires)
    timestamp: Duration,

    offset: Offset,
}

// constructors
impl Time {
    // Time#now
    pub fn now() -> Self {
        todo!()
    }

    // Time#new
    // Also called form From<ToA>?
    pub fn new() -> Self {
        todo!()
    }

    // Time#at
    pub fn at() -> Self {
        todo!()
    }
}

// Time#[gm|local|mktime|utc|]
impl From<ToA> for Time {
   fn from(to_a: ToA) -> Self {
       todo!()
   }
}

// Core
impl Time {
    // Time#[to_i|tv_sec]
    pub fn to_int(&self) -> i64 {
        todo!()
    }

    // Time#to_f
    pub fn to_float(&self) -> f64 {
        todo!()
    }

    // Time#to_r
    pub fn to_rational(&self) -> String {
        todo!()
    }
}

// Conversions
impl Time {
    // Time#[asctime|ctime]
    pub fn to_string(&self) -> String {
        todo!()
    }

    // Time#strftime
    // Time#[to_s|inspect] uses "%Y-%m-%d %H:%M:%S UTC
    pub fn strftime(&self, format: String) -> String {
        todo!()
    }

    // Time#to_a
    pub fn to_array(&self) -> ToA {
        todo!()
    }

    // Time#getlocal, Time#[getgm|getutc]
    pub fn in_timezone(&self, tz: TimeZone) -> Self {
        todo!()
    }
}

// Mutators
impl Time {
    // for Time#localtime, Time#[gmtime|utc]
    pub fn set_timezone(&mut self) {
        todo!()
    }

    pub fn round(&mut self, digits: u32) {
        todo!()
    }
}

// Parts
impl Time {
    // Time#[nsec|tv_nsec]
    pub fn nano_second(&self) -> u64 {
        todo!()
    }
    // Time#[usec|tv_usec]
    pub fn micro_second(&self) -> u64 {
        todo!()
    }
    // Time#sec
    pub fn second(&self) -> u32 {
        todo!()
    }
    // Time#min
    pub fn minute(&self) -> u32 {
        todo!()
    }
    // Time#hour
    pub fn hour(&self) -> u32 {
        todo!()
    }
    // Time#[m]day
    pub fn day(&self) -> u32 {
        todo!()
    }
    // Time#mon[th]
    pub fn month(&self) -> u32 {
        todo!()
    }
    // Time#year
    pub fn year(&self) -> i32 {
        todo!()
    }

    // Time#[gmt?|utc?]
    pub fn time_zone(&self) -> TimeZoneRef<'_> {
        todo!()
    }

    // Time#[isdst|dst?]
    pub fn is_dst(&self) -> bool {
        todo!()
    }

    // Time#wday
    // Time#[monday?|tuesday?...]
    // 0 indexed to Sunday
    pub fn day_of_week(&self) -> u32 {
        todo!()
    }

    // Time#yday
    pub fn day_of_year(&self) -> u32 {
        todo!()
    }

    // Time#subsec
    // Good luck!
    pub fn sub_sec(&self) -> String {
        todo!()
    }
}
