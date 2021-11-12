use core::convert::TryFrom;
use core::hash::{BuildHasher, Hash, Hasher};

use artichoke_core::hash::Hash as _;
use bstr::ByteSlice;

use crate::convert::implicitly_convert_to_int;
use crate::convert::implicitly_convert_to_string;
use crate::extn::core::array::Array;
#[cfg(feature = "core-regexp")]
use crate::extn::core::matchdata::MatchData;
#[cfg(feature = "core-regexp")]
use crate::extn::core::regexp::{self, Regexp};
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;

pub fn mul(interp: &mut Artichoke, mut value: Value, count: Value) -> Result<Value, Error> {
    let count = implicitly_convert_to_int(interp, count)?;
    let count = usize::try_from(count).map_err(|_| ArgumentError::with_message("negative argument"))?;

    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // This guard ensures `repeat` below does not panic on usize overflow.
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
    // The borrowed byte slice is immediately memcpy'd into the `s` byte
    // buffer. There are no intervening interpreter accesses.
    let to_append = unsafe { implicitly_convert_to_string(interp, &mut other)? };

    let mut concatenated = s.clone();
    concatenated.extend_from_slice(to_append);
    super::String::alloc_value(concatenated, interp)
}

pub fn push(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The byteslice is immediately used and discarded after extraction. There
    // are no intervening interpreter accesses.
    let other = unsafe { implicitly_convert_to_string(interp, &mut other)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
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
    // The byteslice is immediately discarded after extraction. There are no
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

pub fn aref(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn aset(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
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
    let mut dup = s.clone();
    dup.make_binary();
    super::String::alloc_value(dup, interp)
}

pub fn bytesize(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let bytesize = s.bytesize();
    interp.try_convert(bytesize)
}

pub fn byteslice(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
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

pub fn capitalize(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut dup = s.clone();
    dup.make_capitalized();
    super::String::alloc_value(dup, interp)
}

pub fn capitalize_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let mut s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let string_mut = s.as_inner_mut();
        // `make_capitalized` might reallocate the string and invalidate the
        // boxed pointer/capa/len.
        string_mut.make_capitalized();

        let s = s.take();
        super::String::box_into_value(s, value, interp)
    }
}

pub fn casecmp_ascii(interp: &mut Artichoke, mut value: Value, mut other: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The byteslice is immediately discarded after extraction. There are no
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
    // TODO: this needs to do an implicit conversion, but we need a spinoso string.
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
    // The byteslice is immediately discarded after extraction and turned into
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
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn chomp(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn chomp_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn chop(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn chop_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn chr(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn clear(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
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
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn concat(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn downcase(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn downcase_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
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

pub fn getbyte(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn hash(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    let mut hasher = interp.build_hasher()?.build_hasher();
    s.as_slice().hash(&mut hasher);
    #[allow(clippy::cast_possible_wrap)]
    let hash = hasher.finish() as i64;
    Ok(interp.convert(hash))
}

pub fn include(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn index(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
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
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    // Safety:
    //
    // The extracted slice is immediately copied to an owned buffer.
    //
    // No intervening operations on the mruby VM occur.
    let buf = unsafe {
        let from = implicitly_convert_to_string(interp, &mut other)?;
        from.to_vec()
    };
    let replacement = super::String::with_bytes_and_encoding(buf, s.encoding());
    // Safety:
    //
    // The string is reboxed before any intervening operations on the
    // interpreter.
    // The string is reboxed without any intervening mruby allocations.
    unsafe {
        let old = s.take();
        drop(old);

        super::String::box_into_value(replacement, value, interp)
    }
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

pub fn ord(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    let string = value.try_convert_into_mut::<&[u8]>(interp)?;
    // NOTE: This implementation assumes all `String`s have encoding =
    // `Encoding::UTF_8`. Artichoke does not implement the `Encoding` APIs and
    // `String`s are assumed to be UTF-8 encoded.
    let (ch, size) = bstr::decode_utf8(string);
    let ord = match ch {
        // All `char`s are valid `u32`s
        // https://github.com/rust-lang/rust/blob/1.48.0/library/core/src/char/convert.rs#L12-L20
        Some(ch) => u32::from(ch),
        None if size == 0 => return Err(ArgumentError::with_message("empty string").into()),
        None => return Err(ArgumentError::with_message("invalid byte sequence in UTF-8").into()),
    };
    Ok(interp.convert(ord))
}

pub fn replace(interp: &mut Artichoke, value: Value, other: Value) -> Result<Value, Error> {
    initialize_copy(interp, value, other)
}

pub fn reverse(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn reverse_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
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
            let regex = Regexp::lazy(pattern_bytes.clone());
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
            let regex = Regexp::lazy(pattern_bytes.clone());
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

pub fn setbyte(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
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

pub fn to_i(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn to_s(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Ok(value)
}

pub fn to_str(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn upcase(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn upcase_bang(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let _s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Err(NotImplementedError::new().into())
}

pub fn is_valid_encoding(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let s = unsafe { super::String::unbox_from_value(&mut value, interp)? };
    Ok(interp.convert(s.is_valid_encoding()))
}
