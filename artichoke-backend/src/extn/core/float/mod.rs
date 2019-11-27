use artichoke_core::eval::Eval;

use crate::types;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Float>("Float", None, None);
    interp.eval(&include_bytes!("float.rb")[..])?;
    // TODO: Add proper constant defs to class::Spec, see GH-158.
    interp.eval(format!("class Float; EPSILON={} end", Float::EPSILON).as_bytes())?;
    Ok(())
}

pub struct Float;

impl Float {
    pub const EPSILON: types::Float = std::f64::EPSILON;
}
