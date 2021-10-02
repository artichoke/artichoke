//! Helper routines for reimplementing functions in mruby.

use core::num::Wrapping;

/// Simple, non-cryptographic, non-collision-resistant bytestring hasher.
///
/// This hash routine is used in [mruby 3.0.0] and [MRI Ruby 1.8.7].
///
/// MRI Ruby 1.8.7 calls this hash routine "Perl hash".
///
/// [mruby 3.0.0]: https://github.com/artichoke/mruby/blob/3.0.0/src/string.c#L1751-L1769
/// [MRI Ruby 1.8.7]: https://github.com/ruby/ruby/blob/v1_8_7/string.c#L869-L903
pub fn mrb_str_hash<T: AsRef<[u8]>>(s: T) -> u32 {
    fn inner(s: &[u8]) -> u32 {
        let mut hash = Wrapping(0_u32);
        for &b in s.iter() {
            hash += Wrapping(u32::from(b));
            hash += hash << 10;
            hash ^= hash >> 6;
        }
        hash += hash << 3;
        hash ^= hash >> 11;
        hash += hash << 15;

        hash.0
    }

    inner(s.as_ref())
}
