//! Parser for Ruby code that determines if it is fit to eval on an interpreter.

use artichoke_backend::sys;
use artichoke_backend::Artichoke;
use std::convert::TryFrom;
use std::ffi::CStr;

/// State shows whether artichoke can parse some code or why it cannot.
///
/// This enum only encapsulates whether artichoke can parse the code. It may
/// still have syntactic or semantic errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// Internal parser error. This is a fatal error.
    ParseError,
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
    /// Does this state indicate a code block is open? Used by the REPL to know
    /// whether to buffer code before attempting to eval it on the interpreter.
    pub fn is_code_block_open(&self) -> bool {
        match self {
            State::Valid | State::UnexpectedEnd | State::UnexpectedRegexpBegin => false,
            _ => true,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Valid
    }
}

/// Errors encountered during parsing some Ruby code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Code must be fewer than `isize::max_value` bytes.
    CodeTooLong,
    /// Fatal error with message.
    Fatal(String),
}

/// Wraps a `mruby_sys` mruby parser.
pub struct Parser {
    parser: *mut sys::mrb_parser_state,
    context: *mut sys::mrbc_context,
}

impl Parser {
    /// Create a new parser from an interpreter instance.
    pub fn new(interp: &Artichoke) -> Option<Self> {
        let mrb = interp.0.borrow().mrb;
        let context = interp.0.borrow().ctx;
        let parser = unsafe { sys::mrb_parser_new(mrb) };
        if parser.is_null() {
            None
        } else {
            Some(Self { parser, context })
        }
    }

    /// Parse the code buffer to determine if the code is a complete expression
    /// that could be evaluated even though it may not be syntactically or
    /// semantically valid.
    pub fn parse<T>(&self, code: T) -> Result<State, Error>
    where
        T: AsRef<[u8]>,
    {
        let bytes = code.as_ref();
        let len = isize::try_from(bytes.len()).map_err(|_| Error::CodeTooLong)?;
        let code_has_unterminated_expression = unsafe {
            let ptr = bytes.as_ptr() as *const i8;
            (*self.parser).s = ptr;
            (*self.parser).send = ptr.offset(len);
            (*self.parser).lineno = (*self.context).lineno;
            sys::mrb_parser_parse(self.parser, self.context);

            if !(*self.parser).parsing_heredoc.is_null() {
                return Ok(State::UnterminatedHeredoc);
            }
            if !(*self.parser).lex_strterm.is_null() {
                return Ok(State::UnterminatedString);
            }
            if (*self.parser).nerr > 0 {
                let errmsg = (*self.parser).error_buffer[0].message;
                if errmsg.is_null() {
                    return Ok(State::ParseError);
                }
                let cstring = CStr::from_ptr(errmsg);
                let message = cstring.to_str().map(ToOwned::to_owned).map_err(|_| {
                    Error::Fatal("parser error with unparseable message".to_owned())
                })?;
                #[allow(clippy::match_same_arms)]
                return match message.as_str() {
                    "syntax error, unexpected $end" => Ok(State::UnexpectedProgramEnd),
                    "syntax error, unexpected keyword_end" => Ok(State::UnexpectedEnd),
                    "syntax error, unexpected tREGEXP_BEG" => Ok(State::UnexpectedRegexpBegin),
                    _ => Ok(State::ParseError),
                };
            }
            #[allow(clippy::match_same_arms)]
            match (*self.parser).lstate {
                // beginning of a statement, that means previous line ended
                sys::mrb_lex_state_enum::EXPR_BEG => false,
                // a message dot was the last token, there has to come more
                sys::mrb_lex_state_enum::EXPR_DOT => true,
                // class keyword is not enough! we need also a name of the class
                sys::mrb_lex_state_enum::EXPR_CLASS => true,
                // a method name is necessary
                sys::mrb_lex_state_enum::EXPR_FNAME => true,
                // if, elsif, etc. without condition
                sys::mrb_lex_state_enum::EXPR_VALUE => true,
                // an argument is the last token
                sys::mrb_lex_state_enum::EXPR_ARG => false,
                // a block/proc/lambda argument is the last token
                sys::mrb_lex_state_enum::EXPR_CMDARG => false,
                // an expression was ended
                sys::mrb_lex_state_enum::EXPR_END => false,
                // closing parenthesis
                sys::mrb_lex_state_enum::EXPR_ENDARG => false,
                // definition end
                sys::mrb_lex_state_enum::EXPR_ENDFN => false,
                // jump keyword like break, return, ...
                sys::mrb_lex_state_enum::EXPR_MID => false,
                // this token is unreachable and is used to do integer math on the
                // values of `mrb_lex_state_enum`.
                sys::mrb_lex_state_enum::EXPR_MAX_STATE => false,
            }
        };
        if code_has_unterminated_expression {
            Ok(State::UnterminatedBlock)
        } else {
            Ok(State::Valid)
        }
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        unsafe {
            sys::mrb_parser_free(self.parser);
        }
    }
}
