use bstr::ByteSlice;

pub use crate::encoding_registry::Spec;
use artichoke_core::encoding::Encoding as CoreEncoding;
pub use spinoso_string::Encoding as SpinosoEncoding;

#[derive(Debug, Clone, Copy)]
pub struct Encoding(SpinosoEncoding);

impl From<SpinosoEncoding> for Encoding {
    fn from(enc: SpinosoEncoding) -> Self {
        Encoding(enc)
    }
}

impl From<Encoding> for u8 {
    fn from(enc: Encoding) -> Self {
        enc.flag()
    }
}

impl CoreEncoding for Encoding {
    fn flag(&self) -> u8 {
        self.0.to_flag()
    }

    fn aliases(&self) -> Vec<Vec<u8>> {
        // Some of the `names` specified contain characters which would
        // require character escaping, however in MRI they are converted to
        // underscores.

        self.names().iter().map(|name| name.replace("-", "_")).collect()
    }

    fn is_ascii_compatible(&self) -> bool {
        self.0.is_ascii_compatible()
    }

    fn is_dummy(&self) -> bool {
        self.0.is_dummy()
    }

    fn inspect(&self) -> Vec<u8> {
        self.0.inspect().into()
    }

    fn name(&self) -> Vec<u8> {
        self.0.name().into()
    }

    fn names(&self) -> Vec<Vec<u8>> {
        self.0.names().iter().map(|name| name.as_bytes().into()).collect()
    }
}
