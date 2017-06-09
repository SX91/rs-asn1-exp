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
                     RawEncoder as Asn1Raw, SeqSerializer as Asn1SeqSerializer};

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
    struct TestStruct(i8, i32, i32);
    asn1_info!(TestStruct => [APPLICATION 30], "TEST");

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
    struct TestStruct2(TestStruct, TestStruct);
    asn1_info!(TestStruct2 => [APPLICATION 31], "TEST2");

    impl ser::Serialize for TestStruct {
        fn asn1_serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Err> {
            let mut s = s.serialize_implicit(Self::asn1_tag())?
                .serialize_sequence()?;
            s.serialize_field(&self.0)?;
            s.serialize_field(&self.1)?;
            s.serialize_field(&self.2)?;
            s.finish()
        }
    }

        }
    }

    impl<'a> ser::Serialize for TestStruct2 {
        fn asn1_serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Err> {
            let mut s = s.serialize_implicit(Self::asn1_tag())?
                .serialize_sequence()?;
            s.serialize_field(&self.0)?;
            s.serialize_field(&self.1)?;
            s.finish()
        }
    }

    impl de::Asn1Deserialize for TestStruct2 {
        fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                               -> Result<Self, D::Err> {
            Self::_asn1_deserialize_tagged(deserializer, Self::asn1_tag())
        }
        }
    }

    #[test]
    fn struct_to_asn1() {
        let mut buf = Vec::with_capacity(128);
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);

        {
            let writer = der::Serializer::new(&mut buf);
            test_obj.asn1_serialize(writer).unwrap();
        }

        assert_eq!(buf.as_slice(),
                   &[0x7e, 0x0e, 0x02, 0x01, 0x81, 0x02, 0x03, 0x80, 0x00, 0x01, 0x02, 0x04,
                     0x0f, 0xff, 0xff, 0xff]);
    }

    #[bench]
    fn bench_struct_ser(b: &mut test::Bencher) {
        let mut buf = Vec::with_capacity(128);
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);

        b.iter(|| {
                   {
                       let writer = der::Serializer::new(&mut buf);
                       test_obj.asn1_serialize(writer).unwrap();
                   }
                   buf.clear()
               })
    }

        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);

    #[bench]
    fn bench_seq_of_struct2_ser(b: &mut test::Bencher) {
        let mut buf = Vec::with_capacity(512);

        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);
        let test_obj2 = TestStruct2(test_obj, test_obj);
        let mut test_seq: Vec<TestStruct2> = Vec::new();
        for _ in 0..64 {
            test_seq.push(test_obj2)
        }

        b.iter(|| {
                   {
                       let writer = der::Serializer::new(&mut buf);
                       test_seq.asn1_serialize(writer).unwrap();
                   }
                   buf.clear()
               })
    }

    #[bench]
    fn bench_seq_of_i64_ser(b: &mut test::Bencher) {
        let mut buf = Vec::with_capacity(512);
        let mut test_seq: Vec<i64> = Vec::with_capacity(64);

        for i in 0..64 {
            test_seq.push(i * 1024);
        }

        b.iter(|| {
                   {
                       let writer = der::Serializer::new(&mut buf);
                       test_seq.asn1_serialize(writer).unwrap();
                   }
                   buf.clear()
               })
    }

    #[test]
    fn struct2_to_asn1() {
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);
        let test_obj2 = TestStruct2(test_obj, test_obj);

        let mut buf = Vec::with_capacity(128);
        {
            let writer = der::Serializer::new(&mut buf);
            test_obj2.asn1_serialize(writer).unwrap();
        }
        assert_eq!(buf.as_slice(),
                   vec![0x7fu8, 0x1f, 0x20, 0x7e, 0x0e, 0x02, 0x01, 0x81, 0x02, 0x03, 0x80, 0x00,
                        0x01, 0x02, 0x04, 0x0f, 0xff, 0xff, 0xff, 0x7e, 0x0e, 0x02, 0x01, 0x81,
                        0x02, 0x03, 0x80, 0x00, 0x01, 0x02, 0x04, 0x0f, 0xff, 0xff, 0xff]
                       .as_slice());
    }

    #[bench]
    fn bench_struct2_ser(b: &mut test::Bencher) {
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);
        let test_obj2 = TestStruct2(test_obj, test_obj);

        let mut buf = Vec::with_capacity(128);

        b.iter(|| {
                   {
                       let writer = der::Serializer::new(&mut buf);
                       test_obj2.asn1_serialize(writer).unwrap();
                   }
                   buf.clear()
               })
    }

    #[bench]


        b.iter(|| {
               })
    }

    // #[bench]
    // fn bench_taglen(b: &mut test::Bencher) {
    //     let mut buf = Vec::with_capacity(128);

    //     let tag = info::Tag::new(info::Class::Universal, 0x02, false);
    //     let len = info::Len::new(0x7e);

    //     b.iter(move || {
    //                {
    //                    let mut raw_writer = der::Serializer::new(&mut buf).serialize_raw().unwrap();
    //                    raw_writer.write_tag(&tag).unwrap();
    //                    raw_writer.write_length(&len).unwrap();
    //                }
    //                buf.clear()
    //            })
    // }
}

