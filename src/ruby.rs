//! Infrastructure for `ruby` CLI.
//!
//! Exported as `ruby` and `artichoke` binaries.

use ansi_term::Style;
use std::ffi::{OsStr, OsString};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use crate::backend::exception::Exception;
use crate::backend::exception::RubyException;
use crate::backend::extn::core::exception::{IOError, LoadError};
use crate::backend::ffi;
use crate::backend::state::parser::Context;
use crate::backend::string;
use crate::backend::sys;
use crate::backend::{ConvertMut, Eval, Intern, Parser as _};

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

#[derive(Debug, StructOpt)]
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
pub fn entrypoint() -> Result<(), Exception> {
    let opt = Opt::from_args();
    if opt.copyright {
        let mut interp = crate::interpreter()?;
        let _ = interp.eval(b"puts RUBY_COPYRIGHT")?;
        Ok(())
    } else if !opt.commands.is_empty() {
        execute_inline_eval(opt.commands, opt.fixture.as_ref().map(Path::new))
    } else if let Some(programfile) = opt.programfile {
        execute_program_file(programfile.as_path(), opt.fixture.as_ref().map(Path::new))
    } else {
        let mut interp = crate::interpreter()?;
        let mut program = vec![];
        io::stdin()
            .read_to_end(&mut program)
            .map_err(|_| IOError::new(&interp, "Could not read program from STDIN"))?;
        let _ = interp
            .eval(program.as_slice())
            .map_err(|exc| handle_exception(&mut interp, exc));

        Ok(())
    }
}

fn execute_inline_eval(commands: Vec<OsString>, fixture: Option<&Path>) -> Result<(), Exception> {
    let mut interp = crate::interpreter()?;
    // safety:
    //
    // - `Context::new_unchecked` requires that its argument has no NUL bytes.
    // - `INLINE_EVAL_SWITCH_FILENAME` is controlled by this crate.
    // - A test asserts that `INLINE_EVAL_SWITCH_FILENAME` has no NUL bytes.
    interp.push_context(unsafe { Context::new_unchecked(INLINE_EVAL_SWITCH_FILENAME) });
    if let Some(ref fixture) = fixture {
        let data = if let Ok(data) = std::fs::read(fixture) {
            data
        } else {
            return Err(Exception::from(LoadError::new(
                &interp,
                load_error(fixture.as_os_str(), "No such file or directory")?,
            )));
        };
        let sym = interp.intern_symbol(&b"$fixture"[..]);
        let mrb = interp.0.borrow().mrb;
        let value = interp.convert_mut(data);
        unsafe {
            sys::mrb_gv_set(mrb, sym, value.inner());
        }
    }
    for command in commands {
        if let Err(exc) = interp.eval_os_str(command.as_os_str()) {
            handle_exception(&mut interp, exc)?;
            return Ok(()); // short circuit, but don't return an error since we already printed it
        }
    }
    Ok(())
}

fn execute_program_file(programfile: &Path, fixture: Option<&Path>) -> Result<(), Exception> {
    let mut interp = crate::interpreter()?;
    if let Some(ref fixture) = fixture {
        let data = if let Ok(data) = std::fs::read(fixture) {
            data
        } else {
            return Err(Exception::from(LoadError::new(
                &interp,
                load_error(fixture.as_os_str(), "No such file or directory")?,
            )));
        };
        let sym = interp.intern_symbol(&b"$fixture"[..]);
        let mrb = interp.0.borrow().mrb;
        let value = interp.convert_mut(data);
        unsafe {
            sys::mrb_gv_set(mrb, sym, value.inner());
        }
    }
    let program = match std::fs::read(programfile) {
        Ok(programfile) => programfile,
        Err(err) => {
            return match err.kind() {
                io::ErrorKind::NotFound => Err(Exception::from(LoadError::new(
                    &interp,
                    load_error(programfile.as_os_str(), "No such file or directory")?,
                ))),
                io::ErrorKind::PermissionDenied => Err(Exception::from(LoadError::new(
                    &interp,
                    load_error(programfile.as_os_str(), "Permission denied")?,
                ))),
                _ => Err(Exception::from(LoadError::new(
                    &interp,
                    load_error(programfile.as_os_str(), "Could not read file")?,
                ))),
            }
        }
    };

    let _ = interp
        .eval(program.as_slice())
        .map_err(|exc| handle_exception(&mut interp, exc));

    Ok(())
}

fn load_error(file: &OsStr, message: &str) -> Result<String, Exception> {
    let mut buf = String::from(message);
    buf.push_str(" -- ");
    let path = ffi::os_str_to_bytes(file)?;
    string::format_unicode_debug_into(&mut buf, &path)?;
    Ok(buf)
}

fn handle_exception(
    interp: &mut artichoke_backend::Artichoke,
    exc: artichoke_backend::exception::Exception,
) -> Result<(), Exception> {
    if let Some(backtrace) = exc.vm_backtrace(interp) {
        writeln!(
            io::stderr(),
            "{} (most recent call last)",
            Style::new().bold().paint("Traceback")
        )?;
        for (num, frame) in backtrace.into_iter().enumerate().rev() {
            write!(io::stderr(), "\t{}: from ", num + 1)?;
            io::stderr().write_all(frame.as_slice())?;
            writeln!(io::stderr())?;
        }
    }

    write!(
        io::stderr(),
        "{} {}",
        Style::new().bold().paint(exc.name()),
        Style::new().bold().paint("(")
    )?;

    Style::new()
        .bold()
        .underline()
        .paint(exc.message())
        .write_to(&mut io::stderr())?;

    writeln!(io::stderr(), "{}", Style::new().bold().paint(")"))?;

    Ok(())
}
