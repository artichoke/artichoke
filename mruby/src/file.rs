use crate::interpreter::Mrb;

pub trait File {
    fn require(interp: Mrb);
}
