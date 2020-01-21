//! Parse encoding parameter to `Regexp#initialize` and `Regexp::compile`.

use std::hash::{Hash, Hasher};

use crate::extn::core::regexp;
use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    InvalidEncoding,
}

#[derive(Debug, Clone, Copy)]
#[must_use]
pub enum Encoding {
    Fixed,
    No,
    None,
}

impl Encoding {
    #[must_use]
    pub fn flags(self) -> Int {
        match self {
            Self::Fixed => regexp::FIXEDENCODING,
            Self::No => regexp::NOENCODING,
            Self::None => 0,
        }
    }

    #[must_use]
    pub fn string(self) -> &'static str {
        match self {
            Self::Fixed | Self::None => "",
            Self::No => "n",
        }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Self::None
    }
}

impl Hash for Encoding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string().hash(state);
    }
}

impl PartialEq for Encoding {
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::No, Self::No)
            | (Self::No, Self::None)
            | (Self::None, Self::No)
            | (Self::None, Self::None)
            | (Self::Fixed, Self::Fixed) => true,
            _ => false,
        }
    }
}

impl Eq for Encoding {}

pub fn parse(value: &Value) -> Result<Encoding, Error> {
    if let Ok(encoding) = value.itself::<Int>() {
        // Only deal with Encoding opts
        let encoding = encoding & !regexp::ALL_REGEXP_OPTS;
        if encoding == regexp::FIXEDENCODING {
            Ok(Encoding::Fixed)
        } else if encoding == regexp::NOENCODING {
            Ok(Encoding::No)
        } else if encoding == 0 {
            Ok(Encoding::default())
        } else {
            Err(Error::InvalidEncoding)
        }
    } else if let Ok(encoding) = value.itself::<&str>() {
        if encoding.contains('u') && encoding.contains('n') {
            return Err(Error::InvalidEncoding);
        }
        let mut enc = vec![];
        for flag in encoding.chars() {
            match flag {
                'u' | 's' | 'e' => enc.push(Encoding::Fixed),
                'n' => enc.push(Encoding::No),
                'i' | 'm' | 'x' | 'o' => continue,
                _ => return Err(Error::InvalidEncoding),
            }
        }
        if enc.len() > 1 {
            return Err(Error::InvalidEncoding);
        }
        Ok(enc.pop().unwrap_or_default())
    } else {
        Ok(Encoding::default())
    }
}
