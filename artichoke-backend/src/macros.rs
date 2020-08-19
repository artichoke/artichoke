// This module defines macros for working with interpreters and `Value`s. This
// source module is included first in `lib.rs`, which means the macros are
// available to all modules within the artichoke-backend crate in addition to
// being exported.

/// Extract an [`Artichoke`] instance from the userdata on a [`sys::mrb_state`].
///
/// If there is an error when extracting the Rust wrapper around the
/// interpreter, return `nil` or a user-provided default.
///
/// This macro calls `unsafe` functions.
///
/// [`Artichoke`]: crate::Artichoke
/// [`sys::mrb_state`]: crate::sys::mrb_state
#[macro_export]
macro_rules! unwrap_interpreter {
    ($mrb:expr, or_else = ()) => {
        if let Ok(interp) = $crate::ffi::from_user_data($mrb) {
            interp
        } else {
            return;
        }
    };
    ($mrb:expr, or_else = $default:expr) => {
        if let Ok(interp) = $crate::ffi::from_user_data($mrb) {
            interp
        } else {
            return $default;
        }
    };
    ($mrb:expr) => {
        unwrap_interpreter!($mrb, or_else = $crate::sys::mrb_sys_nil_value())
    };
}

#[doc(hidden)]
pub mod argspec {
    pub const NONE: &[u8] = b"\0";
    pub const REQ1: &[u8] = b"o\0";
    pub const OPT1: &[u8] = b"|o\0";
    pub const REQ1_OPT1: &[u8] = b"o|o\0";
    pub const REQ1_OPT2: &[u8] = b"o|oo\0";
    pub const REQ1_REQBLOCK: &[u8] = b"o&\0";
    pub const REQ1_REQBLOCK_OPT1: &[u8] = b"o&|o?\0";
    pub const REQ2: &[u8] = b"oo\0";
    pub const OPT2: &[u8] = b"|oo\0";
    pub const OPT2_OPTBLOCK: &[u8] = b"&|o?o?\0";
    pub const REQ2_OPT1: &[u8] = b"oo|o\0";
    pub const REST: &[u8] = b"*\0";
}

/// Extract [`sys::mrb_value`]s from a [`sys::mrb_state`] to adapt a C
/// entrypoint to a Rust implementation of a Ruby function.
///
/// This macro exists because the mruby VM [does not validate argspecs] attached
/// to native functions.
///
/// This macro calls `unsafe` functions.
///
/// [`sys::mrb_value`]: crate::sys::mrb_value
/// [`sys::mrb_state`]: crate::sys::mrb_state
/// [does not validate argspecs]: https://github.com/mruby/mruby/issues/4688
#[macro_export]
macro_rules! mrb_get_args {
    ($mrb:expr, none) => {{
        $crate::sys::mrb_get_args($mrb, $crate::macros::argspec::NONE.as_ptr() as *const i8);
        ()
    }};
    ($mrb:expr, required = 1) => {{
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ1.as_ptr() as *const i8,
            req1.as_mut_ptr(),
        );
        match argc {
            1 => req1.assume_init(),
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, optional = 1) => {{
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::OPT1.as_ptr() as *const i8,
            opt1.as_mut_ptr(),
        );
        match argc {
            1 => {
                let opt1 = opt1.assume_init();
                Some(opt1)
            }
            0 => None,
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, required = 1, optional = 1) => {{
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
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
                (req1, None)
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, required = 1, optional = 2) => {{
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut opt2 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
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
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut block = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ1_REQBLOCK.as_ptr() as *const i8,
            req1.as_mut_ptr(),
            block.as_mut_ptr(),
        );
        match argc {
            2 | 1 => {
                let req1 = req1.assume_init();
                let block = block.assume_init();
                (req1, $crate::block::Block::new(block))
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, required = 1, optional = 1, &block) => {{
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut has_opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_bool>::uninit();
        let mut block = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ1_REQBLOCK_OPT1.as_ptr() as *const i8,
            req1.as_mut_ptr(),
            block.as_mut_ptr(),
            opt1.as_mut_ptr(),
            has_opt1.as_mut_ptr(),
        );
        let has_opt1 = has_opt1.assume_init() != 0;
        match argc {
            3 => {
                let req1 = req1.assume_init();
                let opt1 = opt1.assume_init();
                let block = block.assume_init();
                (req1, Some(opt1), $crate::block::Block::new(block))
            }
            2 => {
                let req1 = req1.assume_init();
                let opt1 = if has_opt1 {
                    Some(opt1.assume_init())
                } else {
                    None
                };
                let block = block.assume_init();
                (req1, opt1, $crate::block::Block::new(block))
            }
            1 => {
                let req1 = req1.assume_init();
                let block = block.assume_init();
                (req1, None, $crate::block::Block::new(block))
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, required = 2) => {{
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut req2 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ2.as_ptr() as *const i8,
            req1.as_mut_ptr(),
            req2.as_mut_ptr(),
        );
        match argc {
            2 => {
                let req1 = req1.assume_init();
                let req2 = req2.assume_init();
                (req1, req2)
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, optional = 2) => {{
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut opt2 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::OPT2.as_ptr() as *const i8,
            opt1.as_mut_ptr(),
            opt2.as_mut_ptr(),
        );
        match argc {
            2 => {
                let opt1 = opt1.assume_init();
                let opt2 = opt2.assume_init();
                (Some(opt1), Some(opt2))
            }
            1 => {
                let opt1 = opt1.assume_init();
                (Some(opt1), None)
            }
            0 => (None, None),
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, optional = 2, &block) => {{
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut has_opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_bool>::uninit();
        let mut opt2 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut has_opt2 = std::mem::MaybeUninit::<$crate::sys::mrb_bool>::uninit();
        let mut block = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::OPT2_OPTBLOCK.as_ptr() as *const i8,
            block.as_mut_ptr(),
            opt1.as_mut_ptr(),
            has_opt1.as_mut_ptr(),
            opt2.as_mut_ptr(),
            has_opt2.as_mut_ptr(),
        );
        let has_opt1 = has_opt1.assume_init() != 0;
        let has_opt2 = has_opt2.assume_init() != 0;
        let opt1 = if has_opt1 {
            Some(opt1.assume_init())
        } else {
            None
        };
        let opt2 = if has_opt2 {
            Some(opt2.assume_init())
        } else {
            None
        };
        let block = block.assume_init();
        (opt1, opt2, $crate::block::Block::new(block))
    }};
    ($mrb:expr, required = 2, optional = 1) => {{
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut req2 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ2_OPT1.as_ptr() as *const i8,
            req1.as_mut_ptr(),
            req2.as_mut_ptr(),
            opt1.as_mut_ptr(),
        );
        match argc {
            3 => {
                let req1 = req1.assume_init();
                let req2 = req2.assume_init();
                let opt1 = opt1.assume_init();
                (req1, req2, Some(opt1))
            }
            2 => {
                let req1 = req1.assume_init();
                let req2 = req2.assume_init();
                (req1, req2, None)
            }
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, *args) => {{
        let mut args = std::mem::MaybeUninit::<*const $crate::sys::mrb_value>::uninit();
        let mut count = std::mem::MaybeUninit::<usize>::uninit();
        let _argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REST.as_ptr() as *const i8,
            args.as_mut_ptr(),
            count.as_mut_ptr(),
        );
        std::slice::from_raw_parts(args.assume_init(), count.assume_init())
    }};
}
