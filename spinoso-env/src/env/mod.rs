//! ENV is a hash-like accessor for environment variables.
//!
//! This module contains implementations of an environ accessor and mutator.
//!
//! [`Memory`](memory::Memory) is based on a Rust [`HashMap`] and offers a
//! virtualized `ENV` API that cannot modify the host system.
//!
//! [`System`](system::System) is based on [`env::var_os`] and directly accesses
//! and modifies the host system environ via platform-specific APIs.
//!
//! [`HashMap`]: std::collections::HashMap
//! [`env::var_os`]: std::env::var_os

pub mod memory;
#[cfg(feature = "system-env")]
pub mod system;
