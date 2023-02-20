use std::env;
use std::fs;

use bstr::ByteSlice;
use directories::BaseDirs;
use rustyline::config::EditMode;

/// Read inputc contents according to the GNU Readline hierarchy of config files.
///
/// This routine is ported from GNU Readline's `rl_read_init_file` function as
/// of commit `7274faabe97ce53d6b464272d7e6ab6c1392837b`.
///
/// > Do key bindings from a file.  If FILENAME is NULL it defaults
/// > to the first non-null filename from this list:
/// >   1. the filename used for the previous call
/// >   2. the value of the shell variable `INPUTRC`
/// >   3. ~/.inputrc
/// >   4. /etc/inputrc
/// > If the file existed and could be opened and read, 0 is returned,
/// > otherwise errno is returned. */
pub fn rl_read_init_file() -> Option<Vec<u8>> {
    if let Some(inputrc) = env::var_os("INPUTRC") {
        return fs::read(inputrc).ok();
    }
    if let Some(base_dirs) = BaseDirs::new() {
        let inputrc = base_dirs.home_dir().join(".inputrc");
        if let Ok(content) = fs::read(inputrc) {
            return Some(content);
        }
    }
    if let Ok(content) = fs::read("/etc/inputrc") {
        return Some(content);
    }
    if cfg!(windows) {
        if let Some(base_dirs) = BaseDirs::new() {
            let inputrc = base_dirs.home_dir().join("_inputrc");
            if let Ok(content) = fs::read(inputrc) {
                return Some(content);
            }
        }
    }
    None
}

/// Look for vi editing mode in inputrc, like this:
///
/// ```txt
/// # Vi mode
/// set editing-mode vi
/// ```
pub fn get_readline_edit_mode(contents: impl AsRef<[u8]>) -> Option<EditMode> {
    let contents = contents.as_ref();

    for line in contents.lines() {
        // Skip leading whitespace.
        let line = trim_whitespace_front(line);

        // If the line is not a comment, then parse it.
        if matches!(line.first(), Some(b'#') | None) {
            continue;
        }

        // If this is a command to set a variable, then do that.
        if !line.starts_with_str("set") {
            continue;
        }
        let line = &line[3..];
        // Skip leading whitespace.
        let line = trim_whitespace_front(line);

        if !line.starts_with_str("editing-mode") {
            continue;
        }
        let line = &line[12..];
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

/// Skip leading whitespace.
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
    use rustyline::config::EditMode;

    use super::get_readline_edit_mode;

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
}
