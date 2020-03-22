#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs, intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

//!  `artichoke-frontend` crate provides binaries for interacting with the
//!  artichoke interpreter in the [`artichoke-backend`](artichoke_backend)
//!  crate.

#![doc(html_root_url = "https://docs.rs/artichoke/0.1.0-pre.0")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");
#[cfg(doctest)]
doc_comment::doctest!("../artichoke-backend/README.md");
#[cfg(doctest)]
doc_comment::doctest!("../artichoke-core/README.md");
#[cfg(doctest)]
doc_comment::doctest!("../spec-runner/README.md");

pub use artichoke_backend as backend;

pub mod parser;
pub mod repl;
pub mod ruby;
