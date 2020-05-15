#![warn(missing_docs, intra_doc_link_resolution_failure)]

//! Rust bindings for mruby, customized for Artichoke.
//!
//! Bindings are based on the
//! [vendored mruby sources](https://github.com/artichoke/mruby) and generated
//! with bindgen.

use std::ffi::CStr;
use std::fmt::Write;

mod args;
#[allow(missing_debug_implementations)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(clippy::restriction)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}
pub(crate) mod protect;

#[cfg(test)]
mod ffi_tests;

pub use self::args::*;
pub use self::ffi::*;

/// Version metadata `String` for embedded mruby.
#[must_use]
pub fn mrb_sys_mruby_version(verbose: bool) -> String {
    if verbose {
        // Safety:
        //
        // - `MRUBY_RUBY_ENGINE` is already a `CString` pulled from bindgen.
        // - `MRUBY_RUBY_VERSION` is already a `CString` pulled from bindgen.
        let engine = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_ENGINE) };
        let version = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_VERSION) };
        let mut out = String::new();
        out.push_str(engine.to_str().unwrap_or("unknown"));
        out.push(' ');
        out.push_str(version.to_str().unwrap_or("0.0.0"));
        out.push_str(" [");
        out.push_str(env!("CARGO_PKG_VERSION"));
        out.push(']');
        out
    } else {
        String::from(env!("CARGO_PKG_VERSION"))
    }
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
pub fn mrb_sys_state_debug(mrb: *mut mrb_state) -> String {
    // Safety:
    //
    // - `MRUBY_RUBY_ENGINE` is already a `CString` pulled from bindgen.
    // - `MRUBY_RUBY_VERSION` is already a `CString` pulled from bindgen.
    let engine = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_ENGINE) };
    let version = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_VERSION) };
    let mut debug = String::new();
    // Explicitly supressed error since we are only generating debug info and
    // cannot panic.
    //
    // In practice, this call to `write!` will never panic because the `Display`
    // impls of `str` and `i64` are not buggy and writing to a `String`
    // `fmt::Write` will never panic on its own.
    let _ = write!(
        &mut debug,
        "{} {} (v{}.{}.{}) interpreter at {:p}",
        engine.to_str().unwrap_or("unknown"),
        version.to_str().unwrap_or("0.0.0"),
        MRUBY_RELEASE_MAJOR,
        MRUBY_RELEASE_MINOR,
        MRUBY_RELEASE_TEENY,
        mrb
    );
    debug
}

#[cfg(test)]
mod tests {
    use crate::sys;

    #[test]
    fn interpreter_debug() {
        unsafe {
            let mrb = sys::mrb_open();
            let debug = sys::mrb_sys_state_debug(mrb);
            assert_eq!(
                debug,
                format!("mruby 2.0 (v2.0.1) interpreter at {:p}", &*mrb)
            );
            sys::mrb_close(mrb);
        }
    }
}
