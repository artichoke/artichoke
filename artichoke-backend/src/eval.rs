use artichoke_core::eval::{self, Eval};
use std::borrow::Cow;
use std::ffi::{c_void, CStr, CString};
use std::mem;

use crate::exception::Exception;
use crate::exception_handler::ExceptionHandler;
use crate::extn::core::exception::Fatal;
use crate::sys::{self, DescribeState};
use crate::value::Value;
use crate::Artichoke;

// `Protect` must be `Copy` because the call to `mrb_load_nstring_cxt` can
// unwind with `longjmp` which does not allow Rust to run destructors.
#[derive(Clone, Copy)]
struct Protect<'a> {
    ctx: *mut sys::mrbc_context,
    code: &'a [u8],
}

impl<'a> Protect<'a> {
    fn new(interp: &Artichoke, code: &'a [u8]) -> Self {
        Self {
            ctx: interp.state().ctx,
            code,
        }
    }

    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        // `Protect` must be `Copy` because the call to `mrb_load_nstring_cxt`
        // can unwind with `longjmp` which does not allow Rust to run
        // destructors.
        let protect = Box::from_raw(ptr as *mut Self);

        // Pull all of the args out of the `Box` so we can free the
        // heap-allocated `Box`.
        let ctx = protect.ctx;
        let code = protect.code;

        // Drop the `Box` to ensure it is freed.
        drop(protect);

        // Execute arbitrary ruby code, which may generate objects with C APIs
        // if backed by Rust functions.
        //
        // `mrb_load_nstring_ctx` sets the "stack keep" field on the context
        // which means the most recent value returned by eval will always be
        // considered live by the GC.
        sys::mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), ctx)
    }
}

/// `Context` is used to manipulate the state of a wrapped
/// [`sys::mrb_state`]. [`Artichoke`] maintains a stack of `Context`s and
/// [`Eval::eval`] uses the current context to set the `__FILE__` magic
/// constant on the [`sys::mrbc_context`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Context {
    /// Value of the `__FILE__` magic constant that also appears in stack
    /// frames.
    filename: Cow<'static, [u8]>,
    /// FFI variant of `filename` field.
    filename_cstring: CString,
}

impl Context {
    /// Create a new [`Context`].
    pub fn new<T>(filename: T) -> Option<Self>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let filename = filename.into();
        let cstring = CString::new(filename.as_ref()).ok()?;
        Some(Self {
            filename,
            filename_cstring: cstring,
        })
    }

    /// Create a new [`Context`] without checking for NUL bytes in the filename.
    ///
    /// # Safety
    ///
    /// `filename` must not contain any NUL bytes. `filename` must not contain a
    /// trailing `NUL`.
    pub unsafe fn new_unchecked<T>(filename: T) -> Self
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let filename = filename.into();
        let cstring = CString::from_vec_unchecked(filename.clone().into_owned());
        Self {
            filename,
            filename_cstring: cstring,
        }
    }

    /// Create a root, or default, [`Context`]. The root context sets the
    /// `__FILE__` magic constant to "(eval)".
    pub fn root() -> Self {
        Self::default()
    }

    /// Filename of this `Context`.
    #[must_use]
    pub fn filename(&self) -> &[u8] {
        self.filename.as_ref()
    }

    /// FFI-safe NUL-terminated C String of this `Context`.
    ///
    /// This [`CStr`] is valid as long as this `Context` is not dropped.
    #[must_use]
    pub fn filename_as_c_str(&self) -> &CStr {
        self.filename_cstring.as_c_str()
    }
}

impl Default for Context {
    fn default() -> Self {
        // safety: the `TOP_FILENAME` is controlled by this crate and does
        // not contain NUL bytes, enforced with a test.
        unsafe { Self::new_unchecked(Artichoke::TOP_FILENAME) }
    }
}

#[cfg(test)]
mod context_test {
    use artichoke_core::eval::Eval;

    use crate::Artichoke;

    #[test]
    fn top_filename_does_not_contain_nul_byte() {
        assert_eq!(
            None,
            Artichoke::TOP_FILENAME
                .iter()
                .copied()
                .position(|b| b == b'\0')
        );
    }
}

impl eval::Context for Context {}

impl Eval for Artichoke {
    type Context = Context;

    type Value = Value;

    type Error = Exception;

    fn eval(&mut self, code: &[u8]) -> Result<Self::Value, Self::Error> {
        let mrb = self.mrb_mut();
        let ctx = self.state().ctx;

        // Grab the persistent `Context` from the context on the `State` or
        // the root context if the stack is empty.
        let filename = self
            .state()
            .context_stack
            .last()
            .map(Context::filename_as_c_str)
            .map_or_else(
                || Context::default().filename_as_c_str().to_owned(),
                CStr::to_owned,
            );

        unsafe {
            sys::mrbc_filename(mrb, ctx, filename.as_ptr() as *const i8);
        }

        let protect = Protect::new(self, code);
        trace!("Evaling code on {}", mrb.debug());
        let value = unsafe {
            let data =
                sys::mrb_sys_cptr_value(mrb, Box::into_raw(Box::new(protect)) as *mut c_void);
            let mut state = mem::MaybeUninit::<sys::mrb_bool>::uninit();

            let value = sys::mrb_protect(mrb, Some(Protect::run), data, state.as_mut_ptr());
            if state.assume_init() != 0 {
                (*mrb).exc = sys::mrb_sys_obj_ptr(value);
            }
            value
        };

        if let Some(exc) = self.last_error()? {
            Err(exc)
        } else {
            let value = Value::new(self, value);
            if value.is_unreachable() {
                // Unreachable values are internal to the mruby interpreter and
                // interacting with them via the C API is unspecified and may
                // result in a segfault.
                //
                // See: https://github.com/mruby/mruby/issues/4460
                Err(Exception::from(Fatal::new(self, "Unreachable Ruby value")))
            } else {
                Ok(value)
            }
        }
    }

    #[must_use]
    fn peek_context(&self) -> Option<&Self::Context> {
        self.state().context_stack.last()
    }

    fn push_context(&mut self, context: Self::Context) {
        self.state_mut().context_stack.push(context);
    }

    fn pop_context(&mut self) {
        self.state_mut().context_stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn root_eval_context() {
        let interp = crate::interpreter().expect("init");
        let result = interp.eval(b"__FILE__").expect("eval");
        let result = result.try_into::<&str>().expect("convert");
        assert_eq!(result, "(eval)");
    }

    #[test]
    fn context_is_restored_after_eval() {
        let interp = crate::interpreter().expect("init");
        let context = Context::new(b"context.rb".as_ref()).unwrap();
        interp.push_context(context);
        let _ = interp.eval(b"15").expect("eval");
        assert_eq!(interp.0.borrow().context_stack.len(), 1);
    }

    #[test]
    fn root_context_is_not_pushed_after_eval() {
        let interp = crate::interpreter().expect("init");
        let _ = interp.eval(b"15").expect("eval");
        assert_eq!(interp.0.borrow().context_stack.len(), 0);
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
                if let Ok(value) = interp.eval(b"__FILE__") {
                    value.inner()
                } else {
                    interp.convert(None::<Value>).inner()
                }
            }
        }

        impl File for NestedEval {
            type Artichoke = Artichoke;

            fn require(interp: &Artichoke) -> Result<(), ArtichokeError> {
                let spec = module::Spec::new("NestedEval", None)?;
                module::Builder::for_spec(interp, &spec)
                    .add_self_method("file", Self::nested_eval, sys::mrb_args_none())?
                    .define()?;
                interp.0.borrow_mut().def_module::<Self>(spec);
                Ok(())
            }
        }
        let interp = crate::interpreter().expect("init");
        interp
            .def_file_for_type::<NestedEval>(b"nested_eval.rb")
            .expect("def file");
        let code = br#"
require 'nested_eval'
NestedEval.file
        "#;
        let result = interp.eval(code).expect("eval");
        let result = result.try_into::<&str>().expect("convert");
        assert_eq!(result, "/src/lib/nested_eval.rb");
    }

    #[test]
    fn eval_with_context() {
        let interp = crate::interpreter().expect("init");

        interp.push_context(Context::new(b"source.rb".as_ref()).unwrap());
        let result = interp.eval(b"__FILE__").expect("eval");
        let result = result.try_into::<&str>().expect("convert");
        assert_eq!(result, "source.rb");
        interp.pop_context();

        interp.push_context(Context::new(b"source.rb".as_ref()).unwrap());
        let result = interp.eval(b"__FILE__").expect("eval");
        let result = result.try_into::<&str>().expect("convert");
        assert_eq!(result, "source.rb");
        interp.pop_context();

        interp.push_context(Context::new(b"main.rb".as_ref()).unwrap());
        let result = interp.eval(b"__FILE__").expect("eval");
        let result = result.try_into::<&str>().expect("convert");
        assert_eq!(result, "main.rb");
        interp.pop_context();
    }

    #[test]
    fn unparseable_code_returns_err_syntax_error() {
        let interp = crate::interpreter().expect("init");
        let err = interp.eval(b"'a").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_str());
    }

    #[test]
    fn interpreter_is_usable_after_syntax_error() {
        let interp = crate::interpreter().expect("init");
        let err = interp.eval(b"'a").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_str());
        // Ensure interpreter is usable after evaling unparseable code
        let result = interp.eval(b"'a' * 10 ").expect("eval");
        let result = result.try_into::<&str>().expect("convert");
        assert_eq!(result, "a".repeat(10));
    }

    #[test]
    fn file_magic_constant() {
        let interp = crate::interpreter().expect("init");
        interp
            .def_rb_source_file(b"source.rb", &b"def file; __FILE__; end"[..])
            .expect("def file");
        let result = interp.eval(b"require 'source'; file").expect("eval");
        let result = result.try_into::<&str>().expect("convert");
        assert_eq!(result, "/src/lib/source.rb");
    }

    #[test]
    fn file_not_persistent() {
        let interp = crate::interpreter().expect("init");
        interp
            .def_rb_source_file(b"source.rb", &b"def file; __FILE__; end"[..])
            .expect("def file");
        let result = interp.eval(b"require 'source'; __FILE__").expect("eval");
        let result = result.try_into::<&str>().expect("convert");
        assert_eq!(result, "(eval)");
    }

    #[test]
    fn return_syntax_error() {
        let interp = crate::interpreter().expect("init");
        interp
            .def_rb_source_file(b"fail.rb", &b"def bad; 'as'.scan(; end"[..])
            .expect("def file");
        let err = interp.eval(b"require 'fail'").unwrap_err();
        assert_eq!("SyntaxError", err.name().as_str());
    }
}
