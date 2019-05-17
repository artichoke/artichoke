#![deny(clippy::all, clippy::pedantic)]

use mruby::eval::{EvalContext, MrbEval};
use mruby::gc::GarbageCollection;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::sys::{self, DescribeState};
use mruby::MrbError;
use rustyline::error::ReadlineError;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::process;

const REPL_FILENAME: &str = "(rirb)";

struct Parser(*mut sys::mrb_parser_state);

impl Parser {
    fn new(interp: &Mrb) -> Option<Self> {
        let mrb = interp.borrow().mrb;
        let parser = unsafe { sys::mrb_parser_new(mrb) };
        if parser.is_null() {
            None
        } else {
            Some(Self(parser))
        }
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        unsafe {
            sys::mrb_parser_free(self.0);
        }
    }
}

// Parse the current code buffer to determine if any blocks are open
fn is_code_block_open(interp: &Mrb, parser: &Parser, code: &str) -> Result<bool, MrbError> {
    let ctx = interp.borrow().ctx;
    unsafe {
        let bytes = code.as_bytes();
        let len =
            isize::try_from(bytes.len()).map_err(|_| MrbError::Exec("code too long".to_owned()))?;
        let ptr = bytes.as_ptr() as *const i8;
        (*parser.0).s = ptr;
        (*parser.0).send = ptr.offset(len);
        (*parser.0).lineno = i32::from((*ctx).lineno);
        sys::mrb_parser_parse(parser.0, ctx);

        // open heredoc
        if !(*parser.0).parsing_heredoc.is_null() {
            return Ok(true);
        }
        // unterminated string
        if !(*parser.0).lex_strterm.is_null() {
            return Ok(true);
        }
        if (*parser.0).nerr > 0 {
            let errmsg = (*parser.0).error_buffer[0].message;
            if errmsg.is_null() {
                return Ok(true);
            }
            let cstring = CStr::from_ptr(errmsg);
            let message = cstring
                .to_str()
                .map(ToOwned::to_owned)
                .map_err(|_| MrbError::Exec("parser error with unparseable message".to_owned()))?;
            #[allow(clippy::match_same_arms)]
            return match message.as_str() {
                "syntax error, unexpected $end" => Ok(true),
                "syntax error, unexpected keyword_end" => Ok(true),
                "syntax error, unexpected tREGEXP_BEG" => Ok(false),
                _ => Ok(true),
            };
        }
        #[allow(clippy::match_same_arms)]
        let open = match (*parser.0).lstate {
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
        };
        Ok(open)
    }
}

fn main() -> Result<(), MrbError> {
    let interp = Interpreter::create()?;
    let parser = Parser::new(&interp).ok_or(MrbError::New)?;
    interp.push_context(EvalContext::new(REPL_FILENAME));

    let mut rl = rustyline::Editor::<()>::new();
    // If a code block is open, accumulate code from multiple readlines in this
    // mutable `String` buffer.
    let mut buf = String::new();
    let mut code_block_open = false;
    loop {
        // Allow shell users to identify that they have an open code block.
        let prompt = if code_block_open {
            format!("[{}] * ", interp.borrow().mrb.version())
        } else {
            format!("[{}] > ", interp.borrow().mrb.version())
        };

        let readline = rl.readline(&prompt);
        let code = match readline {
            Ok(line) => line,
            // Gracefully exit on CTRL-D EOF
            Err(ReadlineError::Eof) => break,
            Err(_) => process::exit(1),
        };
        buf.push_str(&code);
        if is_code_block_open(&interp, &parser, buf.as_str())? {
            // Add a newline to the code buffer to mirror the multiple lines
            // that the shell user is typing into readline.
            buf.push('\n');
            code_block_open = true;
        } else {
            match interp.eval(buf.as_str()) {
                Ok(value) => println!("=> {}", value.inspect()),
                Err(err) => eprintln!("{}", err),
            };
            for line in buf.lines() {
                rl.add_history_entry(line);
            }
            // We have successfully evaled some Ruby code. Reset REPL state.
            interp.incremental_gc();
            buf.clear();
            code_block_open = false;
        }
    }
    Ok(())
}
