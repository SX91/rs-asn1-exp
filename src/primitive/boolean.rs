use info::{Asn1Tagged, Tag};
use ser;
use de;

asn1_info!(bool => UNIVERSAL 1, "BOOLEAN");

impl ser::Serialize for bool {
    fn asn1_serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        self._asn1_serialize_tagged(serializer, Self::asn1_tag())
    }

    fn _asn1_serialize_tagged<S: ser::Serializer>(&self,
                                                  serializer: S,
                                                  tag: &Tag)
                                                  -> Result<S::Ok, S::Err> {
        serializer.serialize_boolean(tag, *self)
    }
}

impl<'de> de::Deserialize<'de> for bool {
    fn asn1_deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Err> {
        Self::_asn1_deserialize_tagged(deserializer, Self::asn1_tag())
    }

    fn _asn1_deserialize_tagged<D: de::Deserializer<'de>>(deserializer: D,
                                                          tag: &Tag)
                                                          -> Result<Self, D::Err> {
        deserializer.deserialize_boolean(tag)
    }
}

#[cfg(test)]
mod tests {
    use test;
    use de::{self, Deserialize as Asn1Deserialize};
    use ser::{self, Serialize as Asn1Serialize};
    use info::{Asn1Tagged, Class};

    #[test]
    fn tag() {
        let tag = bool::asn1_tag();
        assert_eq!(tag.class(), Class::Universal, "tag class check failed");
        assert_eq!(tag.tagnum(), 0x01, "tag num check failed");
        assert_eq!(tag.is_constructed(),
                   false,
                   "tag constructed flag check failed");
        assert_eq!(tag.tag_byte(), 0x00, "tag byte check failed");
    }

    #[test]
    fn encode() {
        let mut buf = ser::der::Serializer::new();
        let test_set = [(true, vec![0x01, 0x01, 0xff]),
                        (false, vec![0x01, 0x01, 0x00])];

        for &(v, ref expect) in &test_set {
            v.asn1_serialize(&mut buf).unwrap();
            assert_eq!(buf.as_slice(), expect.as_slice());
            buf.clear();
        }
    }

    #[test]
    fn decode() {
        let test_set = [(true, vec![0x01, 0x01, 0xff]),
                        (false, vec![0x01, 0x01, 0x00])];
        for &(expect, ref bytes) in &test_set {
            let mut deser = de::der::Deserializer::new(&bytes[..]);
            let value = bool::asn1_deserialize(&mut deser).unwrap();
            assert_eq!(expect, value);
        }
    }
}

#[cfg(test)]
mod bench {
    use test;
    use ser::{der, Serialize as Asn1Serialize};
    use info::{Asn1Tagged, Class};

    #[bench]
    fn bool_bench(b: &mut test::Bencher) {

        let mut writer: der::Serializer<Vec<u8>> = der::Serializer::new();
        let v: bool = false;

        b.iter(|| {
                   v.asn1_serialize(&mut writer).unwrap();
                   writer.clear()
               })
    }
}

