use artichoke_backend::eval::{Context, Eval};
use artichoke_backend::fs;
use artichoke_core::ArtichokeError;
use bstr::BStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
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

    #[structopt(parse(from_os_str))]
    programfile: Option<PathBuf>,
}

enum Error {
    Artichoke(ArtichokeError),
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

fn try_main() -> Result<(), Error> {
    let opt = Opt::from_args();
    if opt.copyright {
        let interp = artichoke_backend::interpreter()?;
        interp.eval("puts RUBY_COPYRIGHT")?;
    } else if !opt.commands.is_empty() {
        let interp = artichoke_backend::interpreter()?;
        interp.push_context(Context::new(b"-e".as_ref()));
        for command in opt.commands {
            if let Ok(command) = fs::osstr_to_bytes(&interp, command.as_os_str()) {
                interp.eval(command)?;
            } else {
                return Err(Error::from(
                    "Unable to parse non-UTF-8 command line arguments on this platform",
                ));
            }
        }
    } else if let Some(programfile) = opt.programfile {
        let interp = artichoke_backend::interpreter()?;
        let mut file = File::open(programfile.as_path()).map_err(|_| {
            if let Ok(file) = fs::osstr_to_bytes(&interp, programfile.as_os_str()) {
                let file = format!("{:?}", <&BStr>::from(file));
                format!(
                    "No such file or directory -- {} (LoadError)",
                    &file[1..file.len() - 1]
                )
            } else {
                format!("No such file or directory -- {:?} (LoadError)", programfile)
            }
        })?;
        let mut program = Vec::new();
        let result = file.read_to_end(&mut program);
        match result {
            Ok(_) => interp.eval(program.as_slice())?,
            Err(err) => {
                let reason = match err.kind() {
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
                };
                return Err(Error::Fail(reason));
            }
        };
    } else {
        let mut program = Vec::new();
        let result = io::stdin().read_to_end(&mut program);
        if result.is_ok() {
            let interp = artichoke_backend::interpreter()?;
            interp.eval(program.as_slice())?;
        } else {
            return Err(Error::from("Could not read program from STDIN"));
        }
    }

    Ok(())
}

fn main() {
    match try_main() {
        Ok(_) => {}
        Err(Error::Artichoke(err)) => eprintln!("{}", err),
        Err(Error::Fail(err)) => eprintln!("{}", err),
    }
}
