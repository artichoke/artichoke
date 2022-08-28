use std::borrow::Cow;

use intaglio::SymbolOverflowError;
use spinoso_exception::Fatal;

use crate::core::{ClassRegistry, Intern, TryConvertMut};
use crate::error::{Error, RubyException};
use crate::ffi::InterpreterExtractError;
use crate::sys;
use crate::Artichoke;

impl Artichoke {
    pub fn lookup_symbol_with_trailing_nul(&self, symbol: u32) -> Result<Option<&[u8]>, Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
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

    pub fn intern_bytes_with_trailing_nul<T>(&mut self, bytes: T) -> Result<u32, Error>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let bytes = bytes.into();
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let symbol = state.symbols.intern(bytes)?;
        let symbol = u32::from(symbol);
        // mruby expects symbols to be non-zero.
        let symbol = symbol
            .checked_add(<Self as Intern>::SYMBOL_RANGE_START)
            .ok_or_else(SymbolOverflowError::new)?;
        Ok(symbol)
    }

    pub fn check_interned_bytes_with_trailing_nul(&self, bytes: &[u8]) -> Result<Option<u32>, Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let symbol = state.symbols.check_interned(bytes);
        if let Some(symbol) = symbol {
            let symbol = u32::from(symbol);
            // mruby expects symbols to be non-zero.
            let symbol = symbol
                .checked_add(<Self as Intern>::SYMBOL_RANGE_START)
                .ok_or_else(SymbolOverflowError::new)?;
            Ok(Some(symbol))
        } else {
            Ok(None)
        }
    }
}

impl Intern for Artichoke {
    type Symbol = u32;

    type Error = Error;

    const SYMBOL_RANGE_START: Self::Symbol = 1;

    fn intern_bytes<T>(&mut self, bytes: T) -> Result<Self::Symbol, Self::Error>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let bytes = bytes.into();
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let mut bytes = bytes.into_owned();
        bytes.push(b'\0');
        let symbol = state.symbols.intern(bytes)?;
        let symbol = u32::from(symbol);
        // mruby expects symbols to be non-zero.
        let symbol = symbol
            .checked_add(Self::SYMBOL_RANGE_START)
            .ok_or_else(SymbolOverflowError::new)?;
        Ok(symbol)
    }

    fn check_interned_bytes(&self, bytes: &[u8]) -> Result<Option<Self::Symbol>, Self::Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let mut bytes = bytes.to_vec();
        bytes.push(b'\0');
        let symbol = state.symbols.check_interned(&bytes);
        if let Some(symbol) = symbol {
            let symbol = u32::from(symbol);
            // mruby expects symbols to be non-zero.
            let symbol = symbol
                .checked_add(Self::SYMBOL_RANGE_START)
                .ok_or_else(SymbolOverflowError::new)?;
            Ok(Some(symbol))
        } else {
            Ok(None)
        }
    }

    fn lookup_symbol(&self, symbol: Self::Symbol) -> Result<Option<&[u8]>, Self::Error> {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        if let Some(symbol) = symbol.checked_sub(Self::SYMBOL_RANGE_START) {
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

    fn symbol_count(&self) -> usize {
        if let Some(state) = self.state.as_deref() {
            state.symbols.len()
        } else {
            0
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
        let message = interp.try_convert_mut(self.message()).ok()?;
        let value = interp.new_instance::<Fatal>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<SymbolOverflowError> for Error {
    #[inline]
    fn from(exception: SymbolOverflowError) -> Self {
        let err: Box<dyn RubyException> = Box::new(exception);
        Self::from(err)
    }
}
