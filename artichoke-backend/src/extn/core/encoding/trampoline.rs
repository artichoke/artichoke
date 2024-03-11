use super::{Encoding, ReplicaEncoding};

use crate::extn::core::array::Array;
use crate::extn::core::string::{Encoding as SpinosoEncoding, String};
use artichoke_core::encoding::Encoding as _;

use crate::extn::prelude::*;

pub(super) const AVAILABLE_ENCODINGS: [SpinosoEncoding; 3] =
    [SpinosoEncoding::Utf8, SpinosoEncoding::Ascii, SpinosoEncoding::Binary];

pub fn aliases(interp: &mut Artichoke) -> Result<Value, Error> {
    let _ = interp;
    Err(NotImplementedError::new().into())
}

pub fn compatible(interp: &mut Artichoke, lhs: Value, rhs: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = lhs;
    let _ = rhs;
    Err(NotImplementedError::new().into())
}

pub fn find(interp: &mut Artichoke, encoding: Value) -> Result<Value, Error> {
    let _ = interp;
    let _ = encoding;
    Err(NotImplementedError::new().into())
}

pub fn list(interp: &mut Artichoke) -> Result<Value, Error> {
    let _ = interp.encodings();
    let _ = interp;
    Err(NotImplementedError::new().into())
}

pub fn locale_charmap(interp: &mut Artichoke) -> Result<Value, Error> {
    let _ = interp;
    Err(NotImplementedError::new().into())
}

pub fn name_list(interp: &mut Artichoke) -> Result<Value, Error> {
    let _ = interp;
    Err(NotImplementedError::new().into())
}

pub fn ascii_compatible(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let encoding = unsafe { Encoding::unbox_from_value(&mut value, interp)? };

    let result = encoding.is_ascii_compatible();

    Ok(interp.convert(result))
}

pub fn dummy(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let encoding = unsafe { Encoding::unbox_from_value(&mut value, interp)? };

    let result = encoding.is_dummy();

    Ok(interp.convert(result))
}

pub fn inspect(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let encoding = unsafe { Encoding::unbox_from_value(&mut value, interp)? };

    let result = encoding.inspect();

    interp.try_convert_mut(result)
}

pub fn name(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let encoding = unsafe { Encoding::unbox_from_value(&mut value, interp)? };

    let name = encoding.name();

    // The result of `Encoding#name` is always 7bit ascii.
    //
    // ```irb
    // 3.1.2 > Encoding::UTF_8.name.encoding
    // => #<Encoding:US-ASCII>
    // ```
    let result = String::with_bytes_and_encoding(name, SpinosoEncoding::Ascii);

    String::alloc_value(result, interp)
}

pub fn names(interp: &mut Artichoke, mut value: Value) -> Result<Value, Error> {
    let encoding = unsafe { Encoding::unbox_from_value(&mut value, interp)? };

    // The result of `Encoding#names` is always 7bit ascii.
    //
    // ```irb
    // 3.1.2 > Encoding::ISO_8859_1.names
    // => ["ISO-8859-1", "ISO8859-1"]
    // 3.1.2 > Encoding::ISO_8859_1.names.map(&:encoding)
    // => [#<Encoding:US-ASCII>, #<Encoding:US-ASCII>]
    // ```
    let names: Vec<Value> = encoding
        .names()
        .iter()
        .map(|name| {
            let name = String::with_bytes_and_encoding(name.clone(), SpinosoEncoding::Ascii);
            String::alloc_value(name, interp)
        })
        .collect::<Result<Vec<Value>, Error>>()?;

    let result = Array::from(names);

    Array::alloc_value(result, interp)
}

pub fn replicate(interp: &mut Artichoke, mut value: Value, mut target: Value) -> Result<Value, Error> {
    let encoding = unsafe { Encoding::unbox_from_value(&mut value, interp)? };

    let name = unsafe { String::unbox_from_value(&mut target, interp)? };

    let enc = match encoding.clone() {
        Encoding::Spinoso(e) => e,
        Encoding::Replica(e) => e.replicates(),
    };

    let replica = ReplicaEncoding::with_name(name.to_vec(), enc);

    Encoding::alloc_value(replica.into(), interp)
}
