use artichoke_core::coerce_to_numeric::CoerceToNumeric;
use artichoke_core::convert::TryConvert;
use artichoke_core::debug::Debug;
use artichoke_core::eval::Eval;
use artichoke_core::value::Value as _;
use spinoso_exception::TypeError;

use crate::types::Ruby;
use crate::value::Value;
use crate::{Artichoke, Error};

impl CoerceToNumeric for Artichoke {
    type Value = Value;

    type Float = f64;

    type Error = Error;

    #[allow(clippy::cast_precision_loss)]
    fn coerce_to_float(&mut self, value: Self::Value) -> Result<Self::Float, Self::Error> {
        match value.ruby_type() {
            Ruby::Float => return value.try_convert_into(self),
            Ruby::Fixnum => return value.try_convert_into::<i64>(self).map(|int| int as f64),
            Ruby::Nil => return Err(TypeError::with_message("can't convert nil into Float").into()),
            _ => {}
        }
        // TODO: This branch should use `numeric::coerce`
        let class_of_numeric = self.eval(b"Numeric")?;
        let is_a_numeric = value.funcall(self, "is_a?", &[class_of_numeric], None)?;
        let is_a_numeric = self.try_convert(is_a_numeric);
        if let Ok(true) = is_a_numeric {
            if !value.respond_to(self, "to_f")? {
                let mut message = String::from("can't convert ");
                message.push_str(self.inspect_type_name_for_value(value));
                message.push_str(" into Float");
                return Err(TypeError::from(message).into());
            }
            let coerced = value.funcall(self, "to_f", &[], None)?;
            if let Ruby::Float = coerced.ruby_type() {
                coerced.try_convert_into::<f64>(self)
            } else {
                let mut message = String::from("can't convert ");
                let name = self.inspect_type_name_for_value(value);
                message.push_str(name);
                message.push_str(" into Float (");
                message.push_str(name);
                message.push_str("#to_f gives ");
                message.push_str(self.inspect_type_name_for_value(coerced));
                message.push(')');
                Err(TypeError::from(message).into())
            }
        } else {
            let mut message = String::from("can't convert ");
            message.push_str(self.inspect_type_name_for_value(value));
            message.push_str(" into Float");
            Err(TypeError::from(message).into())
        }
    }
}
