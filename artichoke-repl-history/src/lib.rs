#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::question_mark)] // https://github.com/rust-lang/rust-clippy/issues/8281
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Helpers for persisting Artichoke `airb` REPL history to disk.
//!
//! This crate provides platform support for resolving the Aritchoke Ruby airb
//! REPL's application data folder and path to a history file within it.
//!
//! # Platform Support
//!
//! On macOS, the history file is located in the current user's Application
//! Support directory.
//!
//! On Windows, the history file is located in the current user's `LocalAppData`
//! known folder.
//!
//! On Linux and other non-macOS unix targets, the history file is located in
//! the `XDG_STATE_DIR` according to the [XDG Base Directory Specification],
//! with the specified fallback if the environment variable is not set.
//!
//! # Examples
//!
//! ```
//! use artichoke_repl_history::repl_history_file;
//!
//! if let Some(hist_file) = repl_history_file() {
//!     // load history ...
//! }
//! ```
//!
//! [XDG Base Directory Specification]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

use std::env;
use std::path::PathBuf;

/// Retrieve the path to the REPL history file.
///
/// This function will attempt to create all parent directories of the returned
/// path. If creating parent directories fails, the error is ignored.
///
/// Callers should call this function once at startup and retain the returned
/// value for later use. Some platforms depend on ambient global state in the
/// environment, so subsequent calls may return different results.
///
/// # Platform Notes
///
/// The file is stored in the application data directory for the host operating
/// system.
///
/// On macOS, the history file is located at a path like:
///
/// ```text
/// /Users/username/Library/Application Support/org.artichokeruby.airb/history
/// ```
///
/// On Windows, the history file is located at a path like:
///
/// ```text
/// C:\Users\username\AppData\Local\Artichoke Ruby\airb\data\history.txt
/// ```
///
/// On Linux and other unix platforms excluding macOS, the history file is
/// located in the XDG state home following the [XDG Base Directory
/// Specification]. By default, the history file is located at:
///
/// ```txt
/// $HOME/.local/state/artichokeruby/airb_history
/// ```
///
/// # Examples
///
/// ```
/// use artichoke_repl_history::repl_history_file;
///
/// if let Some(hist_file) = repl_history_file() {
///     // load history ...
/// }
/// ```
///
/// [XDG Base Directory Specification]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
#[must_use]
pub fn repl_history_file() -> Option<PathBuf> {
    let data_dir = repl_history_dir()?;

    // Ensure the data directory exists but ignore failures (e.g. the dir
    // already exists) because all operations on the history file are best
    // effort and non-blocking.
    //
    // On Windows, the data dir is a path like:
    //
    // ```
    // C:\Users\username\AppData\Local\Artichoke Ruby\airb\data
    // ```
    //
    // When this path doesn't exist, it contains several directories that
    // must be created, so we must use `fs::create_dir_all`.
    #[cfg(not(any(test, doctest, miri)))] // don't create side effects in tests
    let _ignored = std::fs::create_dir_all(&data_dir);

    Some(data_dir.join(history_file_basename()))
}

#[must_use]
#[cfg(target_os = "macos")]
fn repl_history_dir() -> Option<PathBuf> {
    use std::ffi::{c_char, CStr, OsString};
    use std::os::unix::ffi::OsStringExt;

    use sysdir::{
        sysdir_get_next_search_path_enumeration, sysdir_search_path_directory_t, sysdir_start_search_path_enumeration,
        PATH_MAX, SYSDIR_DOMAIN_MASK_USER,
    };

    let mut path = [0; PATH_MAX as usize];

    let dir = sysdir_search_path_directory_t::SYSDIR_DIRECTORY_APPLICATION_SUPPORT;
    let domain_mask = SYSDIR_DOMAIN_MASK_USER;
    let application_support_bytes = unsafe {
        // We don't need to loop here, just take the first result.
        let mut state = sysdir_start_search_path_enumeration(dir, domain_mask);
        let path = path.as_mut_ptr().cast::<c_char>();
        state = sysdir_get_next_search_path_enumeration(state, path);
        if state.is_finished() {
            return None;
        }
        let path = CStr::from_ptr(path);
        path.to_bytes()
    };

    // `std::env::home_dir` does not have problematic behavior on `unix`
    // targets, which includes all apple target OSes and Redox. Per the docs:
    //
    // > Deprecated since 1.29.0: This function's behavior may be unexpected on
    // > Windows. Consider using a crate from crates.io instead.
    // >
    // > -- https://doc.rust-lang.org/1.69.0/std/env/fn.home_dir.html
    //
    // Additionally, the `home` crate on crates.io, which is owned by the
    // @rust-lang organization and used in Rustup and Cargo, uses `std::env::home_dir`
    // to implement `home::home_dir` on `unix` and `target_os = "redox"` targets:
    //
    // https://docs.rs/home/0.5.5/src/home/lib.rs.html#71-75
    #[allow(deprecated)]
    let application_support = match application_support_bytes {
        [] => return None,
        [b'~'] => env::home_dir()?,
        // Per the `sysdir` man page:
        //
        // > Directory paths returned in the user domain will contain "~" to
        // > refer to the user's directory.
        //
        // Below we expand `~/` to `$HOME/` using APIs from `std`.
        [b'~', b'/', tail @ ..] => {
            let home = env::home_dir()?;
            let mut home = home.into_os_string().into_vec();

            home.try_reserve_exact(1 + tail.len()).ok()?;
            home.push(b'/');
            home.extend_from_slice(tail);

            OsString::from_vec(home).into()
        }
        path => {
            let mut buf = vec![];
            buf.try_reserve_exact(path.len()).ok()?;
            buf.extend_from_slice(path);
            OsString::from_vec(buf).into()
        }
    };
    Some(application_support.join("org.artichokeruby.airb"))
}

#[must_use]
#[cfg(all(unix, not(target_os = "macos")))]
fn repl_history_dir() -> Option<PathBuf> {
    use std::env;

    // Use state dir from XDG Base Directory Specification/
    //
    // https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
    //
    // `$XDG_STATE_HOME` defines the base directory relative to which
    // user-specific state files should be stored. If `$XDG_STATE_HOME` is
    // either not set or empty, a default equal to `$HOME/.local/state` should
    // be used.

    let state_home = match env::var_os("XDG_STATE_HOME") {
        // if `XDG_STATE_HOME` is empty, ignore it and use the default.
        Some(path) if path.is_empty() => None,
        Some(path) => Some(path),
        // if `XDG_STATE_HOME` is not set, use the default.
        None => None,
    };

    let state_dir = if let Some(state_dir) = state_dir {
        state_dir
    } else {
        // `std::env::home_dir` does not have problematic behavior on `unix`
        // targets, which includes all apple target OSes and Redox. Per the docs:
        //
        // > Deprecated since 1.29.0: This function's behavior may be unexpected on
        // > Windows. Consider using a crate from crates.io instead.
        // >
        // > -- https://doc.rust-lang.org/1.69.0/std/env/fn.home_dir.html
        //
        // Additionally, the `home` crate on crates.io, which is owned by the
        // @rust-lang organization and used in Rustup and Cargo, uses `std::env::home_dir`
        // to implement `home::home_dir` on `unix` and `target_os = "redox"` targets:
        //
        // https://docs.rs/home/0.5.5/src/home/lib.rs.html#71-75
        #[allow(deprecated)]
        let state_dir = env::home_dir()?;
        state_dir.extend([".local", "state"])
    };

    Some(state_dir.join("artichokeruby"))
}

#[must_use]
#[cfg(windows)]
fn repl_history_dir() -> Option<PathBuf> {
    use known_folders::{get_known_folder_path, KnownFolder};

    let local_app_data = get_known_folder_path(KnownFolder::LocalAppData)?;
    Some(local_app_data.join("Artichoke Ruby").join("airb").join("data"))
}

/// Basename for history file.
///
/// # Platform Notes
///
/// - On Windows, this function returns `history.txt`.
/// - On macOS, this function returns `history`.
/// - On non-macOS unix targets, this function returns `airb_history`.
#[must_use]
fn history_file_basename() -> &'static str {
    if cfg!(windows) {
        return "history.txt";
    }
    if cfg!(target_os = "macos") {
        return "history";
    }
    if cfg!(unix) {
        return "airb_history";
    }
    "history"
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::path;

    use super::*;

    // Lock for coordinating access to system env for unix target tests.
    #[cfg(all(unix, not(target_os = "macos")))]
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn history_file_basename_is_non_empty() {
        assert!(!history_file_basename().is_empty());
    }

    #[test]
    fn history_file_basename_does_not_contain_path_separators() {
        let filename = history_file_basename();
        for c in filename.chars() {
            assert!(!path::is_separator(c));
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn history_dir_on_macos() {
        let dir = repl_history_dir().unwrap();
        let mut components = dir.components();

        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Users"));
        let _skip_user_dir = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Library"));
        assert_eq!(
            components.next().unwrap().as_os_str(),
            OsStr::new("Application Support")
        );
        assert_eq!(
            components.next().unwrap().as_os_str(),
            OsStr::new("org.artichokeruby.airb")
        );
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn history_file_on_macos() {
        let file = repl_history_file().unwrap();
        let mut components = file.components();

        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Users"));
        let _skip_user_dir = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Library"));
        assert_eq!(
            components.next().unwrap().as_os_str(),
            OsStr::new("Application Support")
        );
        assert_eq!(
            components.next().unwrap().as_os_str(),
            OsStr::new("org.artichokeruby.airb")
        );
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("history"));
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn history_dir_on_unix_xdg_unset() {
        use std::env;

        let _guard = ENV_LOCK.lock();

        env::remove_env("XDG_STATE_DIR");

        let dir = repl_history_dir().unwrap();
        let mut components = dir.components();

        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("home"));
        let _skip_user_dir = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new(".local"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("state"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("artichokeruby"));
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn history_file_on_unix_xdg_unset() {
        use std::env;

        let _guard = ENV_LOCK.lock();

        env::remove_env("XDG_STATE_DIR");

        let file = repl_history_file().unwrap();
        let mut components = file.components();

        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("home"));
        let _skip_user_dir = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new(".local"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("state"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("artichokeruby"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("airb_history"));
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn history_dir_on_unix_empty_xdg_state_dir() {
        use std::env;

        let _guard = ENV_LOCK.lock();

        env::remove_env("XDG_STATE_DIR");
        env::set_var("XDG_STATE_DIR", "");

        let dir = repl_history_dir().unwrap();
        let mut components = dir.components();

        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("home"));
        let _skip_user_dir = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new(".local"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("state"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("artichokeruby"));
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn history_file_on_unix_empty_xdg_state_dir() {
        use std::env;

        let _guard = ENV_LOCK.lock();

        env::remove_env("XDG_STATE_DIR");
        env::set_var("XDG_STATE_DIR", "");

        let file = repl_history_file().unwrap();
        let mut components = file.components();

        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("home"));
        let _skip_user_dir = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new(".local"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("state"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("artichokeruby"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("airb_history"));
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn history_dir_on_unix_set_xdg_state_dir() {
        use std::env;

        let _guard = ENV_LOCK.lock();

        env::remove_env("XDG_STATE_DIR");
        env::set_var("XDG_STATE_DIR", "/opt/artichoke/state");

        let dir = repl_history_dir().unwrap();
        let mut components = dir.components();

        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("opt"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("artichoke"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("state"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("artichokeruby"));
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn history_file_on_unix_set_xdg_state_dir() {
        use std::env;

        let _guard = ENV_LOCK.lock();

        env::remove_env("XDG_STATE_DIR");
        env::set_var("XDG_STATE_DIR", "/opt/artichoke/state");

        let file = repl_history_file().unwrap();
        let mut components = file.components();

        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("opt"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("artichoke"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("state"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("artichokeruby"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("airb_history"));
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(windows)]
    fn history_dir_on_windows() {
        let dir = repl_history_dir().unwrap();
        let mut components = dir.components();

        let _skip_prefix = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Users"));
        let _skip_user_dir = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("AppData"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Local"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Artichoke Ruby"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("airb"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("data"));
        assert!(components.next().is_none());
    }

    #[test]
    #[cfg(windows)]
    fn history_file_on_windows() {
        let file = repl_history_file().unwrap();
        let mut components = file.components();

        let _skip_prefix = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("/"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Users"));
        let _skip_user_dir = components.next().unwrap();
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("AppData"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Local"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("Artichoke Ruby"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("airb"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("data"));
        assert_eq!(components.next().unwrap().as_os_str(), OsStr::new("history.txt"));
        assert!(components.next().is_none());
    }
}
