#![no_main]
use artichoke::prelude::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut interp = artichoke::interpreter().unwrap();
    let _ignore_errors_from_evaling_arbitrary_bytes = interp.eval(data);
    interp.close();
});
