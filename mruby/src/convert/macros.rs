#[macro_export]
macro_rules! mrb_array_impl {
    ($type:ty as $wrapper_module:ident) => {
        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<std::vec::Vec<$type>> for $crate::Value {
            type From = $crate::Rust;
            type To = $crate::Ruby;

            unsafe fn try_from_mrb(
                mrb: *mut $crate::sys::mrb_state,
                value: std::vec::Vec<$type>,
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
                std::result::Result::Ok(Self::new(array))
            }
        }

        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<$crate::Value> for std::vec::Vec<$type> {
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
                        std::result::Result::Ok(vec)
                    }
                    type_tag => std::result::Result::Err($crate::convert::Error {
                        from: type_tag,
                        to: $crate::Rust::Vec,
                    }),
                }
            }
        }

        #[cfg(test)]
        mod $wrapper_module {
            mod tests {
                use quickcheck_macros::quickcheck;
                use std::convert::TryFrom;

                use $crate::convert::*;
                use $crate::value::*;

                #[test]
                fn fail_convert() {
                    unsafe {
                        let mrb = mrb_open();
                        let value = Value::new(mrb_sys_true_value());
                        let expected = Error {
                            from: Ruby::Bool,
                            to: Rust::Vec,
                        };
                        let result = <std::vec::Vec<$type>>::try_from_mrb(mrb, value).map(|_| ());
                        mrb_close(mrb);
                        assert_eq!(result, Err(expected));
                    }
                }

                #[allow(clippy::needless_pass_by_value)]
                #[quickcheck]
                fn convert_to_value(v: std::vec::Vec<$type>) -> bool
                where
                    $type: std::clone::Clone
                        + std::cmp::PartialEq
                        + TryFromMrb<Value, From = Ruby, To = Rust>,
                    Value: TryFromMrb<std::vec::Vec<$type>, From = Rust, To = Ruby>,
                    std::vec::Vec<$type>:
                        std::clone::Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
                {
                    unsafe {
                        let mrb = mrb_open();
                        let value = match Value::try_from_mrb(mrb, v.clone()) {
                            std::result::Result::Ok(value) => value,
                            // we don't care about inner conversion failures for `T`
                            std::result::Result::Err(_) => return true,
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
                fn roundtrip(v: std::vec::Vec<$type>) -> bool
                where
                    $type: std::clone::Clone
                        + std::cmp::PartialEq
                        + TryFromMrb<Value, From = Ruby, To = Rust>,
                    Value: TryFromMrb<std::vec::Vec<$type>, From = Rust, To = Ruby>,
                    std::vec::Vec<$type>:
                        std::clone::Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
                {
                    unsafe {
                        let mrb = mrb_open();
                        let value = match Value::try_from_mrb(mrb, v.clone()) {
                            std::result::Result::Ok(value) => value,
                            // we don't care about inner conversion failures for `T`
                            std::result::Result::Err(_) => return true,
                        };
                        let good =
                            <std::vec::Vec<$type>>::try_from_mrb(mrb, value).expect("convert") == v;
                        mrb_close(mrb);
                        good
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! mrb_nilable_impl {
    ($type:ty as $wrapper_module:ident) => {
        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<std::option::Option<$type>> for $crate::Value {
            type From = $crate::Rust;
            type To = $crate::Ruby;

            unsafe fn try_from_mrb(
                mrb: *mut $crate::sys::mrb_state,
                value: std::option::Option<$type>,
            ) -> std::result::Result<Self, $crate::convert::Error<Self::From, Self::To>> {
                match value {
                    std::option::Option::Some(value) => Self::try_from_mrb(mrb, value),
                    std::option::Option::None => {
                        std::result::Result::Ok(Self::new($crate::sys::mrb_sys_nil_value()))
                    }
                }
            }
        }

        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<$crate::Value> for std::option::Option<$type> {
            type From = $crate::Ruby;
            type To = $crate::Rust;

            unsafe fn try_from_mrb(
                mrb: *mut $crate::sys::mrb_state,
                value: $crate::Value,
            ) -> std::result::Result<Self, $crate::convert::Error<Self::From, Self::To>> {
                match value.ruby_type() {
                    $crate::Ruby::Nil => std::result::Result::Ok(None),
                    _ => <$type>::try_from_mrb(mrb, value).map(std::option::Option::Some),
                }
            }
        }

        #[cfg(test)]
        mod $wrapper_module {
            mod tests {
                use quickcheck_macros::quickcheck;

                use $crate::convert::*;
                use $crate::value::*;

                #[test]
                fn fail_convert() {
                    unsafe {
                        let mrb = mrb_open();
                        let context = mrbc_context_new(mrb);
                        // get a mrb_value that can't be converted to a
                        // primitive type.
                        let code = "Object.new";
                        let value = mrb_load_nstring_cxt(
                            mrb,
                            code.as_ptr() as *const i8,
                            code.len(),
                            context,
                        );
                        let value = Value::new(value);
                        let result =
                            <std::option::Option<$type>>::try_from_mrb(mrb, value).map(|_| ());
                        mrb_close(mrb);
                        assert_eq!(
                            result.map_err(|e| e.from),
                            std::result::Result::Err(Ruby::Object)
                        );
                    }
                }

                #[allow(clippy::clone_on_copy)]
                #[quickcheck]
                fn convert_to_value(v: std::option::Option<$type>) -> bool
                where
                    $type: std::clone::Clone
                        + std::cmp::PartialEq
                        + TryFromMrb<Value, From = Ruby, To = Rust>,
                    Value: TryFromMrb<std::option::Option<$type>, From = Rust, To = Ruby>,
                    std::option::Option<$type>:
                        std::clone::Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
                {
                    unsafe {
                        let mrb = mrb_open();
                        let value = match Value::try_from_mrb(mrb, v.clone()) {
                            std::result::Result::Ok(value) => value,
                            // we don't care about inner conversion failures for `T`
                            std::result::Result::Err(_) => return true,
                        };
                        let good = if let std::option::Option::Some(v) = v {
                            <$type>::try_from_mrb(mrb, value).expect("convert") == v
                        } else {
                            let inner = value.inner();
                            mrb_sys_value_is_nil(inner)
                        };
                        mrb_close(mrb);
                        good
                    }
                }

                #[allow(clippy::clone_on_copy)]
                #[quickcheck]
                fn roundtrip(v: std::option::Option<$type>) -> bool
                where
                    $type: std::clone::Clone
                        + std::cmp::PartialEq
                        + TryFromMrb<Value, From = Ruby, To = Rust>,
                    Value: TryFromMrb<std::option::Option<$type>, From = Rust, To = Ruby>,
                    std::option::Option<$type>:
                        std::clone::Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
                {
                    unsafe {
                        let mrb = mrb_open();
                        let value = match Value::try_from_mrb(mrb, v.clone()) {
                            std::result::Result::Ok(value) => value,
                            // we don't care about inner conversion failures for `T`
                            std::result::Result::Err(_) => return true,
                        };
                        let good = <std::option::Option<$type>>::try_from_mrb(mrb, value)
                            .expect("convert")
                            == v;
                        mrb_close(mrb);
                        good
                    }
                }
            }
        }
    };
}
