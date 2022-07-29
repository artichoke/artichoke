mod nul_terminated_vec;
mod vec;

pub use imp::Buf;
#[cfg(feature = "always-nul-terminated-c-string-compat")]
use nul_terminated_vec as imp;
#[cfg(not(feature = "always-nul-terminated-c-string-compat"))]
use vec as imp;
