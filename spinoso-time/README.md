# spinoso-time

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-time.svg)](https://crates.io/crates/spinoso-time)
[![API](https://docs.rs/spinoso-time/badge.svg)](https://docs.rs/spinoso-time)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_time/)

Time is an abstraction of dates and times.

This module implements the [`Time`] class from Ruby Core.

In Artichoke, Time is represented as a 64-bit signed integer of seconds since
January 1, 1970 UTC (the Unix Epoch) and an unsigned 32-bit integer of subsecond
nanoseconds. This allows representing roughly 584 billion years.

You can use this class in your application by accessing it directly. As a Core
class, it is globally available:

```ruby
Time.now
```

This implementation of `Time` is dependent on the selected feature. The **tzrs**
feature uses the [`tzdb`] crate for getting the local timezone information, and
combines with the [`tz-rs`] crate to generate the time.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-time = { version = "0.7.1", features = ["tzrs"] }
```

## Examples

Assuming feature `tzrs` is selected:

```rust
use spinoso_time::tzrs::Time;
// Get a local time set to the current time.
let now = Time::now().unwrap();
// Convert the local time to UTC.
let utc = now.to_utc().unwrap();
assert!(utc.is_utc());
// Extract the Unix timestamp.
let timestamp = utc.to_int();
```

## Crate features

This crate can support several backends, which are designed to be independent of
each other. The availability of different backends is controlled by Cargo
features, all of which are enabled by default:

- **tzrs**: Enable a `Time` backend which is implemented by the [`tz-rs`] and
  [`tzdb`] crates.

### Additional features

- **tzrs-local**: Enable the detection of the system timezone with the **tzrs**
  backend. This feature is enabled by default. Enabling this feature also
  activates the **tzrs** feature.

  If the **tzrs-local** feature is disabled, the local timezone is defaulted to
  GMT (not UTC).

This crate requires [`std`], the Rust Standard Library.

## License

`spinoso-time` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[`time`]: https://ruby-doc.org/core-3.1.2/Time.html
[`tz-rs`]: https://crates.io/crates/tz-rs
[`tzdb`]: https://crates.io/crates/tzdb
[`std`]: https://doc.rust-lang.org/stable/std/
