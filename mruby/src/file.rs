use crate::interpreter::MrbApi;

pub trait File {
    fn require(api: &MrbApi);
}
