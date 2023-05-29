use core::convert::TryFrom;
use core::fmt::Write as _;
use core::hash::{BuildHasher, Hash, Hasher};
use core::str;

use artichoke_core::hash::Hash as _;
use artichoke_core::value::Value as _;
use bstr::ByteSlice;

use super::Encoding;
use crate::convert::implicitly_convert_to_int;
use crate::convert::implicitly_convert_to_nilable_string;
use crate::convert::implicitly_convert_to_spinoso_string;
use crate::convert::implicitly_convert_to_string;
use crate::extn::core::array::Array;
#[cfg(feature = "core-regexp")]
use crate::extn::core::matchdata::{self, MatchData};
#[cfg(feature = "core-regexp")]
use crate::extn::core::regexp::{self, Regexp};
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;
use crate::sys::protect;

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
    // SAFETY: The borrowed byte slice is immediately copied into the `s` byte
    // buffer. There are no intervening interpreter accesses.
    let to_append = unsafe { implicitly_convert_to_string(interp, &mut other)? };

    let mut concatenated = s.clone();
    // XXX: This call doesn't do a check to see if we'll exceed the max allocation
    //    size and may panic or abort.
    concatenated.extend_from_slice(to_append);
    super::String::alloc_value(concatenated, interp)
}

pub fn append(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
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
        // SAFETY: The string is repacked before any intervening uses of
        // `interp` which means no mruby heap allocations can occur.
        unsafe {
            let string_mut = s.as_inner_mut();
            // XXX: This call doesn't do a check to see if we'll exceed the max allocation
            //    size and may panic or abort.
            string_mut
                .try_push_int(int)
                .map_err(|err| RangeError::from(err.message()))?;
            let s = s.take();
            return super::String::box_into_value(s, value, interp);
        }
    }
    // SAFETY: The byte slice is immediately used and discarded after extraction.
    // There are no intervening interpreter accesses.
    let other = unsafe { implicitly_convert_to_spinoso_string(interp, &mut other)? };
    match s.encoding() {
        // ```
        // [3.1.2] > s = ""
        // => ""
        // [3.1.2] > b = "abc".b
        // => "abc"
        // [3.1.2] > s << b
        // => "abc"
        // [3.1.2] > s.encoding
        // => #<Encoding:UTF-8>
        // [3.1.2] > b = "\xFF\xFE".b
        // => "\xFF\xFE"
        // [3.1.2] > s = ""
        // => ""
        // [3.1.2] > s << b
        // => "\xFF\xFE"
        // [3.1.2] > s.encoding
        // => #<Encoding:ASCII-8BIT>
        // [3.1.2] > s = "abc"
        // => "abc"
        // [3.1.2] > s << b
        // => "abc\xFF\xFE"
        // [3.1.2] > s.encoding
        // => #<Encoding:ASCII-8BIT>
        // [3.1.2] > s = ""
        // => ""
        // [3.1.2] > b = "abc".b
        // => "abc"
        // [3.1.2] > b.ascii_only?
        // => true
        // [3.1.2] > s << b
        // => "abc"
        // [3.1.2] > s.encoding
        // => #<Encoding:UTF-8>
        // [3.1.2] > a = "\xFF\xFE".force_encoding(Encoding::ASCII)
        // => "\xFF\xFE"
        // [3.1.2] > s = ""
        // => ""
        // [3.1.2] > s << a
        // => "\xFF\xFE"
        // [3.1.2] > s.encoding
        // => #<Encoding:US-ASCII>
        // [3.1.2] > s = "abc"
        // => "abc"
        // [3.1.2] > s << a
        // => "abc\xFF\xFE"
        // [3.1.2] > s.encoding
        // => #<Encoding:US-ASCII>
        // ```
        Encoding::Utf8 => {
            // SAFETY: The string is repacked before any intervening uses of
            // `interp` which means no mruby heap allocations can occur.
            unsafe {
                let string_mut = s.as_inner_mut();
                // XXX: This call doesn't do a check to see if we'll exceed the max allocation
                //    size and may panic or abort.
                string_mut.extend_from_slice(other.as_slice());

                if !matches!(other.encoding(), Encoding::Utf8) && !other.is_ascii_only() {
                    // encodings are incompatible if other is not UTF-8 and is non-ASCII
                    string_mut.set_encoding(other.encoding());
                }

                let s = s.take();
                super::String::box_into_value(s, value, interp)
            }
        }
        // Empty ASCII strings take on the encoding of the argument if the
        // argument is not ASCII-compatible, even if the argument does not have
        // a valid encoding.
        //
        // ```
        // [3.1.2] > ae = "".force_encoding(Encoding::ASCII)
        // => ""
        // [3.1.2] > ae << "😀"
        // => "😀"
        // [3.1.2] > ae.encoding
        // => #<Encoding:UTF-8>
        // [3.1.2] > ae = "".force_encoding(Encoding::ASCII)
        // => ""
        // [3.1.2] > ae << "abc"
        // => "abc"
        // [3.1.2] > ae.encoding
        // => #<Encoding:US-ASCII>
        // [3.1.2] > ae = "".force_encoding(Encoding::ASCII)
        // => ""
        // [3.1.2] > ae << "\xFF\xFE"
        // => "\xFF\xFE"
        // [3.1.2] > ae.encoding
        // => #<Encoding:UTF-8>
        // [3.1.2] > ae = "".force_encoding(Encoding::ASCII)
        // => ""
        // [3.1.2] > ae << "\xFF\xFE".b
        // => "\xFF\xFE"
        // [3.1.2] > ae.encoding
        // => #<Encoding:ASCII-8BIT>
        // ```
        Encoding::Ascii if s.is_empty() => {
            // SAFETY: The string is repacked before any intervening uses of
            // `interp` which means no mruby heap allocations can occur.
            unsafe {
                let string_mut = s.as_inner_mut();
                // XXX: This call doesn't do a check to see if we'll exceed the max allocation
                //    size and may panic or abort.
                string_mut.extend_from_slice(other.as_slice());

                // Set encoding to `other.encoding()` if other is non-ASCII.
                if !other.is_ascii_only() {
                    string_mut.set_encoding(other.encoding());
                }

                let s = s.take();
                super::String::box_into_value(s, value, interp)
            }
        }
        // ```
        // [3.1.2] > a
        // => "\xFF\xFE"
        // [3.1.2] > a.encoding
        // => #<Encoding:US-ASCII>
        // [3.1.2] > a << "\xFF".b
        // (irb):46:in `<main>': incompatible character encodings: US-ASCII and ASCII-8BIT (Encoding::CompatibilityError)
        //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
        // [3.1.2] > a << "abc"
        // => "\xFF\xFEabc"
        // [3.1.2] > s.encoding
        // => #<Encoding:US-ASCII>
        // [3.1.2] > a << "😀"
        // (irb):49:in `<main>': incompatible character encodings: US-ASCII and UTF-8 (Encoding::CompatibilityError)
        //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
        // [3.1.2] > a << "😀".b
        // (irb):50:in `<main>': incompatible character encodings: US-ASCII and ASCII-8BIT (Encoding::CompatibilityError)
        //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
        //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
        // ```
        Encoding::Ascii => {
            if !other.is_ascii() {
                let code = format!(
                    "raise Encoding::CompatibilityError, 'incompatible character encodings: {} and {}",
                    s.encoding(),
                    other.encoding()
                );
                interp.eval(code.as_bytes())?;
                unreachable!("raised exception");
            }
            // SAFETY: The string is repacked before any intervening uses of
            // `interp` which means no mruby heap allocations can occur.
            unsafe {
                let string_mut = s.as_inner_mut();
                // XXX: This call doesn't do a check to see if we'll exceed the max allocation
                //    size and may panic or abort.
                string_mut.extend_from_slice(other.as_slice());

                let s = s.take();
                super::String::box_into_value(s, value, interp)
            }
        }
        // If the receiver is an empty string with `Encoding::Binary` encoding
        // and the argument is non-ASCII, take on the encoding of the argument.
        //
        // This requires the implicit conversion to string to return the
        // underlying Spinoso string.
        //
        // ```
        // [3.1.2] > be = "".b
        // => ""
        // [3.1.2] > be << "abc"
        // => "abc"
        // [3.1.2] > be.encoding
        // => #<Encoding:ASCII-8BIT>
        // [3.1.2] > be = "".b
        // => ""
        // [3.1.2] > be << "😀"
        // => "😀"
        // [3.1.2] > be.encoding
        // => #<Encoding:UTF-8>
        // [3.1.2] > be = "".b
        // => ""
        // [3.1.2] > be << "\xFF\xFE".force_encoding(Encoding::ASCII)
        // => "\xFF\xFE"
        // [3.1.2] > be.encoding
        // => #<Encoding:US-ASCII>
        // [3.1.2] > be = "".b
        // => ""
        // [3.1.2] > be << "abc".force_encoding(Encoding::ASCII)
        // => "abc"
        // [3.1.2] > be.encoding
        // => #<Encoding:ASCII-8BIT>
        // ```
        Encoding::Binary if s.is_empty() => {
            // SAFETY: The string is repacked before any intervening uses of
            // `interp` which means no mruby heap allocations can occur.
            unsafe {
                let string_mut = s.as_inner_mut();
                // XXX: This call doesn't do a check to see if we'll exceed the max allocation
                //    size and may panic or abort.
                string_mut.extend_from_slice(other.as_slice());

                if !other.is_ascii_only() {
                    string_mut.set_encoding(other.encoding());
                }

                let s = s.take();
                super::String::box_into_value(s, value, interp)
            }
        }
        Encoding::Binary => {
            // SAFETY: The string is repacked before any intervening uses of
            // `interp` which means no mruby heap allocations can occur.
            unsafe {
                let string_mut = s.as_inner_mut();
                // XXX: This call doesn't do a check to see if we'll exceed the max allocation
                //    size and may panic or abort.
                string_mut.extend_from_slice(other.as_slice());

                let s = s.take();
                super::String::box_into_value(s, value, interp)
            }
        }
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
    if value.respond_to(interp, "to_str")? {
        let result = other.funcall(interp, "==", &[value], None)?;
        // any falsy returned value yields `false`, otherwise `true`.
        if let Ok(result) = result.try_convert_into::<Option<bool>>(interp) {
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

        let index = match aref::offset_to_index(index, s.len()) {
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
            let index = match aref::offset_to_index(index, s.len()) {
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

    if let Some(index) = aref::offset_to_index(index, s.len()) {
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

pub fn byteindex(
    interp: &mut Artichoke,
    mut value: Value,
    mut substring: Value,
    offset: Option<Value>,
) -> Result<Value, Error> {
    #[cfg(feature = "core-regexp")]
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    if let Ok(pattern) = unsafe { Regexp::unbox_from_value(&mut substring, interp) } {
        let offset_number = match implicitly_convert_to_int(interp, offset.into()) {
            Err(_) => None,
            Ok(value) => Some(value),
        };

        let mut matches = pattern.match_(interp, Some(s.as_slice()), offset_number, None)?;
        if matches.is_nil() {
            return Ok(Value::nil());
        }
        let first_ocurrence = unsafe { MatchData::unbox_from_value(&mut matches, interp)? };
        let ocurrence_index = first_ocurrence.begin(matchdata::Capture::GroupIndex(0)).unwrap();
        match ocurrence_index {
            None => return Ok(Value::nil()),
            Some(n) => return Ok(interp.convert(n as i64)),
        }
    }
    let needle = unsafe { implicitly_convert_to_string(interp, &mut substring)? };
    let offset = if let Some(offset) = offset {
        let offset = implicitly_convert_to_int(interp, offset)?;
        match aref::offset_to_index(offset, s.len()) {
            None => return Ok(Value::nil()),
            Some(offset) if offset > s.len() => return Ok(Value::nil()),
            Some(offset) => Some(offset),
        }
    } else {
        None
    };
    interp.try_convert(s.byteindex(needle, offset))
}

pub fn byterindex(
    interp: &mut Artichoke,
    mut value: Value,
    mut substring: Value,
    offset: Option<Value>,
) -> Result<Value, Error> {
    #[cfg(feature = "core-regexp")]
    if let Ok(_pattern) = unsafe { Regexp::unbox_from_value(&mut substring, interp) } {
        return Err(NotImplementedError::from("String#byterindex with Regexp pattern").into());
    }
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let needle = unsafe { implicitly_convert_to_string(interp, &mut substring)? };
    let offset = if let Some(offset) = offset {
        let offset = implicitly_convert_to_int(interp, offset)?;
        match aref::offset_to_index(offset, s.len()) {
            None => return Ok(Value::nil()),
            Some(offset) if offset > s.len() => return Ok(Value::nil()),
            Some(offset) => Some(offset),
        }
    } else {
        None
    };
    interp.try_convert(s.byterindex(needle, offset))
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
            let index = match aref::offset_to_index(index, s.len()) {
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

    let index = match aref::offset_to_index(index, s.len()) {
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
        // Per the docs -- https://ruby-doc.org/core-3.1.2/String.html#method-i-byteslice
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
    // SAFETY: The string is repacked before any intervening uses of `interp`
    // which means no mruby heap allocations can occur.
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
    // SAFETY: The byte slice is immediately discarded after extraction. There
    // are no intervening interpreter accesses.
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
    // SAFETY: The byte slice is immediately converted to an owned `Vec` after
    // extraction. There are no intervening interpreter accesses.
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
    // SAFETY: The string is repacked before any intervening uses of `interp`
    // which means no mruby heap allocations can occur.
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
    // SAFETY: The string is repacked before any intervening uses of `interp`
    // which means no mruby heap allocations can occur.
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

pub fn end_with<T>(interp: &mut Artichoke, mut value: Value, suffixes: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };

    for mut suffix in suffixes {
        // SAFETY: `s` used and discarded immediately before any intervening operations on the VM.
        // This ensures there are no intervening garbage collections which may free the `RString*` that backs this value.
        let needle = unsafe { implicitly_convert_to_string(interp, &mut suffix)? };
        if s.ends_with(needle) {
            return Ok(interp.convert(true));
        }
    }
    Ok(interp.convert(false))
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
    let index = if let Some(index) = aref::offset_to_index(index, s.len()) {
        index
    } else {
        return Ok(Value::nil());
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
        let offset = aref::offset_to_index(offset, s.len());
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
        // SAFETY: The extracted slice is immediately copied to an owned buffer.
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
    // SAFETY: The extracted slice is immediately copied to an owned buffer. No
    // intervening operations on the mruby VM occur.
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
    // SAFETY: The string is repacked before any intervening uses of `interp`
    // which means no mruby heap allocations can occur.
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
        let offset = aref::offset_to_index(offset, s.len());
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
    // SAFETY: `pattern_bytes` is converted to an owned byte vec to ensure the
    // underlying `RString*` is not garbage collected when yielding matches.
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
    // SAFETY: `pattern_bytes` is converted to an owned byte vec to ensure the
    // underlying `RString*` is not garbage collected when yielding matches.
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
    let index = if let Some(index) = aref::offset_to_index(index, s.len()) {
        index
    } else {
        let mut message = String::from("index ");
        // Suppress error because `String`'s `fmt::Write` impl is infallible.
        // (It will abort on OOM).
        let _ignored = write!(&mut message, "{index} out of string");
        return Err(IndexError::from(message).into());
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
    // SAFETY: No need to repack, this is an in-place mutation.
    unsafe {
        let string_mut = s.as_inner_mut();
        let cell = string_mut.get_mut(index).ok_or_else(|| {
            let mut message = String::from("index ");
            // Suppress error because `String`'s `fmt::Write` impl is infallible.
            // (It will abort on OOM).
            let _ignored = write!(&mut message, "{index} out of string");
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

pub fn start_with<T>(interp: &mut Artichoke, mut value: Value, prefixes: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };

    for mut prefix in prefixes {
        if prefix.ruby_type() == Ruby::String {
            let needle = unsafe { super::String::unbox_from_value(&mut prefix, interp)? };
            if s.starts_with(needle.as_inner_ref()) {
                return Ok(interp.convert(true));
            }
        } else {
            #[cfg(feature = "core-regexp")]
            if let Ok(regexp) = unsafe { Regexp::unbox_from_value(&mut prefix, interp) } {
                let mut inner = regexp.inner().match_(interp, &s, None, None)?;
                if inner.is_nil() {
                    continue;
                }

                let match_data = unsafe { MatchData::unbox_from_value(&mut inner, interp)? };
                if match_data.begin(matchdata::Capture::GroupIndex(0))? == Some(0) {
                    return Ok(interp.convert(true));
                }

                regexp::clear_capture_globals(interp)?;
                interp.unset_global_variable(regexp::LAST_MATCH)?;
                interp.unset_global_variable(regexp::STRING_LEFT_OF_MATCH)?;
                interp.unset_global_variable(regexp::STRING_RIGHT_OF_MATCH)?;
                continue;
            }

            // SAFETY: `s` used and discarded immediately before any intervening operations on the VM.
            // This ensures there are no intervening garbage collections which may free the `RString*` that backs this value.
            let needle = unsafe { implicitly_convert_to_string(interp, &mut prefix)? };
            if s.starts_with(needle) {
                return Ok(interp.convert(true));
            }
        }
    }

    Ok(interp.convert(false))
}

pub fn to_f(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn to_i(interp: &mut Artichoke, mut value: Value, base: Option<Value>) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut slice = s.as_slice();
    // ignore preceding whitespace
    if let Some(start) = slice.iter().position(|&c| !posix_space::is_space(c)) {
        slice = &slice[start..];
    } else {
        // All whitespace, but we cant return early because we need to ensure the base is valid too
        slice = &[];
    }

    // Grab sign before prefix matching
    let sign = match slice.first() {
        Some(b'+') => {
            slice = &slice[1..];
            1
        }
        Some(b'-') => {
            slice = &slice[1..];
            -1
        }
        _ => 1,
    };

    let base = base.map_or(Ok(10), |b| implicitly_convert_to_int(interp, b))?;
    let base = match base {
        x if x < 0 || x == 1 || x > 36 => return Err(ArgumentError::from(format!("invalid radix {base}")).into()),
        0 => {
            // Infer base size from prefix
            if slice.len() < 2 || slice[0] != b'0' {
                10
            } else {
                match &slice[1] {
                    b'b' | b'B' => {
                        slice = &slice[2..];
                        2
                    }
                    b'o' | b'O' => {
                        slice = &slice[2..];
                        8
                    }
                    b'd' | b'D' => {
                        slice = &slice[2..];
                        10
                    }
                    b'x' | b'X' => {
                        slice = &slice[2..];
                        16
                    }
                    _ => {
                        // Numbers that start with 0 are assumed to be octal
                        slice = &slice[1..];
                        8
                    }
                }
            }
        }
        x => {
            // Trim leading literal specifier if it exists
            if slice.len() >= 2
                && matches!(
                    (x, &slice[0..2]),
                    (2, b"0b" | b"0B") | (8, b"0o" | b"0O") | (10, b"0d" | b"0D") | (16, b"0x" | b"0X")
                )
            {
                slice = &slice[2..];
            };

            // This can only be 2-36 inclusive in this branch, so unwrap is safe
            u32::try_from(x).unwrap()
        }
    };

    // Check string doesn't start with any special characters
    //  '_' is invalid because they are stripped out elsewhere in the string, but cannot start a number
    //  '+' and '-' invalid because we already have the sign prior to the prefix, but they are accepted
    //  at the begining of a string by str_from_radix, and double sign doesn't make sense
    if matches!(slice.first(), Some(&b'_' | &b'+' | &b'-')) {
        return Ok(interp.convert(0));
    }

    // Double underscores are not valid, and we should stop parsing the string if we encounter one
    if let Some(double_underscore) = slice.find("__") {
        slice = &slice[..double_underscore];
    }

    // Single underscores should be ignored
    let mut slice = std::borrow::Cow::from(slice);
    if slice.find("_").is_some() {
        slice.to_mut().retain(|&c| c != b'_');
    }
    loop {
        use std::num::IntErrorKind;
        // Try to greedily parse the whole string as an int.
        let parsed = str::from_utf8(&slice)
            .map_err(|_| IntErrorKind::InvalidDigit)
            .and_then(|s| i64::from_str_radix(s, base).map_err(|err| err.kind().clone()));
        match parsed {
            Ok(int) => return Ok(interp.convert(sign * int)),
            Err(IntErrorKind::Empty | IntErrorKind::Zero) => return Ok(interp.convert(0)),
            Err(IntErrorKind::PosOverflow | IntErrorKind::NegOverflow) => {
                return Err(NotImplementedError::new().into())
            }
            _ => {
                // if parsing failed, start discarding from the end one byte at a time.
                match slice {
                    std::borrow::Cow::Owned(ref mut data) => {
                        data.pop();
                    }
                    std::borrow::Cow::Borrowed(data) => {
                        slice = std::borrow::Cow::from(&data[..(data.len() - 1)]);
                    }
                }
            }
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
    // SAFETY: The string is repacked before any intervening uses of `interp`
    // which means no mruby heap allocations can occur.
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
