pub use crate::core::{Ruby, Rust};
use crate::sys;

/// Parse a [`Ruby`] type classifier from a [`sys::mrb_value`].
///
/// This function collapses some mruby types into a [`Ruby::Unreachable`] type
/// that force an interaction with the VM to return an error.
#[allow(non_upper_case_globals)]
#[must_use]
pub fn ruby_from_mrb_value(value: sys::mrb_value) -> Ruby {
    use sys::mrb_vtype::{
        MRB_TT_ARRAY, MRB_TT_BIGINT, MRB_TT_BREAK, MRB_TT_CDATA, MRB_TT_CLASS, MRB_TT_COMPLEX, MRB_TT_CPTR,
        MRB_TT_ENV, MRB_TT_EXCEPTION, MRB_TT_FALSE, MRB_TT_FIBER, MRB_TT_FLOAT, MRB_TT_FREE, MRB_TT_HASH,
        MRB_TT_ICLASS, MRB_TT_INTEGER, MRB_TT_ISTRUCT, MRB_TT_MAXDEFINE, MRB_TT_MODULE, MRB_TT_OBJECT, MRB_TT_PROC,
        MRB_TT_RANGE, MRB_TT_RATIONAL, MRB_TT_SCLASS, MRB_TT_STRING, MRB_TT_STRUCT, MRB_TT_SYMBOL, MRB_TT_TRUE,
        MRB_TT_UNDEF,
    };

    // Suppress lint to enumerate match arms in the same order they are defined
    // in the `sys::mrb_vtype` enum C source.
    #[allow(clippy::match_same_arms)]
    match value.tt {
        // `nil` is implemented with the `MRB_TT_FALSE` type tag in mruby
        // (since both values are falsy). The difference is that Booleans are
        // non-zero `Fixnum`s.
        MRB_TT_FALSE if unsafe { sys::mrb_sys_value_is_nil(value) } => Ruby::Nil,
        MRB_TT_FALSE => Ruby::Bool,
        // `MRB_TT_FREE` is a marker type tag that indicates to the mruby
        // VM that an object is unreachable and should be deallocated by the
        // garbage collector.
        MRB_TT_FREE => Ruby::Unreachable,
        MRB_TT_TRUE => Ruby::Bool,
        MRB_TT_INTEGER => Ruby::Fixnum,
        MRB_TT_SYMBOL => Ruby::Symbol,
        // internal use: `#undef`; should not happen
        MRB_TT_UNDEF => Ruby::Unreachable,
        MRB_TT_FLOAT => Ruby::Float,
        // `MRB_TT_CPTR` wraps a borrowed `void *` pointer.
        MRB_TT_CPTR => Ruby::CPointer,
        MRB_TT_OBJECT => Ruby::Object,
        MRB_TT_CLASS => Ruby::Class,
        MRB_TT_MODULE => Ruby::Module,
        // `MRB_TT_ICLASS` is an internal use type tag meant for holding
        // mixed in modules.
        MRB_TT_ICLASS => Ruby::Unreachable,
        // `MRB_TT_SCLASS` represents a singleton class, or a class that is
        // defined anonymously, e.g. `c1` or `c2` below:
        //
        // ```ruby
        // c1 = Class.new {
        //   def foo; :foo; end
        // }
        // c2 = (class <<cls; self; end)
        // ```
        //
        // mruby also uses the term singleton method to refer to methods
        // defined on an object's eigenclass, e.g. `bar` below:
        //
        // ```ruby
        // class Foo; end
        // obj = Foo.new
        // def obj.bar; 'bar'; end
        // ```
        MRB_TT_SCLASS => Ruby::SingletonClass,
        MRB_TT_PROC => Ruby::Proc,
        // `MRB_TT_ARRAY` refers to the mruby `mrb_array` implementation.
        // Artichoke implements its own `Array` as a `Ruby::Data`, so this
        // variant is unreachable.
        MRB_TT_ARRAY => Ruby::Array,
        MRB_TT_HASH => Ruby::Hash,
        MRB_TT_STRING => Ruby::String,
        MRB_TT_RANGE => Ruby::Range,
        MRB_TT_EXCEPTION => Ruby::Exception,
        // NOTE(lopopolo): This might be an internal closure symbol table,
        // rather than the `ENV` core object.
        MRB_TT_ENV => Ruby::Unreachable,
        // `MRB_TT_CDATA` is a type tag for wrapped C pointers. It is used
        // to indicate that an `mrb_value` has an owned pointer to an
        // external data structure stored in its `value.p` field.
        MRB_TT_CDATA => Ruby::Data,
        // NOTE(lopopolo): `Fiber`s are unimplemented in Artichoke.
        MRB_TT_FIBER => Ruby::Fiber,
        MRB_TT_STRUCT => Ruby::Unreachable,
        // `MRB_TT_ISTRUCT` is an "inline structure", or a `mrb_value` that
        // stores data in a `char*` buffer inside an `mrb_value`. These
        // `mrb_value`s cannot have a finalizer and cannot have instance
        // variables.
        //
        // See `vendor/mruby-*/include/mruby/istruct.h`.
        MRB_TT_ISTRUCT => Ruby::InlineStruct,
        // `MRB_TT_BREAK` is used internally to the mruby VM. BREAK is used as
        // the return value of `mrb_yield` when the block has a non-local
        // return.
        //
        // FIXME(lopopolo): The below "unreachable" designation is incorrect.
        // BREAK should be handled by `sys::protect::block_yield`.
        MRB_TT_BREAK => Ruby::Unreachable,
        MRB_TT_COMPLEX => Ruby::Unreachable,
        MRB_TT_RATIONAL => Ruby::Unreachable,
        MRB_TT_BIGINT => Ruby::Unreachable,
        // `MRB_TT_MAXDEFINE` is a marker enum value used by the mruby VM to
        // dynamically check if a type tag is valid using the less than
        // operator. It does not correspond to an instantiated type.
        MRB_TT_MAXDEFINE => Ruby::Unreachable,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::test::prelude::*;
    use crate::types;

    #[test]
    fn parse_nil_ruby_type() {
        let nil = Value::nil();
        assert_eq!(Ruby::Nil, types::ruby_from_mrb_value(nil.inner()));
    }

    #[test]
    fn parse_bool_ruby_type() {
        let interp = interpreter();
        let yes = interp.convert(true);
        assert_eq!(Ruby::Bool, types::ruby_from_mrb_value(yes.inner()));
        let no = interp.convert(false);
        assert_eq!(Ruby::Bool, types::ruby_from_mrb_value(no.inner()));
    }

    #[test]
    fn parse_fixnum_ruby_type() {
        let interp = interpreter();
        let zero = interp.convert(0_i64);
        assert_eq!(Ruby::Fixnum, types::ruby_from_mrb_value(zero.inner()));
        let thousand = interp.convert(1000_i64);
        assert_eq!(Ruby::Fixnum, types::ruby_from_mrb_value(thousand.inner()));
    }

    #[test]
    fn parse_symbol_ruby_type() {
        let mut interp = interpreter();
        let empty = interp.eval(b":''").unwrap();
        assert_eq!(Ruby::Symbol, types::ruby_from_mrb_value(empty.inner()));
        let utf8 = interp.eval(b":Artichoke").unwrap();
        assert_eq!(Ruby::Symbol, types::ruby_from_mrb_value(utf8.inner()));
        let binary = interp.eval(br#":"\xFE""#).unwrap();
        assert_eq!(Ruby::Symbol, types::ruby_from_mrb_value(binary.inner()));
    }

    #[test]
    fn parse_float_ruby_type() {
        let mut interp = interpreter();
        let zero = interp.convert_mut(0.0_f64);
        assert_eq!(Ruby::Float, types::ruby_from_mrb_value(zero.inner()));
        let float = interp.convert_mut(1.5_f64);
        assert_eq!(Ruby::Float, types::ruby_from_mrb_value(float.inner()));
    }

    #[test]
    fn parse_object_ruby_type() {
        let mut interp = interpreter();
        let object = interp.eval(b"Object.new").unwrap();
        assert_eq!(Ruby::Object, types::ruby_from_mrb_value(object.inner()));
        #[cfg(feature = "core-env")]
        {
            let env = interp.eval(b"ENV").unwrap();
            assert_eq!(Ruby::Object, types::ruby_from_mrb_value(env.inner()));
        }
    }

    #[test]
    fn parse_class_ruby_type() {
        let mut interp = interpreter();
        let builtin = interp.eval(b"Object").unwrap();
        assert_eq!(Ruby::Class, types::ruby_from_mrb_value(builtin.inner()));
        let data = interp.eval(b"Array").unwrap();
        assert_eq!(Ruby::Class, types::ruby_from_mrb_value(data.inner()));
        #[cfg(feature = "core-math")]
        {
            let source = interp.eval(b"Math::DomainError").unwrap();
            assert_eq!(Ruby::Class, types::ruby_from_mrb_value(source.inner()));
        }
    }

    #[test]
    fn parse_module_ruby_type() {
        let mut interp = interpreter();
        let builtin = interp.eval(b"Comparable").unwrap();
        assert_eq!(Ruby::Module, types::ruby_from_mrb_value(builtin.inner()));
        #[cfg(feature = "core-math")]
        {
            let data = interp.eval(b"Math").unwrap();
            assert_eq!(Ruby::Module, types::ruby_from_mrb_value(data.inner()));
        }
        let artichoke = interp.eval(b"Artichoke").unwrap();
        assert_eq!(Ruby::Module, types::ruby_from_mrb_value(artichoke.inner()));
    }

    #[test]
    fn parse_proc_ruby_type() {
        let mut interp = interpreter();
        let literal = interp.eval(b"proc {}").unwrap();
        assert_eq!(Ruby::Proc, types::ruby_from_mrb_value(literal.inner()));
        let proc = interp.eval(b"Proc.new {}").unwrap();
        assert_eq!(Ruby::Proc, types::ruby_from_mrb_value(proc.inner()));
        let lambda = interp.eval(b"lambda {}").unwrap();
        assert_eq!(Ruby::Proc, types::ruby_from_mrb_value(lambda.inner()));
        let stabby = interp.eval(b"->() {}").unwrap();
        assert_eq!(Ruby::Proc, types::ruby_from_mrb_value(stabby.inner()));
    }

    #[test]
    fn parse_string_ruby_type() {
        let mut interp = interpreter();
        let empty = interp.try_convert_mut("").unwrap();
        assert_eq!(Ruby::String, types::ruby_from_mrb_value(empty.inner()));
        let utf8 = interp.try_convert_mut("Artichoke").unwrap();
        assert_eq!(Ruby::String, types::ruby_from_mrb_value(utf8.inner()));
        let binary = interp.try_convert_mut(vec![0xFF_u8, 0x00, 0xFE]).unwrap();
        assert_eq!(Ruby::String, types::ruby_from_mrb_value(binary.inner()));
    }

    #[test]
    fn parse_array_ruby_type() {
        let mut interp = interpreter();
        let empty = interp.eval(b"[]").unwrap();
        assert_eq!(Ruby::Array, types::ruby_from_mrb_value(empty.inner()));
        #[cfg(feature = "core-regexp")]
        {
            let array = interp.eval(b"[1, /./, Object.new]").unwrap();
            assert_eq!(Ruby::Array, types::ruby_from_mrb_value(array.inner()));
        }
        let ary = vec!["a", "b", "c"];
        let converted = interp.try_convert_mut(ary).unwrap();
        assert_eq!(Ruby::Array, types::ruby_from_mrb_value(converted.inner()));
    }

    #[test]
    fn parse_hash_ruby_type() {
        let mut interp = interpreter();
        let empty = interp.eval(b"{}").unwrap();
        assert_eq!(Ruby::Hash, types::ruby_from_mrb_value(empty.inner()));
        #[cfg(feature = "core-regexp")]
        {
            let hash = interp.eval(b"{a: 1, b: [/./]}").unwrap();
            assert_eq!(Ruby::Hash, types::ruby_from_mrb_value(hash.inner()));
        }
        let mut map = HashMap::default();
        map.insert(b"a".to_vec(), vec![0_u8]);
        map.insert(b"b".to_vec(), b"binary".to_vec());
        let converted = interp.try_convert_mut(map).unwrap();
        assert_eq!(Ruby::Hash, types::ruby_from_mrb_value(converted.inner()));
    }

    #[test]
    fn parse_range_ruby_type() {
        let mut interp = interpreter();
        let dot2 = interp.eval(b"0..0").unwrap();
        assert_eq!(Ruby::Range, types::ruby_from_mrb_value(dot2.inner()));
        let dot3 = interp.eval(b"0...0").unwrap();
        assert_eq!(Ruby::Range, types::ruby_from_mrb_value(dot3.inner()));
    }

    #[test]
    fn parse_exception_ruby_type() {
        let mut interp = interpreter();
        let root = interp.eval(b"Exception.new").unwrap();
        assert_eq!(Ruby::Exception, types::ruby_from_mrb_value(root.inner()));
        let stderror = interp.eval(b"StandardError.new").unwrap();
        assert_eq!(Ruby::Exception, types::ruby_from_mrb_value(stderror.inner()));
        let index = interp.eval(b"IndexError.new").unwrap();
        assert_eq!(Ruby::Exception, types::ruby_from_mrb_value(index.inner()));
        #[cfg(feature = "core-math")]
        {
            let domain = interp.eval(b"Math::DomainError.new").unwrap();
            assert_eq!(Ruby::Exception, types::ruby_from_mrb_value(domain.inner()));
        }
    }
}
