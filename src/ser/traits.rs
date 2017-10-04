use info::Tag;

pub trait Error: Sized {
    fn invalid_tag() -> Self;
    fn invalid_value() -> Self;
    fn prim_untagged() -> Self;
    fn custom(message: &str) -> Self;
}

pub trait Asn1Serialize {
    fn asn1_serialize<S: Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err>;
}

pub trait Asn1Serializer {
    type Ok;
    type Err: Error;

    type SeqSerializer: SeqSerializer<Ok = Self::Ok, Err = Self::Err>;
    type ImplicitSerializer: Asn1Serializer<Ok = Self::Ok, Err = Self::Err>;
    type ExplicitSerializer: Asn1Serializer<Ok = Self::Ok, Err = Self::Err>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Err>;

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

    fn serialize_tagged(self, tag: Tag) -> Result<Self::ExplicitSerializer, Self::Err>;
    fn serialize_implicit(self, tag: Tag) -> Result<Self::ImplicitSerializer, Self::Err>;
    fn serialize_sequence(self) -> Result<Self::SeqSerializer, Self::Err>;
}

pub trait SeqSerializer {
    type Ok;
    type Err: Error;

    fn serialize_field<V>(&mut self, value: &V) -> Result<(), Self::Err>
        where V: Asn1Serialize + ?Sized;

    fn finish(self) -> Result<Self::Ok, Self::Err>;
}

