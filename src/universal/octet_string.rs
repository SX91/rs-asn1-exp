// Module for OCTET STRING
use std::fmt;

use info;
use ser;
use de::{self, Asn1Visitor, Asn1Error};

#[cfg(feature = "with-serde")]
use ::serde_bytes;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct OctetString(
    #[cfg_attr(feature = "with-serde", serde(with = "serde_bytes"))]
    Vec<u8>
);

impl OctetString {
    pub fn new(data: Vec<u8>) -> Self {
        OctetString(data)
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }

    pub fn from_slice(v: &[u8]) -> Self {
        OctetString(Vec::from(v))
    }

    pub fn from_str(v: &str) -> Self {
        OctetString(Vec::from(v))
    }
}

impl<'a> From<&'a [u8]> for OctetString {
    fn from(v: &[u8]) -> OctetString {
        OctetString::from_slice(v)
    }
}

impl<'a> From<Vec<u8>> for OctetString {
    fn from(v: Vec<u8>) -> OctetString {
        OctetString::new(v)
    }
}

impl fmt::Display for OctetString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::str::from_utf8;
        match from_utf8(self.0.as_slice()) {
            Ok(s) => f.write_str(s),
            Err(_) => {
                let mut use_prefix = false;
                for x in self.0.iter() {
                    let prefix = if use_prefix {
                        " "
                    } else {
                        ""
                    };
                    use_prefix = true;
                    write!(f, "{}{:02X}", prefix, x)?;
                }
                Ok(())
            }
        }
    }
}

asn1_info!(OctetString => info::TAG_OCTET_STRING, info::TYPE_OCTET_STRING);

impl ser::Asn1Serialize for OctetString {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_bytes(self.as_slice())
    }
}

impl de::Asn1Deserialize for OctetString {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct BytesVisitor;
        impl<'de> Asn1Visitor<'de> for BytesVisitor {
            type Value = OctetString;

            fn visit_byte_string<E: Asn1Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(OctetString::new(v))
            }
        }
        deserializer.deserialize_bytes(BytesVisitor)
    }
}



#[cfg(test)]
mod tests {
    use super::OctetString;
    use quickcheck::{Arbitrary, Gen};

    use universal::test_helper::ser_deser;

    impl Arbitrary for OctetString {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            OctetString::new(Arbitrary::arbitrary(g))
        }
    }

    #[test]
    fn null_octet_string() {
        let v = OctetString::new(Vec::new());
        assert_eq!(v, ser_deser(&v))
    }

    #[quickcheck]
    fn octet_string(v: OctetString) -> bool {
        v == ser_deser(&v)
    }
}

