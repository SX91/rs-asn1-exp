use info;
use ser;
use de::{self, Asn1Visitor, Asn1Error};

asn1_info!(bool => info::TAG_BOOLEAN, info::TYPE_BOOLEAN);

impl ser::Asn1Serialize for bool {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_bool(*self)
    }
}

impl de::Asn1Deserialize for bool {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct BooleanVisitor;
        impl<'de> Asn1Visitor<'de> for BooleanVisitor {
            type Value = bool;

            fn visit_bool<E: Asn1Error>(self, v: bool) -> Result<bool, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_bool(BooleanVisitor)
    }
}

#[cfg(test)]
mod tests {
    use universal::test_helper::ser_deser;

    #[quickcheck]
    fn bool(v: bool) -> bool {
        v == ser_deser(&v)
    }
}

#[cfg(test)]
mod bench {
    use test;
    use der;
    use ser::Asn1Serialize;

    #[bench]
    fn bool_bench(b: &mut test::Bencher) {
        let mut buf = Vec::with_capacity(128);
        let v: bool = false;

        b.iter(|| {
                   {
                       let writer = der::Serializer::new(&mut buf);
                       v.asn1_serialize(writer).unwrap();
                   }
                   buf.clear()
               })
    }
}

