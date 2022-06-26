use spinoso_random::Random;

mod vectors;

#[test]
fn bytes_reproducibility() {
    let mut rng = Random::with_seed(33);
    let mut samples = vec![0; 4096];
    rng.fill_bytes(&mut samples);
    assert_eq!(samples[..], vectors::BYTES_SEED_32[..]);
}

#[test]
fn float_reproducibility() {
    let mut rng = Random::with_seed(33);
    let mut samples = Vec::with_capacity(4096);
    for cell in samples.iter_mut() {
        *cell = rng.next_real();
    }
    for (sample, expected) in samples.iter().zip(vectors::REAL_SEED_32.iter()) {
        assert!((sample - expected).abs() < f64::EPSILON);
    }
}

#[test]
fn u32_reproducibility() {
    let mut rng = Random::with_seed(33);
    let mut samples = Vec::with_capacity(4096);
    for _ in 0..4096 {
        samples.push(rng.next_int32());
    }
    assert_eq!(samples[..], vectors::INT32_SEED_32[..]);
}

// # ruby/spec Random
//
// ```ruby
// # Should double check this is official spec
// it "returns the same numeric output for a given seed across all implementations and platforms" do
//   rnd = Random.new(33)
//   rnd.bytes(2).should == "\x14\\"
//   rnd.bytes(1000) # skip some
//   rnd.bytes(2).should == "\xA1p"
// end
// ```
#[test]
fn spec_bytes() {
    let mut rng = Random::with_seed(33);
    let mut buf = [0; 2];
    rng.fill_bytes(&mut buf);
    assert_eq!(buf[..], b"\x14\\"[..]);

    let mut skip = [0; 1000];
    rng.fill_bytes(&mut skip);

    let mut buf = [0; 2];
    rng.fill_bytes(&mut buf);
    assert_eq!(buf[..], b"\xA1p"[..]);
}

// # MSpec helpers
//
// ```ruby
// def bignum_value(plus = 0)
//   0x8000_0000_0000_0000 + plus
// end
// ```
//
// # ruby/spec Random
//
// ```ruby
// it "returns the same numeric output for a given huge seed across all implementations and platforms" do
//   rnd = Random.new(bignum_value ** 4)
//   rnd.bytes(2).should == "_\x91"
//   rnd.bytes(1000) # skip some
//   rnd.bytes(2).should == "\x17\x12"
// end
// ```
#[test]
fn spec_big_num_bytes() {
    // ```console
    // [3.1.2] > big = 0x8000_0000_0000_0000 ** 4
    // => 7237005577332262213973186563042994240829374041602535252466099000494570602496
    // [3.1.2] > bytes = big.to_s(16).split("").each_slice(8).map{|s| "0x#{s.join[0, 4]}_#{s.join[4, 4]}"}
    // => ["0x1000_0000", "0x0000_0000", "0x0000_0000", "0x0000_0000", "0x0000_0000", "0x0000_0000", "0x0000_0000", "0x0000_0000"]
    // [3.1.2] > puts bytes.inspect.gsub '"', ''
    // [0x1000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000]
    // => nil
    // ```
    let seed: [u32; 8] = [
        0x1000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
    ];
    // Ruby "packs" a `Bignum` into a `&mut [u32]` with least significant word
    // first and native byte order.
    //
    // https://github.com/ruby/ruby/blob/v2_6_3/random.c#L383-L384
    let mut rng = Random::with_array_seed(seed.iter().rev().copied());
    let mut buf = [0; 2];
    rng.fill_bytes(&mut buf);
    assert_eq!(buf[..], b"_\x91"[..]);

    let mut skip = [0; 1000];
    rng.fill_bytes(&mut skip);

    let mut buf = [0; 2];
    rng.fill_bytes(&mut buf);
    assert_eq!(buf[..], b"\x17\x12"[..]);
}
