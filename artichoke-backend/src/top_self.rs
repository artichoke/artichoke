use artichoke_core::top_self::TopSelf;

use crate::sys;
use crate::value::Value;
use crate::Artichoke;

impl TopSelf for Artichoke {
    type Value = Value;

    fn top_self(&self) -> Value {
        let mrb = self.0.borrow().mrb;
        Value::new(self, unsafe { sys::mrb_top_self(mrb) })
    }
}
