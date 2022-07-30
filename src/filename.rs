/// Filename for inline code executed on the `ruby` frontend with the `-e`
/// switch.
///
/// # Examples
///
/// ```console
/// $ ruby -e 'puts RUBY_VERSION' -e 'puts __FILE__'
/// 2.6.3
/// -e
/// ```
pub const INLINE_EVAL_SWITCH: &[u8] = b"-e";

/// Filename for code executed on the REPL frontend with the `irb` command.
///
/// # Examples
///
/// ```console
/// $ irb
/// [2.6.3] > RUBY_VERSION
/// => "2.6.3"
/// [2.6.3] > __FILE__
/// => "(irb)"
/// ```
pub const REPL: &[u8] = b"(airb)";

// These tests assert that the filename constants do not contain NUL bytes which
// makes them safe to use with `Context::new_unchecked`.
#[cfg(test)]
mod tests {
    use crate::backend::state::parser::Context;

    #[test]
    fn inline_eval_switch_filename_does_not_contain_nul_byte() {
        let contains_nul_byte = super::INLINE_EVAL_SWITCH.contains(&b'\0');
        assert!(!contains_nul_byte);
    }

    #[test]
    fn inline_eval_switch_context_new_unchecked_safety() {
        Context::new(super::INLINE_EVAL_SWITCH).unwrap();
    }

    #[test]
    fn repl_filename_does_not_contain_nul_byte() {
        let contains_nul_byte = super::REPL.contains(&b'\0');
        assert!(!contains_nul_byte);
    }

    #[test]
    fn repl_context_new_unchecked_safety() {
        Context::new(super::REPL).unwrap();
    }
}
