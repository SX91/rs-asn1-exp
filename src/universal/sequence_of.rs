use std::{fmt, marker};

use info::{self, TAG_SEQUENCE, TYPE_SEQUENCE_OF};
use ser::{self, Asn1Serialize, SeqSerializer};
use de::{self, Asn1Visitor, Asn1Deserialize as Asn1Deserialize, SeqAccess};


asn1_info!(Vec<T>: (T) => TAG_SEQUENCE, TYPE_SEQUENCE_OF);

impl<T: Asn1Serialize> Asn1Serialize for Vec<T> {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        let mut seq_seriliazer = serializer.serialize_sequence()?;
        for field in self.iter() {
            seq_seriliazer.serialize_field(field)?
        }
        seq_seriliazer.finish()
    }
}

impl<T: Asn1Deserialize> de::Asn1Deserialize for Vec<T> {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct SeqOfVisitor<T>(marker::PhantomData<T>);
        impl<'de, T: Asn1Deserialize> Asn1Visitor<'de> for SeqOfVisitor<T> {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "{} {}", info::TYPE_SEQUENCE_OF, T::asn1_type())
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Err>
                where A: SeqAccess<'de>
            {
                let mut out: Vec<T> = Vec::new();
                while seq.remaining() > 0 {
                    out.push(seq.next_field()?)
                }
                Ok(out)
            }
        }

        deserializer.deserialize_seq(SeqOfVisitor(marker::PhantomData))
    }
}


#[cfg(test)]
mod tests {
    use universal::{OctetString, BitString, ObjectIdentifier};
    use universal::test_helper::ser_deser;

    #[quickcheck]
    fn sequence_of_bool(v: Vec<bool>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_i8(v: Vec<i8>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_i16(v: Vec<i16>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_i32(v: Vec<i32>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_i64(v: Vec<i64>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_u8(v: Vec<u8>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_u16(v: Vec<u16>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_u32(v: Vec<u32>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_u64(v: Vec<u64>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_bit_string(v: Vec<BitString>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_octet_string(v: Vec<OctetString>) -> bool {
        v == ser_deser(&v)
    }

    #[quickcheck]
    fn sequence_of_object_identifier(v: Vec<ObjectIdentifier>) -> bool {
        v == ser_deser(&v)
    }
}
