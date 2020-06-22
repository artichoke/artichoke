use std::borrow::Cow;

use crate::core::Intern;
use crate::exception::Exception;
use crate::sys;
use crate::Artichoke;

impl Intern for Artichoke {
    type Symbol = sys::mrb_sym;
    type Error = Exception;

    fn intern_symbol<T>(&mut self, symbol: T) -> Result<Self::Symbol, Self::Error>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let symbol = symbol.into();
        let bytes = symbol.as_ref();
        let sym = unsafe {
            self.with_ffi_boundary(|mrb| {
                sys::mrb_intern(mrb, bytes.as_ptr() as *const i8, bytes.len())
            })?
        };
        Ok(sym)
    }

    fn lookup_symbol(&self, symbol: Self::Symbol) -> Option<&[u8]> {
        let _ = symbol;
        todo!("Implement Intern::lookup_symbol");
    }
}
