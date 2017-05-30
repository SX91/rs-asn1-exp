#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#![cfg_attr(test, feature(custom_attribute))]
#![feature(const_fn)]
#![feature(specialization)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[cfg(test)]
extern crate test;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
extern crate quickcheck_macros;

#[macro_use]
pub mod info;
pub mod ser;
pub mod de;
pub mod primitive;


#[cfg(test)]
mod tests {
    use test;
    use super::*;
    use super::info::{Tag, Asn1Tagged};
    use super::ser::{self, der, Serialize as Asn1Serialize, Serializer as Asn1Serializer,
                     RawEncoder as Asn1Raw, StructSerializer as Asn1StructSerializer};

    #[derive(Debug)]
    struct TestStruct(i8, i32, i32);
    asn1_info!(TestStruct => [APPLICATION 30], "TEST");

    #[derive(Debug)]
    struct TestStruct2<'a>(&'a TestStruct, &'a TestStruct);
    asn1_info!(TestStruct2<'a>: ('a) => [APPLICATION 31], "TEST2");

    impl ser::Serialize for TestStruct {
        fn asn1_serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Err> {
            self._asn1_serialize_tagged(s, Self::asn1_tag())
        }

        fn _asn1_serialize_tagged<S: ser::Serializer>(&self,
                                                      serializer: S,
                                                      tag: &Tag)
                                                      -> Result<S::Ok, S::Err> {
            let mut struct_serializer = serializer.serialize_constructed()?;
            struct_serializer.serialize_field(&self.0)?;
            struct_serializer.serialize_field(&self.1)?;
            struct_serializer.serialize_field(&self.2)?;
            struct_serializer.finish(tag)
        }
    }

    impl<'a> ser::Serialize for TestStruct2<'a> {
        fn asn1_serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Err> {
            self._asn1_serialize_tagged(s, Self::asn1_tag())
        }

        fn _asn1_serialize_tagged<S: ser::Serializer>(&self,
                                                      serializer: S,
                                                      tag: &Tag)
                                                      -> Result<S::Ok, S::Err> {
            let mut struct_serializer = serializer.serialize_constructed()?;
            struct_serializer.serialize_field(self.0)?;
            struct_serializer.serialize_field(self.1)?;
            struct_serializer.finish(tag)
        }
    }

    #[test]
    fn struct_to_asn1() {
        let mut writer = der::Serializer::new();

        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);
        test_obj.asn1_serialize(&mut writer).unwrap();

        assert_eq!(writer.as_slice(),
                   &[0x7e, 0x0e, 0x02, 0x01, 0x81, 0x02, 0x03, 0x80, 0x00, 0x01, 0x02, 0x04,
                     0x0f, 0xff, 0xff, 0xff]);
    }

    #[bench]
    fn bench_struct(b: &mut test::Bencher) {

        let mut writer = der::Serializer::new();
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);

        b.iter(|| {
                   test_obj.asn1_serialize(&mut writer).unwrap();
                   writer.clear()
               })
    }

    #[test]
    fn struct2_to_asn1() {
        let mut writer = der::Serializer::new();

        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);
        let test_obj2 = TestStruct2(&test_obj, &test_obj);
        test_obj2.asn1_serialize(&mut writer).unwrap();

        assert_eq!(writer.as_slice(),
                   vec![0x7fu8, 0x1f, 0x20, 0x7e, 0x0e, 0x02, 0x01, 0x81, 0x02, 0x03, 0x80, 0x00,
                        0x01, 0x02, 0x04, 0x0f, 0xff, 0xff, 0xff, 0x7e, 0x0e, 0x02, 0x01, 0x81,
                        0x02, 0x03, 0x80, 0x00, 0x01, 0x02, 0x04, 0x0f, 0xff, 0xff, 0xff]
                       .as_slice());
    }

    #[bench]
    fn bench_struct2(b: &mut test::Bencher) {

        let mut writer = der::Serializer::new();
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);
        let test_obj2 = TestStruct2(&test_obj, &test_obj);

        b.iter(|| {
                   test_obj2.asn1_serialize(&mut writer).unwrap();
                   writer.clear()
               })
    }

    #[bench]
    fn bench_taglen(b: &mut test::Bencher) {
        let mut writer = der::Serializer::new();

        let tag = info::Tag::new(info::Class::Universal, 0x02, false);
        let len = info::Len::new(0x7e);

        b.iter(|| {
                   let mut raw_writer = writer.serialize_raw().unwrap();
                   raw_writer.encode_tag(&tag).unwrap();
                   raw_writer.encode_len(&len).unwrap();
                   raw_writer.clear()
               })
    }
}

