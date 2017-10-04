use info;
use ser;
use de::{self, Asn1Visitor, Asn1Error};

asn1_info!(i8    => info::TAG_INTEGER, info::TYPE_INTEGER);
asn1_info!(i16   => info::TAG_INTEGER, info::TYPE_INTEGER);
asn1_info!(i32   => info::TAG_INTEGER, info::TYPE_INTEGER);
asn1_info!(i64   => info::TAG_INTEGER, info::TYPE_INTEGER);
asn1_info!(isize => info::TAG_INTEGER, info::TYPE_INTEGER);

asn1_info!(u8    => info::TAG_INTEGER, info::TYPE_INTEGER);
asn1_info!(u16   => info::TAG_INTEGER, info::TYPE_INTEGER);
asn1_info!(u32   => info::TAG_INTEGER, info::TYPE_INTEGER);
asn1_info!(u64   => info::TAG_INTEGER, info::TYPE_INTEGER);
asn1_info!(usize => info::TAG_INTEGER, info::TYPE_INTEGER);

impl ser::Asn1Serialize for i8 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i8(*self)
    }
}

impl ser::Asn1Serialize for i16 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i16(*self)
    }
}

impl ser::Asn1Serialize for i32 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i32(*self)
    }
}

impl ser::Asn1Serialize for i64 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i64(*self)
    }
}

impl ser::Asn1Serialize for isize {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i64(*self as i64)
    }
}

impl ser::Asn1Serialize for u8 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u8(*self)
    }
}

impl ser::Asn1Serialize for u16 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u16(*self)
    }
}

impl ser::Asn1Serialize for u32 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u32(*self)
    }
}

impl ser::Asn1Serialize for u64 {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u64(*self)
    }
}

impl ser::Asn1Serialize for usize {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u64(*self as u64)
    }
}

impl de::Asn1Deserialize for i8 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = i8;

            fn visit_i8<E: Asn1Error>(self, v: i8) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_i8(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for i16 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = i16;

            fn visit_i16<E: Asn1Error>(self, v: i16) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_i16(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for i32 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = i32;

            fn visit_i32<E: Asn1Error>(self, v: i32) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_i32(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for i64 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = i64;

            fn visit_i64<E: Asn1Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_i64(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for isize {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = isize;

            fn visit_i64<E: Asn1Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(v as isize)
            }
        }
        deserializer.deserialize_i64(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for u8 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = u8;

            fn visit_u8<E: Asn1Error>(self, v: u8) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_u8(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for u16 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = u16;

            fn visit_u16<E: Asn1Error>(self, v: u16) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_u16(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for u32 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = u32;

            fn visit_u32<E: Asn1Error>(self, v: u32) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_u32(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for u64 {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = u64;

            fn visit_u64<E: Asn1Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(v)
            }
        }
        deserializer.deserialize_u64(IntegerVisitor)
    }
}

impl de::Asn1Deserialize for usize {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct IntegerVisitor;
        impl<'de> Asn1Visitor<'de> for IntegerVisitor {
            type Value = usize;

            fn visit_u64<E: Asn1Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(v as usize)
            }
        }
        deserializer.deserialize_u64(IntegerVisitor)
    }
}


#[cfg(test)]
mod tests {
    use universal::test_helper::ser_deser;

    #[quickcheck]
    fn i8(i: i8) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn i16(i: i16) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn i32(i: i32) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn i64(i: i64) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn isize(i: isize) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn u8(i: u8) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn u16(i: u16) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn u32(i: u32) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn u64(i: u64) -> bool {
        i == ser_deser(&i)
    }

    #[quickcheck]
    fn usize(i: usize) -> bool {
        i == ser_deser(&i)
    }
}

#[cfg(test)]
mod bench {
    use std::io;
    use std::fmt::Debug;
    use test;

    use der;
    use ser::Asn1Serialize;
    use de::Asn1Deserialize;

    #[inline]
    fn ser_helper<T: Asn1Serialize>(b: &mut test::Bencher, v: T) {
        let mut buf = Vec::with_capacity(128);

        b.iter(|| {
                   {
                       let writer = der::Serializer::new(&mut buf);
                       v.asn1_serialize(writer).unwrap();
                   }
                   buf.clear()
               })
    }

    fn serialize<T: Asn1Serialize>(v: &T) -> Vec<u8> {
        let mut buf = Vec::new();

        {
            let writer = der::Serializer::new(&mut buf);
            v.asn1_serialize(writer).unwrap();
        }

        buf
    }

    #[inline]
    fn de_helper<T>(b: &mut test::Bencher, v: T)
        where T: PartialEq + Debug + Asn1Serialize + Asn1Deserialize
    {
        let bytes = serialize(&v);
        let mut cur = io::Cursor::new(bytes.as_slice());

        b.iter(move || {
                   let new_v = {
                       let reader = der::Deserializer::new(&mut cur);
                       T::asn1_deserialize(reader).unwrap()
                   };
                   cur.set_position(0);
                   assert_eq!(v, new_v)
               })
    }

    #[bench]
    fn ser_i8(b: &mut test::Bencher) {
        let v: i8 = -0x7f;
        ser_helper(b, v)
    }

    #[bench]
    fn ser_i16(b: &mut test::Bencher) {
        let v: i16 = -0x7fff;
        ser_helper(b, v)
    }

    #[bench]
    fn ser_i32(b: &mut test::Bencher) {
        let v: i32 = -0x7fffffff;
        ser_helper(b, v)
    }

    #[bench]
    fn ser_i64(b: &mut test::Bencher) {
        let v: i64 = -0x7fffffffffffffff;
        ser_helper(b, v)
    }

    #[bench]
    fn ser_u8(b: &mut test::Bencher) {
        let v: u8 = 0x7f;
        ser_helper(b, v)
    }

    #[bench]
    fn ser_u16(b: &mut test::Bencher) {
        let v: u16 = 0x7fff;
        ser_helper(b, v)
    }

    #[bench]
    fn ser_u32(b: &mut test::Bencher) {

        let v: u32 = 0x7fffffff;
        ser_helper(b, v)
    }

    #[bench]
    fn ser_u64(b: &mut test::Bencher) {
        let v: u64 = 0x7fffffffffffffff;
        ser_helper(b, v)
    }

    #[bench]
    fn de_i8(b: &mut test::Bencher) {
        let v: i8 = -0x7f;
        de_helper(b, v)
    }

    #[bench]
    fn de_i16(b: &mut test::Bencher) {
        let v: i16 = -0x7fff;
        de_helper(b, v)
    }

    #[bench]
    fn de_i32(b: &mut test::Bencher) {
        let v: i32 = -0x7fffffff;
        de_helper(b, v)
    }

    #[bench]
    fn de_i64(b: &mut test::Bencher) {
        let v: i64 = -0x7fffffffffffffff;
        de_helper(b, v)
    }

    #[bench]
    fn de_u8(b: &mut test::Bencher) {
        let v: u8 = 0x7f;
        de_helper(b, v)
    }

    #[bench]
    fn de_u16(b: &mut test::Bencher) {
        let v: u16 = 0x7fff;
        de_helper(b, v)
    }

    #[bench]
    fn de_u32(b: &mut test::Bencher) {
        let v: u32 = 0x7fffffff;
        de_helper(b, v)
    }

    #[bench]
    fn de_u64(b: &mut test::Bencher) {
        let v: u64 = 0x7fffffffffffffff;
        de_helper(b, v)
    }
}

