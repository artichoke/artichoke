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
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Helpers for integrating REPLs with GNU Readline.
//!
//! This crate can be used to parse Readline editing mode from the standard set
//! of GNU Readline config files.
//!
//! # Examples
//!
//! ```
//! use artichoke_readline::{get_readline_edit_mode, rl_read_init_file, EditMode};
//!
//! if let Some(config) = rl_read_init_file() {
//!     if matches!(get_readline_edit_mode(config), Some(EditMode::Vi)) {
//!         println!("You have chosen wisely");
//!     }
//! }
//! ```
//!
//! # Crate Features
//!
//! The **rustyline** feature (enabled by default) adds trait implementations to
//! allow [`EditMode`] to interoperate with the corresponding enum in the
//! `rustyline` crate.

use std::env;
use std::fs;
use std::path::PathBuf;

use bstr::ByteSlice;

/// Readline editing mode.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EditMode {
    /// Emacs keymap.
    ///
    /// Emacs is the default keymap.
    #[default]
    Emacs,
    /// Vi keymap.
    Vi,
}

#[cfg(feature = "rustyline")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustyline")))]
impl From<EditMode> for rustyline::config::EditMode {
    fn from(mode: EditMode) -> Self {
        match mode {
            EditMode::Emacs => Self::Emacs,
            EditMode::Vi => Self::Vi,
        }
    }
}

#[cfg(feature = "rustyline")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustyline")))]
impl From<rustyline::config::EditMode> for EditMode {
    fn from(mode: rustyline::config::EditMode) -> Self {
        match mode {
            rustyline::config::EditMode::Emacs => Self::Emacs,
            rustyline::config::EditMode::Vi => Self::Vi,
        }
    }
}

#[cfg(feature = "rustyline")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustyline")))]
impl PartialEq<EditMode> for rustyline::config::EditMode {
    fn eq(&self, other: &EditMode) -> bool {
        self == other.into()
    }
}

#[cfg(feature = "rustyline")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustyline")))]
impl PartialEq<rustyline::config::EditMode> for EditMode {
    fn eq(&self, other: &rustyline::config::EditMode) -> bool {
        self == other.into()
    }
}

/// Read inputrc contents according to the GNU Readline hierarchy of config
/// files.
///
/// This function will try each file in the config hierarchy (with the addition
/// of `%USERPROFILE%\_inputrc` on Windows). This function returns the contents
/// of the first file that exists and is successfully read. If no config file is
/// found, [`None`] is returned.
///
/// # Upstream Implementation
///
/// This routine is ported from GNU Readline's `rl_read_init_file` function as
/// of commit [`7274faabe97ce53d6b464272d7e6ab6c1392837b`][upstream], which has
/// the following documentation:
///
/// > Do key bindings from a file. If FILENAME is NULL it defaults to the first
/// > non-null filename from this list:
/// >
/// > 1. the filename used for the previous call
/// > 2. the value of the shell variable `INPUTRC`
/// > 3. ~/.inputrc
/// > 4. /etc/inputrc
/// >
/// > If the file existed and could be opened and read, 0 is returned, otherwise
/// > errno is returned.
///
/// [upstream]: https://git.savannah.gnu.org/cgit/readline.git/tree/bind.c?h=7274faabe97ce53d6b464272d7e6ab6c1392837b#n1032
#[must_use]
pub fn rl_read_init_file() -> Option<Vec<u8>> {
    if let Some(inputrc) = env::var_os("INPUTRC") {
        return fs::read(inputrc).ok();
    }

    let home_dir = home_dir();
    if let Some(ref home_dir) = home_dir {
        let inputrc = home_dir.join(".inputrc");
        if let Ok(content) = fs::read(inputrc) {
            return Some(content);
        }
    }

    if let Ok(content) = fs::read("/etc/inputrc") {
        return Some(content);
    }

    if cfg!(windows) {
        if let Some(home_dir) = home_dir {
            let inputrc = home_dir.join("_inputrc");
            if let Ok(content) = fs::read(inputrc) {
                return Some(content);
            }
        }
    }

    None
}

#[cfg(not(any(unix, windows)))]
fn home_dir() -> Option<PathBuf> {
    None
}

#[cfg(unix)]
fn home_dir() -> Option<PathBuf> {
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
    env::home_dir()
}

#[cfg(windows)]
fn home_dir() -> Option<PathBuf> {
    use known_folders::{get_known_folder_path, KnownFolder};

    get_known_folder_path(KnownFolder::Profile)
}

/// Parse readline editing mode from the given byte content, which should be
/// the contents of an inputrc config file.
///
/// See [`rl_read_init_file`] for how to retrieve the contents of the effective
/// inputrc file.
///
/// # Examples
///
/// ```
/// use artichoke_readline::{get_readline_edit_mode, EditMode};
///
/// const INPUTRC: &str = "
/// # Vi mode
/// set editing-mode vi
/// ";
///
/// assert_eq!(get_readline_edit_mode(INPUTRC), Some(EditMode::Vi));
/// ```
#[must_use]
pub fn get_readline_edit_mode(contents: impl AsRef<[u8]>) -> Option<EditMode> {
    fn inner(contents: &[u8]) -> Option<EditMode> {
        for line in contents.lines() {
            // Skip leading whitespace.
            let line = trim_whitespace_front(line);

            // If the line is not a comment, then parse it.
            if matches!(line.first(), Some(b'#') | None) {
                continue;
            }

            // If this is a command to set a variable, then do that.
            let line = match line.strip_prefix(b"set") {
                Some(rest) => rest,
                None => continue,
            };
            // Skip leading whitespace.
            let line = trim_whitespace_front(line);

            let line = match line.strip_prefix(b"editing-mode") {
                Some(rest) => rest,
                None => continue,
            };
            // Skip leading whitespace.
            let line = trim_whitespace_front(line);

            match line {
                b"vi" | br#""vi""# => return Some(EditMode::Vi),
                b"emacs" | br#""emacs""# => return Some(EditMode::Emacs),
                _ => return None,
            }
        }

        None
    }

    inner(contents.as_ref())
}

/// Skip leading POSIX whitespace.
fn trim_whitespace_front(mut s: &[u8]) -> &[u8] {
    loop {
        if let Some((&head, tail)) = s.split_first() {
            if posix_space::is_space(head) {
                s = tail;
                continue;
            }
        }
        break s;
    }
}

#[cfg(test)]
mod tests {
    use super::{get_readline_edit_mode, EditMode};

    #[test]
    fn parse_empty() {
        let test_cases = [
            "",
            "              ",
            "\t\t\t",
            "          \n              ",
            "\n",
            "\r\n",
            "              \r\n           ",
        ];
        for contents in test_cases {
            let result = get_readline_edit_mode(contents);
            assert_eq!(result, None);
        }
    }

    #[test]
    fn integration_inputrc_vi() {
        let test_case = "\
# Vi mode
set editing-mode vi
set keymap vi

# Ignore case on tab completion
set completion-ignore-case On
";
        let result = get_readline_edit_mode(test_case);
        assert_eq!(result, Some(EditMode::Vi));
    }

    #[test]
    fn integration_inputrc_emacs() {
        let test_case = "\
# Ignore case on tab completion
set completion-ignore-case On

set editing-mode emacs
";
        let result = get_readline_edit_mode(test_case);
        assert_eq!(result, Some(EditMode::Emacs));
    }

    #[test]
    fn integration_inputrc_vi_quoted() {
        let test_case = r#"\
# Vi mode
set editing-mode "vi"
set keymap vi

# Ignore case on tab completion
set completion-ignore-case On
"#;
        let result = get_readline_edit_mode(test_case);
        assert_eq!(result, Some(EditMode::Vi));
    }

    #[test]
    fn integration_inputrc_emacs_quoted() {
        let test_case = r#"\
# Ignore case on tab completion
set completion-ignore-case On

set editing-mode "emacs"
"#;
        let result = get_readline_edit_mode(test_case);
        assert_eq!(result, Some(EditMode::Emacs));
    }

    #[test]
    fn default_edit_mode_is_emacs() {
        assert_eq!(EditMode::default(), EditMode::Emacs);
    }

    #[test]
    #[cfg(feature = "rustyline")]
    fn edit_mode_rustyline_into() {
        assert_eq!(EditMode::Emacs.into(), rustyline::config::EditMode::Emacs);
        assert_eq!(EditMode::Vi.into(), rustyline::config::EditMode::Vi);
    }
}
