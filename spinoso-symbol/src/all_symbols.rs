use artichoke_core::intern::Intern;
use core::iter::FusedIterator;
use core::ops::Range;

use crate::Symbol;

/// Extension trait to return an iterator that returns all symbol identifiers
/// stored in an [interner] as [`Symbol`]s.
///
/// The returned iterator yields [`Symbol`] as its item type.
///
/// Implementors of this trait must issue symbol identifiers as an [arithmetic
/// progression] with a common difference of 1. The sequence of symbol
/// identifiers must be representable by a [`Range<u32>`].
///
/// This trait is automatically implemented for all types that implement
/// [`Intern`] from [`artichoke_core`].
///
/// # Examples
///
/// ```
/// # extern crate alloc;
/// use alloc::borrow::Cow;
/// use alloc::boxed::Box;
/// use artichoke_core::intern::Intern;
/// use spinoso_symbol::{InternerAllSymbols, Symbol};
///
/// #[derive(Default)]
/// struct Interner(u32);
///
/// impl Intern for Interner {
///     type Symbol = u32;
///     type Error = &'static str;
///     const SYMBOL_RANGE_START: u32 = 1;
///
///     fn intern_bytes<T>(&mut self, symbol: T) -> Result<Self::Symbol, Self::Error>
///     where
///         T: Into<Cow<'static, [u8]>>
///     {
///         let boxed = Box::<[u8]>::from(symbol.into());
///         Box::leak(boxed);
///         self.0 += 1;
///         let sym = self.0;
///         Ok(sym)
///     }
///
///     fn check_interned_bytes(&self, symbol: &[u8]) -> Result<Option<Self::Symbol>, Self::Error> {
///         Err("not implemented")
///     }
///
///     fn lookup_symbol(&self, symbol: Self::Symbol) -> Result<Option<&[u8]>, Self::Error> {
///         Err("not implemented")
///     }
///
///     fn symbol_count(&self) -> usize {
///         self.0 as usize
///     }
/// }
///
/// let mut interner = Interner::default();
/// let mut all_symbols = interner.all_symbols();
/// assert_eq!(all_symbols.count(), 0);
///
/// interner.intern_bytes(&b"Spinoso"[..]);
/// interner.intern_bytes(&b"Artichoke"[..]);
///
/// let mut all_symbols = interner.all_symbols();
/// assert_eq!(all_symbols.next(), Some(Symbol::new(1)));
/// assert_eq!(all_symbols.next(), Some(Symbol::new(2)));
/// assert_eq!(all_symbols.next(), None);
/// ```
///
/// [interner]: Intern
/// [arithmetic progression]: https://en.wikipedia.org/wiki/Arithmetic_progression
/// [`Range<u32>`]: core::ops::Range
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
pub trait InternerAllSymbols: Intern {
    /// Returns an iterator that returns all symbol identifiers stored in an
    /// [interner] as [`Symbol`]s.
    ///
    /// The returned iterator yields [`Symbol`] as its item type.
    ///
    /// This function requires that the interner issues symbol identifiers as an
    /// [arithmetic progression] with a common difference of 1. The sequence of
    /// symbol identifiers must be representable by a [`Range<u32>`].
    ///
    /// [`AllSymbols`] supports yielding up to `u32::MAX - 1` `Symbol`s.
    ///
    /// # Examples
    ///
    /// See trait-level documentation for examples.
    ///
    /// [interner]: Intern
    /// [arithmetic progression]: https://en.wikipedia.org/wiki/Arithmetic_progression
    /// [`Range<u32>`]: core::ops::Range
    fn all_symbols(&self) -> AllSymbols;
}

impl<T, U> InternerAllSymbols for T
where
    T: Intern<Symbol = U>,
    U: Copy + Into<u32>,
{
    #[inline]
    #[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
    fn all_symbols(&self) -> AllSymbols {
        self.into()
    }
}

impl<T, U> From<&T> for AllSymbols
where
    T: Intern<Symbol = U>,
    U: Copy + Into<u32>,
{
    /// Construct a [`AllSymbols`] iterator from an interner.
    #[inline]
    fn from(interner: &T) -> Self {
        let min = T::SYMBOL_RANGE_START.into();
        let max_idx = interner.symbol_count().try_into().unwrap_or(u32::MAX);
        let max = min.saturating_add(max_idx);
        AllSymbols(min..max)
    }
}

/// An iterator that returns all the Symbols in an [interner].
///
/// This iterator yields [`Symbol`] as its item type.
///
/// This iterator supports yielding up to `u32::MAX - 1` `Symbol`s.
///
/// This struct is created by the [`all_symbols`] method in the
/// [`InternerAllSymbols`] trait.  See its documentation for more.
///
/// [interner]: Intern
/// [`all_symbols`]: InternerAllSymbols::all_symbols
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[must_use = "this `AllSymbols` is an `Iterator`, which should be consumed if constructed"]
#[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
pub struct AllSymbols(Range<u32>);

impl Iterator for AllSymbols {
    type Item = Symbol;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Symbol::from)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(Symbol::from)
    }

    #[inline]
    fn count(self) -> usize {
        // Inline implementation of `Step::steps_between` since
        // `<Range as Iterator>::count` is not specialized to use it for integer
        // ranges.
        if self.0.start <= self.0.end {
            (self.0.end - self.0.start) as usize
        } else {
            0
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0.last().map(Symbol::from)
    }

    #[inline]
    fn min(mut self) -> Option<Self::Item> {
        self.0.next().map(Symbol::from)
    }

    #[inline]
    fn max(self) -> Option<Self::Item> {
        self.0.max().map(Symbol::from)
    }
}

impl DoubleEndedIterator for AllSymbols {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(Symbol::from)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(Symbol::from)
    }
}

impl ExactSizeIterator for AllSymbols {}

impl FusedIterator for AllSymbols {}

#[cfg(test)]
mod tests {
    use artichoke_core::intern::Intern;
    use std::borrow::Cow;

    use super::InternerAllSymbols;
    use crate::Symbol;

    #[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct Interner(u32);

    impl Intern for Interner {
        type Symbol = u32;
        type Error = &'static str;
        const SYMBOL_RANGE_START: Self::Symbol = 0;

        fn intern_bytes<T>(&mut self, symbol: T) -> Result<Self::Symbol, Self::Error>
        where
            T: Into<Cow<'static, [u8]>>,
        {
            drop(symbol.into());
            Err("not implemented")
        }

        fn check_interned_bytes(&self, symbol: &[u8]) -> Result<Option<Self::Symbol>, Self::Error> {
            let _ = symbol;
            Err("not implemented")
        }

        fn lookup_symbol(&self, symbol: Self::Symbol) -> Result<Option<&[u8]>, Self::Error> {
            let _ = symbol;
            Err("not implemented")
        }

        fn symbol_count(&self) -> usize {
            self.0 as usize
        }
    }

    #[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct OffByOneInterner(u32);

    impl Intern for OffByOneInterner {
        type Symbol = u32;
        type Error = &'static str;
        const SYMBOL_RANGE_START: Self::Symbol = 1;

        fn intern_bytes<T>(&mut self, symbol: T) -> Result<Self::Symbol, Self::Error>
        where
            T: Into<Cow<'static, [u8]>>,
        {
            drop(symbol.into());
            Err("not implemented")
        }

        fn check_interned_bytes(&self, symbol: &[u8]) -> Result<Option<Self::Symbol>, Self::Error> {
            let _ = symbol;
            Err("not implemented")
        }

        fn lookup_symbol(&self, symbol: Self::Symbol) -> Result<Option<&[u8]>, Self::Error> {
            let _ = symbol;
            Err("not implemented")
        }

        fn symbol_count(&self) -> usize {
            self.0 as usize
        }
    }

    #[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct NonZeroInterner(u32);

    impl Intern for NonZeroInterner {
        type Symbol = u32;
        type Error = &'static str;
        const SYMBOL_RANGE_START: Self::Symbol = u32::MAX - 16;

        fn intern_bytes<T>(&mut self, symbol: T) -> Result<Self::Symbol, Self::Error>
        where
            T: Into<Cow<'static, [u8]>>,
        {
            drop(symbol.into());
            Err("not implemented")
        }

        fn check_interned_bytes(&self, symbol: &[u8]) -> Result<Option<Self::Symbol>, Self::Error> {
            let _ = symbol;
            Err("not implemented")
        }

        fn lookup_symbol(&self, symbol: Self::Symbol) -> Result<Option<&[u8]>, Self::Error> {
            let _ = symbol;
            Err("not implemented")
        }

        fn symbol_count(&self) -> usize {
            self.0 as usize
        }
    }

    #[test]
    fn zero_offset_count() {
        let interner = Interner::default();
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), 0);

        let interner = Interner(100);
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), 100);

        let interner = Interner(u32::MAX);
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), u32::MAX as usize);
    }

    #[test]
    fn zero_offset_yielded() {
        let interner = Interner::default();
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), None);

        let interner = Interner(5);
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), Some(Symbol::new(0)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(1)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(2)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(3)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(4)));
        assert_eq!(all_symbols.next(), None);

        let interner = Interner(u32::MAX);
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), Some(Symbol::new(0)));
        assert_eq!(all_symbols.next_back(), Some(Symbol::new(u32::MAX - 1)));
    }

    #[test]
    fn one_offset_count() {
        let interner = OffByOneInterner::default();
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), 0);

        let interner = OffByOneInterner(100);
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), 100);

        let interner = OffByOneInterner(u32::MAX);
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), (u32::MAX - 1) as usize);
    }

    #[test]
    fn one_offset_yielded() {
        let interner = OffByOneInterner::default();
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), None);

        let interner = OffByOneInterner(5);
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), Some(Symbol::new(1)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(2)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(3)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(4)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(5)));
        assert_eq!(all_symbols.next(), None);

        let interner = OffByOneInterner(u32::MAX);
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), Some(Symbol::new(1)));
        assert_eq!(all_symbols.next_back(), Some(Symbol::new(u32::MAX - 1)));
    }

    #[test]
    fn high_offset_count() {
        let interner = NonZeroInterner::default();
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), 0);

        let interner = NonZeroInterner(100);
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), 16);

        let interner = NonZeroInterner(u32::MAX);
        let all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.count(), 16);
    }

    #[test]
    fn high_offset_yielded() {
        let interner = NonZeroInterner::default();
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), None);

        let interner = NonZeroInterner(5);
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), Some(Symbol::new(u32::MAX - 16)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(u32::MAX - 15)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(u32::MAX - 14)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(u32::MAX - 13)));
        assert_eq!(all_symbols.next(), Some(Symbol::new(u32::MAX - 12)));
        assert_eq!(all_symbols.next(), None);

        let interner = NonZeroInterner(u32::MAX);
        let mut all_symbols = interner.all_symbols();
        assert_eq!(all_symbols.next(), Some(Symbol::new(u32::MAX - 16)));
        assert_eq!(all_symbols.next_back(), Some(Symbol::new(u32::MAX - 1)));
    }
}
