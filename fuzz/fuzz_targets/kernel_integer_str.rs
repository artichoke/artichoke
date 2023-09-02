#![no_main]
use libfuzzer_sys::fuzz_target;
use scolapasta_int_parse::parse;

fuzz_target!(|data: &str| {
    // We're looking for panics, not correctness, so drop the results.
    let _ignored = parse(data, None);
    for radix in -500..500 {
        let _ignored = parse(data, Some(radix));
    }
    let _ignored = parse(data, Some(i32::MAX.into()));
    let _ignored = parse(data, Some(i32::MIN.into()));
    let _ignored = parse(data, Some(i64::MAX));
    let _ignored = parse(data, Some(i64::MAX));
});
