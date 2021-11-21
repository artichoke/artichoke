//! ASCII case folding comparisons for byte content resolved from `Symbol`s.

use core::cmp::Ordering;

use artichoke_core::intern::Intern;

/// Compare the byte contents of two symbols using ASCII case-insensitive
/// comparison.
///
/// The byte slice associated with each symbol is resolved via the given
/// interner. Unresolved symbols are compared as if they resolve to `&[]`.
///
/// This function can be used to implement [`Symbol#casecmp`] for the [`Symbol`]
/// type defined in Ruby Core.
///
/// # Errors
///
/// If the interner returns an error while retrieving a symbol, that error is
/// returned. See [`Intern::lookup_symbol`].
///
/// [`Symbol#casecmp`]: https://ruby-doc.org/core-2.6.3/Symbol.html#method-i-casecmp
/// [`Symbol`]: https://ruby-doc.org/core-2.6.3/Symbol.html
#[inline]
#[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
pub fn casecmp<T, U>(interner: &T, left: U, right: U) -> Result<Ordering, T::Error>
where
    T: Intern<Symbol = U>,
    U: Copy,
{
    let left = interner.lookup_symbol(left)?.unwrap_or_default();
    let right = interner.lookup_symbol(right)?.unwrap_or_default();
    Ok(focaccia::ascii_casecmp(left, right))
}
