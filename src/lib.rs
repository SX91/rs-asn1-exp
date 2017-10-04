#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#![cfg_attr(test, feature(custom_attribute))]
#![feature(const_fn)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "with-serde")]
extern crate serde_bytes;

#[cfg(test)]
extern crate test;

#[cfg(test)]
extern crate quickcheck;

#[macro_use]
pub mod info;
pub mod ser;
pub mod de;
pub mod der;
pub mod universal;

pub use info::{ContentType, Tag, Len, Asn1Tagged, Asn1Typed, Asn1DisplayExt};
pub use ser::{Asn1Serialize, Asn1Serializer, SeqSerializer};
pub use de::{Asn1Deserialize, Asn1Deserializer, Asn1Visitor, SeqAccess};
pub use universal::{ObjectIdentifier, OctetString, BitString};

pub fn to_asn1<T: Asn1Serialize>(value: &T) -> Result<Vec<u8>, der::EncodeError> {
    let mut buf: Vec<u8> = Vec::with_capacity(128);
    {
        let serializer = der::Serializer::new(&mut buf);
        value.asn1_serialize(serializer)?;
    }
    Ok(buf)
}

pub fn from_asn1<T: Asn1Deserialize>(buf: &[u8]) -> Result<T, der::DecodeError> {
    let deserializer = der::Deserializer::new(buf);
    T::asn1_deserialize(deserializer)
}

#[macro_export]
macro_rules! asn1_newtype {
    ($ty:ident ::= $inner:ty) => (
        asn1_alias_info!($ty ::= $inner);
        asn1_alias_ser!($ty ::= $inner);
        asn1_alias_de!($ty ::= $inner);
    )
}

#[macro_export]
macro_rules! asn1_alias {
    ($ty:ident ::= [$($args:tt)*] $tagging:tt $pty:ty, $asn1_type:expr) => (
        asn1_alias_info!($ty ::= [$($args)*] $pty, $asn1_type);
        asn1_alias_ser!($ty ::= $tagging $pty);
        asn1_alias_de!($ty ::= $tagging $pty);
    );
    ($ty:ident ::= $pty:ident, $asn1_type:expr) => (
        asn1_alias_info!($ty ::= $pty, $asn1_type);
        asn1_alias_ser!($ty ::= $pty);
        asn1_alias_de!($ty ::= $pty);
    );
}

#[macro_export]
macro_rules! asn1_alias_info {
    ($ty:ident ::= [$($args:tt)*] $pty:ty, $asn1_type:expr) => (
        asn1_tagged!($ty, [$($args)*]);
        asn1_typed!($ty, $asn1_type);
    );
    ($ty:ident ::= $pty:ty, $asn1_type:expr) => (
        impl $crate::Asn1Tagged for $ty {
            fn asn1_tag() -> $crate::Tag {
                <$pty as $crate::Asn1Tagged>::asn1_tag()
            }
        }
        asn1_typed!($ty, $asn1_type);
    );
    ($ty:ident ::= $pty:ty) => (
        impl $crate::Asn1Tagged for $ty {
            fn asn1_tag() -> $crate::Tag {
                <$pty as $crate::Asn1Tagged>::asn1_tag()
            }
        }
        impl $crate::Asn1Typed for $ty {
            fn asn1_type() -> &'static str {
                <$pty as $crate::Asn1Typed>::asn1_type()
            }
        }
    );
}

#[macro_export]
macro_rules! asn1_alias_ser {
    ($ty:ident ::= IMPLICIT $pty:ty) => (
        impl $crate::Asn1Serialize for $ty {
            fn asn1_serialize<S: $crate::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
                self.0.asn1_serialize(serializer.serialize_implicit(Self::asn1_tag())?)
            }
        }
    );
    ($ty:ident ::= EXPLICIT $pty:ty) => (
        impl $crate::Asn1Serialize for $ty {
            fn asn1_serialize<S: $crate::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
                self.0.asn1_serialize(serializer.serialize_tagged(Self::asn1_tag())?)
            }
        }
    );
    ($ty:ident ::= $pty:ty) => (
        impl $crate::Asn1Serialize for $ty {
            fn asn1_serialize<S: $crate::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
                self.0.asn1_serialize(serializer)
            }
        }
    );
}

#[macro_export]
macro_rules! asn1_alias_de {
    ($ty:ident ::= IMPLICIT $pty:ty) => (
        impl $crate::Asn1Deserialize for $ty {
            fn asn1_deserialize<'de, D: $crate::Asn1Deserializer<'de>>(deserializer: D)
                                                                -> Result<Self, D::Err> {
                $crate::Asn1Deserialize::asn1_deserialize(deserializer.deserialize_tagged_implicit(Self::asn1_tag())?).map($ty)
            }
        }
    );
    ($ty:ident ::= EXPLICIT $pty:ty) => (
        impl $crate::Asn1Deserialize for $ty {
            fn asn1_deserialize<'de, D: $crate::Asn1Deserializer<'de>>(deserializer: D)
                                                                -> Result<Self, D::Err> {
                $crate::Asn1Deserialize::asn1_deserialize(deserializer.deserialize_tagged(Self::asn1_tag())?).map($ty)
            }
        }
    );
    ($ty:ident ::= $pty:ty) => (
        impl $crate::Asn1Deserialize for $ty {
            fn asn1_deserialize<'de, D: $crate::Asn1Deserializer<'de>>(deserializer: D)
                                                                -> Result<Self, D::Err> {
                $crate::Asn1Deserialize::asn1_deserialize(deserializer).map($ty)
            }
        }
    );
}

struct TestNewtype(i32);
asn1_newtype!(TestNewtype ::= i32);

struct TestAlias(i32);
asn1_alias!(TestAlias ::= i32, "MY ALIAS");

struct TestTagged(i32);
asn1_alias!(TestTagged ::= [APPLICATION 3] IMPLICIT i32, "TEST TAGGED");

#[cfg(test)]
mod tests {
    use std::fmt;
    use test;
    use der;

    use super::info::{Tag, Asn1Tagged};
    use super::ser::{self, Asn1Serialize, Asn1Serializer, SeqSerializer as Asn1SeqSerializer};
    use super::de::{self, Asn1Error, Asn1Deserialize, Asn1Deserializer, Asn1Visitor, SeqAccess};

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
    struct TestStruct(i8, i32, i32);
    asn1_info!(TestStruct => [APPLICATION 30], "TEST");

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
    struct TestStruct2(TestStruct, TestStruct);
    asn1_info!(TestStruct2 => [APPLICATION 31], "TEST2");

    impl ser::Asn1Serialize for TestStruct {
        fn asn1_serialize<S: ser::Asn1Serializer>(&self, s: S) -> Result<S::Ok, S::Err> {
            let mut s = s.serialize_tagged(Self::asn1_tag())?.serialize_sequence()?;
            s.serialize_field(&self.0)?;
            s.serialize_field(&self.1)?;
            s.serialize_field(&self.2)?;
            s.finish()
        }
    }

    impl de::Asn1Deserialize for TestStruct {
        fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                               -> Result<Self, D::Err> {
            struct SeqVisitor;
            impl<'de> Asn1Visitor<'de> for SeqVisitor {
                type Value = TestStruct;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("an integer between -2^31 and 2^31")
                }

                #[inline]
                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Err>
                    where A: SeqAccess<'de>
                {
                    Ok(TestStruct(seq.next_field()?, seq.next_field()?, seq.next_field()?))
                }
            }


            deserializer
                .deserialize_tagged(TestStruct::asn1_tag())?
                .deserialize_seq(SeqVisitor)
        }
    }

    impl<'a> ser::Asn1Serialize for TestStruct2 {
        fn asn1_serialize<S: ser::Asn1Serializer>(&self, s: S) -> Result<S::Ok, S::Err> {
            let mut s = s.serialize_tagged(Self::asn1_tag())?.serialize_sequence()?;
            s.serialize_field(&self.0)?;
            s.serialize_field(&self.1)?;
            s.finish()
        }
    }

    impl de::Asn1Deserialize for TestStruct2 {
        fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                               -> Result<Self, D::Err> {
            struct SeqVisitor;
            impl<'de> Asn1Visitor<'de> for SeqVisitor {
                type Value = TestStruct2;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("an integer between -2^31 and 2^31")
                }

                #[inline]
                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Err>
                    where A: SeqAccess<'de>
                {
                    Ok(TestStruct2(seq.next_field()?, seq.next_field()?))
                }
            }
            deserializer
                .deserialize_tagged(TestStruct2::asn1_tag())?
                .deserialize_seq(SeqVisitor)
        }
    }

    #[test]
    fn choice() {
        let mut buf = Vec::with_capacity(128);
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);

        {
            let writer = der::Serializer::new(&mut buf);
            test_obj.asn1_serialize(writer).unwrap();
        }

        let reader = der::Deserializer::new(buf.as_slice());
        struct MyVisitor;
        impl<'de> de::Asn1Visitor<'de> for MyVisitor {
            type Value = TestStruct;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("TestStruct")
            }

            fn visit_choice<A>(self, tag: &Tag, deserializer: A) -> Result<Self::Value, A::Err>
                where A: Asn1Deserializer<'de>
            {
                if *tag == TestStruct::asn1_tag() {
                    TestStruct::asn1_deserialize(deserializer)
                } else {
                    Err(Asn1Error::invalid_tag(""))
                }
            }
        }

        let actual_obj = reader.deserialize_choice(MyVisitor).unwrap();

        assert_eq!(test_obj, actual_obj);
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

    #[test]
    fn struct_from_asn1() {
        let buf: Vec<u8> = vec![0x7e, 0x0e, 0x02, 0x01, 0x81, 0x02, 0x03, 0x80, 0x00, 0x01, 0x02,
                                0x04, 0x0f, 0xff, 0xff, 0xff];
        let reader = der::Deserializer::new(&buf[..]);

        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);
        assert_eq!(TestStruct::asn1_deserialize(reader).unwrap(), test_obj);
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

    #[bench]
    fn bench_struct_de(b: &mut test::Bencher) {
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);

        let mut buf = Vec::with_capacity(128);
        {
            let writer = der::Serializer::new(&mut buf);
            test_obj.asn1_serialize(writer).unwrap();
        }

        b.iter(|| {
                   let reader = der::Deserializer::new(buf.as_slice());
                   assert_eq!(TestStruct::asn1_deserialize(reader).unwrap(), test_obj)
               })
    }

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

    #[bench]
    fn bench_seq_of_i64_de(b: &mut test::Bencher) {
        let mut buf = Vec::with_capacity(512);
        let mut test_seq: Vec<i64> = Vec::with_capacity(64);

        for i in 0..64 {
            test_seq.push(i * 1024);
        }

        {
            let writer = der::Serializer::new(&mut buf);
            test_seq.asn1_serialize(writer).unwrap();
        }

        b.iter(|| {
                   let reader = der::Deserializer::new(buf.as_slice());
                   Vec::<i64>::asn1_deserialize(reader).unwrap()
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
    fn bench_struct2_de(b: &mut test::Bencher) {
        let test_obj = TestStruct(-127, -0x7fffff, 0x0fffffff);
        let test_obj2 = TestStruct2(test_obj, test_obj);

        let mut buf = Vec::with_capacity(128);
        {
            let writer = der::Serializer::new(&mut buf);
            test_obj2.asn1_serialize(writer).unwrap();
        }

        b.iter(|| {
                   let _ = {
                       let reader = der::Deserializer::new(buf.as_slice());
                       TestStruct2::asn1_deserialize(reader).unwrap();
                   };
               })
    }
}

