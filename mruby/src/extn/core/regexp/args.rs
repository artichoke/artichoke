use crate::convert::TryFromMrb;
use crate::sys;
use crate::value::Value;
use crate::Mrb;
use crate::MrbError;
use std::io::Write;
use std::mem;

#[derive(Debug)]
pub struct Rest {
    pub rest: Vec<Value>,
}

impl Rest {
    pub unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
        let args = mem::uninitialized::<*const sys::mrb_value>();
        let count = mem::uninitialized::<usize>();
        let mut argspec = vec![];
        argspec
            .write_all(sys::specifiers::REST.as_bytes())
            .map_err(|_| MrbError::ArgSpec)?;
        argspec.write_all(b"\0").map_err(|_| MrbError::ArgSpec)?;
        sys::mrb_get_args(
            interp.borrow().mrb,
            argspec.as_ptr() as *const i8,
            &args,
            &count,
        );
        let args = std::slice::from_raw_parts(args, count);
        let args = args
            .iter()
            .map(|value| Value::new(&interp, *value))
            .collect::<Vec<_>>();
        Ok(Self { rest: args })
    }
}

#[derive(Debug)]
pub struct Match {
    pub string: Result<Option<String>, MrbError>,
    pub pos: Option<i64>,
    pub block: Option<Value>,
}

impl Match {
    pub unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
        let string = mem::uninitialized::<sys::mrb_value>();
        let pos = mem::uninitialized::<sys::mrb_value>();
        let has_pos = mem::uninitialized::<sys::mrb_bool>();
        let block = mem::uninitialized::<sys::mrb_value>();
        let mut argspec = vec![];
        argspec
            .write_all(
                format!(
                    "{}{}{}{}{}\0",
                    sys::specifiers::OBJECT,
                    sys::specifiers::BLOCK,
                    sys::specifiers::FOLLOWING_ARGS_OPTIONAL,
                    sys::specifiers::OBJECT,
                    sys::specifiers::PREVIOUS_OPTIONAL_ARG_GIVEN,
                )
                .as_bytes(),
            )
            .map_err(|_| MrbError::ArgSpec)?;
        sys::mrb_get_args(
            interp.borrow().mrb,
            argspec.as_ptr() as *const i8,
            &string,
            &block,
            &pos,
            &has_pos,
        );
        let string = <Option<String>>::try_from_mrb(&interp, Value::new(interp, string))
            .map_err(MrbError::ConvertToRust);
        let pos = if has_pos == 0 {
            None
        } else {
            let pos = i64::try_from_mrb(&interp, Value::new(&interp, pos))
                .map_err(MrbError::ConvertToRust)?;
            Some(pos)
        };
        let block = if sys::mrb_sys_value_is_nil(block) {
            None
        } else {
            Some(Value::new(interp, block))
        };
        Ok(Self { string, pos, block })
    }
}
