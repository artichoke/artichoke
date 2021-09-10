use artichoke_core::debug::Debug;
use artichoke_core::value::Value as _;

use crate::types::Ruby;
use crate::value::Value;
use crate::Artichoke;

impl Debug for Artichoke {
    type Value = Value;

    fn inspect_type_name_for_value(&mut self, value: Self::Value) -> &str {
        match value.try_convert_into(self) {
            Ok(Some(true)) => "true",
            Ok(Some(false)) => "false",
            Ok(None) => "nil",
            Err(_) if matches!(value.ruby_type(), Ruby::Data | Ruby::Object) => self.class_name_for_value(value),
            Err(_) => value.ruby_type().class_name(),
        }
    }

    fn class_name_for_value(&mut self, value: Self::Value) -> &str {
        if let Ok(class) = value.funcall(self, "class", &[], None) {
            if let Ok(class) = class.funcall(self, "name", &[], None) {
                if let Ok(class) = class.try_convert_into_mut(self) {
                    return class;
                }
            }
        }
        ""
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn debug_true_value_as_classlike() {
        let mut interp = interpreter().unwrap();
        let value = interp.convert(true);
        assert_eq!(interp.inspect_type_name_for_value(value), "true");
    }

    #[test]
    fn debug_false_value_as_classlike() {
        let mut interp = interpreter().unwrap();
        let value = interp.convert(false);
        assert_eq!(interp.inspect_type_name_for_value(value), "false");
    }

    #[test]
    fn debug_nil_value_as_classlike() {
        let mut interp = interpreter().unwrap();
        let value = interp.convert(None::<Value>);
        assert_eq!(interp.inspect_type_name_for_value(value), "nil");
    }

    #[test]
    fn debug_fixnum_value_as_classlike() {
        let mut interp = interpreter().unwrap();
        let value = interp.convert(1_i64);
        assert_eq!(interp.inspect_type_name_for_value(value), "Integer");
    }

    #[test]
    fn debug_hash_value_as_classlike() {
        let mut interp = interpreter().unwrap();
        let value = interp.try_convert_mut(vec![(b"foo".to_vec(), vec![1, 2, 3])]).unwrap();
        assert_eq!(interp.inspect_type_name_for_value(value), "Hash");
    }

    #[test]
    fn debug_array_value_as_classlike() {
        let mut interp = interpreter().unwrap();
        let value = interp.try_convert_mut(vec![1_i64]).unwrap();
        assert_eq!(interp.inspect_type_name_for_value(value), "Array");
    }
}
