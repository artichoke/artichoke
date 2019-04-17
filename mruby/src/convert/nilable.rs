use mruby_sys::*;

use crate::convert::fixnum::Int;
use crate::convert::{Error, TryFromMrb};
use crate::value::{Ruby, Rust, Value};

impl TryFromMrb<Option<Int>> for Value {
    type From = Rust;
    type To = Ruby;

    fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Option<Int>,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value {
            Some(value) => Self::try_from_mrb(mrb, value),
            None => Ok(Self::new(unsafe { mrb_sys_nil_value() })),
        }
    }
}

impl TryFromMrb<Value> for Option<Int> {
    type From = Ruby;
    type To = Rust;

    fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Nil => Ok(None),
            _ => Int::try_from_mrb(mrb, value).map(Some),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use crate::convert::*;
    use crate::value::*;

    mod fixnum {
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn convert_to_value(v: Option<i64>) -> bool {
            super::convert_to_value(v)
        }

        #[quickcheck]
        fn roundtrip(v: Option<i64>) -> bool {
            super::roundtrip(v)
        }
    }

    #[test]
    fn fail_covert() {
        let mrb = unsafe { mrb_open() };
        let value = Value::new(unsafe { mrb_sys_true_value() });
        let expected = Error {
            from: Ruby::Bool,
            to: Rust::SignedInt,
        };
        let result = <Option<i64>>::try_from_mrb(mrb, value).map(|_| ());
        assert_eq!(result, Err(expected));
    }

    fn convert_to_value<T>(v: Option<T>) -> bool
    where
        T: Clone + PartialEq + TryFromMrb<Value, From = Ruby, To = Rust>,
        Value: TryFromMrb<Option<T>, From = Rust, To = Ruby>,
        Option<T>: Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
    {
        let mrb = unsafe { mrb_open() };
        let value = match Value::try_from_mrb(mrb, v.clone()) {
            Ok(value) => value,
            // we don't care about inner conversion failures for `T`
            Err(_) => return true,
        };
        let good = if let Some(v) = v {
            T::try_from_mrb(mrb, value).expect("convert") == v
        } else {
            let inner = value.inner();
            unsafe { mrb_sys_value_is_nil(inner) }
        };
        unsafe { mrb_close(mrb) };
        good
    }

    fn roundtrip<T>(v: Option<T>) -> bool
    where
        T: Clone + PartialEq + TryFromMrb<Value, From = Ruby, To = Rust>,
        Value: TryFromMrb<Option<T>, From = Rust, To = Ruby>,
        Option<T>: Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
    {
        let mrb = unsafe { mrb_open() };
        let value = match Value::try_from_mrb(mrb, v.clone()) {
            Ok(value) => value,
            // we don't care about inner conversion failures for `T`
            Err(_) => return true,
        };
        let good = match v {
            Some(v) => T::try_from_mrb(mrb, value).expect("convert") == v,
            None => <Option<T>>::try_from_mrb(mrb, value)
                .expect("convert")
                .is_none(),
        };
        unsafe { mrb_close(mrb) };
        good
    }
}
