use crate::UrandomError;

/// Read random bytes, using platform-provided randomness.
///
/// `dest` is completely filled with bytes that are expected to be a
/// cryptographically secure pseudo-random number in binary form.
///
/// In 2017, Linux manpage random(7) writes that "no cryptographic primitive
/// available today can hope to promise more than 256 bits of security". So it
/// might be questionable to pass a slice where `dest.len() > 32` to this
/// method.
///
/// # Examples
///
/// ```
/// use spinoso_random::Error;
///
/// # fn example() -> Result<(), Error> {
/// let mut bytes = [0_u8; 32];
/// spinoso_random::urandom(&mut bytes)?;
/// assert!(!bytes.iter().all(|&b| b == 0));
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// # Errors
///
/// If the randomness feature provided by the platform is not present or failed
/// to completely fill `dest`, an error is returned. This error should be raised
/// as a [Ruby `RuntimeError`].
///
/// [Ruby `RuntimeError`]: https://ruby-doc.org/core-3.1.2/RuntimeError.html
pub fn urandom(dest: &mut [u8]) -> Result<(), UrandomError> {
    if getrandom::getrandom(dest).is_err() {
        return Err(UrandomError::new());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::urandom;

    #[test]
    fn read_random_bytes() {
        let mut buf_a = [0; 256];
        let mut buf_b = [0; 256];
        urandom(&mut buf_a).unwrap();
        urandom(&mut buf_b).unwrap();
        assert_ne!(buf_a, buf_b);
    }
}
