#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), artichoke_frontend::repl::Error> {
    artichoke_frontend::repl::run(std::io::stdout(), std::io::stderr(), None)
}

#[cfg(target_arch = "wasm32")]
fn main() {}
