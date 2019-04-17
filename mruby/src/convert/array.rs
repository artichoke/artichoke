use mruby_sys::*;
use std::convert::TryFrom;

use crate::convert::fixnum::Int;
use crate::convert::{Error, TryFromMrb};
use crate::value::{Ruby, Rust, Value};

impl TryFromMrb<Vec<Int>> for Value {
    type From = Rust;
    type To = Ruby;

    fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Vec<Int>,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let size = i64::try_from(value.len()).map_err(|_| Error {
            from: Rust::Vec,
            to: Ruby::Array,
        })?;
        let array = unsafe { mrb_ary_new_capa(mrb, size) };
        for (i, item) in value.iter().enumerate() {
            let idx = i64::try_from(i).map_err(|_| Error {
                from: Rust::Vec,
                to: Ruby::Array,
            })?;
            let ary_item = Self::try_from_mrb(mrb, *item)?;
            let inner = ary_item.inner();
            unsafe { mrb_ary_set(mrb, array, idx, inner) };
        }
        Ok(Self::new(array))
    }
}

impl TryFromMrb<Value> for Vec<Int> {
    type From = Ruby;
    type To = Rust;

    fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Array => {
                let inner = value.inner();
                let len = unsafe { mrb_sys_ary_len(inner) };
                let cap = usize::try_from(len).map_err(|_| Error {
                    from: Ruby::Array,
                    to: Rust::Vec,
                })?;
                let mut vec = Self::with_capacity(cap);
                for i in 0..cap {
                    let idx = i64::try_from(i).map_err(|_| Error {
                        from: Ruby::Array,
                        to: Rust::Vec,
                    })?;
                    let item = Value::new(unsafe { mrb_ary_ref(mrb, inner, idx) });
                    vec.push(Int::try_from_mrb(mrb, item)?);
                }
                Ok(vec)
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;
    use std::convert::TryFrom;

    use crate::convert::*;
    use crate::value::*;

    mod fixnum {
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn convert_to_value(v: Vec<i64>) -> bool {
            super::convert_to_value(v)
        }

        #[quickcheck]
        fn roundtrip(v: Vec<i64>) -> bool {
            super::roundtrip(v)
        }
    }

    #[test]
    fn fail_covert() {
        let mrb = unsafe { mrb_open() };
        let value = Value::new(unsafe { mrb_sys_true_value() });
        let expected = Error {
            from: Ruby::Bool,
            to: Rust::Vec,
        };
        let result = <Vec<i64>>::try_from_mrb(mrb, value).map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[allow(clippy::needless_pass_by_value)]
    fn convert_to_value<T>(v: Vec<T>) -> bool
    where
        T: Clone + PartialEq + TryFromMrb<Value, From = Ruby, To = Rust>,
        Value: TryFromMrb<Vec<T>, From = Rust, To = Ruby>,
        Vec<T>: Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
    {
        let mrb = unsafe { mrb_open() };
        let value = match Value::try_from_mrb(mrb, v.clone()) {
            Ok(value) => value,
            // we don't care about inner conversion failures for `T`
            Err(_) => return true,
        };
        let inner = value.inner();
        let size = i64::try_from(v.len()).expect("vec size");
        let good = unsafe { mrb_sys_ary_len(inner) } == size;
        unsafe { mrb_close(mrb) };
        good
    }

    #[allow(clippy::needless_pass_by_value)]
    fn roundtrip<T>(v: Vec<T>) -> bool
    where
        T: Clone + PartialEq + TryFromMrb<Value, From = Ruby, To = Rust>,
        Value: TryFromMrb<Vec<T>, From = Rust, To = Ruby>,
        Vec<T>: Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
    {
        let mrb = unsafe { mrb_open() };
        let value = match Value::try_from_mrb(mrb, v.clone()) {
            Ok(value) => value,
            // we don't care about inner conversion failures for `T`
            Err(_) => return true,
        };
        let good = <Vec<T>>::try_from_mrb(mrb, value).expect("convert") == v;
        unsafe { mrb_close(mrb) };
        good
    }
}
