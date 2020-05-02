use std::borrow::Cow;

use crate::core::Intern;
use crate::sys;
use crate::Artichoke;

impl Intern for Artichoke {
    type Symbol = sys::mrb_sym;

    fn intern_symbol<T>(&mut self, symbol: T) -> Self::Symbol
    where
        T: Into<Cow<'static, [u8]>>,
    {
        match symbol.into() {
            Cow::Borrowed(bytes) => unsafe {
                sys::mrb_intern(
                    self.0.borrow().mrb,
                    bytes.as_ptr() as *const i8,
                    bytes.len(),
                )
            },
            Cow::Owned(bytes) => unsafe {
                sys::mrb_intern(
                    self.0.borrow().mrb,
                    bytes.as_ptr() as *const i8,
                    bytes.len(),
                )
            },
        }
    }

    fn lookup_symbol(&self, symbol: Self::Symbol) -> Option<&[u8]> {
        let _ = symbol;
        todo!("Implement Intern::lookup_symbol");
    }
}
