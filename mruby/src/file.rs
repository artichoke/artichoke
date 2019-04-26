use crate::interpreter::Mrb;

pub trait MrbFile
where
    Self: Sized,
{
    fn require(interp: Mrb)
    where
        Self: Sized;
}
