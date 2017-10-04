use info;
use ser;
use de::{self, Asn1Visitor, Asn1Error};

asn1_info!(() => info::TAG_NULL, info::TYPE_NULL);

impl ser::Asn1Serialize for () {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_null()
    }
}

impl de::Asn1Deserialize for () {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct NullVisitor;
        impl<'de> Asn1Visitor<'de> for NullVisitor {
            type Value = ();

            fn visit_null<E: Asn1Error>(self) -> Result<Self::Value, E> {
                Ok(())
            }
        }
        deserializer.deserialize_null(NullVisitor)
    }
}

