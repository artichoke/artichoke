use std::borrow::Cow;
use std::collections::HashMap;

use crate::extn::prelude::*;

pub mod memory;
pub mod system;

pub trait EnvType {
    fn get<'a>(
        &'a self,
        interp: &Artichoke,
        name: &[u8],
    ) -> Result<Option<Cow<'a, [u8]>>, Exception>;

    fn put(
        &mut self,
        interp: &Artichoke,
        name: &[u8],
        value: Option<&[u8]>,
    ) -> Result<(), Exception>;

    fn as_map(&self, interp: &Artichoke) -> Result<HashMap<Vec<u8>, Vec<u8>>, Exception>;
}
