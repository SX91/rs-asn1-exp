// Module for BIT STRING
use std::fmt;

use info;
use ser;
use de::{self, Asn1Error, Asn1Visitor};

#[cfg(feature = "with-serde")]
use ::serde_bytes;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct BitString {
    unused: usize,
    #[cfg_attr(feature = "with-serde", serde(with = "serde_bytes"))]
    data: Vec<u8>,
}

impl BitString {
    fn from_vec(data: Vec<u8>, unused: usize) -> Self {
        assert!(unused < 8);
        assert!(data.len() > 0 || unused == 0);
        BitString {
            unused: unused,
            data: data,
        }
    }
}

asn1_info!(BitString => info::TAG_BIT_STRING, info::TYPE_BIT_STRING);

impl fmt::Display for BitString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in self.data.iter() {
            write!(f, "{:b}", b)?;
        }
        Ok(())
    }
}

impl ser::Asn1Serialize for BitString {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_bit_string((self.unused as u8, self.data.as_slice()))
    }
}

impl de::Asn1Deserialize for BitString {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        use std::fmt;

        struct BitStringVisitor;

        impl<'de> Asn1Visitor<'de> for BitStringVisitor {
            type Value = BitString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("BIT STRING")
            }

            fn visit_bit_string<E: Asn1Error>(self, v: (u8, Vec<u8>)) -> Result<BitString, E> {
                // todo: check unused value
                let (unused, bytes) = v;
                Ok(BitString::from_vec(bytes, unused as usize))
            }
        }

        deserializer.deserialize_bit_string(BitStringVisitor)
    }
}


#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::BitString;
    use universal::test_helper::ser_deser;

    impl Arbitrary for BitString {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let buf: Vec<u8> = Arbitrary::arbitrary(g);
            let unused = if buf.len() == 0 {
                0
            } else {
                g.gen_range(0, 7)
            };

            BitString::from_vec(buf, unused)
        }
    }

    #[quickcheck]
    fn bit_string(v: BitString) -> bool {
        v == ser_deser(&v)
    }
}

