use std::mem;

use crate::extn::prelude::*;

// TODO(GH-308): extract this function into `sys::protect`
pub unsafe fn is_range<'a>(
    interp: &'a Artichoke,
    first: &Value,
    length: Int,
) -> Result<Option<(Int, Int)>, TypeError> {
    use sys::mrb_range_beg_len::*;

    let mut start = mem::MaybeUninit::<sys::mrb_int>::uninit();
    let mut len = mem::MaybeUninit::<sys::mrb_int>::uninit();
    let mrb = interp.0.borrow().mrb;
    // NOTE: `mrb_range_beg_len` can raise.
    // TODO(GH-308): wrap this in a call to `mrb_protect`.
    let check_range = sys::mrb_range_beg_len(
        mrb,
        first.inner(),
        start.as_mut_ptr(),
        len.as_mut_ptr(),
        length,
        0_u8,
    );
    let start = start.assume_init();
    let len = len.assume_init();
    if check_range == MRB_RANGE_OK {
        Ok(Some((start, len)))
    } else {
        Ok(None)
    }
}
