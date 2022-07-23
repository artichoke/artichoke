use core::convert::TryFrom;
use core::fmt::Write as _;
use core::hash::{BuildHasher, Hash, Hasher};
use core::str;

use artichoke_core::hash::Hash as _;
use artichoke_core::value::Value as _;
use bstr::ByteSlice;

use crate::convert::implicitly_convert_to_int;
use crate::convert::implicitly_convert_to_nilable_string;
use crate::convert::implicitly_convert_to_string;
use crate::extn::core::array::Array;
#[cfg(feature = "core-regexp")]
use crate::extn::core::matchdata::{self, MatchData};
#[cfg(feature = "core-regexp")]
use crate::extn::core::regexp::{self, Regexp};
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;
use crate::sys::protect;

use super::Encoding;

pub fn mul(interp: &mut Artichoke, mut value: Value, count: Value) -> Result<Value, Error> {
    let count = implicitly_convert_to_int(interp, count)?;
    let count = usize::try_from(count).map_err(|_| ArgumentError::with_message("negative argument"))?;

    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // This guard ensures `repeat` below does not panic on `usize` overflow.
    if count.checked_mul(s.len()).is_none() {
        return Err(RangeError::with_message("bignum too big to convert into `long'").into());
    }
    let repeated_s = s.as_slice().repeat(count).into();
    super::String::alloc_value(repeated_s, interp)
}

pub fn add(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The borrowed byte slice is immediately `memcpy`'d into the `s` byte
    // buffer. There are no intervening interpreter accesses.
    let to_append = unsafe { implicitly_convert_to_string(interp, &mut other)? };

    let mut concatenated = s.clone();
    // XXX: This call doesn't do a check to see if we'll exceed the max allocation
    //    size and may panic or abort.
    concatenated.extend_from_slice(to_append);
    super::String::alloc_value(concatenated, interp)
}

pub fn push(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    if value.is_frozen(interp) {
        let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
        let message = "can't modify frozen String: "
            .chars()
            .chain(s.inspect())
            .collect::<super::String>();
        return Err(FrozenError::from(message.into_vec()).into());
    }

    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Ok(int) = other.try_convert_into::<i64>(interp) {
        return match s.encoding() {
            Encoding::Utf8 => {
                // Safety:
                //
                // The string is reboxed before any intervening operations on the
                // interpreter.
                // The string is reboxed without any intervening mruby allocations.
                unsafe {
                    let string_mut = s.as_inner_mut();
                    // XXX: This call doesn't do a check to see if we'll exceed the max allocation
                    //    size and may panic or abort.
                    string_mut
                        .try_push_codepoint(int)
                        .map_err(|err| RangeError::from(err.message()))?;
                    let s = s.take();
                    super::String::box_into_value(s, value, interp)
                }
            }
            Encoding::Ascii => {
                let byte = u8::try_from(int).map_err(|_| RangeError::from(format!("{int} out of char range")))?;
                // Safety:
                //
                // The string is reboxed before any intervening operations on the
                // interpreter.
                // The string is reboxed without any intervening mruby allocations.
                unsafe {
                    let string_mut = s.as_inner_mut();
                    // XXX: This call doesn't do a check to see if we'll exceed the max allocation
                    //    size and may panic or abort.
                    string_mut.push_byte(byte);
                    if !byte.is_ascii() {
                        string_mut.set_encoding(Encoding::Binary);
                    }
                    let s = s.take();
                    super::String::box_into_value(s, value, interp)
                }
            }
            Encoding::Binary => {
                let byte = u8::try_from(int).map_err(|_| RangeError::from(format!("{int} out of char range")))?;
                // Safety:
                //
                // The string is reboxed before any intervening operations on the
                // interpreter.
                // The string is reboxed without any intervening mruby allocations.
                unsafe {
                    let string_mut = s.as_inner_mut();
                    // XXX: This call doesn't do a check to see if we'll exceed the max allocation
                    //    size and may panic or abort.
                    string_mut.push_byte(byte);
                    let s = s.take();
                    super::String::box_into_value(s, value, interp)
                }
            }
        };
    }
    // Safety:
    //
    // The byte slice is immediately used and discarded after extraction. There
    // are no intervening interpreter accesses.
    let other = unsafe { implicitly_convert_to_string(interp, &mut other)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
        // XXX: This call doesn't do a check to see if we'll exceed the max allocation
        //    size and may panic.
        string_mut.extend_from_slice(other);

        let s = s.take();
        super::String::box_into_value(s, value, interp)
    }
}

pub fn cmp_rocket(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { super::String::unbox_from_value(&mut other, interp) } {
        let cmp = s.cmp(&*other);
        Ok(interp.convert(cmp as i64))
    } else {
        Ok(Value::nil())
    }
}

pub fn equals_equals(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { super::String::unbox_from_value(&mut other, interp) } {
        let equals = *s == *other;
        return Ok(interp.convert(equals));
    }
    // Safety:
    //
    // The byte slice is immediately discarded after extraction. There are no
    // intervening interpreter accesses.
    if value.respond_to(interp, "to_str")? {
        let result = other.funcall(interp, "==", &[value], None)?;
        // any falsy returned value yields `false`, otherwise `true`.
        if let Ok(result) = TryConvert::<_, Option<bool>>::try_convert(interp, result) {
            let result = result.unwrap_or_default();
            Ok(interp.convert(result))
        } else {
            Ok(interp.convert(true))
        }
    } else {
        Ok(interp.convert(false))
    }
}

#[allow(unused_mut)]
pub fn aref(
    interp: &mut Artichoke,
    mut value: Value,
    mut first: Value,
    second: Option<Value>,
) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Some(second) = second {
        #[cfg(feature = "core-regexp")]
        if let Ok(regexp) = unsafe { Regexp::unbox_from_value(&mut first, interp) } {
            let match_data = regexp.match_(interp, Some(s.as_slice()), None, None)?;
            if match_data.is_nil() {
                return Ok(Value::nil());
            }
            return matchdata::trampoline::element_reference(interp, match_data, second, None);
        }
        let index = implicitly_convert_to_int(interp, first)?;
        let length = implicitly_convert_to_int(interp, second)?;

        // ```
        // [3.0.1] > s = "abc"
        // => "abc"
        // [3.0.1] > s[-2, 10]
        // => "bc"
        // [3.0.1] > s[-3, 10]
        // => "abc"
        // [3.0.1] > s[-4, 10]
        // => nil
        // ```
        let index = if let Ok(index) = usize::try_from(index) {
            Some(index)
        } else {
            index
                .checked_neg()
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| s.len().checked_sub(index))
        };
        let index = match index {
            None => return Ok(Value::nil()),
            // Short circuit with `nil` if `index > len`.
            //
            // ```
            // [3.0.1] > s = "abc"
            // => "abc"
            // [3.0.1] > s[3, 10]
            // => ""
            // [3.0.1] > s[4, 10]
            // => nil
            // ```
            //
            // Don't specialize on the case where `index == len` because the provided
            // length can change the result. Even if the length argument is not
            // given, we still need to preserve the encoding of the source string,
            // so fall through to the happy path below.
            Some(index) if index > s.len() => return Ok(Value::nil()),
            Some(index) => index,
        };

        // ```
        // [3.0.1] > s = "abc"
        // => "abc"
        // [3.0.1] > s[1, -1]
        // => nil
        // ```
        if let Ok(length) = usize::try_from(length) {
            // ```
            // [3.0.1] > s = "abc"
            // => "abc"
            // [3.0.1] > s[2**64, 2**64]
            // (irb):26:in `[]': bignum too big to convert into `long' (RangeError)
            // 	from (irb):26:in `<main>'
            // 	from /usr/local/var/rbenv/versions/3.0.1/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
            // 	from /usr/local/var/rbenv/versions/3.0.1/bin/irb:23:in `load'
            // 	from /usr/local/var/rbenv/versions/3.0.1/bin/irb:23:in `<main>'
            // ```
            let end = index
                .checked_add(length)
                .ok_or_else(|| RangeError::with_message("bignum too big to convert into `long'"))?;
            if let Some(slice) = s.get_char_slice(index..end) {
                // Encoding from the source string is preserved.
                //
                // ```
                // [3.0.1] > s = "abc"
                // => "abc"
                // [3.0.1] > s.encoding
                // => #<Encoding:UTF-8>
                // [3.0.1] > s[1, 2].encoding
                // => #<Encoding:UTF-8>
                // [3.0.1] > t = s.force_encoding(Encoding::ASCII)
                // => "abc"
                // [3.0.1] > t[1, 2].encoding
                // => #<Encoding:US-ASCII>
                // ```
                let s = super::String::with_bytes_and_encoding(slice.to_vec(), s.encoding());
                // ```
                // [3.0.1] > class S < String; end
                // => nil
                // [3.0.1] > S.new("abc")[1, 2].class
                // => String
                // ```
                //
                // The returned `String` is never frozen:
                //
                // ```
                // [3.0.1] > s = "abc"
                // => "abc"
                // [3.0.1] > s.frozen?
                // => false
                // [3.0.1] > s[1, 2].frozen?
                // => false
                // [3.0.1] > t = "abc".freeze
                // => "abc"
                // [3.0.1] > t[1, 2].frozen?
                // => false
                // ```
                return super::String::alloc_value(s, interp);
            }
        }
        return Ok(Value::nil());
    }
    #[cfg(feature = "core-regexp")]
    if let Ok(regexp) = unsafe { Regexp::unbox_from_value(&mut first, interp) } {
        let match_data = regexp.match_(interp, Some(s.as_slice()), None, None)?;
        if match_data.is_nil() {
            return Ok(Value::nil());
        }
        return matchdata::trampoline::element_reference(interp, match_data, interp.convert(0), None);
    }
    match first.is_range(interp, s.char_len() as i64)? {
        None => {}
        // ```
        // [3.1.2] > ""[-1..-1]
        // => nil
        // ``
        Some(protect::Range::Out) => return Ok(Value::nil()),
        Some(protect::Range::Valid { start: index, len }) => {
            let index = if let Ok(index) = usize::try_from(index) {
                Some(index)
            } else {
                index
                    .checked_neg()
                    .and_then(|index| usize::try_from(index).ok())
                    .and_then(|index| s.len().checked_sub(index))
            };
            let index = match index {
                None => return Ok(Value::nil()),
                Some(index) if index > s.len() => return Ok(Value::nil()),
                Some(index) => index,
            };
            if let Ok(length) = usize::try_from(len) {
                let end = index
                    .checked_add(length)
                    .ok_or_else(|| RangeError::with_message("bignum too big to convert into `long'"))?;
                if let Some(slice) = s.get_char_slice(index..end) {
                    let s = super::String::with_bytes_and_encoding(slice.to_vec(), s.encoding());
                    return super::String::alloc_value(s, interp);
                }
            }
            return Ok(Value::nil());
        }
    }
    // The overload of `String#[]` that takes a `String` **only** takes `String`s.
    // No implicit conversion is performed.
    //
    // ```
    // [3.0.1] > s = "abc"
    // => "abc"
    // [3.0.1] > s["bc"]
    // => "bc"
    // [3.0.1] > class X; def to_str; "bc"; end; end
    // => :to_str
    // [3.0.1] > s[X.new]
    // (irb):4:in `[]': no implicit conversion of X into Integer (TypeError)
    // 	from (irb):4:in `<main>'
    // 	from /usr/local/var/rbenv/versions/3.0.1/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
    // 	from /usr/local/var/rbenv/versions/3.0.1/bin/irb:23:in `load'
    // 	from /usr/local/var/rbenv/versions/3.0.1/bin/irb:23:in `<main>'
    // 	```
    if let Ok(substring) = unsafe { super::String::unbox_from_value(&mut first, interp) } {
        if s.index(&*substring, None).is_some() {
            // Indexing with a `String` returns a newly allocated object that
            // has the same encoding as the index, regardless of the encoding on
            // the receiver.
            //
            // ```
            // [3.0.2] > s = "abc"
            // => "abc"
            // [3.0.2] > s.encoding
            // => #<Encoding:UTF-8>
            // [3.0.2] > s["bc"].encoding
            // => #<Encoding:UTF-8>
            // [3.0.2] > t = s.force_encoding(Encoding::ASCII_8BIT)
            // => "abc"
            // [3.0.2] > t.encoding
            // => #<Encoding:ASCII-8BIT>
            // [3.0.2] > t["bc"].encoding
            // => #<Encoding:UTF-8>
            // [3.0.2] > x = "bc"
            // => "bc"
            // [3.0.2] > x.encoding
            // => #<Encoding:UTF-8>
            // [3.0.2] > x.object_id
            // => 260
            // [3.0.2] > y = t[x]
            // => "bc"
            // [3.0.2] > y.encoding
            // => #<Encoding:UTF-8>
            // [3.0.2] > y.object_id
            // => 280
            // [3.0.2] > z = "bc".force_encoding(Encoding::ASCII)
            // => "bc"
            // [3.0.2] > y[z].encoding
            // => #<Encoding:US-ASCII>
            // [3.0.2] > t[z].encoding
            // => #<Encoding:US-ASCII>
            // [3.0.2] > s[z].encoding
            // => #<Encoding:US-ASCII>
            // ```
            let encoding = substring.encoding();
            let s = super::String::with_bytes_and_encoding(substring.to_vec(), encoding);
            return super::String::alloc_value(s, interp);
        }
        return Ok(Value::nil());
    }
    let index = implicitly_convert_to_int(interp, first)?;

    // ```
    // [3.0.1] > s = "abc"
    // => "abc"
    // [3.0.1] > s[-2]
    // => "b"
    // [3.0.1] > s[-3]
    // => "a"
    // [3.0.1] > s[-4]
    // => nil
    // ```
    let index = if let Ok(index) = usize::try_from(index) {
        Some(index)
    } else {
        index
            .checked_neg()
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| s.len().checked_sub(index))
    };
    if let Some(index) = index {
        // Index the byte, non existent indexes return `nil`.
        //
        // ```
        // [3.0.1] > s = "abc"
        // => "abc"
        // [3.0.1] > s[2]
        // => "c"
        // [3.0.1] > s[3]
        // => nil
        // [3.0.1] > s[4]
        // => nil
        // ```
        if let Some(bytes) = s.get_char(index) {
            let s = super::String::with_bytes_and_encoding(bytes.to_vec(), s.encoding());
            return super::String::alloc_value(s, interp);
        }
    }
    Ok(Value::nil())
}

pub fn aset(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    if value.is_frozen(interp) {
        let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
        let message = "can't modify frozen String: "
            .chars()
            .chain(s.inspect())
            .collect::<super::String>();
        return Err(FrozenError::from(message.into_vec()).into());
    }

    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn is_ascii_only(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let is_ascii_only = s.is_ascii_only();
    Ok(interp.convert(is_ascii_only))
}

pub fn b(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    super::String::alloc_value(s.to_binary(), interp)
}

pub fn bytes(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let bytes = s
        .bytes()
        .map(i64::from)
        .map(|byte| interp.convert(byte))
        .collect::<Array>();
    Array::alloc_value(bytes, interp)
}

pub fn bytesize(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let bytesize = s.bytesize();
    interp.try_convert(bytesize)
}

pub fn byteslice(
    interp: &mut Artichoke,
    mut value: Value,
    index: Value,
    length: Option<Value>,
) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let maybe_range = if length.is_none() {
        index.is_range(interp, s.bytesize() as i64)?
    } else {
        None
    };
    match maybe_range {
        None => {}
        Some(protect::Range::Out) => return Ok(Value::nil()),
        Some(protect::Range::Valid { start: index, len }) => {
            let index = if let Ok(index) = usize::try_from(index) {
                Some(index)
            } else {
                index
                    .checked_neg()
                    .and_then(|index| usize::try_from(index).ok())
                    .and_then(|index| s.len().checked_sub(index))
            };
            let index = match index {
                None => return Ok(Value::nil()),
                Some(index) if index > s.len() => return Ok(Value::nil()),
                Some(index) => index,
            };
            if let Ok(length) = usize::try_from(len) {
                let end = index
                    .checked_add(length)
                    .ok_or_else(|| RangeError::with_message("bignum too big to convert into `long'"))?;
                if let Some(slice) = s.get(index..end).or_else(|| s.get(index..)) {
                    // Encoding from the source string is preserved.
                    //
                    // ```
                    // [3.1.2] > s = "abc"
                    // => "abc"
                    // [3.1.2] > s.encoding
                    // => #<Encoding:UTF-8>
                    // [3.1.2] > s.byteslice(1..3).encoding
                    // => #<Encoding:UTF-8>
                    // [3.1.2] > t = s.force_encoding(Encoding::ASCII)
                    // => "abc"
                    // [3.1.2] > t.byteslice(1..3).encoding
                    // => #<Encoding:US-ASCII>
                    // ```
                    let s = super::String::with_bytes_and_encoding(slice.to_vec(), s.encoding());
                    // ```
                    // [3.1.2] > class S < String; end
                    // => nil
                    // [3.1.2] > S.new("abc").byteslice(1..3).class
                    // => String
                    // ```
                    //
                    // The returned `String` is never frozen:
                    //
                    // ```
                    // [3.1.2] > s = "abc"
                    // => "abc"
                    // [3.1.2] > s.frozen?
                    // => false
                    // [3.1.2] > s.byteslice(1..3).frozen?
                    // => false
                    // [3.1.2] > t = "abc".freeze
                    // => "abc"
                    // [3.1.2] > t.byteslice(1..3).frozen?
                    // => false
                    // ```
                    return super::String::alloc_value(s, interp);
                }
            }
        }
    }
    // ```
    // [3.0.2] > class A; def to_int; 1; end; end
    // => :to_int
    // [3.0.2] > s = "abc"
    // => "abc"
    // [3.0.2] > s.byteslice(A.new)
    // => "b"
    // [3.0.2] > s.byteslice(//)
    // (irb):16:in `byteslice': no implicit conversion of Regexp into Integer (TypeError)
    // 	from (irb):16:in `<main>'
    // 	from /usr/local/var/rbenv/versions/3.0.2/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
    // 	from /usr/local/var/rbenv/versions/3.0.2/bin/irb:23:in `load'
    // 	from /usr/local/var/rbenv/versions/3.0.2/bin/irb:23:in `<main>'
    // ```
    let index = implicitly_convert_to_int(interp, index)?;

    // ```
    // [3.0.2] > s = "abc"
    // => "abc"
    // [3.0.2] > s.byteslice(-2, 10)
    // => "bc"
    // [3.0.2] > s.byteslice(-3, 10)
    // => "abc"
    // [3.0.2] > s.byteslice(-4, 10)
    // => nil
    // ```
    let index = if let Ok(index) = usize::try_from(index) {
        Some(index)
    } else {
        index
            .checked_neg()
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| s.len().checked_sub(index))
    };
    let index = match index {
        None => return Ok(Value::nil()),
        // Short circuit with `nil` if `index > len`.
        //
        // ```
        // [3.0.1] > s = "abc"
        // => "abc"
        // [3.0.1] > s.byteslice(3, 10)
        // => ""
        // [3.0.1] > s.byteslice(4, 10)
        // => nil
        // ```
        //
        // Don't specialize on the case where `index == len` because the provided
        // length can change the result. Even if the length argument is not
        // given, we still need to preserve the encoding of the source string,
        // so fall through to the happy path below.
        Some(index) if index > s.len() => return Ok(Value::nil()),
        Some(index) => index,
    };

    let length = if let Some(length) = length {
        length
    } else {
        // Per the docs -- https://ruby-doc.org/core-3.0.2/String.html#method-i-byteslice
        //
        // > If passed a single Integer, returns a substring of one byte at that position.
        //
        // NOTE: Index out a single byte rather than a slice to avoid having
        // to do an overflow check on the addition.
        if let Some(&byte) = s.get(index) {
            let s = super::String::with_bytes_and_encoding(vec![byte], s.encoding());
            // ```
            // [3.0.1] > class S < String; end
            // => nil
            // [3.0.1] > S.new("abc").byteslice(1, 2).class
            // => String
            // ```
            //
            // The returned `String` is never frozen:
            //
            // ```
            // [3.0.1] > s = "abc"
            // => "abc"
            // [3.0.1] > s.frozen?
            // => false
            // [3.0.1] > s.byteslice(1, 2).frozen?
            // => false
            // [3.0.1] > t = "abc".freeze
            // => "abc"
            // [3.0.1] > t.byteslice(1, 2).frozen?
            // => false
            // ```
            return super::String::alloc_value(s, interp);
        }
        return Ok(Value::nil());
    };

    // ```
    // [3.0.2] > class A; def to_int; 1; end; end
    // => :to_int
    // [3.0.2] > s = "abc"
    // => "abc"
    // [3.0.2] > s.byteslice(A.new)
    // => "b"
    // [3.0.2] > s.byteslice(A.new, A.new)
    // => "b"
    // [3.0.2] > s.byteslice(2, //)
    // (irb):17:in `byteslice': no implicit conversion of Regexp into Integer (TypeError)
    // 	from (irb):17:in `<main>'
    // 	from /usr/local/var/rbenv/versions/3.0.2/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
    // 	from /usr/local/var/rbenv/versions/3.0.2/bin/irb:23:in `load'
    // 	from /usr/local/var/rbenv/versions/3.0.2/bin/irb:23:in `<main>
    // ```
    let length = implicitly_convert_to_int(interp, length)?;

    // ```
    // [3.0.2] > s = "abc"
    // => "abc"
    // [3.0.2] > s.byteslice(1, -1)
    // => nil
    // ```
    if let Ok(length) = usize::try_from(length) {
        // ```
        // [3.0.2] > s = "abc"
        // => "abc"
        // [3.0.2] > s.byteslice(2**64, 2**64)
        // (irb):38:in `byteslice': bignum too big to convert into `long' (RangeError)
        // 	from (irb):38:in `<main>'
        // 	from /usr/local/var/rbenv/versions/3.0.2/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
        // 	from /usr/local/var/rbenv/versions/3.0.2/bin/irb:23:in `load'
        // 	from /usr/local/var/rbenv/versions/3.0.2/bin/irb:23:in `<main>'
        // ```
        let end = index
            .checked_add(length)
            .ok_or_else(|| RangeError::with_message("bignum too big to convert into `long'"))?;
        if let Some(slice) = s.get(index..end).or_else(|| s.get(index..)) {
            // Encoding from the source string is preserved.
            //
            // ```
            // [3.0.2] > s = "abc"
            // => "abc"
            // [3.0.2] > s.encoding
            // => #<Encoding:UTF-8>
            // [3.0.2] > s.byteslice(1, 2).encoding
            // => #<Encoding:UTF-8>
            // [3.0.2] > t = s.force_encoding(Encoding::ASCII)
            // => "abc"
            // [3.0.2] > t.byteslice(1, 2).encoding
            // => #<Encoding:US-ASCII>
            // ```
            let s = super::String::with_bytes_and_encoding(slice.to_vec(), s.encoding());
            // ```
            // [3.0.1] > class S < String; end
            // => nil
            // [3.0.1] > S.new("abc").byteslice(1, 2).class
            // => String
            // ```
            //
            // The returned `String` is never frozen:
            //
            // ```
            // [3.0.1] > s = "abc"
            // => "abc"
            // [3.0.1] > s.frozen?
            // => false
            // [3.0.1] > s.byteslice(1, 2).frozen?
            // => false
            // [3.0.1] > t = "abc".freeze
            // => "abc"
            // [3.0.1] > t.byteslice(1, 2).frozen?
            // => false
            // ```
            return super::String::alloc_value(s, interp);
        }
    }
    Ok(Value::nil())
}

pub fn capitalize(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut dup = s.clone();
    dup.make_capitalized();
    super::String::alloc_value(dup, interp)
}

pub fn capitalize_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    if value.is_frozen(interp) {
        let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
        let message = "can't modify frozen String: "
            .chars()
            .chain(s.inspect())
            .collect::<super::String>();
        return Err(FrozenError::from(message.into_vec()).into());
    }

    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
        // `make_capitalized` might reallocate the string and invalidate the
        // boxed pointer, capacity, length triple.
        string_mut.make_capitalized();

        let s = s.take();
        super::String::box_into_value(s, value, interp)
    }
}

pub fn casecmp_ascii(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The byte slice is immediately discarded after extraction. There are no
    // intervening interpreter accesses.
    if let Ok(other) = unsafe { implicitly_convert_to_string(interp, &mut other) } {
        let cmp = s.ascii_casecmp(other) as i64;
        Ok(interp.convert(cmp))
    } else {
        Ok(Value::nil())
    }
}

pub fn casecmp_unicode(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // TODO: this needs to do an implicit conversion, but we need a Spinoso string.
    if let Ok(other) = unsafe { super::String::unbox_from_value(&mut other, interp) } {
        let eql = *s == *other;
        Ok(interp.convert(eql))
    } else {
        Ok(interp.convert(false))
    }
}

pub fn center(interp: &mut Artichoke, mut value: Value, width: Value, padstr: Option<Value>) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let width = implicitly_convert_to_int(interp, width)?;
    let width = if let Ok(width) = usize::try_from(width) {
        width
    } else {
        // ```ruby
        // [3.0.2] > "a".center(-1)
        // => "a"
        // [3.0.2] > "a".center(-10)
        // => "a"
        // [3.0.2] > x = "a"
        // => "a"
        // [3.0.2] > y = "a".center(-10)
        // => "a"
        // [3.0.2] > x.object_id == y.object_id
        // => false
        // ```
        let dup = s.clone();
        return super::String::alloc_value(dup, interp);
    };
    // Safety:
    //
    // The byte slice is immediately discarded after extraction and turned into
    // an owned value. There are no intervening interpreter accesses.
    let padstr = if let Some(mut padstr) = padstr {
        let padstr = unsafe { implicitly_convert_to_string(interp, &mut padstr)? };
        Some(padstr.to_vec())
    } else {
        None
    };
    let centered = s
        .center(width, padstr.as_deref())
        .map_err(|e| ArgumentError::with_message(e.message()))?
        .collect::<super::String>();
    super::String::alloc_value(centered, interp)
}

pub fn chars(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let chars = s.chars().collect::<Vec<&[u8]>>();
    interp.try_convert_mut(chars)
}

pub fn chomp(interp: &mut Artichoke, mut value: Value, separator: Option<Value>) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut dup = s.clone();
    if let Some(mut separator) = separator {
        if let Some(sep) = unsafe { implicitly_convert_to_nilable_string(interp, &mut separator)? } {
            let _ = dup.chomp(Some(sep));
        } else {
            return interp.try_convert_mut("");
        }
    } else {
        let _ = dup.chomp(None::<&[u8]>);
    }
    super::String::alloc_value(dup, interp)
}

pub fn chomp_bang(interp: &mut Artichoke, mut value: Value, separator: Option<Value>) -> Result<Value, Error> {
    if value.is_frozen(interp) {
        let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
        let message = "can't modify frozen String: "
            .chars()
            .chain(s.inspect())
            .collect::<super::String>();
        return Err(FrozenError::from(message.into_vec()).into());
    }

    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    unsafe {
        let string_mut = s.as_inner_mut();
        let modified = if let Some(mut separator) = separator {
            if let Some(sep) = implicitly_convert_to_nilable_string(interp, &mut separator)? {
                string_mut.chomp(Some(sep))
            } else {
                return Ok(Value::nil());
            }
        } else {
            string_mut.chomp(None::<&[u8]>)
        };
        if modified {
            let s = s.take();
            return super::String::box_into_value(s, value, interp);
        }
    }
    Ok(Value::nil())
}

pub fn chop(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut dup = s.clone();
    let _ = dup.chop();
    super::String::alloc_value(dup, interp)
}

pub fn chop_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    if value.is_frozen(interp) {
        let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
        let message = "can't modify frozen String: "
            .chars()
            .chain(s.inspect())
            .collect::<super::String>();
        return Err(FrozenError::from(message.into_vec()).into());
    }

    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if s.is_empty() {
        return Ok(Value::nil());
    }
    unsafe {
        let string_mut = s.as_inner_mut();
        let modified = string_mut.chop();
        if modified {
            let s = s.take();
            return super::String::box_into_value(s, value, interp);
        }
    }
    Ok(value)
}

pub fn chr(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let chr = s.chr();
    interp.try_convert_mut(chr)
}

pub fn clear(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    if value.is_frozen(interp) {
        let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
        let message = "can't modify frozen String: "
            .chars()
            .chain(s.inspect())
            .collect::<super::String>();
        return Err(FrozenError::from(message.into_vec()).into());
    }

    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
        string_mut.clear();

        let s = s.take();
        super::String::box_into_value(s, value, interp)
    }
}

pub fn codepoints(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let codepoints = s
        .codepoints()
        .map_err(|err| ArgumentError::with_message(err.message()))?;
    let codepoints = codepoints.map(|ch| interp.convert(ch)).collect::<Array>();
    Array::alloc_value(codepoints, interp)
}

pub fn concat(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn downcase(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut dup = s.clone();
    dup.make_lowercase();
    super::String::alloc_value(dup, interp)
}

pub fn downcase_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
        // `make_lowercase` might reallocate the string and invalidate the
        // boxed pointer, capacity, length triple.
        string_mut.make_lowercase();

        let s = s.take();
        super::String::box_into_value(s, value, interp)
    }
}

pub fn is_empty(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Ok(interp.convert(s.is_empty()))
}

pub fn eql(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Ok(other) = unsafe { super::String::unbox_from_value(&mut other, interp) } {
        let eql = *s == *other;
        Ok(interp.convert(eql))
    } else {
        Ok(interp.convert(false))
    }
}

pub fn getbyte(interp: &mut Artichoke, mut value: Value, index: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let index = implicitly_convert_to_int(interp, index)?;
    let index = if let Ok(index) = usize::try_from(index) {
        index
    } else {
        let index = index
            .checked_neg()
            .ok_or_else(|| RangeError::with_message("bignum too big to convert into `long'"))?;
        let index = usize::try_from(index).ok().and_then(|index| s.len().checked_sub(index));
        if let Some(index) = index {
            index
        } else {
            return Ok(Value::nil());
        }
    };
    let byte = s.get(index).copied().map(i64::from);
    Ok(interp.convert(byte))
}

pub fn hash(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut hasher = interp.global_build_hasher()?.build_hasher();
    s.as_slice().hash(&mut hasher);
    #[allow(clippy::cast_possible_wrap)]
    let hash = hasher.finish() as i64;
    Ok(interp.convert(hash))
}

pub fn include(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let other_str = unsafe { implicitly_convert_to_string(interp, &mut other)? };
    let includes = s.index(other_str, None).is_some();
    Ok(interp.convert(includes))
}

pub fn index(
    interp: &mut Artichoke,
    mut value: Value,
    mut needle: Value,
    offset: Option<Value>,
) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    #[cfg(feature = "core-regexp")]
    if let Ok(_pattern) = unsafe { Regexp::unbox_from_value(&mut needle, interp) } {
        return Err(NotImplementedError::from("String#index with Regexp pattern").into());
    }
    let needle = unsafe { implicitly_convert_to_string(interp, &mut needle)? };
    let index = if let Some(offset) = offset {
        let offset = implicitly_convert_to_int(interp, offset)?;
        let offset = if let Ok(offset) = usize::try_from(offset) {
            Some(offset)
        } else {
            offset
                .checked_neg()
                .and_then(|offset| usize::try_from(offset).ok())
                .and_then(|offset| s.len().checked_sub(offset))
        };
        s.index(needle, offset)
    } else {
        s.index(needle, None)
    };
    let index = index.and_then(|index| i64::try_from(index).ok());
    interp.try_convert(index)
}

pub fn initialize(interp: &mut Artichoke, mut value: Value, from: Option<Value>) -> Result<Value, Error> {
    // We must convert `from` to a byte buffer first in case `#to_str` raises.
    //
    // If we don't, the following scenario could leave `value` with a dangling
    // pointer.
    //
    // ```console
    // [3.0.2] > s = "abc"
    // => "abc"
    // [3.0.2] > class B; def to_str; raise 'oh no'; end; end
    // => :to_str
    // [3.0.2] > s.send(:initialize, B.new)
    // (irb):6:in `to_str': oh no (RuntimeError)
    // 	from (irb):7:in `initialize'
    // 	from (irb):7:in `<main>'
    // 	from /usr/local/var/rbenv/versions/3.0.2/lib/ruby/gems/3.0.0/gems/irb-1.3.5/exe/irb:11:in `<top (required)>'
    // 	from /usr/local/var/rbenv/versions/3.0.2/bin/irb:23:in `load'
    // 	from /usr/local/var/rbenv/versions/3.0.2/bin/irb:23:in `<main>'
    // [3.0.2] > s
    // => "abc"
    // ```
    let buf = if let Some(mut from) = from {
        // Safety:
        //
        // The extracted slice is immediately copied to an owned buffer.
        //
        // No intervening operations on the mruby VM occur.
        let from = unsafe { implicitly_convert_to_string(interp, &mut from)? };
        from.to_vec()
    } else {
        Vec::new()
    };

    // If we are calling `initialize` on an already initialized `String`,
    // pluck out the inner buffer and drop it so we don't leak memory.
    //
    // ```console
    // [3.0.2] > s = "abc"
    // => "abc"
    // [3.0.2] > class A; def to_str; 'foo'; end; end
    // => :to_str
    // [3.0.2] > s.send(:initialize, A.new)
    // => "foo"
    // [3.0.2] > s
    // => "foo"
    // ```
    if let Ok(s) = unsafe { super::String::unbox_from_value(&mut value, interp) } {
        unsafe {
            let inner = s.take();
            drop(inner);
        }
    }
    // We are no guaranteed that the `buf` pointer in `value` is either dangling
    // or uninitialized. We will ensure that we box the given bytes into it.
    let buf = super::String::from(buf);
    super::String::box_into_value(buf, value, interp)
}

pub fn initialize_copy(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    // Safety:
    //
    // The extracted slice is immediately copied to an owned buffer.
    //
    // No intervening operations on the mruby VM occur.
    let buf = unsafe {
        let from = implicitly_convert_to_string(interp, &mut other)?;
        from.to_vec()
    };
    // If we are calling `initialize_copy` on an already initialized `String`,
    // pluck out the inner buffer and drop it so we don't leak memory.
    if let Ok(s) = unsafe { super::String::unbox_from_value(&mut value, interp) } {
        unsafe {
            let inner = s.take();
            drop(inner);
        }
    }
    // XXX: This should use the encoding of the given `other`.
    let replacement = super::String::with_bytes_and_encoding(buf, super::Encoding::Utf8);
    super::String::box_into_value(replacement, value, interp)
}

pub fn inspect(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let inspect = s.inspect().collect::<super::String>();
    super::String::alloc_value(inspect, interp)
}

pub fn intern(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let bytes = s.as_slice();
    let sym = if let Some(sym) = interp.check_interned_bytes(bytes)? {
        sym
    } else {
        interp.intern_bytes(bytes.to_vec())?
    };
    Symbol::alloc_value(sym.into(), interp)
}

pub fn length(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let length = s.char_len();
    interp.try_convert(length)
}

pub fn ord(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let ord = s.ord().map_err(|err| ArgumentError::with_message(err.message()))?;
    Ok(interp.convert(ord))
}

pub fn replace(interp: &mut Artichoke, value: Value, other: Value) -> Result<Value, Error> {
    initialize_copy(interp, value, other)
}

pub fn reverse(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut reversed = s.clone();
    reversed.reverse();
    super::String::alloc_value(reversed, interp)
}

pub fn reverse_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    if value.is_frozen(interp) {
        let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
        let message = "can't modify frozen String: "
            .chars()
            .chain(s.inspect())
            .collect::<super::String>();
        return Err(FrozenError::from(message.into_vec()).into());
    }

    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
        string_mut.reverse();

        let s = s.take();
        super::String::box_into_value(s, value, interp)
    }
}

pub fn rindex(
    interp: &mut Artichoke,
    mut value: Value,
    mut needle: Value,
    offset: Option<Value>,
) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    #[cfg(feature = "core-regexp")]
    if let Ok(_pattern) = unsafe { Regexp::unbox_from_value(&mut needle, interp) } {
        return Err(NotImplementedError::from("String#index with Regexp pattern").into());
    }
    let needle = unsafe { implicitly_convert_to_string(interp, &mut needle)? };
    let index = if let Some(offset) = offset {
        let offset = implicitly_convert_to_int(interp, offset)?;
        let offset = if let Ok(offset) = usize::try_from(offset) {
            Some(offset)
        } else {
            offset
                .checked_neg()
                .and_then(|offset| usize::try_from(offset).ok())
                .and_then(|offset| s.len().checked_sub(offset))
        };
        s.rindex(needle, offset)
    } else {
        s.rindex(needle, None)
    };
    let index = index.and_then(|index| i64::try_from(index).ok());
    interp.try_convert(index)
}

pub fn scan(interp: &mut Artichoke, value: Value, mut pattern: Value, block: Option<Block>) -> Result<Value, Error> {
    if let Ruby::Symbol = pattern.ruby_type() {
        let mut message = String::from("wrong argument type ");
        message.push_str(interp.inspect_type_name_for_value(pattern));
        message.push_str(" (expected Regexp)");
        return Err(TypeError::from(message).into());
    }
    #[cfg(feature = "core-regexp")]
    if let Ok(regexp) = unsafe { Regexp::unbox_from_value(&mut pattern, interp) } {
        let haystack = value.try_convert_into_mut::<&[u8]>(interp)?;
        let scan = regexp.inner().scan(interp, haystack, block)?;
        return Ok(interp.try_convert_mut(scan)?.unwrap_or(value));
    }
    #[cfg(feature = "core-regexp")]
    // Safety:
    //
    // Convert `pattern_bytes` to an owned byte vec to ensure the underlying
    // `RString` is not garbage collected when yielding matches.
    if let Ok(pattern_bytes) = unsafe { implicitly_convert_to_string(interp, &mut pattern) } {
        let pattern_bytes = pattern_bytes.to_vec();

        let string = value.try_convert_into_mut::<&[u8]>(interp)?;
        if let Some(ref block) = block {
            let regex = Regexp::try_from(pattern_bytes.clone())?;
            let matchdata = MatchData::new(string.to_vec(), regex, ..);
            let patlen = pattern_bytes.len();
            if let Some(pos) = string.find(&pattern_bytes) {
                let mut data = matchdata.clone();
                data.set_region(pos..pos + patlen);
                let data = MatchData::alloc_value(data, interp)?;
                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let block_arg = interp.try_convert_mut(pattern_bytes.as_slice())?;
                block.yield_arg(interp, &block_arg)?;

                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let offset = pos + patlen;
                let string = string.get(offset..).unwrap_or_default();
                for pos in string.find_iter(&pattern_bytes) {
                    let mut data = matchdata.clone();
                    data.set_region(offset + pos..offset + pos + patlen);
                    let data = MatchData::alloc_value(data, interp)?;
                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                    let block_arg = interp.try_convert_mut(pattern_bytes.as_slice())?;
                    block.yield_arg(interp, &block_arg)?;

                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;
                }
            } else {
                interp.unset_global_variable(regexp::LAST_MATCH)?;
            }
            return Ok(value);
        }
        let (matches, last_pos) = string
            .find_iter(&pattern_bytes)
            .enumerate()
            .last()
            .map(|(m, p)| (m + 1, p))
            .unwrap_or_default();
        let mut result = Vec::with_capacity(matches);
        for _ in 0..matches {
            result.push(interp.try_convert_mut(pattern_bytes.as_slice())?);
        }
        if matches > 0 {
            let regex = Regexp::try_from(pattern_bytes.clone())?;
            let matchdata = MatchData::new(string.to_vec(), regex, last_pos..last_pos + pattern_bytes.len());
            let data = MatchData::alloc_value(matchdata, interp)?;
            interp.set_global_variable(regexp::LAST_MATCH, &data)?;
        } else {
            interp.unset_global_variable(regexp::LAST_MATCH)?;
        }
        return interp.try_convert_mut(result);
    }
    #[cfg(not(feature = "core-regexp"))]
    // Safety:
    //
    // Convert `pattern_bytes` to an owned byte vec to ensure the underlying
    // `RString` is not garbage collected when yielding matches.
    if let Ok(pattern_bytes) = unsafe { implicitly_convert_to_string(interp, &mut pattern) } {
        let pattern_bytes = pattern_bytes.to_vec();

        let string = value.try_convert_into_mut::<&[u8]>(interp)?;
        if let Some(ref block) = block {
            let patlen = pattern_bytes.len();
            if let Some(pos) = string.find(&pattern_bytes) {
                let block_arg = interp.try_convert_mut(pattern_bytes.as_slice())?;
                block.yield_arg(interp, &block_arg)?;

                let offset = pos + patlen;
                let string = string.get(offset..).unwrap_or_default();
                for _ in string.find_iter(&pattern_bytes) {
                    let block_arg = interp.try_convert_mut(pattern_bytes.as_slice())?;
                    block.yield_arg(interp, &block_arg)?;
                }
            }
            return Ok(value);
        }
        let matches = string
            .find_iter(&pattern_bytes)
            .enumerate()
            .last()
            .map(|(m, _)| m + 1)
            .unwrap_or_default();
        let mut result = Vec::with_capacity(matches);
        for _ in 0..matches {
            result.push(interp.try_convert_mut(pattern_bytes.as_slice())?);
        }
        return interp.try_convert_mut(result);
    }
    let mut message = String::from("wrong argument type ");
    message.push_str(interp.inspect_type_name_for_value(pattern));
    message.push_str(" (expected Regexp)");
    Err(TypeError::from(message).into())
}

pub fn setbyte(interp: &mut Artichoke, mut value: Value, index: Value, byte: Value) -> Result<Value, Error> {
    let index = implicitly_convert_to_int(interp, index)?;
    let i64_byte = implicitly_convert_to_int(interp, byte)?;

    if value.is_frozen(interp) {
        let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
        let message = "can't modify frozen String: "
            .chars()
            .chain(s.inspect())
            .collect::<super::String>();
        return Err(FrozenError::from(message.into_vec()).into());
    }

    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let index = if let Ok(index) = usize::try_from(index) {
        index
    } else {
        let idx = index
            .checked_neg()
            .ok_or_else(|| RangeError::with_message("bignum too big to convert into `long'"))?;
        let idx = usize::try_from(idx).ok().and_then(|index| s.len().checked_sub(index));
        if let Some(idx) = idx {
            idx
        } else {
            let mut message = String::from("index ");
            // Suppress error because `String`'s `fmt::Write` impl is infallible.
            // (It will abort on OOM).
            let _ignored = write!(&mut message, "{} out of string", index);
            return Err(IndexError::from(message).into());
        }
    };
    // Wrapping when negative is intentional
    //
    // ```
    // [3.0.2] > s = "abc"
    // => "abc"
    // [3.0.2] > s.setbyte(-3, 99)
    // => 99
    // [3.0.2] > s
    // => "cbc"
    // [3.0.2] > s.setbyte(-3, 255)
    // => 255
    // [3.0.2] > s
    // => "\xFFbc"
    // [3.0.2] > s.setbyte(-3, 256)
    // => 256
    // [3.0.2] > s
    // => "\u0000bc"
    // [3.0.2] > s.setbyte(-3, 257)
    // => 257
    // [3.0.2] > s
    // => "\u0001bc"
    // [3.0.2] > s.setbyte(-3, -1)
    // => -1
    // [3.0.2] > s
    // => "\xFFbc"
    // ```
    let u8_byte = (i64_byte % 256)
        .try_into()
        .expect("taking mod 256 guarantees the resulting i64 is in range for u8");
    // Safety:
    //
    // No need to repack, this is an in-place mutation.
    unsafe {
        let string_mut = s.as_inner_mut();
        let cell = string_mut.get_mut(index).ok_or_else(|| {
            let mut message = String::from("index ");
            // Suppress error because `String`'s `fmt::Write` impl is infallible.
            // (It will abort on OOM).
            let _ignored = write!(&mut message, "{} out of string", index);
            IndexError::from(message)
        })?;
        *cell = u8_byte;
    }

    // Return the original byte argument.
    Ok(byte)
}

pub fn slice_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn split(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn to_f(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn to_i(interp: &mut Artichoke, mut value: Value, base: Option<Value>) -> Result<Value, Error> {
    fn try_parse(slice: &[u8], base: u32) -> Option<i64> {
        let s = str::from_utf8(slice).ok()?;
        i64::from_str_radix(s, base).ok()
    }

    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let base = if let Some(base) = base {
        let base = implicitly_convert_to_int(interp, base)?;
        let base = u32::try_from(base).map_err(|_| ArgumentError::from(format!("invalid radix {base}")))?;
        match base {
            0 => 10,
            1 => return Err(ArgumentError::with_message("invalid radix 1").into()),
            x if x > 36 => return Err(ArgumentError::from(format!("invalid radix {x}")).into()),
            x => x,
        }
    } else {
        10_u32
    };
    let mut slice = s.as_slice();
    let mut squeezed = false;
    // squeeze preceding zeros.
    while let Some(&b'0') = slice.get(0) {
        slice = &slice[1..];
        squeezed = true;
    }
    // Trim leading literal specifier but only if there was a leading 0.
    if squeezed {
        #[allow(clippy::match_same_arms)]
        match (base, slice.get(0).copied()) {
            (2, Some(b'b' | b'B')) => slice = &slice[1..],
            (8, Some(b'o' | b'O')) => slice = &slice[1..],
            (10, Some(b'd' | b'D')) => slice = &slice[1..],
            (16, Some(b'x' | b'X')) => slice = &slice[1..],
            _ => {}
        }
    }

    if slice.is_empty() {
        return Ok(interp.convert(0));
    }
    loop {
        // Try to greedily parse the whole string as an int.
        if let Some(int) = try_parse(slice, base) {
            return Ok(interp.convert(int));
        }
        // if parsing failed, start discarding from the end one byte at a time.
        if let Some((_, head)) = slice.split_last() {
            slice = head;
        } else {
            return Ok(interp.convert(0));
        }
    }
}

pub fn to_s(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Ok(value)
}

pub fn upcase(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut dup = s.clone();
    dup.make_uppercase();
    super::String::alloc_value(dup, interp)
}

pub fn upcase_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
        // `make_uppercase` might reallocate the string and invalidate the
        // boxed pointer, capacity, length triple.
        string_mut.make_uppercase();

        let s = s.take();
        super::String::box_into_value(s, value, interp)
    }
}

pub fn is_valid_encoding(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Ok(interp.convert(s.is_valid_encoding()))
}
