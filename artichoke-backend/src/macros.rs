// This module defines macros for working with interpreters and `Value`s. This
// source module is included first in `lib.rs`, which means the macros are
// available to all modules within the artichoke-backend crate in addition to
// being exported.

/// Extract an [`Artichoke`](interpreter::Artichoke) instance from the userdata on a
/// [`sys::mrb_state`].
///
/// If there is an error when extracting the Rust wrapper around the
/// interpreter, return `nil`.
///
/// This macro is `unsafe`.
#[macro_export]
macro_rules! unwrap_interpreter {
    ($mrb:expr) => {
        if let Ok(interp) = $crate::ffi::from_user_data($mrb) {
            interp
        } else {
            return $crate::sys::mrb_sys_nil_value();
        }
    };
}

pub mod argspec {
    pub const NONE: &'static [u8] = b"\0";
    pub const REQ1: &'static [u8] = b"o\0";
    pub const REQ1_OPT1: &'static [u8] = b"o|o\0";
    pub const REQ1_OPT2: &'static [u8] = b"o|oo\0";
    pub const REQ1_REQBLOCK: &'static [u8] = b"o&\0";
    pub const REQ1_REQBLOCK_OPT1: &'static [u8] = b"o&|o\0";
    pub const REST: &'static [u8] = b"*\0";
}

/// Extract [`sys::mrb_value`]s from a [`sys::mrb_state`] to adapt a C
/// entrypoint to a Rust implementation of a Ruby function.
///
/// This macro exists because argspecs attached to function definitions in the
/// mruby VM are not validated: <https://github.com/mruby/mruby/issues/4688>.
///
/// This macro is `unsafe`.
#[macro_export]
macro_rules! mrb_get_args {
    ($mrb:expr, none) => {{
        $crate::sys::mrb_get_args($mrb, $crate::macros::argspec::NONE.as_ptr());
        ()
    }};
    ($mrb:expr, required = 1) => {{
        let mut req1 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ1.as_ptr() as *const i8,
            req1.as_mut_ptr(),
        );
        match argc {
            1 => {
                let req1 = req1.assume_init();
                req1
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, required = 1, optional = 1) => {{
        let mut req1 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let mut opt1 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ1_OPT1.as_ptr() as *const i8,
            req1.as_mut_ptr(),
            opt1.as_mut_ptr(),
        );
        match argc {
            2 => {
                let req1 = req1.assume_init();
                let opt1 = opt1.assume_init();
                (req1, Some(opt1))
            }
            1 => {
                let req1 = req1.assume_init();
                (req1, None, None)
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, required = 1, optional = 2) => {{
        let mut req1 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let mut opt1 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let mut opt2 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ1_OPT2.as_ptr() as *const i8,
            req1.as_mut_ptr(),
            opt1.as_mut_ptr(),
            opt2.as_mut_ptr(),
        );
        match argc {
            3 => {
                let req1 = req1.assume_init();
                let opt1 = opt1.assume_init();
                let opt2 = opt2.assume_init();
                (req1, Some(opt1), Some(opt2))
            }
            2 => {
                let req1 = req1.assume_init();
                let opt1 = opt1.assume_init();
                (req1, Some(opt1), None)
            }
            1 => {
                let req1 = req1.assume_init();
                (req1, None, None)
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, required = 1, &block) => {{
        let mut req1 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let mut block = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ1_REQBLOCK.as_ptr() as *const i8,
            req1.as_mut_ptr(),
            block.as_mut_ptr(),
        );
        match argc {
            2 => {
                let req1 = req1.assume_init();
                let block = block.assume_init();
                (req1, block)
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, required = 1, optional = 1, &block) => {{
        let mut req1 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let mut opt1 = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let mut block = <std::mem::MaybeUninit<$crate::sys::mrb_value>>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ1_REQBLOCK_OPT1.as_ptr() as *const i8,
            req1.as_mut_ptr(),
            block.as_mut_ptr(),
            opt1.as_mut_ptr(),
        );
        match argc {
            3 => {
                let req1 = req1.assume_init();
                let opt1 = opt1.assume_init();
                let block = block.assume_init();
                (req1, Some(opt1), block)
            }
            2 => {
                let req1 = req1.assume_init();
                let block = block.assume_init();
                (req1, None, block)
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, *args) => {{
        let mut args = <std::mem::MaybeUninit<*const $crate::sys::mrb_value>>::uninit();
        let mut count = <std::mem::MaybeUninit<usize>>::uninit();
        let _argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REST.as_ptr() as *const i8,
            args.as_mut_ptr(),
            count.as_mut_ptr(),
        );
        std::slice::from_raw_parts(args.assume_init(), count.assume_init())
    }};
}
