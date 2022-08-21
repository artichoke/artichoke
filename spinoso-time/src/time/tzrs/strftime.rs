use super::Time;

impl strftime::Time for Time {
    fn year(&self) -> i32 {
        Time::year(self)
    }

    fn month(&self) -> u8 {
        Time::month(self)
    }

    fn day(&self) -> u8 {
        Time::day(self)
    }

    fn hour(&self) -> u8 {
        Time::hour(self)
    }

    fn minute(&self) -> u8 {
        Time::minute(self)
    }

    fn second(&self) -> u8 {
        Time::second(self)
    }

    fn nanoseconds(&self) -> u32 {
        Time::nanoseconds(self)
    }

    fn day_of_week(&self) -> u8 {
        Time::day_of_week(self)
    }

    fn day_of_year(&self) -> u16 {
        Time::day_of_year(self)
    }

    fn to_int(&self) -> i64 {
        Time::to_int(self)
    }

    fn is_utc(&self) -> bool {
        Time::is_utc(self)
    }

    fn utc_offset(&self) -> i32 {
        Time::utc_offset(self)
    }

    fn time_zone(&self) -> &str {
        Time::time_zone(self)
    }
}
