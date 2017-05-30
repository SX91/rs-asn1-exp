use info::{Tag, Len};

pub trait Error {
    // add code here
}


pub trait Deserialize<'de>: Sized {
    fn asn1_deserialize<D>(deserializer: D) -> Result<Self, D::Err> where D: Deserializer<'de>;
    fn _asn1_deserialize_tagged<D>(deserializer: D, expected_tag: &Tag) -> Result<Self, D::Err>
        where D: Deserializer<'de>;
}


pub trait RawDecoder<'de> {
    type Err: Error;

    fn decode_tag(&mut self) -> Result<Tag, Self::Err>;
    fn decode_length(&mut self) -> Result<Len, Self::Err>;
    // fn decode_value(&mut self, len: usize) -> Result<&'de [u8], Self::Err>;

    fn decode_byte(&mut self) -> Result<u8, Self::Err>;
    fn decode_base128(&mut self) -> Result<u64, Self::Err>;

    // fn decode_nested<T, F>(&mut self, len: usize, f: F) -> Result<T, Self::Err>
    //     where F: FnMut(&mut Self) -> Result<T, Self::Err>;

    // fn decode_tlv(&mut self) -> Result<(Tag, Len, &'de [u8]), Self::Err>;
}


pub trait Deserializer<'de> {
    type Err: Error;

    type RawDecoder: RawDecoder<'de>;
    type NestedDeserializer: Deserializer<'de>;

    fn deserialize_boolean(self, expected_tag: &Tag) -> Result<bool, Self::Err>;

    fn deserialize_i8(self, expected_tag: &Tag) -> Result<i8, Self::Err>;
    fn deserialize_i16(self, expected_tag: &Tag) -> Result<i16, Self::Err>;
    fn deserialize_i32(self, expected_tag: &Tag) -> Result<i32, Self::Err>;
    fn deserialize_i64(self, expected_tag: &Tag) -> Result<i64, Self::Err>;
    fn deserialize_isize(self, expected_tag: &Tag) -> Result<isize, Self::Err>;

    fn deserialize_u8(self, expected_tag: &Tag) -> Result<u8, Self::Err>;
    fn deserialize_u16(self, expected_tag: &Tag) -> Result<u16, Self::Err>;
    fn deserialize_u32(self, expected_tag: &Tag) -> Result<u32, Self::Err>;
    fn deserialize_u64(self, expected_tag: &Tag) -> Result<u64, Self::Err>;
    fn deserialize_usize(self, expected_tag: &Tag) -> Result<usize, Self::Err>;

    fn deserialize_f32(self, tag: &Tag, value: f32) -> Result<f32, Self::Err>;
    fn deserialize_f64(self, tag: &Tag, value: f64) -> Result<f64, Self::Err>;

    fn deserialize_bit_string(self, expected_tag: &Tag) -> Result<(u8, Vec<u8>), Self::Err>;
    fn deserialize_bytes(self, expected_tag: &Tag) -> Result<Vec<u8>, Self::Err>;
    fn deserialize_null(self, expected_tag: &Tag) -> Result<(), Self::Err>;

    fn deserialize_raw(self) -> Result<Self::RawDecoder, Self::Err>;
    fn deserialize_constructed(self,
                               expected_tag: &Tag)
                               -> Result<Self::NestedDeserializer, Self::Err>;
}

