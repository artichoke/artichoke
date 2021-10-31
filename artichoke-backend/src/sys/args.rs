//! Helpers for retrieving args from mruby function calls.

use super::mrb_aspec;

/// Function requires n arguments.
///
/// ```text
/// @param n
///     The number of required arguments.
/// ```
#[inline]
#[must_use]
pub const fn mrb_args_req(n: u32) -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_REQ(n)     ((mrb_aspec)((n)&0x1f) << 18)
    // ```
    (n & 0x1f) << 18
}

/// Function takes `n` optional arguments
///
/// ```text
/// @param n
///      The number of optional arguments.
/// ```
#[inline]
#[must_use]
pub const fn mrb_args_opt(n: u32) -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_OPT(n)     ((mrb_aspec)((n)&0x1f) << 13)
    // ```
    (n & 0x1f) << 13
}

/// Function takes `n1` mandatory arguments and `n2` optional arguments
///
/// ```text
/// @param n1
///      The number of required arguments.
/// @param n2
///      The number of optional arguments.
/// ```
#[inline]
#[must_use]
pub const fn mrb_args_req_and_opt(n_req: u32, n_opt: u32) -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_ARG(n1,n2)   (MRB_ARGS_REQ(n1)|MRB_ARGS_OPT(n2))
    // ```
    mrb_args_req(n_req) | mrb_args_opt(n_opt)
}

/// rest argument
///
/// ```ruby
/// def foo(n1, *rest); end
/// ```
#[inline]
#[must_use]
pub const fn mrb_args_rest() -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_REST()     ((mrb_aspec)(1 << 12))
    // ```
    1 << 12
}

/// required arguments after rest
#[inline]
#[must_use]
pub const fn mrb_args_post(n: u32) -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_POST(n)    ((mrb_aspec)((n)&0x1f) << 7)
    // ```
    (n & 0x1f) << 7
}

/// keyword arguments (`n` of keys, `kdict`)
#[inline]
#[must_use]
pub const fn mrb_args_key(n1: u32, n2: u32) -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_KEY(n1,n2) ((mrb_aspec)((((n1)&0x1f) << 2) | ((n2)?(1<<1):0)))
    // ```
    if n2 == 0 {
        (n1 & 0x1f) << 2
    } else {
        (n1 & 0x1f) << 2 | 1 << 1
    }
}

/// Function takes a block argument
#[inline]
#[must_use]
pub const fn mrb_args_block() -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_BLOCK()    ((mrb_aspec)1)
    // ```
    1
}

/// Function accepts any number of arguments
#[inline]
#[must_use]
pub const fn mrb_args_any() -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_ANY()      MRB_ARGS_REST()
    // ```
    mrb_args_rest()
}

/// Function accepts no arguments
#[inline]
#[must_use]
pub const fn mrb_args_none() -> mrb_aspec {
    // ```c
    // #define MRB_ARGS_NONE()     ((mrb_aspec)0)
    // ```
    0
}

/// Format specifiers for [`mrb_get_args`](crate::sys::mrb_get_args) function.
///
/// `mrb_get_args` has the following prototype and returns the number of
/// arguments parsed.
///
/// ```c
/// MRB_API mrb_int mrb_get_args(mrb_state *mrb, const char *format, ...)
/// ```
///
/// `format` must be a C string composed of the following format specifiers:
///
/// ```text
///   string  mruby type     C type                 note
///   ----------------------------------------------------------------------------------------------
///   o:      Object         `mrb_value`
///   C:      class/module   `mrb_value`
///   S:      String         `mrb_value`            when ! follows, the value may be nil
///   A:      Array          `mrb_value`            when ! follows, the value may be nil
///   H:      Hash           `mrb_value`            when ! follows, the value may be nil
///   s:      String         `char*`,`mrb_int`      Receive two arguments; s! gives (NULL,0) for nil
///   z:      String         `char*`                NUL terminated string; z! gives NULL for nil
///   a:      Array          `mrb_value*`,`mrb_int` Receive two arguments; a! gives (NULL,0) for nil
///   f:      Float          `mrb_float`
///   i:      Integer        `mrb_int`
///   b:      Boolean        `mrb_bool`
///   n:      Symbol         `mrb_sym`
///   d:      Data           `void*`,`mrb_data_type`  2nd argument will be used to check data type so it won't be modified
///   I:      Inline struct  `void*`
///   &:      Block          `mrb_value`            &! raises exception if no block given
///   *:      rest argument  `mrb_value*`,`mrb_int` The rest of the arguments as an array; *! avoid copy of the stack
///   |:      optional                              Following arguments are optional
///   ?:      optional given `mrb_bool`             true if preceding argument (optional) is given
/// ```
pub mod specifiers {
    /// Could be used to retrieve any type of argument
    pub const OBJECT: &str = "o";

    /// Retrieve a Class argument
    pub const CLASS: &str = "C";

    /// Retrieve a Module argument
    pub const MODULE: &str = "C";

    /// Retrieve a String argument
    pub const STRING: &str = "S";

    /// Retrieve a String argument or `nil`
    pub const NILABLE_STRING: &str = "S!";

    /// Retrieve an Array argument
    pub const ARRAY: &str = "A";

    /// Retrieve an Array argument or `nil`
    pub const NILABLE_ARRAY: &str = "A!";

    /// Retrieve a Hash argument
    pub const HASH: &str = "H";

    /// Retrieve a Hash argument or `nil`
    pub const NILABLE_HASH: &str = "H!";

    /// Retrieve a `CString` and its length. Usable like:
    ///
    /// ```c
    /// mrb_get_args(mrb, "s", &ptr, &plen);
    /// ```
    pub const CSTRING_AND_LEN: &str = "s";

    /// Retrieve a `CString` and its length. Gives (NULL, 0) for `nil`. Usable
    /// like:
    ///
    /// ```c
    /// mrb_get_args(mrb, "s", &ptr, &plen);
    /// ```
    pub const NULLABLE_CSTRING_AND_LEN: &str = "s!";

    /// Retrieve a NUL-terminated `CString` argument
    pub const CSTRING: &str = "z";

    /// Retrieve a NUL-terminated `CString` argument. Gives NULL for `nil`
    pub const NULLABLE_CSTRING: &str = "z!";

    /// Receive two arguments, a C Array of `mrb_value`s and its length. Usable
    /// like:
    ///
    /// ```c
    /// mrb_get_args(mrb, "a", &ptr, &blen);
    /// ```
    pub const CARRAY_AND_LEN: &str = "a";

    /// Receive two arguments, a C Array of `mrb_value`s and its length. Gives
    /// (NULL, 0) for `nil`. Usable like:
    ///
    /// ```c
    /// mrb_get_args(mrb, "a", &ptr, &blen);
    /// ```
    pub const NULLABLE_CARRAY_AND_LEN: &str = "a!";

    /// Retrieve a Float argument.
    pub const FLOAT: &str = "f";

    /// Retrieve an Integer argument.
    pub const INTEGER: &str = "i";

    /// Retrieve a Boolean argument.
    pub const BOOLEAN: &str = "b";

    /// Retrieve a Symbol argument.
    pub const SYMBOL: &str = "n";

    /// Receive two arguments, a `void *` pointer to data and an
    /// `mrb_data_type`.
    ///
    /// 2nd argument will be used to check data type so it won't be modified.
    pub const DATA: &str = "d";

    /// Internal, retrieve a `void *`.
    pub const INLINE_STRUCT: &str = "I";

    /// Retrieve a Block argument.
    pub const BLOCK: &str = "&";

    /// Retrieve a Block argument and raise an exception if none is given.
    pub const BLOCK_REQUIRED: &str = "&!";

    /// Retrieve the rest of arguments as an array; Usable like:
    ///
    /// ```c
    /// mrb_get_args(mrb, "*", &argv, &argc);
    /// ```
    pub const REST: &str = "*";

    /// Retrieve the rest of arguments as an array; avoid copy of the stack.
    ///
    /// ```c
    /// mrb_get_args(mrb, "*", &argv, &argc);
    /// ```
    pub const REST_NO_COPY: &str = "*!";

    /// The following args specified are optional.
    pub const FOLLOWING_ARGS_OPTIONAL: &str = "|";

    /// Retrieve a Boolean indicating whether the previous optional argument
    /// was given.
    pub const PREVIOUS_OPTIONAL_ARG_GIVEN: &str = "?";
}
