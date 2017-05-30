// Module for BIT STRING
use info::{Asn1Info, Tag, Len};

use ser::{self, RawEncoder};

#[derive(Debug, PartialEq, PartialOrd)]
struct BitString {
    data: Vec<u8>,
    bitsize: usize,
}

asn1_info!(BitString, UNIVERSAL 3, "BIT STRING");

impl ser::Serialize for BitString {
    fn asn1_serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        let tag = Self::asn1_tag().unwrap();

        self._asn1_serialize_tagged(serializer, tag)
    }

    fn _asn1_serialize_tagged<S: ser::Serializer>(&self, serializer: S, tag: &Tag) -> Result<S::Ok, S::Err> {
        let mut raw = serializer.serialize_raw()?;
        let data_len = self.data.len();

        raw.encode_header(tag, &Len::Def(self.data.len() + 1))?;
        raw.encode_byte((data_len - self.bitsize) as u8)?;
        raw.encode_bytes(self.data.as_slice())
    }
}
