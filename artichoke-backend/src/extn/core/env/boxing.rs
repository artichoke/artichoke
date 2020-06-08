use crate::convert::HeapAllocatedData;
use crate::extn::core::env::Environ;

impl HeapAllocatedData for Environ {
    const RUBY_TYPE: &'static str = "Artichoke::Environ";
}
