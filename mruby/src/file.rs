use crate::interpreter::Mrb;

pub trait File {
    fn require(mrb: Mrb);
}
