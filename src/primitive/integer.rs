use info::{Asn1Info, Class, Tag};
use ser;

macro_rules! encode_int {
    ($ty:ty => $($args:tt)+) => {
        impl ser::Serialize for $ty {
            fn asn1_serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
                let tag = Self::asn1_tag().unwrap();
                self._asn1_serialize_tagged(serializer, &tag)
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

asn1_info!(i8,    Class::Universal, 0x02, false, "INTEGER");
asn1_info!(i16,   Class::Universal, 0x02, false, "INTEGER");
asn1_info!(i32,   Class::Universal, 0x02, false, "INTEGER");
asn1_info!(i64,   Class::Universal, 0x02, false, "INTEGER");
asn1_info!(isize, Class::Universal, 0x02, false, "INTEGER");

asn1_info!(u8,    Class::Universal, 0x02, false, "INTEGER");
asn1_info!(u16,   Class::Universal, 0x02, false, "INTEGER");
asn1_info!(u32,   Class::Universal, 0x02, false, "INTEGER");
asn1_info!(u64,   Class::Universal, 0x02, false, "INTEGER");
asn1_info!(usize, Class::Universal, 0x02, false, "INTEGER");

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


#[cfg(test)]
mod tests {
    use test;
    use ::ser::{der, Serialize as Asn1Serialize};

    #[bench]
    fn bench_i8(b: &mut test::Bencher) {

        let mut writer = der::Serializer::new();
        let v: i8 = -0x7f;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn bench_i16(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: i16 = -0x7fff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn bench_i32(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: i32 = -0x7fffffff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn bench_i64(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: i64 = -0x7fffffffffffffff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn bench_u8(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: u8 = 0x7f;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn bench_u16(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: u16 = 0x7fff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn bench_u32(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: u32 = 0x7fffffff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }

    #[bench]
    fn bench_u64(b: &mut test::Bencher) {
        
        let mut writer = der::Serializer::new();
        let v: u64 = 0x7fffffffffffffff;

        b.iter(|| {
            v.asn1_serialize(&mut writer).unwrap();
            writer.clear()
        })
    }
}