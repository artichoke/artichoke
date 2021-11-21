//! Format Ruby `Exception` backtraces.

use std::error;
use std::io;

use termcolor::{ColorSpec, WriteColor};

use crate::prelude::*;

/// Format an `Exception` backtrace into an [`io::Write`] suitable for
/// displaying in a Ruby REPL.
///
/// This backtrace has the same style and formatting as one returned from the
/// `irb` command in MRI.
///
/// # Examples
///
/// Executing the following Ruby code:
///
/// ```ruby
/// def fail; raise RuntimeError, "bang!"; end
/// fail
/// ```
///
/// Results in this stack trace:
///
/// ```txt
/// Traceback (most recent call last):
///     2: from (airb):2
///     1: from (airb):1:in fail
/// RuntimeError (bang!)
/// ```
///
/// # Errors
///
/// If writing into the provided `out` writer fails, an error is returned.
pub fn format_repl_trace_into<W, E>(mut error: W, interp: &mut Artichoke, exc: &E) -> Result<(), Box<dyn error::Error>>
where
    W: io::Write + WriteColor,
    E: RubyException,
{
    // reset colors
    error.reset()?;

    // Format backtrace if present
    if let Some(backtrace) = exc.vm_backtrace(interp) {
        error.set_color(ColorSpec::new().set_bold(true))?;
        write!(error, "Traceback")?;
        error.reset()?;
        writeln!(error, " (most recent call last):")?;
        for (num, frame) in backtrace.into_iter().enumerate().rev() {
            write!(error, "\t{}: from ", num + 1)?;
            error.write_all(frame.as_slice())?;
            writeln!(error)?;
        }
    }

    // Format exception class and message
    error.set_color(ColorSpec::new().set_bold(true))?;
    write!(error, "{} (", exc.name())?;
    error.set_color(ColorSpec::new().set_bold(true).set_underline(true))?;
    error.write_all(&exc.message())?;
    error.set_color(ColorSpec::new().set_bold(true))?;
    writeln!(error, ")")?;

    // reset colors
    error.reset()?;

    Ok(())
}

/// Format an `Exception` backtrace into an [`io::Write`] suitable for
/// displaying in a Ruby CLI.
///
/// This backtrace has the same style and formatting as one returned from the
/// `ruby` command in MRI.
///
/// # Examples
///
/// Executing the following Ruby code:
///
/// ```ruby
/// def fail; raise RuntimeError, "bang!"; end
/// fail
/// ```
///
/// Results in this stack trace:
///
/// ```txt
/// Traceback (most recent call last):
///     2: from -e:1
/// -e:1:in fail: bang! (RuntimeError)
/// ```
///
/// # Errors
///
/// If writing into the provided `out` writer fails, an error is returned.
pub fn format_cli_trace_into<W, E>(mut error: W, interp: &mut Artichoke, exc: &E) -> Result<(), Box<dyn error::Error>>
where
    W: io::Write + WriteColor,
    E: RubyException,
{
    // reset colors
    error.reset()?;

    let mut top = None;

    // Format backtrace if present
    if let Some(backtrace) = exc.vm_backtrace(interp) {
        error.set_color(ColorSpec::new().set_bold(true))?;
        write!(error, "Traceback")?;
        error.reset()?;
        writeln!(error, " (most recent call last):")?;
        let mut iter = backtrace.into_iter().enumerate();
        top = iter.next();
        for (num, frame) in iter.rev() {
            write!(error, "\t{}: from ", num + 1)?;
            error.write_all(frame.as_slice())?;
            writeln!(error)?;
        }
    }

    if let Some((_, frame)) = top {
        error.write_all(frame.as_slice())?;
        write!(error, ": ")?;
    }

    // Format exception class and message
    error.set_color(ColorSpec::new().set_bold(true))?;
    error.write_all(&exc.message())?;
    error.set_color(ColorSpec::new().set_bold(true))?;
    write!(error, " (")?;
    error.set_color(ColorSpec::new().set_bold(true).set_underline(true))?;
    write!(error, "{}", exc.name())?;
    error.set_color(ColorSpec::new().set_bold(true))?;
    writeln!(error, ")")?;

    // reset colors
    error.reset()?;

    Ok(())
}
