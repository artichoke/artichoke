//! Detect if Ruby code parses successfully.
//!
//! The REPL needs to check if code is valid to determine whether it should
//! enter multiline editing mode.

use std::ffi::CStr;
use std::ptr::NonNull;

use crate::backend::sys;
use crate::backend::Artichoke;

/// State shows whether artichoke can parse some code or why it cannot.
///
/// This enum only encapsulates whether artichoke can parse the code. It may
/// still have syntactic or semantic errors.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    /// Internal parser error. This is a fatal error.
    ParseError,
    /// Code must be fewer than [`isize::MAX`] bytes.
    CodeTooLong,
    /// The code has too many end statements.
    UnexpectedEnd,
    /// The code has unclosed blocks.
    UnexpectedProgramEnd,
    /// The current expression is an unterminated `Regexp`.
    UnexpectedRegexpBegin,
    /// The current expression is an unterminated block.
    UnterminatedBlock,
    /// The current expression is an unterminated heredoc.
    UnterminatedHeredoc,
    /// The current expression is an unterminated `String`.
    UnterminatedString,
    /// Code is valid and fit to eval.
    Valid,
}

impl State {
    /// Construct a new, default `State`.
    #[must_use]
    pub const fn new() -> Self {
        Self::Valid
    }

    /// Whether this variant indicates a code block is open.
    ///
    /// This method can be used by a REPL to check whether to buffer code or
    /// begin a multi-line editing session before attempting to eval the code on
    /// an interpreter.
    #[must_use]
    pub fn is_code_block_open(self) -> bool {
        !matches!(
            self,
            Self::Valid | Self::UnexpectedEnd | Self::UnexpectedRegexpBegin | Self::CodeTooLong
        )
    }

    /// Whether this variant is a recoverable error.
    ///
    /// Recoverable errors should be handled by resetting the parser and input
    /// buffer.
    #[must_use]
    pub fn is_recoverable_error(self) -> bool {
        matches!(self, Self::CodeTooLong)
    }

    /// Whether this variant is a fatal parse error.
    ///
    /// Fatal parser states indicate the parser is corrupted and cannot be used
    /// again.
    #[must_use]
    pub fn is_fatal(self) -> bool {
        matches!(self, Self::ParseError)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// Wraps a [`artichoke_backend`] mruby parser.
#[derive(Debug)]
pub struct Parser {
    parser: NonNull<sys::mrb_parser_state>,
    context: NonNull<sys::mrbc_context>,
}

impl Parser {
    /// Create a new parser from an interpreter instance.
    #[must_use]
    pub fn new(interp: &mut Artichoke) -> Option<Self> {
        let state = interp.state.as_deref_mut()?;
        let context = state.parser.as_mut()?.context_mut();
        let context = NonNull::new(context)?;
        let parser = unsafe { interp.with_ffi_boundary(|mrb| sys::mrb_parser_new(mrb)).ok()? };
        let parser = NonNull::new(parser)?;
        Some(Self { parser, context })
    }

    /// Parse the code buffer to determine if the code is a complete expression
    /// that could be evaluated even though it may not be syntactically or
    /// semantically valid.
    ///
    /// # Errors
    ///
    /// If the supplied code is more than `isize::MAX` bytes long, an error is
    /// returned,
    ///
    /// If the underlying parser returns a UTF-8 invalid error message, an error
    /// is returned.
    #[allow(clippy::enum_glob_use)]
    pub fn parse(&mut self, code: &[u8]) -> State {
        use sys::mrb_lex_state_enum::*;

        let len = if let Ok(len) = isize::try_from(code.len()) {
            len
        } else {
            return State::CodeTooLong;
        };
        let parser = unsafe { self.parser.as_mut() };
        let context = unsafe { self.context.as_mut() };

        let ptr = code.as_ptr().cast::<i8>();
        parser.s = ptr;
        parser.send = unsafe { ptr.offset(len) };
        parser.lineno = context.lineno;
        unsafe {
            sys::mrb_parser_parse(parser, context);
        }

        if !parser.parsing_heredoc.is_null() {
            return State::UnterminatedHeredoc;
        }
        if !parser.lex_strterm.is_null() {
            return State::UnterminatedString;
        }
        if parser.nerr > 0 {
            let errmsg = parser.error_buffer[0].message;
            if errmsg.is_null() {
                return State::ParseError;
            }
            let cstring = unsafe { CStr::from_ptr(errmsg) };
            if let Ok(message) = cstring.to_str() {
                match message {
                    "syntax error, unexpected $end" => State::UnexpectedProgramEnd,
                    "syntax error, unexpected keyword_end" => State::UnexpectedEnd,
                    "syntax error, unexpected tREGEXP_BEG" => State::UnexpectedRegexpBegin,
                    _ => State::ParseError,
                }
            } else {
                State::ParseError
            }
        } else {
            #[allow(clippy::match_same_arms)]
            let code_has_unterminated_expression = match parser.lstate {
                // beginning of a statement, that means previous line ended
                EXPR_BEG => false,
                // a message dot was the last token, there has to come more
                EXPR_DOT => true,
                // class keyword is not enough! we need also a name of the class
                EXPR_CLASS => true,
                // a method name is necessary
                EXPR_FNAME => true,
                // if, elsif, etc. without condition
                EXPR_VALUE => true,
                // an argument is the last token
                EXPR_ARG => false,
                // a block/proc/lambda argument is the last token
                EXPR_CMDARG => false,
                // an expression was ended
                EXPR_END => false,
                // closing parenthesis
                EXPR_ENDARG => false,
                // definition end
                EXPR_ENDFN => false,
                // jump keyword like break, return, ...
                EXPR_MID => false,
                // this token is unreachable and is used to do integer math on the
                // values of `mrb_lex_state_enum`.
                EXPR_MAX_STATE => false,
            };
            if code_has_unterminated_expression {
                State::UnterminatedBlock
            } else {
                State::Valid
            }
        }
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        unsafe {
            sys::mrb_parser_free(self.parser.as_mut());
        }
    }
}
