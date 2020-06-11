use crate::convert::HeapAllocatedData;
use crate::extn::core::matchdata::MatchData;

impl HeapAllocatedData for MatchData {
    const RUBY_TYPE: &'static str = "MatchData";
}
