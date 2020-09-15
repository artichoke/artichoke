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

// ```ruby
// # Should double check this is official spec
// it "returns the same numeric output for a given seed across all implementations and platforms" do
//   rnd = Random.new(33)
//   rnd.bytes(2).should == "\x14\\"
//   rnd.bytes(1000) # skip some
//   rnd.bytes(2).should == "\xA1p"
// end
//
// it "returns the same numeric output for a given huge seed across all implementations and platforms" do
//   rnd = Random.new(bignum_value ** 4)
//   rnd.bytes(2).should == "_\x91"
//   rnd.bytes(1000) # skip some
//   rnd.bytes(2).should == "\x17\x12"
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
