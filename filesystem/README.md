# filesystem-rs

### Real, fake, and mock implementations of file system operations.

[![Build Status](https://travis-ci.org/iredelmeier/filesystem-rs.svg?branch=master)](https://travis-ci.org/iredelmeier/filesystem-rs)
[![Docs](https://docs.rs/filesystem/badge.svg)](https://docs.rs/filesystem)
[![Crates.io](https://img.shields.io/crates/v/filesystem.svg)](https://crates.io/crates/filesystem)

[Documentation](https://docs.rs/filesystem)

filesystem-rs provides real, fake, and mock implementations of file system-related functionality. It abstracts away details of certain common but complex operations (e.g., setting permissions) and makes it easier to test any file system-related logic without having to wait for slow I/O operations or coerce the file system into particular states.
