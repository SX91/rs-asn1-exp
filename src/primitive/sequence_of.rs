use std::{fmt, marker};

use info::{self, Asn1Info, Asn1Tagged, Tag, Len};
use ser::{self, Serialize as Asn1Serialize, SeqSerializer};
use de::{self, Asn1Visitor, Asn1Deserialize as Asn1Deserialize, RawAccess, SeqAccess};


asn1_info!(Vec<T>: (T) => [UNIVERSAL 16], "SEQUENCE OF");
impl<T: Asn1Serialize> Asn1Serialize for Vec<T> {
    fn asn1_serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
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
        Self::_asn1_deserialize_tagged(deserializer, Self::asn1_tag())
    }

    fn _asn1_deserialize_tagged<'de, D: de::Asn1Deserializer<'de>>(deserializer: D,
                                                                   exp_tag: &Tag)
                                                                   -> Result<Self, D::Err> {
        struct SeqOfVisitor<T>(marker::PhantomData<T>);
        impl<'de, T: Asn1Deserialize> Asn1Visitor<'de> for SeqOfVisitor<T> {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between -2^31 and 2^31")
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

        let seq: Vec<T> = deserializer
            .deserialize_seq(exp_tag, SeqOfVisitor(marker::PhantomData))?;
        Ok(seq)
    }
}

