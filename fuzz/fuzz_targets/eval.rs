#![no_main]
use libfuzzer_sys::fuzz_target;

use artichoke::backend::Eval;

fuzz_target!(|data: &[u8]| {
    let mut interp = artichoke::interpreter().unwrap();
    let _ = interp.eval(data);
    interp.close();
});
