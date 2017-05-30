use info::{Asn1Tagged, Tag};
use ser;
use de;

macro_rules! encode_int {
    ($ty:ty => $($args:tt)+) => {
        impl ser::Serialize for $ty {
            fn asn1_serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
                self._asn1_serialize_tagged(serializer, Self::asn1_tag())
            }

            encode_int!{__impl $ty => $($args)*}
        }
    };
    (__impl $ty:ty => $ident:ident) => {
        fn _asn1_serialize_tagged<S: ser::Serializer>(&self, serializer: S, tag: &Tag) -> Result<S::Ok, S::Err> {
            serializer.$ident(tag, *self)
        }
    };
}

macro_rules! decode_int {
    ($ty:ty => $($args:tt)+) => {
        impl<'de> de::Deserialize<'de> for $ty {
            fn asn1_deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Err> {
                Self::_asn1_deserialize_tagged(deserializer, Self::asn1_tag())
            }

            decode_int!{__impl $ty => $($args)*}
        }
    };
    (__impl $ty:ty => $ident:ident) => {
        fn _asn1_deserialize_tagged<D: de::Deserializer<'de>>(deserializer: D, tag: &Tag) -> Result<Self, D::Err> {
            deserializer.$ident(tag)
        }
    };
}

asn1_info!(i8    => UNIVERSAL 0x02, "INTEGER");
asn1_info!(i16   => UNIVERSAL 0x02, "INTEGER");
asn1_info!(i32   => UNIVERSAL 0x02, "INTEGER");
asn1_info!(i64   => UNIVERSAL 0x02, "INTEGER");
asn1_info!(isize => UNIVERSAL 0x02, "INTEGER");

asn1_info!(u8    => UNIVERSAL 0x02, "INTEGER");
asn1_info!(u16   => UNIVERSAL 0x02, "INTEGER");
asn1_info!(u32   => UNIVERSAL 0x02, "INTEGER");
asn1_info!(u64   => UNIVERSAL 0x02, "INTEGER");
asn1_info!(usize => UNIVERSAL 0x02, "INTEGER");

encode_int!(i8 => serialize_i8);
encode_int!(i16 => serialize_i16);
encode_int!(i32 => serialize_i32);
encode_int!(i64 => serialize_i64);
encode_int!(isize => serialize_isize);

encode_int!(u8 => serialize_u8);
encode_int!(u16 => serialize_u16);
encode_int!(u32 => serialize_u32);
encode_int!(u64 => serialize_u64);
encode_int!(usize => serialize_usize);

decode_int!(i8 => deserialize_i8);
decode_int!(i16 => deserialize_i16);
decode_int!(i32 => deserialize_i32);
decode_int!(i64 => deserialize_i64);
decode_int!(isize => deserialize_isize);

decode_int!(u8 => deserialize_u8);
decode_int!(u16 => deserialize_u16);
decode_int!(u32 => deserialize_u32);
decode_int!(u64 => deserialize_u64);
decode_int!(usize => deserialize_usize);


#[cfg(test)]
mod tests {
    use std::io::{self, Read};

    use ser;
    use de;

    use self::ser::Serialize as Asn1Serialize;
    use self::de::Deserialize as Asn1Deserialize;

    fn ser_deser<T>(v: &T) -> T
        where T: Asn1Serialize + for<'de> Asn1Deserialize<'de>
    {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut serializer = ser::der::Serializer::from_vec(&mut buf);
            v.asn1_serialize(&mut serializer).unwrap();
        }
        let mut deserializer = de::der::Deserializer::new(&buf[..]);
        T::asn1_deserialize(&mut deserializer).unwrap()
    }

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
    use test;
    use ::ser::{der, Serialize as Asn1Serialize};

    #[bench]
    fn ser_i8(b: &mut test::Bencher) {

        let mut writer = der::Serializer::new();
        let v: i8 = -0x7f;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn ser_i16(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: i16 = -0x7fff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn ser_i32(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: i32 = -0x7fffffff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn ser_i64(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: i64 = -0x7fffffffffffffff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn ser_u8(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: u8 = 0x7f;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn ser_u16(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: u16 = 0x7fff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn ser_u32(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: u32 = 0x7fffffff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn ser_u64(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: u64 = 0x7fffffffffffffff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }
}