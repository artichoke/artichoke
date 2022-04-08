use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

use crate::{Symbol, SymbolOverflowError};

impl From<u8> for Symbol {
    #[inline]
    fn from(id: u8) -> Self {
        Self(id.into())
    }
}

impl From<NonZeroU8> for Symbol {
    #[inline]
    fn from(sym: NonZeroU8) -> Self {
        Self(sym.get().into())
    }
}

impl From<u16> for Symbol {
    #[inline]
    fn from(id: u16) -> Self {
        Self(id.into())
    }
}

impl From<NonZeroU16> for Symbol {
    #[inline]
    fn from(sym: NonZeroU16) -> Self {
        Self(sym.get().into())
    }
}

impl From<u32> for Symbol {
    #[inline]
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<NonZeroU32> for Symbol {
    #[inline]
    fn from(sym: NonZeroU32) -> Self {
        Self(sym.get())
    }
}

impl TryFrom<u64> for Symbol {
    type Error = SymbolOverflowError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let id = u32::try_from(value)?;
        Ok(id.into())
    }
}

impl TryFrom<NonZeroU64> for Symbol {
    type Error = SymbolOverflowError;

    #[inline]
    fn try_from(value: NonZeroU64) -> Result<Self, Self::Error> {
        let id = u32::try_from(value.get())?;
        Ok(id.into())
    }
}

impl TryFrom<usize> for Symbol {
    type Error = SymbolOverflowError;

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let id = u32::try_from(value)?;
        Ok(id.into())
    }
}

impl TryFrom<NonZeroUsize> for Symbol {
    type Error = SymbolOverflowError;

    #[inline]
    fn try_from(value: NonZeroUsize) -> Result<Self, Self::Error> {
        let id = u32::try_from(value.get())?;
        Ok(id.into())
    }
}

impl From<&u8> for Symbol {
    #[inline]
    fn from(id: &u8) -> Self {
        Self((*id).into())
    }
}

impl From<&NonZeroU8> for Symbol {
    #[inline]
    fn from(sym: &NonZeroU8) -> Self {
        Self(sym.get().into())
    }
}

impl From<&u16> for Symbol {
    #[inline]
    fn from(id: &u16) -> Self {
        Self((*id).into())
    }
}

impl From<&NonZeroU16> for Symbol {
    #[inline]
    fn from(sym: &NonZeroU16) -> Self {
        Self(sym.get().into())
    }
}

impl From<&u32> for Symbol {
    #[inline]
    fn from(id: &u32) -> Self {
        Self(*id)
    }
}

impl From<&NonZeroU32> for Symbol {
    #[inline]
    fn from(sym: &NonZeroU32) -> Self {
        Self(sym.get())
    }
}

impl TryFrom<&u64> for Symbol {
    type Error = SymbolOverflowError;

    #[inline]
    fn try_from(value: &u64) -> Result<Self, Self::Error> {
        let id = u32::try_from(*value)?;
        Ok(id.into())
    }
}

impl TryFrom<&NonZeroU64> for Symbol {
    type Error = SymbolOverflowError;

    #[inline]
    fn try_from(value: &NonZeroU64) -> Result<Self, Self::Error> {
        let id = u32::try_from(value.get())?;
        Ok(id.into())
    }
}

impl TryFrom<&usize> for Symbol {
    type Error = SymbolOverflowError;

    #[inline]
    fn try_from(value: &usize) -> Result<Self, Self::Error> {
        let id = u32::try_from(*value)?;
        Ok(id.into())
    }
}

impl TryFrom<&NonZeroUsize> for Symbol {
    type Error = SymbolOverflowError;

    #[inline]
    fn try_from(value: &NonZeroUsize) -> Result<Self, Self::Error> {
        let id = u32::try_from(value.get())?;
        Ok(id.into())
    }
}

impl From<Symbol> for u32 {
    #[inline]
    fn from(sym: Symbol) -> Self {
        sym.id()
    }
}

impl From<Symbol> for u64 {
    #[inline]
    fn from(sym: Symbol) -> Self {
        sym.id().into()
    }
}

impl From<Symbol> for usize {
    #[inline]
    fn from(sym: Symbol) -> Self {
        // Ensure this cast is lossless.
        const_assert!(usize::BITS >= u32::BITS);

        sym.id() as usize
    }
}

impl From<Symbol> for i64 {
    #[inline]
    fn from(sym: Symbol) -> Self {
        sym.id().into()
    }
}

impl From<&Symbol> for u32 {
    #[inline]
    fn from(sym: &Symbol) -> Self {
        sym.id()
    }
}

impl From<&Symbol> for u64 {
    #[inline]
    fn from(sym: &Symbol) -> Self {
        sym.id().into()
    }
}

impl From<&Symbol> for usize {
    #[inline]
    fn from(sym: &Symbol) -> Self {
        sym.id() as usize
    }
}

impl From<&Symbol> for i64 {
    #[inline]
    fn from(sym: &Symbol) -> Self {
        sym.id().into()
    }
}
