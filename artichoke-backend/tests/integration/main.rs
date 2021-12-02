#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::non_ascii_literal)]
#![allow(unknown_lints)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

#[macro_use]
extern crate artichoke_backend;

mod extension;
mod gc;
mod leak;
mod mruby_3_regression;
