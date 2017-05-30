use info::{Asn1Info, Tag};
use ser;

asn1_info!(bool, UNIVERSAL 1, "BOOLEAN");

impl ser::Serialize for bool {
    fn asn1_serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        let tag = Self::asn1_tag().unwrap();
        self._asn1_serialize_tagged(serializer, tag)
    }

    fn _asn1_serialize_tagged<S: ser::Serializer>(&self, serializer: S, tag: &Tag) -> Result<S::Ok, S::Err> {
        serializer.serialize_boolean(tag, *self)
    }
}

#[cfg(test)]
mod tests {
    use test;
    use ::ser::{der, Serialize as Asn1Serialize};
    use ::info::{Asn1Info, Class};

    #[test]
    fn bool_tag() {
        let tag = bool::asn1_tag().unwrap();
        assert_eq!(tag.class(), Class::Universal, "tag class check failed");
        assert_eq!(tag.tagnum(), 0x01, "tag num check failed");
        assert_eq!(tag.is_constructed(), false, "tag constructed flag check failed");
        assert_eq!(tag.tag_byte(), 0x00, "tag byte check failed");
    }

    #[test]
    fn bool_encode() {
        let mut buf = der::Serializer::new();
        let test_set = [
            (true, vec![0x01, 0x01, 0xff]),
            (false, vec![0x01, 0x01, 0x00]),
        ];

        for &(v, ref expect) in &test_set {
            v.asn1_serialize(&mut buf).unwrap();
            assert_eq!(buf.as_slice(), expect.as_slice());
            buf.clear();
        }
    }

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