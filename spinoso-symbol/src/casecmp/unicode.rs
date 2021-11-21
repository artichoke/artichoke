//! Unicode case folding comparisons for byte content resolved from `Symbol`s.

use core::str;

use artichoke_core::intern::Intern;
use focaccia::CaseFold;

/// Compare the byte contents of two symbols using Unicode case-folding
/// comparison for equality.
///
/// The byte slice associated with each symbol is resolved via the given
/// interner. Unresolved symbols are compared as if they resolve to `&[]`.
///
/// This comparison function attempts to convert each symbol's byte content to a
/// UTF-8 [`str`](prim@str). If both symbols resolve to UTF-8 contents, [Unicode
/// case folding] is used when comparing the contents and this function returns
/// `Ok(Some(bool))`. If neither symbol resolves to UTF-8 contents, this
/// function falls back to [`ascii_casecmp`] and returns `Ok(Some(bool))`.
/// Otherwise, the two symbols have byte contents with different encodings and
/// `Ok(None)` is returned.
///
/// This function can be used to implement [`Symbol#casecmp?`] for the
/// [`Symbol`] type defined in Ruby Core.
///
/// # Errors
///
/// If the interner returns an error while retrieving a symbol, that error is
/// returned. See [`Intern::lookup_symbol`].
///
/// [Unicode case folding]: https://www.w3.org/International/wiki/Case_folding
/// [`ascii_casecmp`]: crate::casecmp::ascii_casecmp
/// [`Symbol#casecmp?`]: https://ruby-doc.org/core-2.6.3/Symbol.html#method-i-casecmp-3F
/// [`Symbol`]: https://ruby-doc.org/core-2.6.3/Symbol.html
#[inline]
#[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
pub fn case_eq<T, U>(interner: &T, left: U, right: U, fold: CaseFold) -> Result<Option<bool>, T::Error>
where
    T: Intern<Symbol = U>,
    U: Copy,
{
    let left = interner.lookup_symbol(left)?.unwrap_or_default();
    let right = interner.lookup_symbol(right)?.unwrap_or_default();
    let cmp = match (str::from_utf8(left), str::from_utf8(right)) {
        // Both slices are UTF-8, compare with the given Unicode case folding
        // scheme.
        (Ok(left), Ok(right)) => fold.case_eq(left, right),
        // Both slices are not UTF-8, fallback to ASCII comparator.
        (Err(_), Err(_)) => focaccia::ascii_case_eq(left, right),
        // Encoding mismatch, the bytes are not comparable using Unicode case
        // folding.
        //
        // > `nil` is returned if the two symbols have incompatible encodings,
        // > or if `other_symbol` is not a symbol.
        // > <https://ruby-doc.org/core-2.6.3/Symbol.html#method-i-casecmp-3F>
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => return Ok(None),
    };
    Ok(Some(cmp))
}
