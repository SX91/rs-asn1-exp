use info::{Tag, Len};

pub trait Error: Sized {
    fn invalid_tag() -> Self;
    fn invalid_value() -> Self;
    fn prim_untagged() -> Self;
    fn custom(message: &str) -> Self;
}

pub trait Serialize {
    fn asn1_serialize<S: Serializer>(&self, S) -> Result<S::Ok, S::Err>;
    fn _asn1_serialize_tagged<S: Serializer>(&self, serializer: S, tag: &Tag) -> Result<S::Ok, S::Err>;
}

pub trait StructSerializer {
    type Ok;
    type Err: Error;

    fn serialize_field<V>(&mut self, value: &V) -> Result<(), Self::Err>
        where V: Serialize + ?Sized;
    fn finish(self, tag: &Tag) -> Result<Self::Ok, Self::Err>;
}

pub trait RawEncoder {
    type Ok;
    type Err;

    fn encode_tag(&mut self, tag: &Tag) -> Result<Self::Ok, Self::Err>;
    fn encode_len(&mut self, len: &Len) -> Result<Self::Ok, Self::Err>;

    fn encode_base128(&mut self, v: u64) -> Result<Self::Ok, Self::Err>;

    fn encode_byte(&mut self, byte: u8) -> Result<Self::Ok, Self::Err>;
    fn encode_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Err>;

    fn encode_header(&mut self, tag: &Tag, len: &Len) -> Result<Self::Ok, Self::Err> {
        self.encode_tag(tag)?;
        self.encode_len(len)
    }
}

pub trait Serializer {
    type Ok;
    type Err: Error;

    type RawEncoder: RawEncoder<Ok = Self::Ok, Err = Self::Err>;
    type StructSerializer: StructSerializer<Ok = Self::Ok, Err = Self::Err>;

    fn serialize_boolean(self, tag: &Tag, value: bool) -> Result<Self::Ok, Self::Err>;


    fn serialize_i8(self, tag: &Tag, value: i8) -> Result<Self::Ok, Self::Err>;
    fn serialize_i16(self, tag: &Tag, value: i16) -> Result<Self::Ok, Self::Err>;
    fn serialize_i32(self, tag: &Tag, value: i32) -> Result<Self::Ok, Self::Err>;
    fn serialize_i64(self, tag: &Tag, value: i64) -> Result<Self::Ok, Self::Err>;
    fn serialize_isize(self, tag: &Tag, value: isize) -> Result<Self::Ok, Self::Err>;

    fn serialize_u8(self, tag: &Tag, value: u8) -> Result<Self::Ok, Self::Err>;
    fn serialize_u16(self, tag: &Tag, value: u16) -> Result<Self::Ok, Self::Err>;
    fn serialize_u32(self, tag: &Tag, value: u32) -> Result<Self::Ok, Self::Err>;
    fn serialize_u64(self, tag: &Tag, value: u64) -> Result<Self::Ok, Self::Err>;
    fn serialize_usize(self, tag: &Tag, value: usize) -> Result<Self::Ok, Self::Err>;

    fn serialize_f32(self, tag: &Tag, value: f32) -> Result<Self::Ok, Self::Err>;
    fn serialize_f64(self, tag: &Tag, value: f64) -> Result<Self::Ok, Self::Err>;

    fn serialize_bit_string(self, tag: &Tag, value: (u8, &[u8])) -> Result<Self::Ok, Self::Err>;
    fn serialize_bytes(self, tag: &Tag, value: &[u8]) -> Result<Self::Ok, Self::Err>;
    fn serialize_null(self, tag: &Tag) -> Result<Self::Ok, Self::Err>;

    fn serialize_object_identifier(self, tag: &Tag, value: &[u64]) -> Result<Self::Ok, Self::Err>;

    fn serialize_raw(self) -> Result<Self::RawEncoder, Self::Err>;
    fn serialize_constructed(self) -> Result<Self::StructSerializer, Self::Err>;
}