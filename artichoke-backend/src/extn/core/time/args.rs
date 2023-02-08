use crate::convert::to_int;
use crate::extn::prelude::*;

pub type Year = i32;
pub type Month = u8;
pub type Day = u8;
pub type Hour = u8;
pub type Minute = u8;
pub type Second = u8;
pub type Nanoseconds = u32;

#[derive(Debug, Clone, Copy)]
pub enum ValueArgPart {
    Year(Value),
    Month(Value),
    Day(Value),
    Hour(Value),
    Minute(Value),
    Second(Value),
    Micros(Value),
}

#[derive(Debug, Clone, Copy)]
pub enum ArgPart {
    Year(Year),
    Month(Month),
    Day(Day),
    Hour(Hour),
    Minute(Minute),
    Second(Second),
    Nanoseconds(Nanoseconds),
}

#[derive(Debug, Clone, Copy)]
pub struct Args {
    year: ArgPart,
    month: ArgPart,
    day: ArgPart,
    hour: ArgPart,
    minute: ArgPart,
    second: ArgPart,
    nanoseconds: ArgPart,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            year: ArgPart::Year(0),
            month: ArgPart::Month(1),
            day: ArgPart::Day(1),
            hour: ArgPart::Hour(0),
            minute: ArgPart::Minute(0),
            second: ArgPart::Second(0),
            nanoseconds: ArgPart::Nanoseconds(0),
        }
    }
}

impl Args {
    pub fn year(&self) -> Year {
        match self.year {
            ArgPart::Year(year) => year,
            _ => unreachable!(),
        }
    }

    pub fn month(&self) -> Month {
        match self.month {
            ArgPart::Month(month) => month,
            _ => unreachable!(),
        }
    }

    pub fn day(&self) -> Day {
        match self.day {
            ArgPart::Day(day) => day,
            _ => unreachable!(),
        }
    }

    pub fn hour(&self) -> Hour {
        match self.hour {
            ArgPart::Hour(hour) => hour,
            _ => unreachable!(),
        }
    }

    pub fn minute(&self) -> Minute {
        match self.minute {
            ArgPart::Minute(minute) => minute,
            _ => unreachable!(),
        }
    }

    pub fn second(&self) -> Second {
        match self.second {
            ArgPart::Second(second) => second,
            _ => unreachable!(),
        }
    }

    pub fn nanoseconds(&self) -> Nanoseconds {
        match self.nanoseconds {
            ArgPart::Nanoseconds(nanos) => nanos,
            _ => unreachable!(),
        }
    }
}

impl TryConvertMut<ValueArgPart, ArgPart> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: ValueArgPart) -> Result<ArgPart, Self::Error> {
        match value {
            ValueArgPart::Year(value) => {
                let value = to_int(self, value)?;
                let value: i64 = value.try_convert_into::<Option<i64>>(self)?.unwrap();

                i32::try_from(value)
                    .map(|value| ArgPart::Year(value))
                    .map_err(|_| ArgumentError::with_message("year out of range").into())
            },
            ValueArgPart::Month(value) => {
                // TODO: This should support 3 letter month names
                // as per the docs. https://ruby-doc.org/3.1.2/Time.html#method-c-new

                let value = to_int(self, value)?;
                let value: i64 = value.try_convert_into::<Option<i64>>(self)?.unwrap();

                match u8::try_from(value) {
                    Ok(month @ 1..=12) => Ok(ArgPart::Month(month)),
                    _ => Err(ArgumentError::with_message("mon out of range").into()),
                }
            },
            ValueArgPart::Day(value) => {
                let value = to_int(self, value)?;
                let value: i64 = value.try_convert_into::<Option<i64>>(self)?.unwrap();

                match u8::try_from(value) {
                    Ok(day @ 1..=31) => Ok(ArgPart::Day(day)),
                    _ => Err(ArgumentError::with_message("mday out of range").into()),
                }
            },
            ValueArgPart::Hour(value) => {
                let value = to_int(self, value)?;
                let value: i64 = value.try_convert_into::<Option<i64>>(self)?.unwrap();

                match u8::try_from(value) {
                    Ok(hour @ 0..=23) => Ok(ArgPart::Hour(hour)),
                    _ => Err(ArgumentError::with_message("hour out of range").into()),
                }
            },
            ValueArgPart::Minute(value) => {
                let value = to_int(self, value)?;
                let value: i64 = value.try_convert_into::<Option<i64>>(self)?.unwrap();

                match u8::try_from(value) {
                    Ok(minute @ 0..=59) => Ok(ArgPart::Minute(minute)),
                    _ => Err(ArgumentError::with_message("min out of range").into()),
                }
            },
            ValueArgPart::Second(value) => {
                // TODO: This should support f64 seconds and drop
                // the remainder into micros.
                // ```irb
                // 3.1.2 > Time.utc(1, 2, 3, 4, 5, 6.1)
                // => 0001-02-03 04:05:06 56294995342131/562949953421312 UTC
                // ```

                let value = to_int(self, value)?;
                let value: i64 = value.try_convert_into::<Option<i64>>(self)?.unwrap();

                match u8::try_from(value) {
                    Ok(second @ 0..=60) => Ok(ArgPart::Second(second)),
                    _ => Err(ArgumentError::with_message("sec out of range").into()),
                }
            },
            ValueArgPart::Micros(value) => {
                let value = to_int(self, value)?;
                let value: i64 = value.try_convert_into::<Option<i64>>(self)?.unwrap();

                match u32::try_from(value) {
                    Ok(micros @ 0..=999_999) => Ok(ArgPart::Nanoseconds(micros * 1000)),
                    _ => Err(ArgumentError::with_message("subsecx out of range").into()),
                }
            }
        }
    }
}

impl TryConvertMut<&[Value], Args> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, args: &[Value]) -> Result<Args, Self::Error> {
        // Args are in order of year, month, day, hour, minute, second, micros.
        // This is unless there are 10 arguments provided (`Time#to_a` format),
        // at which points it is second, minute, hour, day, month, year. The
        // number of expected parameters doesn't give this hint though.

        match args.len() {
            0 | 9 | 11.. => {
                let mut message = br#"wrong number of arguments (given "#.to_vec();
                message.extend(args.len().to_string().bytes());
                message.extend_from_slice(b", expected 1..8)");
                Err(ArgumentError::from(message).into())
            }
            1..=8 => {
                // For 0..=7 params, we need to validate to_int
                let mut result = Args::default();
                for (i, &arg) in args.iter().enumerate() {
                    // The eighth parameter is never used, and thus no
                    // conversion is needed
                    if i == 7 {
                        continue;
                    }

                    match i {
                        0 => result.year = self.try_convert_mut(ValueArgPart::Year(arg))?,
                        1 => result.month = self.try_convert_mut(ValueArgPart::Month(arg))?,
                        2 => result.day = self.try_convert_mut(ValueArgPart::Day(arg))?,
                        3 => result.hour = self.try_convert_mut(ValueArgPart::Hour(arg))?,
                        4 => result.minute = self.try_convert_mut(ValueArgPart::Minute(arg))?,
                        5 => result.second = self.try_convert_mut(ValueArgPart::Second(arg))?,
                        6 => result.nanoseconds = self.try_convert_mut(ValueArgPart::Micros(arg))?,
                        7 => {
                            // NOOP
                            // The 8th parameter can be anything, even an error
                            //
                            // ```irb
                            // Time.utc(2022, 1, 1, 0, 0, 0, 0, StandardError)
                            // => 2022-01-01 00:00:00 UTC
                            // ```
                        }
                        _ => unreachable!(),
                    }
                }
                Ok(result)
            }
            10 => {
                let mut result = Args::default();

                // Only arguments in position 0..=6 are parsed.
                let args = args.iter().enumerate().filter(|&(i, _)| i < 6);
                for (i, &arg) in args {
                    match i {
                        0 => result.second = self.try_convert_mut(ValueArgPart::Second(arg))?,
                        1 => result.minute = self.try_convert_mut(ValueArgPart::Minute(arg))?,
                        2 => result.hour = self.try_convert_mut(ValueArgPart::Hour(arg))?,
                        3 => result.day = self.try_convert_mut(ValueArgPart::Day(arg))?,
                        4 => result.month = self.try_convert_mut(ValueArgPart::Month(arg))?,
                        5 => result.year = self.try_convert_mut(ValueArgPart::Year(arg))?,
                        _ => unreachable!(),
                    }
                }
                Ok(result)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::Args;
    use crate::test::prelude::*;

    #[test]
    fn requires_at_least_one_param() {
        let mut interp = interpreter();

        let args = vec![];

        let result: Result<Args, Error> = interp.try_convert_mut(args.as_slice());
        let error = result.unwrap_err();

        assert_eq!(error.name(), "ArgumentError");
        assert_eq!(
            error.message().as_bstr(),
            b"wrong number of arguments (given 0, expected 1..8)".as_bstr()
        );
    }

    #[test]
    fn eight_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5, 6, 7, nil]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year());
        assert_eq!(2, result.month());
        assert_eq!(3, result.day());
        assert_eq!(4, result.hour());
        assert_eq!(5, result.minute());
        assert_eq!(6, result.second());
        assert_eq!(7000, result.nanoseconds());
    }

    #[test]
    fn seven_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5, 6, 7]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year());
        assert_eq!(2, result.month());
        assert_eq!(3, result.day());
        assert_eq!(4, result.hour());
        assert_eq!(5, result.minute());
        assert_eq!(6, result.second());
        assert_eq!(7000, result.nanoseconds());
    }

    #[test]
    fn six_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5, 6]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year());
        assert_eq!(2, result.month());
        assert_eq!(3, result.day());
        assert_eq!(4, result.hour());
        assert_eq!(5, result.minute());
        assert_eq!(6, result.second());
        assert_eq!(0, result.nanoseconds());
    }

    #[test]
    fn five_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year());
        assert_eq!(2, result.month());
        assert_eq!(3, result.day());
        assert_eq!(4, result.hour());
        assert_eq!(5, result.minute());
        assert_eq!(0, result.second());
        assert_eq!(0, result.nanoseconds());
    }

    #[test]
    fn four_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year());
        assert_eq!(2, result.month());
        assert_eq!(3, result.day());
        assert_eq!(4, result.hour());
        assert_eq!(0, result.minute());
        assert_eq!(0, result.second());
        assert_eq!(0, result.nanoseconds());
    }

    #[test]
    fn three_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year());
        assert_eq!(2, result.month());
        assert_eq!(3, result.day());
        assert_eq!(0, result.hour());
        assert_eq!(0, result.minute());
        assert_eq!(0, result.second());
        assert_eq!(0, result.nanoseconds());
    }

    #[test]
    fn two_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year());
        assert_eq!(2, result.month());
        assert_eq!(1, result.day());
        assert_eq!(0, result.hour());
        assert_eq!(0, result.minute());
        assert_eq!(0, result.second());
        assert_eq!(0, result.nanoseconds());
    }

    #[test]
    fn one_param() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year());
        assert_eq!(1, result.month());
        assert_eq!(1, result.day());
        assert_eq!(0, result.hour());
        assert_eq!(0, result.minute());
        assert_eq!(0, result.second());
        assert_eq!(0, result.nanoseconds());
    }

    #[test]
    fn subsec_is_valid_micros_not_nanos() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 1, 1, 0, 0, 0, 1]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        let nanos = result.nanoseconds();
        assert_eq!(1000, nanos);

        let args = interp.eval(b"[2022, 1, 1, 0, 0, 0, 999_999]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        let nanos = result.nanoseconds();
        assert_eq!(999_999_000, nanos);
    }

    #[test]
    fn subsec_does_not_wrap_around() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 1, 1, 0, 0, 0, -1]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Result<Args, Error> = interp.try_convert_mut(ary_args.as_slice());
        let error = result.unwrap_err();
        assert_eq!(error.message().as_bstr(), b"subsecx out of range".as_bstr());

        let args = interp.eval(b"[2022, 1, 1, 0, 0, 0, 1_000_000]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Result<Args, Error> = interp.try_convert_mut(ary_args.as_slice());
        let error = result.unwrap_err();
        assert_eq!(error.message().as_bstr(), b"subsecx out of range".as_bstr());
    }

    #[test]
    fn fractional_seconds_return_nanos() {}

    #[test]
    fn nine_args_not_supported() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5, 6, 7, nil, 0]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Result<Args, Error> = interp.try_convert_mut(ary_args.as_slice());
        let error = result.unwrap_err();

        assert_eq!(
            error.message().as_bstr(),
            b"wrong number of arguments (given 9, expected 1..8)".as_bstr()
        );
        assert_eq!(error.name(), "ArgumentError");
    }

    #[test]
    fn ten_args_changes_unit_order() {
        let mut interp = interpreter();

        let args = interp.eval(b"[1, 2, 3, 4, 5, 2022, nil, nil, nil, nil]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();

        assert_eq!(1, result.second());
        assert_eq!(2, result.minute());
        assert_eq!(3, result.hour());
        assert_eq!(4, result.day());
        assert_eq!(5, result.month());
        assert_eq!(2022, result.year());
    }

    #[test]
    fn eleven_args_is_too_many() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5, 6, 7, nil, 0, 0, 0]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Result<Args, Error> = interp.try_convert_mut(ary_args.as_slice());
        let error = result.unwrap_err();

        assert_eq!(
            error.message().as_bstr(),
            b"wrong number of arguments (given 11, expected 1..8)".as_bstr()
        );
        assert_eq!(error.name(), "ArgumentError");
    }
}
