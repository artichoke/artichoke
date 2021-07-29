//! Case folding comparisons for byte content resolved from `Symbol`s.

#![allow(clippy::module_name_repetitions)]

mod ascii;
mod unicode;

pub use ascii::casecmp as ascii_casecmp;
pub use unicode::case_eq as unicode_case_eq;
