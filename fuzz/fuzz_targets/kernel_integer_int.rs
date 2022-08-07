#![no_main]
use libfuzzer_sys::fuzz_target;

use scolapasta_int_parse::parse;

fuzz_target!(|data: i64| {
    let data = data.to_string();
    // We're looking for panics, not correctness, so drop the results.
    drop(parse(&data, None));
    for radix in -500..500 {
        drop(parse(&data, Some(radix)));
    }
    drop(parse(&data, Some(i32::MAX.into())));
    drop(parse(&data, Some(i32::MIN.into())));
    drop(parse(&data, Some(i64::MAX)));
    drop(parse(&data, Some(i64::MAX)));
});
