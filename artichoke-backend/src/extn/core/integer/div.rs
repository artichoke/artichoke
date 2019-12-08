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

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use artichoke_core::value::Value;
    use quickcheck_macros::quickcheck;

    use crate::types::Int;

    #[quickcheck]
    fn integer_division_vm_opcode(x: Int, y: Int) -> bool {
        let interp = crate::interpreter().expect("init");
        let mut result = true;
        match (x, y) {
            (0, 0) => result &= interp.eval(b"0 / 0").is_err(),
            (x, 0) | (0, x) => {
                let expr = format!("{} / 0", x).into_bytes();
                result &= interp.eval(expr.as_slice()).is_err();
                let expr = format!("0 / {}", x).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .and_then(Value::try_into::<Int>);
                result &= division == Ok(0)
            }
            (x, y) => {
                let expr = format!("{} / {}", x, y).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .and_then(Value::try_into::<Int>);
                result &= division == Ok(x / y)
            }
        }
        result
    }
}
