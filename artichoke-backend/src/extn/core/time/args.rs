use crate::convert::to_int;
use crate::extn::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Args {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
    micros: i64,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            year: 0,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            micros: 0,
        }
    }
}

impl Args {
    pub fn year(&self) -> Result<i32, Error> {
        i32::try_from(self.year).map_err(|_| ArgumentError::with_message("year out of range").into())
    }

    pub fn month(&self) -> Result<u8, Error> {
        match u8::try_from(self.month) {
            Ok(month @ 1..=12) => Ok(month),
            _ => Err(ArgumentError::with_message("mon out of range").into()),
        }
    }

    pub fn day(&self) -> Result<u8, Error> {
        match u8::try_from(self.day) {
            Ok(day @ 1..=31) => Ok(day),
            _ => Err(ArgumentError::with_message("mday out of range").into()),
        }
    }

    pub fn hour(&self) -> Result<u8, Error> {
        match u8::try_from(self.hour) {
            Ok(hour @ 0..=23) => Ok(hour),
            _ => Err(ArgumentError::with_message("hour out of range").into()),
        }
    }

    pub fn minute(&self) -> Result<u8, Error> {
        match u8::try_from(self.minute) {
            Ok(minute @ 0..=59) => Ok(minute),
            _ => Err(ArgumentError::with_message("min out of range").into()),
        }
    }

    pub fn second(&self) -> Result<u8, Error> {
        match u8::try_from(self.second) {
            Ok(second @ 0..=60) => Ok(second),
            _ => Err(ArgumentError::with_message("sec out of range").into()),
        }
    }

    pub fn nanoseconds(&self) -> Result<u32, Error> {
        // Args take a micros parameter, not a nanos value.
        match u32::try_from(self.micros) {
            Ok(micros @ 0..=999_999) => Ok(micros * 1000),
            _ => Err(ArgumentError::with_message("subsecx out of range").into()),
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

                    let arg = to_int(self, arg)?;
                    // unwrap is safe since to_int gaurnatees a non nil
                    // Ruby::Integer
                    let arg: i64 = arg.try_convert_into::<Option<i64>>(self)?.unwrap();

                    match i {
                        0 => result.year = arg,
                        1 => {
                            result.month = {
                                // TODO: This should support 3 letter month names
                                // as per the docs. https://ruby-doc.org/3.1.2/Time.html#method-c-new
                                arg
                            }
                        }
                        2 => result.day = arg,
                        3 => result.hour = arg,
                        4 => result.minute = arg,
                        5 => {
                            result.second = {
                                // TODO: This should support f64 seconds and drop
                                // the remainder into micros.
                                // ```irb
                                // 3.1.2 > Time.utc(1, 2, 3, 4, 5, 6.1)
                                // => 0001-02-03 04:05:06 56294995342131/562949953421312 UTC
                                // ```
                                arg
                            }
                        }
                        6 => result.micros = arg,
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
                Err(NotImplementedError::with_message("Artichoke does not currently support 10 args for Time").into())
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
        assert_eq!(2022, result.year().unwrap());
        assert_eq!(2, result.month().unwrap());
        assert_eq!(3, result.day().unwrap());
        assert_eq!(4, result.hour().unwrap());
        assert_eq!(5, result.minute().unwrap());
        assert_eq!(6, result.second().unwrap());
        assert_eq!(7000, result.nanoseconds().unwrap());
    }

    #[test]
    fn seven_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5, 6, 7]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year().unwrap());
        assert_eq!(2, result.month().unwrap());
        assert_eq!(3, result.day().unwrap());
        assert_eq!(4, result.hour().unwrap());
        assert_eq!(5, result.minute().unwrap());
        assert_eq!(6, result.second().unwrap());
        assert_eq!(7000, result.nanoseconds().unwrap());
    }

    #[test]
    fn six_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5, 6]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year().unwrap());
        assert_eq!(2, result.month().unwrap());
        assert_eq!(3, result.day().unwrap());
        assert_eq!(4, result.hour().unwrap());
        assert_eq!(5, result.minute().unwrap());
        assert_eq!(6, result.second().unwrap());
        assert_eq!(0, result.nanoseconds().unwrap());
    }

    #[test]
    fn five_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4, 5]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year().unwrap());
        assert_eq!(2, result.month().unwrap());
        assert_eq!(3, result.day().unwrap());
        assert_eq!(4, result.hour().unwrap());
        assert_eq!(5, result.minute().unwrap());
        assert_eq!(0, result.second().unwrap());
        assert_eq!(0, result.nanoseconds().unwrap());
    }

    #[test]
    fn four_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3, 4]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year().unwrap());
        assert_eq!(2, result.month().unwrap());
        assert_eq!(3, result.day().unwrap());
        assert_eq!(4, result.hour().unwrap());
        assert_eq!(0, result.minute().unwrap());
        assert_eq!(0, result.second().unwrap());
        assert_eq!(0, result.nanoseconds().unwrap());
    }

    #[test]
    fn three_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2, 3]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year().unwrap());
        assert_eq!(2, result.month().unwrap());
        assert_eq!(3, result.day().unwrap());
        assert_eq!(0, result.hour().unwrap());
        assert_eq!(0, result.minute().unwrap());
        assert_eq!(0, result.second().unwrap());
        assert_eq!(0, result.nanoseconds().unwrap());
    }

    #[test]
    fn two_params() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 2]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year().unwrap());
        assert_eq!(2, result.month().unwrap());
        assert_eq!(1, result.day().unwrap());
        assert_eq!(0, result.hour().unwrap());
        assert_eq!(0, result.minute().unwrap());
        assert_eq!(0, result.second().unwrap());
        assert_eq!(0, result.nanoseconds().unwrap());
    }

    #[test]
    fn one_param() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        assert_eq!(2022, result.year().unwrap());
        assert_eq!(1, result.month().unwrap());
        assert_eq!(1, result.day().unwrap());
        assert_eq!(0, result.hour().unwrap());
        assert_eq!(0, result.minute().unwrap());
        assert_eq!(0, result.second().unwrap());
        assert_eq!(0, result.nanoseconds().unwrap());
    }

    #[test]
    fn subsec_is_valid_micros_not_nanos() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 1, 1, 0, 0, 0, 1]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        let nanos = result.nanoseconds().unwrap();
        assert_eq!(1000, nanos);

        let args = interp.eval(b"[2022, 1, 1, 0, 0, 0, 999_999]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Args = interp.try_convert_mut(ary_args.as_slice()).unwrap();
        let nanos = result.nanoseconds().unwrap();
        assert_eq!(999_999_000, nanos);
    }

    #[test]
    fn subsec_does_not_wrap_around() {
        let mut interp = interpreter();

        let args = interp.eval(b"[2022, 1, 1, 0, 0, 0, -1]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Result<Args, Error> = interp.try_convert_mut(ary_args.as_slice());
        let error = result.unwrap().nanoseconds().unwrap_err();
        assert_eq!(error.message().as_bstr(), b"subsecx out of range".as_bstr());

        let args = interp.eval(b"[2022, 1, 1, 0, 0, 0, 1_000_000]").unwrap();
        let ary_args: Vec<Value> = interp.try_convert_mut(args).unwrap();
        let result: Result<Args, Error> = interp.try_convert_mut(ary_args.as_slice());
        let error = result.unwrap().nanoseconds().unwrap_err();
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
    fn ten_args_changes_unit_order() {}

    #[test]
    fn ten_args_removes_micros() {}

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
