pub mod replica;
pub mod spinoso;

use artichoke_core::encoding::Encoding as CoreEncoding;

#[derive(Debug, Clone)]
pub enum Encoding {
    Spinoso(spinoso::Encoding),
    Replica(replica::Encoding),
}

impl CoreEncoding for Encoding {
    fn flag(&self) -> u8 {
        match self {
            Self::Spinoso(e) => e.flag(),
            Self::Replica(e) => e.flag(),
        }
    }

    fn aliases(&self) -> Vec<Vec<u8>> {
        match self {
            Self::Spinoso(e) => e.aliases(),
            Self::Replica(e) => e.aliases(),
        }
    }

    fn is_ascii_compatible(&self) -> bool {
        match self {
            Self::Spinoso(e) => e.is_ascii_compatible(),
            Self::Replica(e) => e.is_ascii_compatible(),
        }
    }

    fn is_dummy(&self) -> bool {
        match self {
            Self::Spinoso(e) => e.is_dummy(),
            Self::Replica(e) => e.is_dummy(),
        }
    }

    fn inspect(&self) -> Vec<u8> {
        match self {
            Self::Spinoso(e) => e.inspect(),
            Self::Replica(e) => e.inspect(),
        }
    }

    fn name(&self) -> Vec<u8> {
        match self {
            Self::Spinoso(e) => e.name(),
            Self::Replica(e) => e.name(),
        }
    }

    fn names(&self) -> Vec<Vec<u8>> {
        match self {
            Self::Spinoso(e) => e.names(),
            Self::Replica(e) => e.names(),
        }
    }
}

impl From<spinoso::SpinosoEncoding> for Encoding {
    fn from(encoding: spinoso::SpinosoEncoding) -> Self {
        Encoding::Spinoso(encoding.into())
    }
}

impl From<spinoso::Encoding> for Encoding {
    fn from(encoding: spinoso::Encoding) -> Self {
        Encoding::Spinoso(encoding)
    }
}

impl From<replica::Encoding> for Encoding {
    fn from(encoding: replica::Encoding) -> Self {
        Encoding::Replica(encoding)
    }
}

impl From<Encoding> for u8 {
    fn from(encoding: Encoding) -> u8 {
        encoding.flag()
    }
}
