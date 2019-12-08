use artichoke_core::value::Value as _;

use crate::convert::Convert;
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::types::{Float, Int};
use crate::value::Value;
use crate::Artichoke;

pub fn method(
    interp: &Artichoke,
    value: Value,
    other: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let x = value.try_into::<Int>().map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Integer from Ruby Integer receiver",
        )
    })?;
    let pretty_name = other.pretty_name();
    if let Ok(y) = other.clone().try_into::<Int>() {
        Ok(interp.convert(x / y))
    } else if let Ok(y) = other.try_into::<Float>() {
        #[allow(clippy::cast_precision_loss)]
        Ok(interp.convert(x as Float / y))
    } else {
        Err(Box::new(TypeError::new(
            interp,
            format!("{} can't be coerced into Integer", pretty_name),
        )))
    }
}
