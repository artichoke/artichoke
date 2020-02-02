use crate::extn::core::float::Float;
use crate::extn::prelude::*;
use crate::types;

pub fn method(interp: &mut Artichoke, value: Value, other: Value) -> Result<Value, Exception> {
    let x = value.try_into::<Int>().map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Integer from Ruby Integer receiver",
        )
    })?;
    let pretty_name = other.pretty_name();
    if let Ok(y) = other.clone().try_into::<Int>() {
        if y == 0 {
            Err(Exception::from(ZeroDivisionError::new(
                interp,
                "divided by 0",
            )))
        } else {
            Ok(interp.convert(x / y))
        }
    } else if let Ok(y) = other.try_into::<types::Float>() {
        if y == 0.0 {
            match x {
                x if x > 0 => Ok(interp.convert_mut(Float::INFINITY)),
                x if x < 0 => Ok(interp.convert_mut(Float::NEG_INFINITY)),
                _ => Ok(interp.convert_mut(Float::NAN)),
            }
        } else {
            #[allow(clippy::cast_precision_loss)]
            Ok(interp.convert_mut(x as types::Float / y))
        }
    } else {
        Err(Exception::from(TypeError::new(
            interp,
            format!("{} can't be coerced into Integer", pretty_name),
        )))
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::test::prelude::*;

    #[quickcheck]
    fn integer_division_vm_opcode(x: Int, y: Int) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let mut result = true;
        match (x, y) {
            (0, 0) => result &= interp.eval(b"0 / 0").is_err(),
            (x, 0) | (0, x) => {
                let expr = format!("{} / 0", x).into_bytes();
                result &= interp.eval(expr.as_slice()).is_err();
                let expr = format!("0 / {}", x).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .unwrap()
                    .try_into::<Int>()
                    .unwrap();
                result &= division == 0
            }
            (x, y) => {
                let expr = format!("{} / {}", x, y).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .unwrap()
                    .try_into::<Int>()
                    .unwrap();
                result &= division == x / y
            }
        }
        result
    }

    #[quickcheck]
    fn integer_division_send(x: Int, y: Int) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let mut result = true;
        match (x, y) {
            (0, 0) => result &= interp.eval(b"0.send('/', 0)").is_err(),
            (x, 0) | (0, x) => {
                let expr = format!("{}.send('/', 0)", x).into_bytes();
                result &= interp.eval(expr.as_slice()).is_err();
                let expr = format!("0.send('/', {})", x).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .unwrap()
                    .try_into::<Int>()
                    .unwrap();
                result &= division == 0
            }
            (x, y) => {
                let expr = format!("{}.send('/', {})", x, y).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .unwrap()
                    .try_into::<Int>()
                    .unwrap();
                result &= division == x / y
            }
        }
        result
    }
}
