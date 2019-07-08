use log::{error, trace, warn};
use std::ffi::{c_void, CString};
use std::mem;
use std::rc::Rc;

use crate::exception::{LastError, MrbExceptionHandler};
use crate::sys::{self, DescribeState};
use crate::value::Value;
use crate::{Mrb, MrbError};

const TOP_FILENAME: &str = "(eval)";

struct Protect {
    ctx: *mut sys::mrbc_context,
    code: Vec<u8>,
}

impl Protect {
    fn new(interp: &Mrb, code: &[u8]) -> Self {
        Self {
            ctx: interp.borrow().ctx,
            code: code.to_vec(),
        }
    }

    unsafe extern "C" fn run_protected(
        mrb: *mut sys::mrb_state,
        data: sys::mrb_value,
    ) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        let args = Rc::from_raw(ptr as *const Self);
        let ctx = args.ctx;
        let code = args.code.as_ptr();
        let len = args.code.len();
        // Drop the `Rc` before calling `mrb_load_nstring_ctx` which can
        // potentially unwind the stack with `longjmp`. To make sure eval and
        // unchecked_eval can free the code buffer, the `Rc::strong_count` must
        // be decreased. This is safe to do and `code` pointer will not be
        // dangling because strong count is always 2 right now.
        drop(args);

        // Execute arbitrary ruby code, which may generate objects with C APIs
        // if backed by Rust functions.
        //
        // `mrb_load_nstring_ctx` sets the "stack keep" field on the context
        // which means the most recent value returned by eval will always be
        // considered live by the GC.
        sys::mrb_load_nstring_cxt(mrb, code as *const i8, len, ctx)
    }
}

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

    /// Eval code on the mruby interpreter using the current [`EvalContext`] or
    /// [`EvalContext::root`] if none is present on the stack.
    ///
    /// This function does not wrap calls to the mruby VM in `mrb_protect` which
    /// means exceptions will unwind past this call.
    fn unchecked_eval<T>(&self, code: T) -> Value
    where
        T: AsRef<[u8]>;

    /// Eval code on the mruby interpreter using a custom [`EvalContext`].
    ///
    /// `EvalContext` allows manipulating interpreter state before eval, for
    /// example, setting the `__FILE__` magic constant.
    fn eval_with_context<T>(&self, code: T, context: EvalContext) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>;

    /// Eval code on the mruby interpreter using a custom [`EvalContext`].
    ///
    /// `EvalContext` allows manipulating interpreter state before eval, for
    /// example, setting the `__FILE__` magic constant.
    ///
    /// This function does not wrap calls to the mruby VM in `mrb_protect` which
    /// means exceptions will unwind past this call.
    fn unchecked_eval_with_context<T>(&self, code: T, context: EvalContext) -> Value
    where
        T: AsRef<[u8]>;

    /// Peek at the top of the [`EvalContext`] stack.
    fn peek_context(&self) -> Option<EvalContext>;

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
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Mrb` to
        // get access to the underlying `MrbState`.
        let (mrb, ctx) = {
            let borrow = self.borrow();
            (borrow.mrb, borrow.ctx)
        };

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

        if let Ok(cfilename) = CString::new(context.filename.to_owned()) {
            unsafe {
                sys::mrbc_filename(mrb, ctx, cfilename.as_ptr() as *const i8);
            }
        } else {
            warn!("Could not set {} as mrc context filename", context.filename);
        }
        drop(context);

        let args = Rc::new(Protect::new(self, code.as_ref()));
        drop(code);
        trace!("Evaling code on {}", mrb.debug());
        let value = unsafe {
            let data = sys::mrb_sys_cptr_value(mrb, Rc::into_raw(Rc::clone(&args)) as *mut c_void);
            let mut state = <mem::MaybeUninit<sys::mrb_bool>>::uninit();

            let value =
                sys::mrb_protect(mrb, Some(Protect::run_protected), data, state.as_mut_ptr());
            drop(args);
            if state.assume_init() != 0 {
                (*mrb).exc = sys::mrb_sys_obj_ptr(value);
            }
            value
        };
        let value = Value::new(self, value);

        match self.last_error() {
            LastError::Some(exception) => {
                warn!("runtime error with exception backtrace: {}", exception);
                Err(MrbError::Exec(exception.to_string()))
            }
            LastError::UnableToExtract(err) => {
                error!("failed to extract exception after runtime error: {}", err);
                Err(err)
            }
            LastError::None if value.is_unreachable() => {
                // Unreachable values are internal to the mruby interpreter and
                // interacting with them via the C API is unspecified and may
                // result in a segfault.
                //
                // See: https://github.com/mruby/mruby/issues/4460
                Err(MrbError::UnreachableValue(value.inner().tt))
            }
            LastError::None => Ok(value),
        }
    }

    fn unchecked_eval<T>(&self, code: T) -> Value
    where
        T: AsRef<[u8]>,
    {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Mrb` to
        // get access to the underlying `MrbState`.
        let (mrb, ctx) = {
            let borrow = self.borrow();
            (borrow.mrb, borrow.ctx)
        };

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

        if let Ok(cfilename) = CString::new(context.filename.to_owned()) {
            unsafe {
                sys::mrbc_filename(mrb, ctx, cfilename.as_ptr() as *const i8);
            }
        } else {
            warn!("Could not set {} as mrc context filename", context.filename);
        }
        drop(context);

        let args = Rc::new(Protect::new(self, code.as_ref()));
        drop(code);
        trace!("Evaling code on {}", mrb.debug());
        let value = unsafe {
            sys::mrb_load_nstring_cxt(
                mrb,
                args.code.as_ptr() as *const i8,
                args.code.len(),
                args.ctx,
            )
        };
        Value::new(self, value)
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

    fn unchecked_eval_with_context<T>(&self, code: T, context: EvalContext) -> Value
    where
        T: AsRef<[u8]>,
    {
        self.push_context(context);
        let result = self.unchecked_eval(code.as_ref());
        self.pop_context();
        result
    }

    fn peek_context(&self) -> Option<EvalContext> {
        let api = self.borrow();
        api.context_stack.last().cloned()
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
    use crate::convert::{FromMrb, TryFromMrb};
    use crate::def::{ClassLike, Define};
    use crate::eval::{EvalContext, MrbEval};
    use crate::file::MrbFile;
    use crate::load::MrbLoadSources;
    use crate::sys;
    use crate::value::Value;
    use crate::{Mrb, MrbError};

    #[test]
    fn root_eval_context() {
        let interp = crate::interpreter().expect("mrb init");
        let result = interp.eval("__FILE__").expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "(eval)");
    }

    #[test]
    fn context_is_restored_after_eval() {
        let interp = crate::interpreter().expect("mrb init");
        let context = EvalContext::new("context.rb");
        interp.push_context(context);
        interp.eval("15").expect("eval");
        assert_eq!(interp.borrow().context_stack.len(), 1);
    }

    #[test]
    fn root_context_is_not_pushed_after_eval() {
        let interp = crate::interpreter().expect("mrb init");
        interp.eval("15").expect("eval");
        assert_eq!(interp.borrow().context_stack.len(), 0);
    }

    #[test]
    #[should_panic]
    // this test is known broken
    fn eval_context_is_a_stack_for_nested_eval() {
        struct NestedEval;

        impl NestedEval {
            unsafe extern "C" fn nested_eval(
                mrb: *mut sys::mrb_state,
                _slf: sys::mrb_value,
            ) -> sys::mrb_value {
                let interp = unwrap_interpreter!(mrb);
                if let Ok(value) = interp.eval("__FILE__") {
                    value.inner()
                } else {
                    Value::from_mrb(&interp, None::<Value>).inner()
                }
            }
        }

        impl MrbFile for NestedEval {
            fn require(interp: Mrb) -> Result<(), MrbError> {
                let spec = {
                    let mut api = interp.borrow_mut();
                    let spec = api.def_module::<Self>("NestedEval", None);
                    spec.borrow_mut().add_self_method(
                        "file",
                        Self::nested_eval,
                        sys::mrb_args_none(),
                    );
                    spec
                };
                spec.borrow().define(&interp)?;
                Ok(())
            }
        }
        let interp = crate::interpreter().expect("mrb init");
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
        let interp = crate::interpreter().expect("mrb init");
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
    fn unparseable_code_returns_err_syntax_error() {
        let interp = crate::interpreter().expect("mrb init");
        let result = interp.eval("'a").map(|_| ());
        assert_eq!(
            result,
            Err(MrbError::Exec("SyntaxError: syntax error".to_owned()))
        );
    }

    #[test]
    fn interpreter_is_usable_after_syntax_error() {
        let interp = crate::interpreter().expect("mrb init");
        let result = interp.eval("'a").map(|_| ());
        assert_eq!(
            result,
            Err(MrbError::Exec("SyntaxError: syntax error".to_owned()))
        );
        // Ensure interpreter is usable after evaling unparseable code
        let result = interp.eval("'a' * 10 ").expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(result, "a".repeat(10));
    }

    #[test]
    fn file_magic_constant() {
        let interp = crate::interpreter().expect("mrb init");
        interp
            .def_rb_source_file("source.rb", "def file; __FILE__; end")
            .expect("def file");
        let result = interp.eval("require 'source'; file").expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "/src/lib/source.rb");
    }

    #[test]
    fn file_not_persistent() {
        let interp = crate::interpreter().expect("mrb init");
        interp
            .def_rb_source_file("source.rb", "def file; __FILE__; end")
            .expect("def file");
        let result = interp.eval("require 'source'; __FILE__").expect("eval");
        let result = unsafe { String::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(&result, "(eval)");
    }

    #[test]
    fn return_syntax_error() {
        let interp = crate::interpreter().expect("mrb init");
        interp
            .def_rb_source_file("fail.rb", "def bad; 'as'.scan(; end")
            .expect("def file");
        let result = interp.eval("require 'fail'").map(|_| ());
        let expected = MrbError::Exec("SyntaxError: syntax error".to_owned());
        assert_eq!(result, Err(expected));
    }
}
