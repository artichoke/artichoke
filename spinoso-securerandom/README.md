# spinoso-securerandom

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-securerandom.svg)](https://crates.io/crates/spinoso-securerandom)
[![API](https://docs.rs/spinoso-securerandom/badge.svg)](https://docs.rs/spinoso-securerandom)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_securerandom/)

Secure random number generator interface.

This module implements the [`SecureRandom`] package from the Ruby Standard
Library. It is an interface to secure random number generators which are
suitable for generating session keys in HTTP cookies, etc.

This implementation of `SecureRandom` supports the system RNG via the
[`getrandom`] crate. This implementation does not depend on OpenSSL.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-securerandom = "0.1"
```

## Examples

Generate cryptographically secure random bytes:

```rust
fn example() -> Result<(), spinoso_securerandom::Error> {
    let bytes = spinoso_securerandom::random_bytes(Some(1024))?;
    assert_eq!(bytes.len(), 1024);
    Ok(())
}
```

Generate base64-encoded random data:

```rust
fn example() -> Result<(), spinoso_securerandom::Error> {
    let bytes = spinoso_securerandom::base64(Some(1024))?;
    assert_eq!(bytes.len(), 1368);
    assert!(bytes.is_ascii());
    Ok(())
}
```

Generate random floats and integers in a range bounded from zero to a maximum:

```rust
use spinoso_securerandom::{DomainError, Max, Rand};

fn example() -> Result<(), DomainError> {
    let rand = spinoso_securerandom::random_number(Max::None)?;
    assert!(matches!(rand, Rand::Float(_)));

    let rand = spinoso_securerandom::random_number(Max::Integer(57))?;
    assert!(matches!(rand, Rand::Integer(_)));

    let rand = spinoso_securerandom::random_number(Max::Float(57.0))?;
    assert!(matches!(rand, Rand::Float(_)));
    Ok(())
}
```

Generate version 4 random UUIDs:

```rust
fn example() -> Result<(), spinoso_securerandom::Error> {
    let uuid = spinoso_securerandom::uuid()?;
    assert_eq!(uuid.len(), 36);
    assert!(uuid.chars().all(|ch| ch == '-' || ch.is_ascii_hexdigit()));
    Ok(())
}
```

## License

`spinoso-securerandom` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.

[`securerandom`]:
  https://ruby-doc.org/stdlib-2.6.3/libdoc/securerandom/rdoc/SecureRandom.html
[`getrandom`]: https://crates.io/crates/getrandom
