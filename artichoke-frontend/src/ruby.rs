//! Infrastructure for `ruby` CLI.
//!
//! Exported as `ruby` and `artichoke` binaries.

use artichoke_backend::convert::Convert;
use artichoke_backend::eval::Context;
use artichoke_backend::fs;
use artichoke_backend::sys;
use artichoke_core::eval::Eval;
use artichoke_core::ArtichokeError;
use bstr::BStr;
use std::ffi::OsString;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "artichoke", about = "Artichoke is a Ruby made with Rust.")]
struct Opt {
    #[structopt(long)]
    /// print the copyright
    copyright: bool,

    #[structopt(short = "e", parse(from_os_str))]
    /// one line of script. Several -e's allowed. Omit [programfile]
    commands: Vec<OsString>,

    #[structopt(long = "with-fixture", parse(from_os_str))]
    fixture: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    programfile: Option<PathBuf>,
}

/// Error from Ruby CLI frontend
pub enum Error {
    /// Error from Artichoke interpreter.
    Artichoke(ArtichokeError),
    /// Fatal error from CLI internals.
    Fail(String),
}

impl From<ArtichokeError> for Error {
    fn from(err: ArtichokeError) -> Self {
        Self::Artichoke(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Fail(err)
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Self {
        Self::Fail(err.to_owned())
    }
}

/// Main entrypoint for Artichoke's version of the `ruby` CLI.
pub fn entrypoint() -> Result<(), Error> {
    let opt = Opt::from_args();
    if opt.copyright {
        let interp = artichoke_backend::interpreter()?;
        interp.eval(b"puts RUBY_COPYRIGHT")?;
        Ok(())
    } else if !opt.commands.is_empty() {
        execute_inline_eval(opt.commands, opt.fixture.as_ref().map(Path::new))
    } else if let Some(programfile) = opt.programfile {
        execute_program_file(programfile.as_path(), opt.fixture.as_ref().map(Path::new))
    } else {
        let mut program = Vec::new();
        let result = io::stdin().read_to_end(&mut program);
        if result.is_ok() {
            let interp = artichoke_backend::interpreter()?;
            interp.eval(program.as_slice())?;
            Ok(())
        } else {
            Err(Error::from("Could not read program from STDIN"))
        }
    }
}

fn execute_inline_eval(commands: Vec<OsString>, fixture: Option<&Path>) -> Result<(), Error> {
    let interp = artichoke_backend::interpreter()?;
    interp.push_context(Context::new(b"-e".as_ref()));
    if let Some(ref fixture) = fixture {
        let data = std::fs::read(fixture).map_err(|_| {
            if let Ok(file) = fs::osstr_to_bytes(&interp, fixture.as_os_str()) {
                let file = format!("{:?}", <&BStr>::from(file));
                format!(
                    "No such file or directory -- {} (LoadError)",
                    &file[1..file.len() - 1]
                )
            } else {
                format!("No such file or directory -- {:?} (LoadError)", fixture)
            }
        })?;
        let sym = interp.0.borrow_mut().sym_intern(b"$fixture".as_ref());
        let mrb = interp.0.borrow().mrb;
        let value = interp.convert(data);
        unsafe {
            sys::mrb_gv_set(mrb, sym, value.inner());
        }
    }
    for command in commands {
        if let Ok(command) = fs::osstr_to_bytes(&interp, command.as_os_str()) {
            interp.eval(command)?;
        } else {
            return Err(Error::from(
                "Unable to parse non-UTF-8 command line arguments on this platform",
            ));
        }
    }
    Ok(())
}

fn execute_program_file(programfile: &Path, fixture: Option<&Path>) -> Result<(), Error> {
    let interp = artichoke_backend::interpreter()?;
    if let Some(ref fixture) = fixture {
        let data = std::fs::read(fixture).map_err(|_| {
            if let Ok(file) = fs::osstr_to_bytes(&interp, fixture.as_os_str()) {
                let file = format!("{:?}", <&BStr>::from(file));
                format!(
                    "No such file or directory -- {} (LoadError)",
                    &file[1..file.len() - 1]
                )
            } else {
                format!("No such file or directory -- {:?} (LoadError)", fixture)
            }
        })?;
        let sym = interp.0.borrow_mut().sym_intern(b"$fixture".as_ref());
        let mrb = interp.0.borrow().mrb;
        let value = interp.convert(data);
        unsafe {
            sys::mrb_gv_set(mrb, sym, value.inner());
        }
    }
    let program = std::fs::read(programfile).map_err(|err| match err.kind() {
        io::ErrorKind::NotFound => {
            if let Ok(file) = fs::osstr_to_bytes(&interp, programfile.as_os_str()) {
                let file = format!("{:?}", <&BStr>::from(file));
                format!(
                    "No such file or directory -- {} (LoadError)",
                    &file[1..file.len() - 1]
                )
            } else {
                format!("No such file or directory -- {:?} (LoadError)", programfile)
            }
        }
        io::ErrorKind::PermissionDenied => {
            if let Ok(file) = fs::osstr_to_bytes(&interp, programfile.as_os_str()) {
                let file = format!("{:?}", <&BStr>::from(file));
                format!(
                    "Permission denied -- {} (LoadError)",
                    &file[1..file.len() - 1]
                )
            } else {
                format!("Permission denied -- {:?} (LoadError)", programfile)
            }
        }
        _ => {
            if let Ok(file) = fs::osstr_to_bytes(&interp, programfile.as_os_str()) {
                let file = format!("{:?}", <&BStr>::from(file));
                format!(
                    "Could not read file -- {} (LoadError)",
                    &file[1..file.len() - 1]
                )
            } else {
                format!("Could not read file -- {:?} (LoadError)", programfile)
            }
        }
    })?;
    interp.eval(program.as_slice())?;
    Ok(())
}
