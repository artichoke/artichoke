#[macro_export]
macro_rules! mrb_array_impl {
    ($type:ty as $wrapper_module:ident) => {
        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<std::vec::Vec<$type>> for $crate::Value {
            type From = $crate::Rust;
            type To = $crate::Ruby;

            unsafe fn try_from_mrb(
                api: &$crate::interpreter::MrbApi,
                value: std::vec::Vec<$type>,
            ) -> std::result::Result<Self, $crate::convert::Error<Self::From, Self::To>> {
                let mut values = std::vec::Vec::with_capacity(value.len());
                for item in value {
                    values.push($crate::Value::try_from_mrb(api, item)?);
                }
                $crate::Value::try_from_mrb(api, values)
            }
        }

        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<$crate::Value> for std::vec::Vec<$type> {
            type From = $crate::Ruby;
            type To = $crate::Rust;

            unsafe fn try_from_mrb(
                api: &$crate::interpreter::MrbApi,
                value: $crate::Value,
            ) -> std::result::Result<Self, $crate::convert::Error<Self::From, Self::To>> {
                let values = <std::vec::Vec<$crate::Value>>::try_from_mrb(api, value)?;
                let mut vec = std::vec::Vec::with_capacity(values.len());
                for item in values {
                    vec.push(<$type>::try_from_mrb(api, item)?);
                }
                std::result::Result::Ok(vec)
            }
        }

        #[cfg(test)]
        mod $wrapper_module {
            mod tests {
                use quickcheck_macros::quickcheck;
                use std::convert::TryFrom;

                use $crate::convert::*;
                use $crate::interpreter::*;
                use $crate::sys::*;
                use $crate::value::*;

                #[test]
                fn fail_convert() {
                    unsafe {
                        let interp = Interpreter::create().expect("mrb init");
                        let api = interp.borrow_mut();
                        let value = api.bool(true).expect("convert");
                        let expected = Error {
                            from: Ruby::Bool,
                            to: Rust::Vec,
                        };
                        let result = <std::vec::Vec<$type>>::try_from_mrb(&api, value).map(|_| ());
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
                        let interp = Interpreter::create().expect("mrb init");
                        let api = interp.borrow_mut();
                        let value = match $crate::Value::try_from_mrb(&api, v.clone()) {
                            std::result::Result::Ok(value) => value,
                            // we don't care about inner conversion failures for `T`
                            std::result::Result::Err(_) => return true,
                        };
                        let inner = value.inner();
                        let size = i64::try_from(v.len()).expect("vec size");
                        mrb_sys_ary_len(inner) == size
                    }
                }

                #[allow(clippy::needless_pass_by_value)]
                #[quickcheck]
                fn roundtrip(v: std::vec::Vec<$type>) -> bool
                where
                    $type: std::clone::Clone
                        + std::cmp::PartialEq
                        + TryFromMrb<Value, From = Ruby, To = Rust>,
                    $crate::Value: TryFromMrb<std::vec::Vec<$type>, From = Rust, To = Ruby>,
                    std::vec::Vec<$type>:
                        std::clone::Clone + TryFromMrb<Value, From = Ruby, To = Rust>,
                {
                    unsafe {
                        let interp = Interpreter::create().expect("mrb init");
                        let api = interp.borrow_mut();
                        let value = match $crate::Value::try_from_mrb(&api, v.clone()) {
                            std::result::Result::Ok(value) => value,
                            // we don't care about inner conversion failures for `T`
                            std::result::Result::Err(_) => return true,
                        };
                        <std::vec::Vec<$type>>::try_from_mrb(&api, value).expect("convert") == v
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! mrb_nilable_impl {
    ($type:ty as $wrapper_module:ident) => {
        mrb_nilable_impl!($type as $wrapper_module with eq = |a: $type, b: $type| a == b);
    };
    ($type:ty as $wrapper_module:ident with eq = $eq:expr) => {
        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<std::option::Option<$type>> for $crate::Value {
            type From = $crate::Rust;
            type To = $crate::Ruby;

            unsafe fn try_from_mrb(
                api: &$crate::interpreter::MrbApi,
                value: std::option::Option<$type>,
            ) -> std::result::Result<Self, $crate::convert::Error<Self::From, Self::To>> {
                match value {
                    std::option::Option::Some(value) => Self::try_from_mrb(api, value),
                    std::option::Option::None => Self::try_from_mrb(api, None as Option<Value>),
                }
            }
        }

        #[allow(clippy::use_self)]
        impl $crate::TryFromMrb<$crate::Value> for std::option::Option<$type> {
            type From = $crate::Ruby;
            type To = $crate::Rust;

            unsafe fn try_from_mrb(
                api: &$crate::interpreter::MrbApi,
                value: $crate::Value,
            ) -> std::result::Result<Self, $crate::convert::Error<Self::From, Self::To>> {
                let value = <std::option::Option<Value>>::try_from_mrb(api, value)?;
                let value = if let std::option::Option::Some(item) = value {
                    std::option::Option::Some(<$type>::try_from_mrb(api, item)?)
                } else {
                    std::option::Option::None
                };
                std::result::Result::Ok(value)
            }
        }

        #[cfg(test)]
        mod $wrapper_module {
            mod tests {
                use quickcheck_macros::quickcheck;

                use $crate::convert::*;
                use $crate::interpreter::*;
                use $crate::sys::*;
                use $crate::value::*;

                #[test]
                fn fail_convert() {
                    unsafe {
                        let interp = Interpreter::create().expect("mrb init");
                        let api = interp.borrow_mut();
                        let context = mrbc_context_new(api.mrb());
                        // get a mrb_value that can't be converted to a
                        // primitive type.
                        let code = "Object.new";
                        let value = mrb_load_nstring_cxt(
                            api.mrb(),
                            code.as_ptr() as *const i8,
                            code.len(),
                            context,
                        );
                        let value = Value::new(value);
                        let result =
                            <std::option::Option<$type>>::try_from_mrb(&api, value)
                                .map(|_| ());
                        assert_eq!(
                            result.map_err(|e| e.from),
                            std::result::Result::Err(Ruby::Object)
                        );
                    }
                }

                #[allow(clippy::clone_on_copy)]
                #[allow(clippy::redundant_closure_call)]
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
                        let interp = Interpreter::create().expect("mrb init");
                        let api = interp.borrow_mut();
                        let value = match Value::try_from_mrb(&api, v.clone()) {
                            std::result::Result::Ok(value) => value,
                            // we don't care about inner conversion failures for `T`
                            std::result::Result::Err(_) => return true,
                        };
                        if let std::option::Option::Some(v) = v {
                            let value = <$type>::try_from_mrb(&api, value).expect("convert");
                            ($eq)(value, v)
                        } else {
                            let inner = value.inner();
                            mrb_sys_value_is_nil(inner)
                        }
                    }
                }

                #[allow(clippy::clone_on_copy)]
                #[allow(clippy::redundant_closure_call)]
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
                        let interp = Interpreter::create().expect("mrb init");
                        let api = interp.borrow_mut();
                        let value = match Value::try_from_mrb(&api, v.clone()) {
                            std::result::Result::Ok(value) => value,
                            // we don't care about inner conversion failures for `T`
                            std::result::Result::Err(_) => return true,
                        };
                        let value = <std::option::Option<$type>>::try_from_mrb(&api, value).expect("convert");
                        if value.is_none() && v.is_none() {
                            true
                        } else {
                            ($eq)(value.unwrap(), v.unwrap())
                        }
                    }
                }
            }
        }
    };
}
