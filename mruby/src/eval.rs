use log::{debug, warn};
use std::ffi::CString;
use std::rc::Rc;

use crate::interpreter::{Mrb, MrbApi};
use crate::sys::{self, DescribeState};
use crate::value::Value;
use crate::MrbError;

const TOP_FILENAME: &str = "(eval)";

/// `EvalContext` is used to manipulate the state of a wrapped
/// [`sys::mrb_state`]. [`Mrb`] maintains a stack of `EvalContext`s and
/// [`MrbEval::eval`] uses the current context to set the `__FILE__` magic
/// constant on the [`sys::mrbc_context`].
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalContext {
    /// Value of the `__FILE__` magic constant that also appears in stack
    /// frames.
    pub filename: String,
}

impl EvalContext {
    /// Create a new [`EvalContext`].
    pub fn new<T>(filename: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            filename: filename.as_ref().to_owned(),
        }
    }

    /// Create a root, or default, [`EvalContext`]. The root context sets the
    /// `__FILE__` magic constant to "(eval)".
    pub fn root() -> Self {
        Self::default()
    }
}

impl Default for EvalContext {
    fn default() -> Self {
        Self {
            filename: TOP_FILENAME.to_owned(),
        }
    }
}

/// Interpreters that implement [`MrbEval`] expose methods for injecting code
/// into a [`sys::mrb_state`] and extracting [`Value`]s from the interpereter.
///
/// Implementations are expected to maintain a stack of [`EvalContext`] objects
/// that maintain filename context across nested invocations of
/// [`MrbEval::eval`].
#[allow(clippy::module_name_repetitions)]
pub trait MrbEval {
    /// Eval code on the mruby interpreter using the current [`EvalContext`] or
    /// [`EvalContext::root`] if none is present on the stack.
    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>;

    /// Eval code on the mruby interpreter using a custom [`EvalContext`].
    ///
    /// `EvalContext` allows manipulating interpreter state before eval, for
    /// example, setting the `__FILE__` magic constant.
    fn eval_with_context<T>(&self, code: T, context: EvalContext) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>;

    /// Push an [`EvalContext`] onto the stack.
    fn push_context(&self, context: EvalContext);

    /// Pop an [`EvalContext`] from the stack.
    fn pop_context(&self);
}

impl MrbEval for Mrb {
    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>,
    {
        // Grab the persistent `EvalContext` from the context on the `State` or
        // the root context if the stack is empty.
        let context = {
            let api = self.borrow();
            if let Some(context) = api.context_stack.last() {
                context.clone()
            } else {
                EvalContext::root()
            }
        };

        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Mrb` to
        // get access to the underlying `MrbState`.
        let (mrb, ctx) = {
            let borrow = self.borrow();
            (borrow.mrb, borrow.ctx)
        };

        if let Ok(cfilename) = CString::new(context.filename.to_owned()) {
            unsafe {
                sys::mrbc_filename(mrb, ctx, cfilename.as_ptr() as *const i8);
            }
        } else {
            warn!("Could not set {} as mrc context filename", context.filename);
        }

        let code = code.as_ref();
        debug!("Evaling code on {}", mrb.debug());
        let result = unsafe {
            // Execute arbitrary ruby code, which may generate objects with C
            // APIs if backed by Rust functions.
            //
            // `mrb_load_nstring_ctx` sets the "stack keep" field on the context
            // which means the most recent value returned by eval will always be
            // considered live by the GC.
            sys::mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), ctx)
        };

        if let Some(backtrace) = self.current_exception() {
            warn!("runtime error with exception backtrace: {}", backtrace);
            return Err(MrbError::Exec(backtrace));
        }

        let value = Value::new(Rc::clone(self), result);
        // Unreachable values are internal to the mruby interpreter and
        // interacting with them via the C API is unspecified and may result in
        // a segfault.
        //
        // See: https://github.com/mruby/mruby/issues/4460
        if value.is_unreachable() {
            Err(MrbError::UnreachableValue(value.inner().tt))
        } else {
            Ok(value)
        }
    }

    fn eval_with_context<T>(&self, code: T, context: EvalContext) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>,
    {
        self.push_context(context);
        let result = self.eval(code.as_ref());
        self.pop_context();
        result
    }

    fn push_context(&self, context: EvalContext) {
        let mut api = self.borrow_mut();
        api.context_stack.push(context);
    }

    fn pop_context(&self) {
        let mut api = self.borrow_mut();
        api.context_stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::TryFromMrb;
    use crate::def::{ClassLike, Define};
    use crate::eval::{EvalContext, MrbEval};
    use crate::file::MrbFile;
    use crate::interpreter::{Interpreter, Mrb};
    use crate::load::MrbLoadSources;
    use crate::sys;
    use crate::{interpreter_or_raise, unwrap_value_or_raise, MrbError};

    #[test]
    fn root_eval_context() {
        let interp = Interpreter::create().expect("mrb init");
        let result = interp.eval("__FILE__").expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "(eval)");
    }

    #[test]
    fn context_is_restored_after_eval() {
        let interp = Interpreter::create().expect("mrb init");
        let context = EvalContext::new("context.rb");
        interp.push_context(context);
        interp.eval("15").expect("eval");
        assert_eq!(interp.borrow().context_stack.len(), 1);
    }

    #[test]
    fn root_context_is_not_pushed_after_eval() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("15").expect("eval");
        assert_eq!(interp.borrow().context_stack.len(), 0);
    }

    #[test]
    #[should_panic]
    // this test is known broken
    fn eval_context_is_a_stack_for_nested_eval() {
        extern "C" fn nested_eval(
            mrb: *mut sys::mrb_state,
            _slf: sys::mrb_value,
        ) -> sys::mrb_value {
            let interp = unsafe { interpreter_or_raise!(mrb) };
            unsafe { unwrap_value_or_raise!(interp, interp.eval("__FILE__")) }
        }
        struct NestedEval;
        impl MrbFile for NestedEval {
            fn require(interp: Mrb) -> Result<(), MrbError> {
                let spec = {
                    let mut api = interp.borrow_mut();
                    let spec = api.def_module::<Self>("NestedEval", None);
                    spec.borrow_mut()
                        .add_self_method("file", nested_eval, sys::mrb_args_none());
                    spec
                };
                spec.borrow().define(&interp)?;
                Ok(())
            }
        }
        let interp = Interpreter::create().expect("mrb init");
        interp
            .def_file_for_type::<_, NestedEval>("nested_eval.rb")
            .expect("def file");
        let code = r#"
require 'nested_eval'
NestedEval.file
        "#;
        let result = interp.eval(code).expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "/src/lib/nested_eval.rb");
    }

    #[test]
    fn eval_with_context() {
        let interp = Interpreter::create().expect("mrb init");
        let result = interp
            .eval_with_context("__FILE__", EvalContext::new("source.rb"))
            .expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "source.rb");
        let result = interp
            .eval_with_context("__FILE__", EvalContext::new("source.rb"))
            .expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "source.rb");
        let result = interp
            .eval_with_context("__FILE__", EvalContext::new("main.rb"))
            .expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "main.rb");
    }

    #[test]
    fn unparseable_code_returns_err_undef() {
        let interp = Interpreter::create().expect("mrb init");
        let result = interp.eval("'a").map(|_| ());
        assert_eq!(
            result,
            Err(MrbError::UnreachableValue(sys::mrb_vtype::MRB_TT_UNDEF))
        );
    }

    #[test]
    fn interpreter_is_usable_after_returning_undef() {
        let interp = Interpreter::create().expect("mrb init");
        let result = interp.eval("'a").map(|_| ());
        assert_eq!(
            result,
            Err(MrbError::UnreachableValue(sys::mrb_vtype::MRB_TT_UNDEF))
        );
        // Ensure interpreter is usable after evaling unparseable code
        let result = interp.eval("'a' * 10 ").expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(result, "a".repeat(10));
    }

    #[test]
    fn file_magic_constant() {
        let interp = Interpreter::create().expect("mrb init");
        interp
            .def_rb_source_file("source.rb", "def file; __FILE__; end")
            .expect("def file");
        let result = interp.eval("require 'source'; file").expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "/src/lib/source.rb");
    }

    #[test]
    fn file_not_persistent() {
        let interp = Interpreter::create().expect("mrb init");
        interp
            .def_rb_source_file("source.rb", "def file; __FILE__; end")
            .expect("def file");
        let result = interp.eval("require 'source'; __FILE__").expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "(eval)");
    }
}
