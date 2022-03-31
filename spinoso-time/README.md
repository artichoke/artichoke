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

This implementation of `Time` supports the system clock via the [`chrono`]
crate.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-time = "0.2.0"
```

## Examples

```rust
use spinoso_time::Time;
// Get a local time set to the current time.
let now = Time::now();
// Convert the local time to UTC.
let utc = now.to_utc();
assert!(utc.is_utc());
// Extract the Unix timestamp.
let timestamp = utc.to_int();
```

## License

`spinoso-time` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[`time`]: https://ruby-doc.org/core-2.6.3/Time.html
[`chrono`]: https://crates.io/crates/chrono
