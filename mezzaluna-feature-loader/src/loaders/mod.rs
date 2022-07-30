//! Strategies for loading Ruby source code and native extensions.
//!
//! This module contains several "loaders" which can be used to retrieve Ruby
//! source code and native extensions for parsing and executing on a Ruby
//! interpreter.
//!
//! The loaders in this module may require a crate feature to be enabled. See
//! the documentation of the types in this module for a description of how each
//! loader interacts with the host system.

#[cfg(feature = "disk")]
mod disk;
mod memory;
#[cfg(feature = "rubylib")]
mod rubylib;

#[cfg(feature = "disk")]
pub use disk::Disk;
pub use memory::Memory;
#[cfg(feature = "rubylib")]
pub use rubylib::Rubylib;
