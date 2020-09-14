use spinoso_random::Mt;

#[test]
fn bytes_reproducibility() {
    // See `scripts/reproducibility.rb`.
    let expected: &[u8] = &[
        20, 92, 158, 63, 135, 9, 143, 176, 216, 150, 49, 115, 66, 2, 86, 171, 146, 106, 51, 105,
        201, 108, 178, 235, 57, 0, 163, 66, 195, 40, 37, 28, 102, 64, 210, 222, 61, 136, 60, 192,
        211, 198, 94, 47, 142, 181, 234, 156, 252, 135, 8, 5, 170, 223, 183, 217, 45, 83, 8, 244,
        35, 103, 59, 225,
    ];

    let mut rng = Mt::with_seed(33);
    let mut samples = [0; 64];
    rng.fill_bytes(&mut samples);
    assert_eq!(samples[..], expected[..]);
}

#[test]
fn float_reproducibility() {
    // See `scripts/reproducibility.rb`.
    let expected: &[f64] = &[
        0.24851012743772993,
        0.44997542105079547,
        0.4109408029965408,
        0.26029969088689986,
        0.8703956883469495,
        0.18503992716190498,
        0.01966142543004401,
        0.9532520315037121,
        0.6804508047310392,
        0.48658812655052786,
        0.9650268198659541,
        0.39339873912163337,
        0.07955757128689422,
        0.35140742441379624,
        0.16363516260509525,
        0.9831668210338976,
        0.8806281842152414,
        0.49406346811156343,
        0.40095924111847847,
        0.4512914630087632,
    ];

    let mut rng = Mt::with_seed(33);
    let mut samples = vec![0.0; expected.len()];
    for cell in samples.iter_mut() {
        *cell = rng.next_real();
    }
    assert_eq!(samples, expected);
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
    let mut rng = Mt::with_seed(33);
    let mut buf = [0; 2];
    rng.fill_bytes(&mut buf);
    assert_eq!(buf[..], b"\x14\\"[..]);

    let mut skip = [0; 1000];
    rng.fill_bytes(&mut skip);

    let mut buf = [0; 2];
    rng.fill_bytes(&mut buf);
    assert_eq!(buf[..], b"\xA1p"[..]);
}
