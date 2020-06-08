use crate::convert::HeapAllocatedData;
use crate::extn::core::array::Array;

impl HeapAllocatedData for Array {
    const RUBY_TYPE: &'static str = "Array";
}
