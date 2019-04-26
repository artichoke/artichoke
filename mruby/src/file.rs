use crate::interpreter::Mrb;

#[allow(clippy::module_name_repetitions)]
pub trait MrbFile
where
    Self: Sized,
{
    fn require(interp: Mrb)
    where
        Self: Sized;
}
