use info::{Tag, Len};

pub trait Error: Sized {
    fn invalid_tag() -> Self;
    fn invalid_value() -> Self;
    fn prim_untagged() -> Self;
    fn custom(message: &str) -> Self;
}

pub trait Serialize {
    fn asn1_serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err>;
}

pub trait SeqSerializer {
    type Ok;
    type Err: Error;

    fn serialize_field<V>(&mut self, value: &V) -> Result<(), Self::Err>
        where V: Serialize + ?Sized;

    fn finish(self) -> Result<Self::Ok, Self::Err>;
}

pub trait RawEncoder {
    type Ok;
    type Err;

    fn write_tag(&mut self, tag: &Tag) -> Result<Self::Ok, Self::Err>;
    fn write_length(&mut self, len: &Len) -> Result<Self::Ok, Self::Err>;

    fn write_base128(&mut self, v: u64) -> Result<Self::Ok, Self::Err>;

    fn write_byte(&mut self, byte: u8) -> Result<Self::Ok, Self::Err>;
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Err>;

    fn write_header(&mut self, tag: &Tag, len: &Len) -> Result<Self::Ok, Self::Err> {
        self.write_tag(tag)?;
        self.write_length(len)
    }
}

pub trait Serializer {
    type Ok;
    type Err: Error;

    // type RawEncoder: RawEncoder<Ok = Self::Ok, Err = Self::Err>;
    type SeqSerializer: SeqSerializer<Ok = Self::Ok, Err = Self::Err>;

    type ImplicitSerializer: Serializer<Ok = Self::Ok, Err = Self::Err>;
    type ExplicitSerializer: Serializer<Ok = Self::Ok, Err = Self::Err>;

    fn serialize_boolean(self, value: bool) -> Result<Self::Ok, Self::Err>;

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Err>;
    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Err>;
    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Err>;
    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Err>;
    fn serialize_isize(self, value: isize) -> Result<Self::Ok, Self::Err>;

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Err>;
    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Err>;
    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Err>;
    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Err>;
    fn serialize_usize(self, value: usize) -> Result<Self::Ok, Self::Err>;

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Err>;
    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Err>;

    fn serialize_bit_string(self, value: (u8, &[u8])) -> Result<Self::Ok, Self::Err>;
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Err>;
    fn serialize_null(self) -> Result<Self::Ok, Self::Err>;

    fn serialize_object_identifier(self, value: &[u64]) -> Result<Self::Ok, Self::Err>;

    // fn serialize_raw(self) -> Result<Self::RawEncoder, Self::Err>;
    fn serialize_tagged(self, tag: &Tag) -> Result<Self::ExplicitSerializer, Self::Err>;
    fn serialize_implicit(self, tag: &Tag) -> Result<Self::ImplicitSerializer, Self::Err>;
    fn serialize_sequence(self) -> Result<Self::SeqSerializer, Self::Err>;
}