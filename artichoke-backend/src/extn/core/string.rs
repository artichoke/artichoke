use artichoke_core::eval::Eval;

use crate::class;
use crate::convert::TryConvert;
use crate::extn::core::exception::{self, ArgumentError, Fatal};
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::{Artichoke, BootError};

mod scan;

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().class_spec::<RString>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("String", None, None)?;
    class::Builder::for_spec(interp, &spec)
        .add_method("ord", RString::ord, sys::mrb_args_none())?
        .add_method("scan", RString::scan, sys::mrb_args_req(1))?
        .define()?;
    interp.0.borrow_mut().def_class::<RString>(spec);
    let _ = interp.eval(&include_bytes!("string.rb")[..])?;
    trace!("Patched String onto interpreter");
    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub struct RString;

impl RString {
    unsafe extern "C" fn ord(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        if let Ok(s) = value.try_into::<&str>() {
            if let Some(first) = s.chars().next() {
                // One UTF-8 character, which are at most 32 bits.
                if let Ok(value) = interp.try_convert(first as u32) {
                    value.inner()
                } else {
                    let exception = ArgumentError::new(&interp, "Unicode out of range");
                    exception::raise(interp, exception)
                }
            } else {
                let exception = ArgumentError::new(&interp, "empty string");
                exception::raise(interp, exception)
            }
        } else {
            let exception = Fatal::new(&interp, "failed to convert String receiver to Rust String");
            exception::raise(interp, exception)
        }
    }

    unsafe extern "C" fn scan(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let (pattern, block) = mrb_get_args!(mrb, required = 1, &block);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = scan::method(&interp, value, Value::new(&interp, pattern), block);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }
}

// Tests from String core docs in Ruby 2.6.3
// https://ruby-doc.org/core-2.6.3/String.html
#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use artichoke_core::value::Value as _;

    use crate::convert::Convert;
    use crate::extn::core::string;

    #[test]
    fn string_equal_squiggle() {
        let interp = crate::interpreter().expect("init");
        string::init(&interp).expect("string init");

        let value = interp.eval(br#""cat o' 9 tails" =~ /\d/"#).unwrap();
        assert_eq!(value.try_into::<Option<i64>>(), Ok(Some(7)));
        let value = interp.eval(br#""cat o' 9 tails" =~ 9"#).unwrap();
        assert_eq!(value.try_into::<Option<i64>>(), Ok(None));
    }

    #[test]
    fn string_idx() {
        let interp = crate::interpreter().expect("init");
        string::init(&interp).expect("string init");

        assert_eq!(
            &interp
                .eval(br"'hello there'[/[aeiou](.)\1/]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "ell"
        );
        assert_eq!(
            &interp
                .eval(br"'hello there'[/[aeiou](.)\1/, 0]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "ell"
        );
        assert_eq!(
            &interp
                .eval(br"'hello there'[/[aeiou](.)\1/, 1]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "l"
        );
        assert_eq!(
            interp
                .eval(br"'hello there'[/[aeiou](.)\1/, 2]")
                .unwrap()
                .try_into::<Option<String>>()
                .unwrap(),
            None
        );
        assert_eq!(
            &interp
                .eval(br"'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'non_vowel']")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "l"
        );
        assert_eq!(
            &interp
                .eval(br"'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'vowel']")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "e"
        );
    }

    #[test]
    fn string_scan() {
        let interp = crate::interpreter().expect("init");
        string::init(&interp).expect("string init");

        let s = interp.convert("abababa");
        let result = s
            .funcall::<Vec<&str>>("scan", &[interp.eval(b"/./").expect("eval")], None)
            .expect("funcall");
        assert_eq!(result, vec!["a", "b", "a", "b", "a", "b", "a"]);
        let result = s
            .funcall::<Vec<&str>>("scan", &[interp.eval(b"/../").expect("eval")], None)
            .expect("funcall");
        assert_eq!(result, vec!["ab", "ab", "ab"]);
        let result = s
            .funcall::<Vec<&str>>("scan", &[interp.eval(b"'aba'").expect("eval")], None)
            .expect("funcall");
        assert_eq!(result, vec!["aba", "aba"]);
        let result = s
            .funcall::<Vec<&str>>("scan", &[interp.eval(b"'no no no'").expect("eval")], None)
            .expect("funcall");
        assert_eq!(result, <Vec<&str>>::new());
    }

    #[test]
    fn string_unary_minus() {
        let interp = crate::interpreter().expect("init");
        string::init(&interp).expect("string init");

        let s = interp.eval(b"-'abababa'").expect("eval");
        let result = s.funcall::<bool>("frozen?", &[], None);
        assert_eq!(result, Ok(true));
        let result = s.funcall::<&str>("itself", &[], None);
        assert_eq!(result, Ok("abababa"));
    }
}
