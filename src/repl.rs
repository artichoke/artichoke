//! A REPL (read–eval–print–loop) for an Artichoke interpreter.
//!
//! The REPL is readline enabled, but does not save history. The REPL supports
//! multi-line Ruby expressions, CTRL-C to break out of an expression, and can
//! inspect return values and exception backtraces.

use std::error;
use std::fmt;
use std::io;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use termcolor::WriteColor;

use crate::backend::state::parser::Context;
use crate::backtrace;
use crate::filename::REPL;
use crate::parser::{Parser, State};
use crate::prelude::{Parser as _, *};

/// Failed to initialize parser during REPL boot.
///
/// The parser is needed to properly enter and exit multi-line editing mode.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParserAllocError {
    _private: (),
}

impl ParserAllocError {
    /// Constructs a new, default `ParserAllocError`.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl fmt::Display for ParserAllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Failed to initialize Ruby parser")
    }
}

impl error::Error for ParserAllocError {}

/// Parser processed too many lines of input.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParserLineCountError {
    _private: (),
}

impl ParserLineCountError {
    /// Constructs a new, default `ParserLineCountError`.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl fmt::Display for ParserLineCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The interpreter has parsed too many lines and must exit")
    }
}

impl error::Error for ParserLineCountError {}

/// Internal fatal parser error.
///
/// This is usually an unknown FFI to Rust translation.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParserInternalError {
    _private: (),
}

impl ParserInternalError {
    /// Constructs a new, default `ParserInternalError`.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl fmt::Display for ParserInternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("A fatal parsing error occurred")
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
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PromptConfig<'a, 'b, 'c> {
    /// Basic prompt for start of a new expression.
    pub simple: &'a str,
    /// Altered prompt when an expression is not terminated.
    pub continued: &'b str,
    /// Prefix for the result of `$expression.inspect`. A newline is printed
    /// after the Ruby result.
    pub result_prefix: &'c str,
}

impl<'a, 'b, 'c> Default for PromptConfig<'a, 'b, 'c> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b, 'c> PromptConfig<'a, 'b, 'c> {
    /// Create a new, default REPL prompt.
    ///
    /// # Default configuration
    ///
    /// The `PromptConfig` is setup with the following literals:
    ///
    /// - `simple`: `>>> `
    /// - `continued`: `... `
    /// - `result_prefix`: `=> `
    ///
    /// # Examples
    ///
    /// ```
    /// # use artichoke::repl::PromptConfig;
    /// let config = PromptConfig {
    ///     simple: ">>> ",
    ///     continued: "... ",
    ///     result_prefix: "=> ",
    /// };
    /// assert_eq!(config, PromptConfig::new());
    /// assert_eq!(config, PromptConfig::default());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            simple: ">>> ",
            continued: "... ",
            result_prefix: "=> ",
        }
    }
}

fn preamble(interp: &mut Artichoke) -> Result<String, Error> {
    let description = interp.eval(b"RUBY_DESCRIPTION")?.try_convert_into_mut::<&str>(interp)?;
    let compiler = interp
        .eval(b"ARTICHOKE_COMPILER_VERSION")?
        .try_convert_into_mut::<&str>(interp)?;
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
pub fn run<Wout, Werr>(
    output: Wout,
    error: Werr,
    config: Option<PromptConfig<'_, '_, '_>>,
) -> Result<(), Box<dyn error::Error>>
where
    Wout: io::Write,
    Werr: io::Write + WriteColor,
{
    let mut interp = crate::interpreter()?;
    let result = repl_loop(&mut interp, output, error, &config.unwrap_or_default());
    interp.close();
    result
}

fn repl_loop<Wout, Werr>(
    interp: &mut Artichoke,
    mut output: Wout,
    mut error: Werr,
    config: &PromptConfig<'_, '_, '_>,
) -> Result<(), Box<dyn error::Error>>
where
    Wout: io::Write,
    Werr: io::Write + WriteColor,
{
    writeln!(output, "{}", preamble(interp)?)?;

    interp.reset_parser()?;
    // SAFETY: `REPL` has no NUL bytes (asserted by tests).
    let context = unsafe { Context::new_unchecked(REPL.to_vec()) };
    interp.push_context(context)?;
    let mut parser = Parser::new(interp).ok_or_else(ParserAllocError::new)?;

    let mut rl = Editor::<()>::new();
    // If a code block is open, accumulate code from multiple read lines in this
    // mutable `String` buffer.
    let mut buf = String::new();
    let mut parser_state = State::new();
    loop {
        // Allow shell users to identify that they have an open code block.
        let prompt = if parser_state.is_code_block_open() {
            config.continued
        } else {
            config.simple
        };

        let readline = rl.readline(prompt);
        match readline {
            Ok(line) if line.is_empty() && buf.is_empty() => (),
            Ok(line) => {
                buf.push_str(line.as_str());
                parser_state = parser.parse(buf.as_bytes())?;

                if parser_state.is_code_block_open() {
                    buf.push('\n');
                    continue;
                }
                if parser_state.is_fatal() {
                    return Err(Box::new(ParserInternalError::new()));
                }
                if parser_state.is_recoverable_error() {
                    writeln!(error, "Could not parse input")?;
                    buf.clear();
                    continue;
                }

                let interp = parser.interp();
                match interp.eval(buf.as_bytes()) {
                    Ok(value) => {
                        let result = value.inspect(interp);
                        output.write_all(config.result_prefix.as_bytes())?;
                        output.write_all(result.as_slice())?;
                        output.write_all(b"\n")?;
                    }
                    Err(ref exc) => backtrace::format_repl_trace_into(&mut error, interp, exc)?,
                }
                for line in buf.lines() {
                    rl.add_history_entry(line);
                    interp.add_fetch_lineno(1).map_err(|_| ParserLineCountError::new())?;
                }
                // Eval successful, so reset the REPL state for the next
                // expression.
                interp.incremental_gc()?;
                buf.clear();
            }
            // Reset the buffer and present the user with a fresh prompt
            Err(ReadlineError::Interrupted) => {
                // Reset buffered code
                buf.clear();
                // clear parser state
                parser_state = State::new();
                writeln!(output, "^C")?;
            }
            // Gracefully exit on CTRL-D EOF
            Err(ReadlineError::Eof) => break,
            Err(err) => return Err(Box::new(UnhandledReadlineError(err))),
        };
    }
    Ok(())
}
