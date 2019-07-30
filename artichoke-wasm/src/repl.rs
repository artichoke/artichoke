//! A REPL (read–eval–print–loop) for an artichoke interpreter exposed by
//! the [`artichoke-backend`](artichoke_backend) crate.
//!
//! The REPL is readline enabled, but does not save history. The REPL supports
//! multi-line Ruby expressions, CTRL-C to break out of an expression, and can
//! inspect return values and exception backtraces.

use artichoke_backend::eval::{Context, Eval};
use artichoke_backend::gc::MrbGarbageCollection;
use artichoke_backend::{Artichoke, ArtichokeError};
use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::parser::{self, Parser};

const REPL_FILENAME: &str = "(rirb)";

/// REPL errors.
#[derive(Debug)]
pub enum Error {
    /// Fatal error.
    Fatal,
    /// Could not initialize REPL.
    ReplInit,
    /// Unrecoverable [`Parser`] error.
    ReplParse(parser::Error),
    /// Unrecoverable [`ArtichokeError`]. [`ArtichokeError::Exec`] are handled gracefully
    /// by the REPL. All other `ArtichokeError`s are fatal.
    Ruby(ArtichokeError),
    /// Code could not be parsed.
    Syntax,
}

pub struct State {
    interp: Artichoke,
    parser: Parser,
}

impl State {
    fn new(interp: Artichoke, parser: Parser) -> Self {
        Self { interp, parser }
    }
}

pub fn init() -> Result<State, Error> {
    let interp = artichoke_backend::interpreter().map_err(Error::Ruby)?;

    let parser = Parser::new(&interp).ok_or(Error::ReplInit)?;
    interp.push_context(Context::new(REPL_FILENAME));
    unsafe {
        let api = interp.borrow();
        (*api.ctx).lineno = 1;
    }

    Ok(State::new(interp, parser))
}

/// Run a REPL for the mruby interpreter exposed by the `mruby` crate.
pub fn try_eval(input: &str, state: State) -> Result<(), Error> {
    let parse_state = state.parser.parse(input).map_err(Error::ReplParse)?;
    if parse_state.is_code_block_open() {
        return Err(Error::Syntax);
    }
    match state.interp.eval(input) {
        Ok(_) => {
            // artichoke eval successful, so reset the REPL state for the next
            // expression.
            state.interp.incremental_gc();
        }
        Err(ArtichokeError::Exec(backtrace)) => {
            console::log(&Array::of1(&JsValue::from_str("Backtrace:")));
            for frame in backtrace.lines() {
                console::log(&Array::of1(&JsValue::from_str(
                    format!("    {}", frame).as_str(),
                )));
            }
            state.interp.incremental_gc();
        }
        Err(err) => return Err(Error::Ruby(err)),
    }
    Ok(())
}
