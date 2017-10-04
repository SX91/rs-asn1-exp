use info;
use ser;
use de::{self, Asn1Visitor, Asn1Error};

asn1_info!(f32    => info::TAG_REAL, info::TYPE_REAL);
asn1_info!(f64    => info::TAG_REAL, info::TYPE_REAL);

impl ser::Asn1Serialize for f32 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_f32(*self)
    }
}

impl de::Asn1Deserialize for f32 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = f32;

            fn visit_f32<E: Asn1Error>(self, v: f32) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_f32(IntegerVisitor)
    }
}

impl ser::Asn1Serialize for f64 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_f64(*self)
    }
}

impl de::Asn1Deserialize for f64 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = f64;

            fn visit_f64<E: Asn1Error>(self, v: f64) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_f64(IntegerVisitor)
    }
}

