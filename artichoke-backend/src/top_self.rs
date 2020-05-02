use crate::core::TopSelf;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

impl TopSelf for Artichoke {
    type Value = Value;

    fn top_self(&mut self) -> Value {
        let mrb = self.0.borrow().mrb;
        let top_self = unsafe { sys::mrb_top_self(mrb) };
        Value::new(self, top_self)
    }
}
