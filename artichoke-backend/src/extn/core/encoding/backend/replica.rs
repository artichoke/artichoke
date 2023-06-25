use super::spinoso::Encoding as SpinosoEncoding;
use artichoke_core::encoding::Encoding as CoreEncoding;
use bstr::ByteSlice;

#[derive(Clone, Debug)]
pub struct Encoding {
    name: Vec<u8>,
    replica: SpinosoEncoding,
}

impl Encoding {
    pub fn with_name(name: Vec<u8>, encoding: SpinosoEncoding) -> Self {
        Self {
            name,
            replica: encoding,
        }
    }

    pub fn replicates(&self) -> SpinosoEncoding {
        self.replica
    }
}

impl CoreEncoding for Encoding {
    fn flag(&self) -> u8 {
        // The first bit being true indicates a replica.
        0x1 | self.replica.flag() << 1
    }

    fn aliases(&self) -> Vec<Vec<u8>> {
        self.names().iter().map(|name| name.replace("-", "_")).collect()
    }

    fn is_ascii_compatible(&self) -> bool {
        self.replica.is_ascii_compatible()
    }

    fn is_dummy(&self) -> bool {
        self.replica.is_dummy()
    }

    fn inspect(&self) -> Vec<u8> {
        let mut inspect = br#"#<Encoding:"#.to_vec();
        inspect.extend_from_slice(&self.name);
        inspect.extend_from_slice(br#">"#.as_ref());
        inspect
    }

    fn name(&self) -> Vec<u8> {
        self.name.clone()
    }

    fn names(&self) -> Vec<Vec<u8>> {
        [self.name()].into()
    }
}
