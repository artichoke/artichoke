use crate::convert::HeapAllocatedData;
use crate::extn::core::time::Time;

impl HeapAllocatedData for Time {
    const RUBY_TYPE: &'static str = "Time";
}
