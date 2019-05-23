#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::error;
use std::fmt;
use std::io;

#[macro_use]
pub mod macros;

pub mod class;
pub mod convert;
pub mod def;
pub mod eval;
pub mod file;
pub mod gc;
pub mod interpreter;
pub mod load;
pub mod method;
pub mod module;
pub mod state;
pub mod value;

pub use mruby_sys as sys;

#[derive(Debug)]
pub enum MrbError {
    ConvertToRuby(convert::Error<value::types::Rust, value::types::Ruby>),
    ConvertToRust(convert::Error<value::types::Ruby, value::types::Rust>),
    Exec(String),
    New,
    NotDefined(String),
    SourceNotFound(String),
    Uninitialized,
    UnreachableValue(sys::mrb_vtype),
    Vfs(io::Error),
}

impl Eq for MrbError {}

impl PartialEq for MrbError {
    fn eq(&self, other: &Self) -> bool {
        // this is a hack because io::Error does not impl PartialEq
        format!("{}", self) == format!("{}", other)
    }
}

impl fmt::Display for MrbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MrbError::ConvertToRuby(inner) => write!(f, "conversion error: {}", inner),
            MrbError::ConvertToRust(inner) => write!(f, "conversion error: {}", inner),
            MrbError::Exec(backtrace) => write!(f, "mruby exception: {}", backtrace),
            MrbError::New => write!(f, "failed to create mrb interpreter"),
            MrbError::NotDefined(fqname) => write!(f, "{} not defined", fqname),
            MrbError::SourceNotFound(source) => write!(f, "Could not load Ruby source {}", source),
            MrbError::Uninitialized => write!(f, "mrb interpreter not initialized"),
            MrbError::UnreachableValue(tt) => {
                write!(f, "extracted unreachable type {:?} from interpreter", tt)
            }
            MrbError::Vfs(err) => write!(f, "mrb vfs io error: {}", err),
        }
    }
}

impl error::Error for MrbError {
    fn description(&self) -> &str {
        "mruby interpreter error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            MrbError::ConvertToRuby(inner) => Some(inner),
            MrbError::ConvertToRust(inner) => Some(inner),
            MrbError::Vfs(inner) => Some(inner),
            _ => None,
        }
    }
}
