//! Artichoke CLI entrypoint.
//!
//! Artichoke's version of the `ruby` CLI. This module is exported as the
//! `artichoke` binary.

use std::error;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use termcolor::WriteColor;

use crate::backend::ffi;
use crate::backend::state::parser::Context;
use crate::backend::string;
use crate::backtrace;
use crate::prelude::*;

const INLINE_EVAL_SWITCH_FILENAME: &[u8] = b"-e";

#[cfg(test)]
mod filename_test {
    #[test]
    fn inline_eval_switch_filename_does_not_contain_nul_byte() {
        let contains_nul_byte = super::INLINE_EVAL_SWITCH_FILENAME
            .iter()
            .copied()
            .any(|b| b == b'\0');
        assert!(!contains_nul_byte);
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, StructOpt)]
#[structopt(name = "artichoke", about = "Artichoke is a Ruby made with Rust.")]
struct Opt {
    /// print the copyright
    #[structopt(long)]
    copyright: bool,

    /// one line of script. Several -e's allowed. Omit [programfile]
    #[structopt(short = "e", parse(from_os_str))]
    commands: Vec<OsString>,

    /// file whose contents will be read into the `$fixture` global
    #[structopt(long = "with-fixture", parse(from_os_str))]
    fixture: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    programfile: Option<PathBuf>,
}

/// Main entrypoint for Artichoke's version of the `ruby` CLI.
///
/// # Errors
///
/// If an exception is raised on the interpreter, then an error is returned.
pub fn entrypoint<R, W>(mut input: R, error: W) -> Result<Result<(), ()>, Box<dyn error::Error>>
where
    R: io::Read,
    W: io::Write + WriteColor,
{
    let opt = Opt::from_args();
    if opt.copyright {
        let mut interp = crate::interpreter()?;
        let _ = interp.eval(b"puts RUBY_COPYRIGHT")?;
        Ok(Ok(()))
    } else if !opt.commands.is_empty() {
        Ok(execute_inline_eval(
            error,
            opt.commands,
            opt.fixture.as_deref(),
        )?)
    } else if let Some(programfile) = opt.programfile.filter(|file| file != Path::new("-")) {
        execute_program_file(error, programfile.as_path(), opt.fixture.as_deref())
    } else {
        let mut interp = crate::interpreter()?;
        let mut program = vec![];
        input
            .read_to_end(&mut program)
            .map_err(|_| IOError::from("Could not read program from STDIN"))?;
        if let Err(ref exc) = interp.eval(program.as_slice()) {
            backtrace::format_cli_trace_into(error, &mut interp, exc)?;
            return Ok(Err(()));
        }
        Ok(Ok(()))
    }
}

fn execute_inline_eval<W>(
    error: W,
    commands: Vec<OsString>,
    fixture: Option<&Path>,
) -> Result<Result<(), ()>, Box<dyn error::Error>>
where
    W: io::Write + WriteColor,
{
    let mut interp = crate::interpreter()?;
    interp.pop_context()?;
    // safety:
    //
    // - `Context::new_unchecked` requires that its argument has no NUL bytes.
    // - `INLINE_EVAL_SWITCH_FILENAME` is controlled by this crate.
    // - A test asserts that `INLINE_EVAL_SWITCH_FILENAME` has no NUL bytes.
    let context = unsafe { Context::new_unchecked(INLINE_EVAL_SWITCH_FILENAME) };
    interp.push_context(context)?;
    if let Some(ref fixture) = fixture {
        setup_fixture_hack(&mut interp, fixture)?;
    }
    for command in commands {
        if let Err(ref exc) = interp.eval_os_str(command.as_os_str()) {
            backtrace::format_cli_trace_into(error, &mut interp, exc)?;
            // short circuit, but don't return an error since we already printed it
            return Ok(Err(()));
        }
        interp.add_fetch_lineno(1)?;
    }
    Ok(Ok(()))
}

fn execute_program_file<W>(
    error: W,
    programfile: &Path,
    fixture: Option<&Path>,
) -> Result<Result<(), ()>, Box<dyn error::Error>>
where
    W: io::Write + WriteColor,
{
    let mut interp = crate::interpreter()?;
    if let Some(ref fixture) = fixture {
        setup_fixture_hack(&mut interp, fixture)?;
    }
    if let Err(ref exc) = interp.eval_file(programfile) {
        backtrace::format_cli_trace_into(error, &mut interp, exc)?;
        return Ok(Err(()));
    }
    Ok(Ok(()))
}

fn load_error<P: AsRef<OsStr>>(file: P, message: &str) -> Result<String, Error> {
    let mut buf = String::from(message);
    buf.push_str(" -- ");
    let path = ffi::os_str_to_bytes(file.as_ref())?;
    string::format_unicode_debug_into(&mut buf, &path)?;
    Ok(buf)
}

// This function exists to provide a workaround for Artichoke not being able to
// read from the local filesystem.
//
// By passing the `--fixture PATH` argument, this function loads the file at
// `PATH` into memory and stores it in the interpreter bound to the `$fixture`
// global.
#[inline]
fn setup_fixture_hack<P: AsRef<Path>>(interp: &mut Artichoke, fixture: P) -> Result<(), Error> {
    let data = if let Ok(data) = std::fs::read(fixture.as_ref()) {
        data
    } else {
        return Err(
            LoadError::from(load_error(fixture.as_ref(), "No such file or directory")?).into(),
        );
    };
    let value = interp.convert_mut(data);
    interp.set_global_variable(&b"$fixture"[..], &value)?;
    Ok(())
}
