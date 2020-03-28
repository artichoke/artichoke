//! Utilities for formatting Ruby backtraces.

use ansi_term::Style;
use std::io;

use crate::backend::exception::{Exception, RubyException};
use crate::backend::Artichoke;

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
/// Results in this stacktrace:
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
pub fn format_repl_trace_into<W, E>(
    mut error: W,
    interp: &mut Artichoke,
    exc: &E,
) -> Result<(), Exception>
where
    W: io::Write,
    E: RubyException,
{
    if let Some(backtrace) = exc.vm_backtrace(interp) {
        writeln!(
            error,
            "{} (most recent call last):",
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
/// Results in this stacktrace:
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
pub fn format_cli_trace_into<W, E>(
    mut error: W,
    interp: &mut Artichoke,
    exc: &E,
) -> Result<(), Exception>
where
    W: io::Write,
    E: RubyException,
{
    let mut top = None;
    if let Some(backtrace) = exc.vm_backtrace(interp) {
        writeln!(
            error,
            "{} (most recent call last):",
            Style::new().bold().paint("Traceback")
        )?;
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
    Style::new()
        .bold()
        .paint(exc.message())
        .write_to(&mut error)?;
    writeln!(
        error,
        " {}{}{}",
        Style::new().bold().paint("("),
        Style::new().bold().underline().paint(exc.name()),
        Style::new().bold().paint(")")
    )?;
    Ok(())
}
