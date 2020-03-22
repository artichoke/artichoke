//! A REPL (read–eval–print–loop) for an artichoke interpreter exposed by
//! the [`artichoke-backend`](artichoke_backend) crate.
//!
//! The REPL is readline enabled, but does not save history. The REPL supports
//! multi-line Ruby expressions, CTRL-C to break out of an expression, and can
//! inspect return values and exception backtraces.

use ansi_term::Style;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::error;
use std::fmt;
use std::io::Write;

use crate::backend::exception::{Exception, RubyException};
use crate::backend::gc::MrbGarbageCollection;
use crate::backend::state::parser::Context;
use crate::backend::{Artichoke, Eval, Parser as _, ValueLike};
use crate::parser::{Parser, State};

const REPL_FILENAME: &[u8] = b"(airb)";

#[cfg(test)]
mod filename_test {
    #[test]
    fn repl_filename_does_not_contain_nul_byte() {
        let contains_nul_byte = super::REPL_FILENAME.iter().copied().any(|b| b == b'\0');
        assert!(!contains_nul_byte);
    }
}

/// Failed to initialize parser during REPL boot.
///
/// The parser is needed to properly enter and exit multi-line editing mode.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ParserAllocError;

impl fmt::Display for ParserAllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to initialize Ruby parser")
    }
}

impl error::Error for ParserAllocError {}

/// Parser processed too many lines of input.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ParserLineCountError;

impl fmt::Display for ParserLineCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The interpreter has parsed too many lines and must exit")
    }
}

impl error::Error for ParserLineCountError {}

/// Internal fatal parser error.
///
/// This is usually an unknown FFI to Rust translation.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ParserInternalError;

impl fmt::Display for ParserInternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A fatal parsing error occurred")
    }
}

impl error::Error for ParserInternalError {}

/// The input loop encountered an unknown error condition.
#[derive(Debug)]
struct UnhandledReadlineError(ReadlineError);

impl fmt::Display for UnhandledReadlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unhandled REPL Readline error: {}", self.0)
    }
}

impl error::Error for UnhandledReadlineError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

/// Configuration for the REPL readline prompt.
#[derive(Debug, Clone)]
pub struct PromptConfig {
    /// Basic prompt for start of a new expression.
    pub simple: String,
    /// Altered prompt when an expression is not terminated.
    pub continued: String,
    /// Prefix for the result of `$expression.inspect`. A newline is printed
    /// after the Ruby result.
    pub result_prefix: String,
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            simple: String::from(">>> "),
            continued: String::from("... "),
            result_prefix: String::from("=> "),
        }
    }
}

fn preamble(interp: &mut Artichoke) -> Result<String, Exception> {
    let description = interp.eval(b"RUBY_DESCRIPTION")?.try_into::<&str>()?;
    let compiler = interp
        .eval(b"ARTICHOKE_COMPILER_VERSION")?
        .try_into::<&str>()?;
    let mut buf = String::with_capacity(description.len() + 2 + compiler.len() + 1);
    buf.push_str(description);
    buf.push_str("\n[");
    buf.push_str(compiler);
    buf.push(']');
    Ok(buf)
}

/// Run a REPL for the mruby interpreter exposed by the `mruby` crate.
///
/// # Errors
///
/// If printing the interpreter copyright or compiler metadata fails, an error
/// is returned.
///
/// If initializing the Ruby parser fails, an error is returned.
///
/// If an exception is raised on the interpreter, then an error is returned.
///
/// If writing expression results or exception backtraces to stdout and stderr
/// fails, an error is returned.
///
/// If an unhandled readline state is encountered, a fatal error is returned.
pub fn run(
    mut output: impl Write,
    mut error: impl Write,
    config: Option<PromptConfig>,
) -> Result<(), Box<dyn error::Error>> {
    let config = config.unwrap_or_default();
    let mut interp = crate::interpreter()?;
    writeln!(output, "{}", preamble(&mut interp)?)?;

    interp.reset_parser();
    // safety:
    // Context::new_unchecked requires that REPL_FILENAME have no NUL bytes.
    // REPL_FILENAME is controlled by this crate and asserts this invariant
    // with a test.
    interp.push_context(unsafe { Context::new_unchecked(REPL_FILENAME.to_vec()) });
    let mut parser = Parser::new(&interp).ok_or(ParserAllocError)?;

    let mut rl = Editor::<()>::new();
    // If a code block is open, accumulate code from multiple readlines in this
    // mutable `String` buffer.
    let mut buf = String::new();
    let mut parser_state = State::default();
    loop {
        // Allow shell users to identify that they have an open code block.
        let prompt = if parser_state.is_code_block_open() {
            config.continued.as_str()
        } else {
            config.simple.as_str()
        };

        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                buf.push_str(line.as_str());
                parser_state = parser.parse(buf.as_bytes());
                if parser_state.is_fatal() {
                    return Err(Box::new(ParserInternalError));
                }
                if parser_state.is_code_block_open() {
                    buf.push('\n');
                    continue;
                }
                if parser_state.is_recoverable_error() {
                    write!(error, "Could not parse input")?;
                    buf.clear();
                    continue;
                }
                match interp.eval(buf.as_bytes()) {
                    Ok(value) => {
                        let result = value.inspect();
                        output.write_all(config.result_prefix.as_bytes())?;
                        output.write_all(result.as_slice())?;
                    }
                    Err(exc) => {
                        if let Some(backtrace) = exc.vm_backtrace(&mut interp) {
                            writeln!(
                                error,
                                "{} (most recent call last)",
                                Style::new().bold().paint("Traceback")
                            )?;
                            for (num, frame) in backtrace.into_iter().enumerate().rev() {
                                write!(error, "\t{}: from ", num + 1)?;
                                error.write_all(frame.as_slice())?;
                                writeln!(error)?;
                            }
                        }
                        write!(
                            error,
                            "{} {}",
                            Style::new().bold().paint(exc.name()),
                            Style::new().bold().paint("(")
                        )?;
                        Style::new()
                            .bold()
                            .underline()
                            .paint(exc.message())
                            .write_to(&mut error)?;
                        writeln!(error, "{}", Style::new().bold().paint(")"))?;
                    }
                }
                for line in buf.lines() {
                    rl.add_history_entry(line);
                    interp
                        .0
                        .borrow_mut()
                        .parser
                        .add_fetch_lineno(1)
                        .map_err(|_| ParserLineCountError)?;
                }
                // Eval successful, so reset the REPL state for the next
                // expression.
                interp.incremental_gc();
                buf.clear();
            }
            // Reset the buf and present the user with a fresh prompt
            Err(ReadlineError::Interrupted) => {
                // Reset buffered code
                buf.clear();
                // clear parser state
                parser_state = State::default();
                writeln!(output, "^C")?;
            }
            // Gracefully exit on CTRL-D EOF
            Err(ReadlineError::Eof) => break,
            Err(err) => return Err(Box::new(UnhandledReadlineError(err))),
        };
    }
    Ok(())
}
