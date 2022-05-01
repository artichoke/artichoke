use std::hash::{Hash, Hasher};
use tz::timezone::TimeZone;

pub type OffsetSeconds = i32;

// Offsets come in two different types. Utc which is a representation of Universal Co-ordinated
// Time, and a Tz which represents a timezone.
// UTC is not a timezone, and is thus not represented in `tzdb` which provides timezone
// information - thus UTC offsets are represented in seconds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    Utc(OffsetSeconds),
    Tz(TimeZone),
}

impl Default for Offset {
    fn default() -> Self {
        Offset::Utc(0)
    }
}

impl Hash for Offset {
   fn hash<H>(&self, _: &mut H)
       where H: Hasher
   {
       todo!()
   }
}
