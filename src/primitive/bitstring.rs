// Module for BIT STRING
use info::{Asn1Tagged, Tag};

use de::{self, RawDecoder};
use ser;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BitString {
    data: Vec<u8>,
    bitsize: usize,
}

asn1_info!(BitString => UNIVERSAL 3, "BIT STRING");

impl ser::Serialize for BitString {
    fn asn1_serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        let unused = self.data.len() * 8 - self.bitsize;
        serializer.serialize_bit_string((unused as u8, self.data.as_slice()))
    }
}

impl<'de> de::Deserialize<'de> for BitString {
    fn asn1_deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Err> {
        Self::_asn1_deserialize_tagged(deserializer, Self::asn1_tag())
    }

    fn _asn1_deserialize_tagged<D: de::Deserializer<'de>>(deserializer: D,
                                                          tag: &Tag)
                                                          -> Result<Self, D::Err> {
        let (unused, data) = deserializer.deserialize_bit_string(tag)?;
        let bitsize = data.len() * 8 - (unused as usize);

        Ok(BitString {
               data: data,
               bitsize: bitsize,
           })
    }
}

