use std::ffi::c_void;

use crate::convert::{Immediate, UnboxedValueGuard};
use crate::extn::core::array::Array;
use crate::extn::prelude::*;
use crate::intern::Symbol as SymbolId;

pub mod ffi;
pub mod mruby;
pub mod trampoline;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(SymbolId);

impl Symbol {
    pub fn all_symbols(interp: &mut Artichoke) -> Result<Array, Exception> {
        let all_symbols = interp.all_symbols()?.map(Symbol::from).collect::<Vec<_>>();
        let array = all_symbols
            .into_iter()
            .map(|symbol| Symbol::alloc_value(symbol, interp))
            .collect::<Result<Vec<Value>, _>>()?;
        Ok(Array::from(array))
    }

    #[inline]
    #[must_use]
    pub fn id(self) -> SymbolId {
        self.0
    }

    #[must_use]
    pub fn is_empty(self, interp: &mut Artichoke) -> bool {
        if let Ok(Some(bytes)) = interp.lookup_symbol(self.into()) {
            bytes.is_empty()
        } else {
            true
        }
    }

    #[must_use]
    pub fn len(self, interp: &mut Artichoke) -> usize {
        if let Ok(Some(bytes)) = interp.lookup_symbol(self.into()) {
            bytes.len()
        } else {
            0_usize
        }
    }

    #[must_use]
    pub fn bytes<'a>(&self, interp: &'a mut Artichoke) -> &'a [u8] {
        if let Ok(Some(bytes)) = interp.lookup_symbol(self.into()) {
            bytes
        } else {
            &[]
        }
    }
}

impl From<SymbolId> for Symbol {
    fn from(id: SymbolId) -> Self {
        Self(id)
    }
}

impl From<intaglio::Symbol> for Symbol {
    fn from(id: intaglio::Symbol) -> Self {
        Self(id.into())
    }
}

impl From<u32> for Symbol {
    fn from(id: u32) -> Self {
        Self(id.into())
    }
}

impl From<Symbol> for SymbolId {
    fn from(sym: Symbol) -> Self {
        sym.id()
    }
}

impl From<&Symbol> for SymbolId {
    fn from(sym: &Symbol) -> Self {
        sym.id()
    }
}

impl From<Symbol> for u32 {
    fn from(sym: Symbol) -> Self {
        sym.id().into()
    }
}

impl From<Symbol> for usize {
    fn from(sym: Symbol) -> Self {
        sym.id().into()
    }
}

impl BoxUnboxVmValue for Symbol {
    type Unboxed = Self;
    type Guarded = Immediate<Self::Unboxed>;

    const RUBY_TYPE: &'static str = "Symbol";

    unsafe fn unbox_from_value<'a>(
        value: &'a mut Value,
        interp: &mut Artichoke,
    ) -> Result<UnboxedValueGuard<'a, Self::Guarded>, Exception> {
        let _ = interp;

        // Make sure we have a Symbol otherwise extraction will fail.
        // This check is critical to the safety of accessing the `value` union.
        if value.ruby_type() != Ruby::Symbol {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        // Safety:
        //
        // The above check on the data type ensures the `value` union holds a
        // `u32` in the `sym` variant.
        let value = value.inner();
        let symbol_id = value.value.sym;
        Ok(UnboxedValueGuard::new(Immediate::new(symbol_id.into())))
    }

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Exception> {
        let _ = interp;

        let symbol_id = u32::from(value);
        let obj = unsafe { sys::mrb_sys_new_symbol(symbol_id) };
        Ok(Value::from(obj))
    }

    fn box_into_value(
        value: Self::Unboxed,
        into: Value,
        interp: &mut Artichoke,
    ) -> Result<Value, Exception> {
        let _ = value;
        let _ = into;
        let _ = interp;
        Err(Fatal::from("Symbols are immutable and cannot be reinitialized").into())
    }

    fn free(data: *mut c_void) {
        // this function is never called. `Symbol` is `Copy`/immediate and does
        // not have a destructor registered in the class registry.
        let _ = data;
    }
}
