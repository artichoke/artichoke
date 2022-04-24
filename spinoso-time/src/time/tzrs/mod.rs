
// Time#[-|+|hash]
#[derive(Add,Sub,Default,Clone,Eq,PartialEq,Hash)]
pub struct Time {

}

// constructors
impl Time {
    // Time#now
    pub fn now() -> Self { }

    // Time#new
    // Also called form From<ToA>?
    pub fn new() -> Self { }

    // Time#at
    pub fn at() -> Self { }
}

// Time#[gm|local|mktime|utc|]
impl From<ToA> for Time {}

// Core
impl Time {
    // Time#to_a
    pub fn to_array(&self) -> {}

    // Time#[to_i|tv_sec]
    pub fn to_int(&self) -> i64 { }

    // Time#to_f
    pub fn to_float(&self) -> f64 { }

    // Time#to_r
    pub fn to_rational(&self) -> String { }
}

// Conversions
impl Time {

    // Time#[asctime|ctime]
    pub fn to_string(&self) -> String { }

    // Time#strftime
    // Time#[to_s|inspect] uses "%Y-%m-%d %H:%M:%S UTC
    pub fn strftime(&self, format: String) -> String { }

    pub fn to_array(&self) -> [i64] { }

    // Time#getlocal, Time#[getgm|getutc]
    pub in_timezone(&self, tz: TimeZone) -> Self { }

    // Time#succ (obselete)
    pub succ(&self) -> Self { }
}

// Mutators
impl Time {
    // for Time#localtime, Time#[gmtime|utc]
    pub fn set_timezone(&mut self) {

    }

    pub fn round(&mut self, digits: u32) {

    }
}

// Maths (apart from Add/Sub)
impl Time {
    pub fn
}

// Parts
impl Time {
    // Time#[nsec|tv_nsec]
    pub fn nano_seconds(&self) -> i64 {}
    // Time#[usec|tv_usec]
    pub fn micro_seconds(&self) -> i64 {}
    // Time#sec
    pub fn second(&self) -> i64 {}
    // Time#min
    pub fn minute(&self) -> i64 {}
    // Time#hour
    pub fn hour(&self) -> i64 {}
    // Time#[m]day
    pub fn day(&self) -> i64 {}
    // Time#mon[th]
    pub fn month(&self) -> i64 {}
    pub fn year(&self) -> i64 {}

    // Time#[gmt?|utc?]
    pub fn timezone(&self) -> Timezone {}

    // Time#[isdst|dst?]
    pub fn dst(&self) -> bool {}

    // Time#wday
    // Time#[monday?|tuesday?...]
    // 0 indexed to Sunday
    pub fn day_of_week(&self) -> u8 {}

    // Time#yday
    pub fn day_of_year(&self) -> u16 {}

    // Time#subsec
    // Good luck!
    pub fn sub_sec(&self) -> String {}
}

pub struct Timezone {

}

impl Timezone {
    // Offset in seconds
    // Time#gmt_off[set]|utc_off[set]
    pub fn offset(&self) -> i64 {}

    // Time#zone
    pub fn name(&self) -> String {}
}
