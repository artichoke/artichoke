use artichoke_core::top_self::TopSelf;

use crate::sys;
use crate::value::Value;
use crate::Artichoke;

impl TopSelf for Artichoke {
    type Value = Value;

    fn top_self(&mut self) -> Value {
        let top = unsafe { sys::mrb_top_self(self.mrb_mut()) };
        Value::new(self, top)
    }
}
