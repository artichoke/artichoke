//! Detect if Ruby code parses successfully.
//!
//! The REPL needs to check if code is valid to determine whether it should
//! enter multiline editing mode.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::NonNull;

use crate::backend::sys;
use crate::backend::{Artichoke, Error};

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
pub struct Parser<'a> {
    interp: &'a mut Artichoke,
    parser: NonNull<sys::mrb_parser_state>,
    context: NonNull<sys::mrbc_context>,
}

impl<'a> Parser<'a> {
    /// Create a new parser from an interpreter instance.
    #[must_use]
    pub fn new(interp: &'a mut Artichoke) -> Option<Self> {
        let state = interp.state.as_deref_mut()?;
        let context = state.parser.as_mut()?.context_mut();
        let context = NonNull::new(context)?;
        let parser = unsafe { interp.with_ffi_boundary(|mrb| sys::mrb_parser_new(mrb)).ok()? };
        let parser = NonNull::new(parser)?;
        Some(Self {
            interp,
            parser,
            context,
        })
    }

    /// Return a reference to the wrapped interpreter.
    #[must_use]
    pub fn interp(&mut self) -> &mut Artichoke {
        self.interp
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
    pub fn parse(&mut self, code: &[u8]) -> Result<State, Error> {
        use sys::mrb_lex_state_enum::*;

        let len = if let Ok(len) = isize::try_from(code.len()) {
            len
        } else {
            return Ok(State::CodeTooLong);
        };
        let parser = unsafe { self.parser.as_mut() };
        let context = unsafe { self.context.as_mut() };

        let ptr = code.as_ptr().cast::<c_char>();
        parser.s = ptr;
        parser.send = unsafe { ptr.offset(len) };
        parser.lineno = context.lineno;
        unsafe {
            self.interp.with_ffi_boundary(|_| {
                sys::mrb_parser_parse(parser, context);
            })?;
        }

        if !parser.parsing_heredoc.is_null() {
            return Ok(State::UnterminatedHeredoc);
        }
        if !parser.lex_strterm.is_null() {
            return Ok(State::UnterminatedString);
        }
        let state = if parser.nerr > 0 {
            let errmsg = parser.error_buffer[0].message;
            if errmsg.is_null() {
                return Ok(State::ParseError);
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
        };
        Ok(state)
    }
}

impl<'a> Drop for Parser<'a> {
    fn drop(&mut self) {
        let Self { interp, parser, .. } = self;

        unsafe {
            let _ignored = interp.with_ffi_boundary(|_| {
                sys::mrb_parser_free(parser.as_mut());
            });
        }
        // There is no need to free `context` since it is owned by the
        // Artichoke state.
    }
}
