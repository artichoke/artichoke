// This module defines macros for working with interpreters and `Value`s. This
// source module is included first in `lib.rs`, which means the macros are
// available to all modules within the artichoke-backend crate in addition to
// being exported.

#[macro_export]
macro_rules! emit_fatal_warning {
    ($($arg:tt)+) => {{
        use ::std::io::Write;

        // Something bad, terrible, and unexpected has happened.
        //
        // Suppress errors from logging to stderr because this function may being
        // called when there are foreign C frames in the stack and panics are
        // either UB or will result in an abort.
        //
        // Ensure the returned error is dropped so we don't leave anything on
        // the stack in the event of a foreign unwind.
        let maybe_err = ::std::write!(::std::io::stderr(), $($arg)+);
        drop(maybe_err);
    }};
}

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
    ($mrb:expr, to => $to:ident, or_else = ()) => {
        let mut interp = if let Ok(interp) = $crate::ffi::from_user_data($mrb) {
            interp
        } else {
            return;
        };
        let mut arena = if let Ok(arena) =
            $crate::gc::MrbGarbageCollection::create_arena_savepoint(&mut interp)
        {
            arena
        } else {
            return;
        };
        #[allow(unused_mut)]
        let mut $to = $crate::Guard::new(arena.interp());
    };
    ($mrb:expr, to => $to:ident, or_else = $default:expr) => {
        let mut interp = if let Ok(interp) = $crate::ffi::from_user_data($mrb) {
            interp
        } else {
            return $default;
        };
        let mut arena = if let Ok(arena) =
            $crate::gc::MrbGarbageCollection::create_arena_savepoint(&mut interp)
        {
            arena
        } else {
            return $default;
        };
        #[allow(unused_mut)]
        let mut $to = $crate::Guard::new(arena.interp());
    };
    ($mrb:expr, to => $to:ident) => {
        unwrap_interpreter!($mrb, to => $to, or_else = $crate::sys::mrb_sys_nil_value())
    };
}

#[doc(hidden)]
pub mod argspec {
    use std::ffi::CStr;

    pub const NONE: &CStr = cstr::cstr!("");
    pub const REQ1: &CStr = cstr::cstr!("o");
    pub const OPT1: &CStr = cstr::cstr!("|o");
    pub const REQ1_OPT1: &CStr = cstr::cstr!("o|o");
    pub const REQ1_OPT2: &CStr = cstr::cstr!("o|oo");
    pub const REQ1_REQBLOCK: &CStr = cstr::cstr!("o&");
    pub const REQ1_REQBLOCK_OPT1: &CStr = cstr::cstr!("o&|o?");
    pub const REQ2: &CStr = cstr::cstr!("oo");
    pub const OPT2: &CStr = cstr::cstr!("|oo");
    pub const OPT2_OPTBLOCK: &CStr = cstr::cstr!("&|o?o?");
    pub const REQ2_OPT1: &CStr = cstr::cstr!("oo|o");
    pub const REST: &CStr = cstr::cstr!("*");
}

/// Extract [`sys::mrb_value`]s from a [`sys::mrb_state`] to adapt a C
/// entry point to a Rust implementation of a Ruby function.
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
        $crate::sys::mrb_get_args($mrb, $crate::macros::argspec::NONE.as_ptr());
    }};
    ($mrb:expr, required = 1) => {{
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args($mrb, $crate::macros::argspec::REQ1.as_ptr(), req1.as_mut_ptr());
        match argc {
            1 => req1.assume_init(),
            _ => unreachable!("mrb_get_args should have raised"),
        }
    }};
    ($mrb:expr, optional = 1) => {{
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args($mrb, $crate::macros::argspec::OPT1.as_ptr(), opt1.as_mut_ptr());
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
            $crate::macros::argspec::REQ1_OPT1.as_ptr(),
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
            $crate::macros::argspec::REQ1_OPT2.as_ptr(),
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
            $crate::macros::argspec::REQ1_REQBLOCK.as_ptr(),
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
            $crate::macros::argspec::REQ1_REQBLOCK_OPT1.as_ptr(),
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
                let opt1 = if has_opt1 { Some(opt1.assume_init()) } else { None };
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
            $crate::macros::argspec::REQ2.as_ptr(),
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
            $crate::macros::argspec::OPT2.as_ptr(),
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
            $crate::macros::argspec::OPT2_OPTBLOCK.as_ptr(),
            block.as_mut_ptr(),
            opt1.as_mut_ptr(),
            has_opt1.as_mut_ptr(),
            opt2.as_mut_ptr(),
            has_opt2.as_mut_ptr(),
        );
        let has_opt1 = has_opt1.assume_init() != 0;
        let has_opt2 = has_opt2.assume_init() != 0;
        let opt1 = if has_opt1 { Some(opt1.assume_init()) } else { None };
        let opt2 = if has_opt2 { Some(opt2.assume_init()) } else { None };
        let block = block.assume_init();
        (opt1, opt2, $crate::block::Block::new(block))
    }};
    ($mrb:expr, required = 2, optional = 1) => {{
        let mut req1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut req2 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let mut opt1 = std::mem::MaybeUninit::<$crate::sys::mrb_value>::uninit();
        let argc = $crate::sys::mrb_get_args(
            $mrb,
            $crate::macros::argspec::REQ2_OPT1.as_ptr(),
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
            $crate::macros::argspec::REST.as_ptr(),
            args.as_mut_ptr(),
            count.as_mut_ptr(),
        );
        std::slice::from_raw_parts(args.assume_init(), count.assume_init())
    }};
}
