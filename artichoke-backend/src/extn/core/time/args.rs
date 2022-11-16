use crate::extn::prelude::*;

#[derive(Debug)]
pub struct TimeArgs {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanoseconds: u32,
}

impl Default for TimeArgs {
    fn default() -> TimeArgs {
        TimeArgs {
            year: 0,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            nanoseconds: 0,
        }
    }
}

pub fn as_time_args<T>(interp: &mut Artichoke, args: T) -> Result<TimeArgs, Error>
where
    T: IntoIterator<Item = Value>,
{
    let _interp = interp;
    let _args = args;
    Ok(TimeArgs::default())
}

#[cfg(test)]
mod tests {
    use super::as_time_args;
    use crate::test::prelude::*;
    use bstr::ByteSlice;

    #[test]
    fn requires_at_least_one_param() {
        let mut interp = interpreter();

        let raw_args = [];

        let err = as_time_args(&mut interp, raw_args.into_iter()).unwrap_err();

        assert_eq!(err.name(), "ArgumentError");
        assert_eq!(
            err.message().as_bstr(),
            b"wrong number of arguments (given 0, expected 1..8)"
                .as_slice()
                .as_bstr()
        );
    }

    #[test]
    fn one_to_eight_params() {
        // TODO: Table test 1..8 params
        let mut interp = interpreter();

        let raw_args = [interp.eval(b"2022").unwrap()];

        let args = as_time_args(&mut interp, raw_args.into_iter()).unwrap();

        assert_eq!(2022, args.year)
    }

    #[test]
    fn subsec_is_micros_not_nanos() {}

    #[test]
    fn fractional_seconds_return_nanos() {}

    #[test]
    fn nine_args_is_not_supported() {}

    #[test]
    fn ten_args_changes_unit_order() {}

    #[test]
    fn ten_args_removes_micros() {}
}
