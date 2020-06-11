use crate::convert::HeapAllocatedData;
use crate::extn::core::regexp::Regexp;

impl HeapAllocatedData for Regexp {
    const RUBY_TYPE: &'static str = "Regexp";
}
