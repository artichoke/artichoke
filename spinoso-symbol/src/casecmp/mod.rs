//! Case folding comparisons for byte content resolved from `Symbol`s.

mod ascii;
mod unicode;

pub use ascii::casecmp as ascii_casecmp;
pub use unicode::case_eq as unicode_case_eq;
