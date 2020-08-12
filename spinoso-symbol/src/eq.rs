use crate::Symbol;

impl PartialEq<u32> for Symbol {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.id() == *other
    }
}

impl PartialEq<&u32> for Symbol {
    #[inline]
    fn eq(&self, other: &&u32) -> bool {
        self.id() == **other
    }
}

impl PartialEq<Symbol> for u32 {
    #[inline]
    fn eq(&self, other: &Symbol) -> bool {
        *self == other.id()
    }
}

impl PartialEq<&Symbol> for u32 {
    #[inline]
    fn eq(&self, other: &&Symbol) -> bool {
        *self == other.id()
    }
}

impl PartialEq<u32> for &Symbol {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.id() == *other
    }
}

impl PartialEq<Symbol> for &u32 {
    #[inline]
    fn eq(&self, other: &Symbol) -> bool {
        **self == other.id()
    }
}
