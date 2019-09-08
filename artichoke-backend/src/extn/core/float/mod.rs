use crate::eval::Eval;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Float>("Float", None, None);
    interp.eval(include_str!("float.rb"))?;
    // TODO: Add proper constant defs to class::Spec, see GH-158.
    interp.eval(format!(
        "class Float; EPSILON={} end",
        Float::EPSILON,
    ))?;
    Ok(())
}

pub struct Float;

impl Float {
    pub const EPSILON: f64 = std::f64::EPSILON;
}
