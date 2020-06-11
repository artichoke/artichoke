use crate::convert::HeapAllocatedData;
use crate::extn::core::random::Random;

impl HeapAllocatedData for Random {
    const RUBY_TYPE: &'static str = "Random";
}
