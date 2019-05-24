use crate::interpreter::Mrb;

#[allow(clippy::module_name_repetitions)]
pub trait MrbFile {
    fn require(interp: Mrb);
}
