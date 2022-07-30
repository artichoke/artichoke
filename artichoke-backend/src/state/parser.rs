use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::core::IncrementLinenoError;
use crate::sys;

/// Filename of the top eval context.
pub const TOP_FILENAME: &[u8] = b"(eval)";

#[derive(Debug)]
pub struct State {
    context: NonNull<sys::mrbc_context>,
    stack: Vec<Context>,
}

impl State {
    pub fn new(mrb: &mut sys::mrb_state) -> Option<Self> {
        let context = unsafe { sys::mrbc_context_new(mrb) };
        let mut context = NonNull::new(context)?;
        reset_context_filename(mrb, unsafe { context.as_mut() });
        Some(Self { context, stack: vec![] })
    }

    pub fn close(mut self, mrb: &mut sys::mrb_state) {
        unsafe {
            let ctx = self.context.as_mut();
            sys::mrbc_context_free(mrb, ctx);
        }
    }

    pub fn context_mut(&mut self) -> &mut sys::mrbc_context {
        unsafe { self.context.as_mut() }
    }

    /// Reset line number to `1`.
    pub fn reset(&mut self, mrb: &mut sys::mrb_state) {
        unsafe {
            let ctx = self.context.as_mut();
            ctx.lineno = 1;
            reset_context_filename(mrb, ctx);
        }
        self.stack.clear();
    }

    /// Fetch the current line number from the parser state.
    #[must_use]
    pub fn fetch_lineno(&self) -> usize {
        let ctx = unsafe { self.context.as_ref() };
        usize::from(ctx.lineno)
    }

    /// Increment line number and return the new value.
    ///
    /// # Errors
    ///
    /// This function returns [`IncrementLinenoError`] if the increment results
    /// in an overflow of the internal parser line number counter.
    pub fn add_fetch_lineno(&mut self, val: usize) -> Result<usize, IncrementLinenoError> {
        let old = usize::from(unsafe { self.context.as_ref() }.lineno);
        let new = old
            .checked_add(val)
            .ok_or_else(|| IncrementLinenoError::Overflow(usize::from(u16::MAX)))?;
        let store = u16::try_from(new).map_err(|_| IncrementLinenoError::Overflow(usize::from(u16::MAX)))?;
        unsafe {
            self.context.as_mut().lineno = store;
        }
        Ok(new)
    }

    /// Push a [`Context`] onto the stack.
    ///
    /// The supplied [`Context`] becomes the currently active context. This
    /// function modifies the parser state so subsequently `eval`ed code will
    /// use the current active `Context`.
    pub fn push_context(&mut self, mrb: &mut sys::mrb_state, context: Context) {
        let filename = context.filename_as_c_str();
        unsafe {
            let ctx = self.context.as_mut();
            sys::mrbc_filename(mrb, ctx, filename.as_ptr());
        }
        self.stack.push(context);
    }

    /// Removes the last element from the context stack and returns it, or
    /// `None` if the stack is empty.
    ///
    /// Calls to this function modify the parser state so subsequently `eval`ed
    /// code will use the current active [`Context`].
    pub fn pop_context(&mut self, mrb: &mut sys::mrb_state) -> Option<Context> {
        let context = self.stack.pop();
        if let Some(current) = self.stack.last() {
            let filename = current.filename_as_c_str();
            unsafe {
                let ctx = self.context.as_mut();
                sys::mrbc_filename(mrb, ctx, filename.as_ptr());
            }
        } else {
            unsafe {
                let ctx = self.context.as_mut();
                reset_context_filename(mrb, ctx);
            }
        }
        context
    }

    /// Returns the last [`Context`], or `None` if the context stack is empty.
    #[must_use]
    pub fn peek_context(&self) -> Option<&Context> {
        self.stack.last()
    }
}

fn reset_context_filename(mrb: &mut sys::mrb_state, context: &mut sys::mrbc_context) {
    let frame = Context::root();
    let filename = frame.filename_as_c_str();
    unsafe {
        sys::mrbc_filename(mrb, context, filename.as_ptr());
    }
}

/// `Context` is used to manipulate the current filename on the parser.
///
/// Parser [`State`] maintains a stack of `Context`s and [`eval`] uses the
/// `Context` stack to set the `__FILE__` magic constant.
///
/// [`eval`]: crate::core::Eval
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Context {
    /// Value of the `__FILE__` magic constant that also appears in stack
    /// frames.
    filename: Cow<'static, [u8]>,
    /// FFI NUL-terminated C string variant of `filename` field.
    filename_cstr: Box<CStr>,
}

impl Default for Context {
    fn default() -> Self {
        // SAFETY: `TOP_FILENAME` has no NUL bytes (asserted by tests).
        unsafe { Self::new_unchecked(TOP_FILENAME) }
    }
}

impl Context {
    /// Create a new [`Context`].
    pub fn new<T>(filename: T) -> Option<Self>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let filename = filename.into();
        let cstring = CString::new(filename.clone()).ok()?;
        Some(Self {
            filename,
            filename_cstr: cstring.into_boxed_c_str(),
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
            filename_cstr: cstring.into_boxed_c_str(),
        }
    }

    /// Create a root, or default, [`Context`].
    ///
    /// The root context sets the `__FILE__` magic constant to "(eval)".
    #[must_use]
    pub fn root() -> Self {
        Self::default()
    }

    /// Filename of this `Context`.
    #[must_use]
    pub fn filename(&self) -> &[u8] {
        &*self.filename
    }

    /// FFI-safe NUL-terminated C String of this `Context`.
    ///
    /// This [`CStr`] is valid as long as this `Context` is not dropped.
    #[must_use]
    pub fn filename_as_c_str(&self) -> &CStr {
        &*self.filename_cstr
    }
}

#[cfg(test)]
mod test {
    use super::Context;

    #[test]
    fn top_filename_does_not_contain_nul_byte() {
        let contains_nul_byte = super::TOP_FILENAME.iter().copied().any(|b| b == b'\0');
        assert!(!contains_nul_byte);
    }

    #[test]
    fn top_filename_context_new_unchecked_safety() {
        Context::new(super::TOP_FILENAME).unwrap();
    }
}
