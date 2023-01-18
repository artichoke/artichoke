//! Virtual file system.
//!
//! Artichoke proxies all file system access through a virtual file system. The
//! file system can store Ruby sources and [extension hooks] in memory and will
//! support proxying to the host file system for reads and writes.
//!
//! Artichoke uses the virtual file system to track metadata about loaded
//! features.
//!
//! Artichoke has several virtual file system implementations. Only some of them
//! support reading from the system file system.
//!
//! [extension hooks]: ExtensionHook

use crate::error::Error;
use crate::Artichoke;

#[cfg(feature = "load-path-native-file-system-loader")]
mod hybrid;
mod memory;
#[cfg(feature = "load-path-native-file-system-loader")]
mod native;

#[cfg(feature = "load-path-native-file-system-loader")]
pub use hybrid::Hybrid;
pub use memory::Memory;
#[cfg(feature = "load-path-native-file-system-loader")]
pub use native::Native;

/// Directory at which Ruby sources and extensions are stored in the virtual
/// file system.
///
/// `RUBY_LOAD_PATH` is the default current working directory for
/// [`Memory`] file systems.
///
/// If the `load-path-native-file-system-loader` feature is enabled, the file
/// system will locate the path on a [`Memory`] file system.
#[cfg(not(windows))]
pub const RUBY_LOAD_PATH: &str = "/artichoke/virtual_root/src/lib";

/// Directory at which Ruby sources and extensions are stored in the virtual
/// file system.
///
/// `RUBY_LOAD_PATH` is the default current working directory for
/// [`Memory`] file systems.
///
/// [`Hybrid`] file systems locate the path on a [`Memory`] file system.
#[cfg(windows)]
pub const RUBY_LOAD_PATH: &str = "c:/artichoke/virtual_root/src/lib";

/// Function type for extension hooks stored in the virtual file system.
///
/// This signature is equivalent to the signature for [`File::require`] as
/// defined by the `artichoke-backend` implementation of [`LoadSources`].
///
/// [`File::require`]: artichoke_core::file::File::require
/// [`LoadSources`]: crate::core::LoadSources
pub type ExtensionHook = fn(&mut Artichoke) -> Result<(), Error>;

#[cfg(all(feature = "load-path-native-file-system-loader", not(any(test, doctest))))]
pub type Adapter = Hybrid;
#[cfg(any(not(feature = "load-path-native-file-system-loader"), test, doctest))]
pub type Adapter = Memory;
