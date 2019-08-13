use artichoke_backend::eval::Eval;
use artichoke_backend::types::Int;

#[no_mangle]
pub fn artichoke_wasm_eval() -> Int {
    let interp = if let Ok(interp) = artichoke_backend::interpreter() {
        interp
    } else {
        return -1;
    };
    let value = if let Ok(value) = interp.eval("10 * 10") {
        value
    } else {
        return -2;
    };
    value.try_into::<Int>().unwrap_or(-3)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
