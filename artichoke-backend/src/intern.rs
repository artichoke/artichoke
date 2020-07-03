use intaglio::bytes::AllSymbols;
use intaglio::SymbolOverflowError;
use std::borrow::Cow;

use crate::class_registry::ClassRegistry;
use crate::core::ConvertMut;
use crate::core::Intern;
use crate::exception::{Exception, RubyException};
use crate::extn::core::exception::Fatal;
use crate::ffi::InterpreterExtractError;
use crate::sys;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(intaglio::Symbol);

impl From<u32> for Symbol {
    fn from(sym: u32) -> Self {
        Self(sym.into())
    }
}

impl From<intaglio::Symbol> for Symbol {
    fn from(sym: intaglio::Symbol) -> Self {
        Self(sym)
    }
}

impl From<Symbol> for intaglio::Symbol {
    fn from(sym: Symbol) -> Self {
        sym.0
    }
}

impl From<Symbol> for u32 {
    fn from(sym: Symbol) -> Self {
        sym.0.into()
    }
}

impl From<Symbol> for u64 {
    fn from(sym: Symbol) -> Self {
        sym.0.into()
    }
}

impl From<Symbol> for usize {
    fn from(sym: Symbol) -> Self {
        sym.0.into()
    }
}

impl Artichoke {
    /// Return a type that yields all `Symbol`s in the interpreter.
    pub fn all_symbols(&self) -> Result<AllSymbols<'_>, InterpreterExtractError> {
        let state = self.state.as_ref().ok_or(InterpreterExtractError)?;
        // this method cannot be part of the `Intern` impl because `AllSymbols`
        // is generic over the lifetime of the symbol table and GATs are not
        // stable.
        Ok(state.symbols.all_symbols())
    }

    pub fn lookup_symbol_with_trailing_nul(
        &self,
        symbol: Symbol,
    ) -> Result<Option<&[u8]>, Exception> {
        let state = self.state.as_ref().ok_or(InterpreterExtractError)?;
        let symbol = u32::from(symbol);
        if let Some(symbol) = symbol.checked_sub(1) {
            if let Some(bytes) = state.symbols.get(symbol.into()) {
                Ok(Some(bytes))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Intern for Artichoke {
    type Symbol = Symbol;

    type Error = Exception;

    fn intern_bytes<T>(&mut self, bytes: T) -> Result<Self::Symbol, Self::Error>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let bytes = bytes.into();
        let state = self.state.as_mut().ok_or(InterpreterExtractError)?;
        let trailing_byte = bytes.len().checked_sub(1).and_then(|idx| bytes.get(idx));
        let symbol = if let Some(&b'\0') = trailing_byte {
            state.symbols.intern(bytes)?
        } else {
            let mut bytes = bytes.into_owned();
            bytes.push(b'\0');
            state.symbols.intern(bytes)?
        };
        let symbol = u32::from(symbol);
        // mruby expexts symbols to be non-zero.
        let symbol = symbol.checked_add(1).ok_or_else(SymbolOverflowError::new)?;
        Ok(symbol.into())
    }

    fn check_interned_bytes(&self, bytes: &[u8]) -> Result<Option<Self::Symbol>, Self::Error> {
        let state = self.state.as_ref().ok_or(InterpreterExtractError)?;
        let trailing_byte = bytes.len().checked_sub(1).and_then(|idx| bytes.get(idx));
        let symbol = if let Some(&b'\0') = trailing_byte {
            state.symbols.check_interned(bytes)
        } else {
            let mut bytes = bytes.to_vec();
            bytes.push(b'\0');
            state.symbols.check_interned(&bytes)
        };
        if let Some(symbol) = symbol {
            let symbol = u32::from(symbol);
            let symbol = symbol.checked_add(1).ok_or_else(SymbolOverflowError::new)?;
            Ok(Some(symbol.into()))
        } else {
            Ok(None)
        }
    }

    fn lookup_symbol(&self, symbol: Self::Symbol) -> Result<Option<&[u8]>, Self::Error> {
        let state = self.state.as_ref().ok_or(InterpreterExtractError)?;
        let symbol = u32::from(symbol);
        if let Some(symbol) = symbol.checked_sub(1) {
            if let Some(bytes) = state.symbols.get(symbol.into()) {
                if bytes.is_empty() {
                    Ok(Some(bytes))
                } else {
                    Ok(bytes.get(..bytes.len() - 1))
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl RubyException for SymbolOverflowError {
    #[inline]
    fn message(&self) -> Cow<'_, [u8]> {
        self.to_string().into_bytes().into()
    }

    #[inline]
    fn name(&self) -> Cow<'_, str> {
        "fatal".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let value = interp.new_instance::<Fatal>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<SymbolOverflowError> for Exception {
    #[inline]
    fn from(exception: SymbolOverflowError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<SymbolOverflowError>> for Exception {
    #[inline]
    fn from(exception: Box<SymbolOverflowError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<SymbolOverflowError> for Box<dyn RubyException> {
    #[inline]
    fn from(exception: SymbolOverflowError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<SymbolOverflowError>> for Box<dyn RubyException> {
    #[inline]
    fn from(exception: Box<SymbolOverflowError>) -> Box<dyn RubyException> {
        exception
    }
}
