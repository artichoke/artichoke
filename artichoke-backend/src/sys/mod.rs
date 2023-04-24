#![warn(missing_docs)]

//! Rust bindings for mruby, customized for Artichoke.
//!
//! Bindings are based on the [vendored mruby sources] and generated with
//! [bindgen].
//!
//! [vendored mruby sources]: https://github.com/artichoke/mruby
//! [bindgen]: https://rust-lang.github.io/rust-bindgen/

use std::ffi::CStr;
use std::fmt::{self, Write};

use crate::types::{self, Ruby};

mod args;
#[allow(missing_debug_implementations)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(trivial_casts)]
#[allow(trivial_numeric_casts)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(clippy::restriction)]
mod ffi;
pub(crate) mod protect;

pub use self::args::*;
pub use self::ffi::*;

impl Default for mrb_value {
    fn default() -> Self {
        unsafe { mrb_sys_nil_value() }
    }
}

impl fmt::Debug for mrb_value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match types::ruby_from_mrb_value(*self) {
            Ruby::Nil => f.write_str("nil"),
            Ruby::Bool if unsafe { mrb_sys_value_is_true(*self) } => f.write_str("true"),
            Ruby::Bool => f.write_str("false"),
            Ruby::Fixnum => {
                let fixnum = unsafe { mrb_sys_fixnum_to_cint(*self) };
                write!(f, "{fixnum}")
            }
            Ruby::Float => {
                let float = unsafe { mrb_sys_float_to_cdouble(*self) };
                write!(f, "{float}")
            }
            type_tag => write!(f, "<{type_tag}>"),
        }
    }
}

/// Version metadata `String` for embedded mruby.
#[must_use]
pub fn mrb_sys_mruby_version(verbose: bool) -> String {
    if !verbose {
        return String::from(env!("CARGO_PKG_VERSION"));
    }
    let engine = CStr::from_bytes_with_nul(MRUBY_RUBY_ENGINE);
    let engine = engine.ok().and_then(|cstr| cstr.to_str().ok()).unwrap_or("unknown");
    let version = CStr::from_bytes_with_nul(MRUBY_RUBY_VERSION);
    let version = version.ok().and_then(|cstr| cstr.to_str().ok()).unwrap_or("0.0.0");
    let mut out = String::new();
    out.push_str(engine);
    out.push(' ');
    out.push_str(version);
    out.push_str(" [");
    out.push_str(env!("CARGO_PKG_VERSION"));
    out.push(']');
    out
}

/// Debug representation for [`mrb_state`].
///
/// Returns Ruby engine, interpreter version, engine version, and [`mrb_state`]
/// address. For example:
///
/// ```text
/// mruby 2.0 (v2.0.1 rev c078758) interpreter at 0x7f85b8800000
/// ```
///
/// This function is infallible and guaranteed not to panic.
#[must_use]
pub fn mrb_sys_state_debug(mrb: *mut mrb_state) -> String {
    let engine = CStr::from_bytes_with_nul(MRUBY_RUBY_ENGINE);
    let engine = engine.ok().and_then(|cstr| cstr.to_str().ok()).unwrap_or("unknown");
    let version = CStr::from_bytes_with_nul(MRUBY_RUBY_VERSION);
    let version = version.ok().and_then(|cstr| cstr.to_str().ok()).unwrap_or("0.0.0");
    let mut debug = String::new();
    // Explicitly suppressed error since we are only generating debug info and
    // cannot panic.
    //
    // In practice, this call to `write!` will never panic because the `Display`
    // impls of `str` and `i64` are not buggy and writing to a `String`
    // `fmt::Write` will never panic on its own.
    let _ = write!(
        &mut debug,
        "{engine} {version} (v{MRUBY_RELEASE_MAJOR}.{MRUBY_RELEASE_MINOR}.{MRUBY_RELEASE_TEENY}) interpreter at {mrb:p}"
    );
    debug
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn interpreter_debug() {
        // Since the introduction of Rust symbol table, `mrb_open` cannot be
        // called without an Artichoke `State`.
        let mut interp = interpreter();
        unsafe {
            let mrb = interp.mrb.as_mut();
            let debug = sys::mrb_sys_state_debug(mrb);
            assert_eq!(debug, format!("mruby 3.2 (v3.2.0) interpreter at {:p}", &*mrb));
        };
    }
}
