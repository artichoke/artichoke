#![deny(missing_docs, warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

//!  Crate mruby-bin provides binaries for interacting with the artichoke
//!  interpreter in the [`artichoke-backend`](artichoke_backend) crate.

pub mod parser;
pub mod repl;
