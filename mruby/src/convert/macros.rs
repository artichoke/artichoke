#[macro_export]
macro_rules! mrb_vec_impl {
    ($type:ty as $wrapper_module:ident) => {
        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<std::vec::Vec<$type>> for $crate::Value {
            type From = $crate::Rust;
            type To = $crate::Ruby;

            unsafe fn try_from_mrb(
                mrb: *mut $crate::sys::mrb_state,
                value: Vec<$type>,
            ) -> std::result::Result<Self, $crate::convert::Error<Self::From, Self::To>> {
                use std::convert::TryFrom;
                let size = i64::try_from(value.len()).map_err(|_| $crate::convert::Error {
                    from: $crate::Rust::Vec,
                    to: $crate::Ruby::Array,
                })?;
                let array = $crate::sys::mrb_ary_new_capa(mrb, size);
                for (i, item) in value.into_iter().enumerate() {
                    let idx = i64::try_from(i).map_err(|_| $crate::convert::Error {
                        from: $crate::Rust::Vec,
                        to: $crate::Ruby::Array,
                    })?;
                    let ary_item = Self::try_from_mrb(mrb, item)?;
                    let inner = ary_item.inner();
                    $crate::sys::mrb_ary_set(mrb, array, idx, inner);
                }
                Ok(Self::new(array))
            }
        }

        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<$crate::Value> for Vec<$type> {
            type From = $crate::Ruby;
            type To = $crate::Rust;

            unsafe fn try_from_mrb(
                mrb: *mut $crate::sys::mrb_state,
                value: $crate::Value,
            ) -> std::result::Result<Self, $crate::convert::Error<Self::From, Self::To>> {
                use std::convert::TryFrom;
                match value.ruby_type() {
                    $crate::Ruby::Array => {
                        let inner = value.inner();
                        let len = $crate::sys::mrb_sys_ary_len(inner);
                        let cap = usize::try_from(len).map_err(|_| $crate::convert::Error {
                            from: $crate::Ruby::Array,
                            to: $crate::Rust::Vec,
                        })?;
                        let mut vec = Self::with_capacity(cap);
                        for i in 0..cap {
                            let idx = i64::try_from(i).map_err(|_| $crate::convert::Error {
                                from: $crate::Ruby::Array,
                                to: $crate::Rust::Vec,
                            })?;
                            let item =
                                $crate::Value::new($crate::sys::mrb_ary_ref(mrb, inner, idx));
                            vec.push(<$type>::try_from_mrb(mrb, item)?);
                        }
                        Ok(vec)
                    }
                    type_tag => Err($crate::convert::Error {
                        from: type_tag,
                        to: $crate::Rust::Vec,
                    }),
                }
            }
        }

        #[cfg(test)]
        mod $wrapper_module {
            use quickcheck_macros::quickcheck;
            use std::convert::TryFrom;

            use $crate::convert::*;
            use $crate::value::*;

            #[test]
            fn fail_covert() {
                unsafe {
                    let mrb = mrb_open();
                    let value = Value::new(mrb_sys_true_value());
                    let expected = Error {
                        from: Ruby::Bool,
                        to: Rust::Vec,
                    };
                    let result = <Vec<i64>>::try_from_mrb(mrb, value).map(|_| ());
                    mrb_close(mrb);
                    assert_eq!(result, Err(expected));
                }
            }

            #[allow(clippy::needless_pass_by_value)]
            #[quickcheck]
            fn convert_to_value(v: Vec<$type>) -> bool
            where
                $type: Clone + PartialEq + TryFromMrb<Value, From = Ruby, To = Rust>,
                Value: TryFromMrb<Vec<$type>, From = Rust, To = Ruby>,
                Vec<$type>: Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
            {
                unsafe {
                    let mrb = mrb_open();
                    let value = match Value::try_from_mrb(mrb, v.clone()) {
                        Ok(value) => value,
                        // we don't care about inner conversion failures for `T`
                        Err(_) => return true,
                    };
                    let inner = value.inner();
                    let size = i64::try_from(v.len()).expect("vec size");
                    let good = mrb_sys_ary_len(inner) == size;
                    mrb_close(mrb);
                    good
                }
            }

            #[allow(clippy::needless_pass_by_value)]
            #[quickcheck]
            fn roundtrip(v: Vec<$type>) -> bool
            where
                $type: Clone + PartialEq + TryFromMrb<Value, From = Ruby, To = Rust>,
                Value: TryFromMrb<Vec<$type>, From = Rust, To = Ruby>,
                Vec<$type>: Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
            {
                unsafe {
                    let mrb = mrb_open();
                    let value = match Value::try_from_mrb(mrb, v.clone()) {
                        Ok(value) => value,
                        // we don't care about inner conversion failures for `T`
                        Err(_) => return true,
                    };
                    let good = <Vec<$type>>::try_from_mrb(mrb, value).expect("convert") == v;
                    mrb_close(mrb);
                    good
                }
            }
        }
    };
}
