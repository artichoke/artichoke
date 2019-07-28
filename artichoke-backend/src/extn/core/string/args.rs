use std::io::Write;
use std::mem;

use crate::convert::{RustBackedValue, TryFromMrb};
use crate::extn::core::regexp::Regexp;
use crate::Mrb;
use crate::sys;
use crate::value::Value;
use crate::MrbError;
