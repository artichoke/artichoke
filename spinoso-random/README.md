# spinoso-random

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-random.svg)](https://crates.io/crates/spinoso-random)
[![API](https://docs.rs/spinoso-random/badge.svg)](https://docs.rs/spinoso-random)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_random/)

An implementation of [Ruby's pseudo-random number generator][ruby-random], or
PRNG.

The PRNG produces a deterministic sequence of bits which approximate true
randomness. The sequence may be represented by integers, floats, or binary
strings.

The generator may be initialized with either a system-generated or user-supplied
seed value.

PRNGs are currently implemented as a modified Mersenne Twister with a period of
2\*\*19937-1.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-random = "0.1"
```

Generate integers:

```rust
use spinoso_random::Random;
let seed = [627457_u32, 697550, 16438, 41926];
let mut random = Random::with_array_seed(seed);
let rand = random.next_int32();
```

Generate random numbers in a range:

```rust
use spinoso_random::{rand, Error, Max, Rand, Random};

fn example() -> Result<(), Error> {
    let mut random = Random::new()?;
    let max = Max::Integer(10);
    let mut rand = rand(&mut random, max)?;
    assert!(matches!(rand, Rand::Integer(x) if x < 10));
    Ok(())
}
```

## `no_std`

This crate is `no_std` compatible when built without the `std` feature. This
crate does not depend on [`alloc`].

## Crate features

All features are enabled by default.

- **random-rand** - Enables range sampling methods for the `rand()` function.
  Activating this feature also activates the **rand_core** feature. Dropping
  this feature removes the [`rand`] dependency.
- **rand-traits** - Enables implementations of [`RngCore`] on `Random` and `Mt`
  types. Dropping this feature removes the [`rand_core`] dependency.
- **std** - Enables a dependency on the Rust Standard Library. Activating this
  feature enables [`std::error::Error`] impls on error types in this crate.

## License

`spinoso-random` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

`spinoso-random` is a substantially rewritten fork of [`rand_mt`] @
[`aeb3274a`][rand-mt-forked-at], which is copyright rust-mersenne-twister
developers and Ryan Lopopolo \<rjl@hyperbo.la\>. `rand_mt` is licensed with
either the [MIT License][rand-mt-mit] or [Apache 2.0 License][rand-mt-apache].
`spinoso-random` opts for the MIT license for its fork.

`spinoso-random` is partially derived from [`random.c`] in Ruby @
[2.6.3][ruby-2.6.3] which is copyright Yukihiro Matsumoto \<matz@netlab.jp\>.
Ruby is licensed with the [2-clause BSDL License][ruby-license].

[ruby-random]: https://ruby-doc.org/core-2.6.3/Random.html
[`alloc`]: https://doc.rust-lang.org/alloc/
[`rngcore`]: https://docs.rs/rand_core/latest/rand_core/trait.RngCore.html
[`rand`]: https://crates.io/crates/rand
[`rand_core`]: https://crates.io/crates/rand_core
[`std::error::error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`rand_mt`]: https://crates.io/crates/rand_mt/3.0.0
[rand-mt-forked-at]:
  https://github.com/artichoke/rand_mt/tree/aeb3274a3aa3caa68ba379c141d2c55b1b21828e
[rand-mt-mit]:
  https://github.com/artichoke/rand_mt/blob/aeb3274a3aa3caa68ba379c141d2c55b1b21828e/LICENSE-MIT
[rand-mt-apache]:
  https://github.com/artichoke/rand_mt/blob/aeb3274a3aa3caa68ba379c141d2c55b1b21828e/LICENSE-APACHE
[`random.c`]: https://github.com/ruby/ruby/blob/v2_6_3/random.c
[ruby-2.6.3]: https://github.com/ruby/ruby/tree/v2_6_3
[ruby-license]: https://github.com/ruby/ruby/blob/v2_6_3/COPYING
