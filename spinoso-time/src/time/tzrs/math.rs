use core::ops::Add;
use core::time::Duration;

use crate::time::tzrs::Time;

impl Add<Duration> for Time {
    type Output = Self;
    fn add(self, _to_add: Duration) -> Self {
        todo!()
    }
}

impl Add<i64> for Time {
    type Output = Self;
    fn add(self, _to_add: i64) -> Self {
        todo!()
    }
}

impl Time {
    // Time#succ (obselete)
    pub fn succ(&self) -> Self {
        todo!()
    }
}
