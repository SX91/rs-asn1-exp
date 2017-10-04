use std::fmt;
use info::{Asn1Typed, Tag, Len};

pub trait Asn1Error {
    fn custom<T>(msg: T) -> Self
        where T: fmt::Display;
    fn invalid_type(descr: &'static str) -> Self;
    fn invalid_tag(descr: &'static str) -> Self;
    fn invalid_length(descr: &'static str) -> Self;
    fn invalid_value(descr: &'static str) -> Self;

    fn tag_mismatch(expected: Tag, got: Tag) -> Self;
    fn length_mismatch(expected: Len, got: Len) -> Self;
}

/// ASN.1 Asn1Deserialize trait allows for deserializing primitive and constructed ASN.1 encoded values into
/// Rust type, enum or struct.
pub trait Asn1Deserialize: Sized + Asn1Typed {
    /// Asn1Deserialize ASN.1 value.
    fn asn1_deserialize<'de, D>(deserializer: D) -> Result<Self, D::Err> where D: Asn1Deserializer<'de>;
}


pub trait Asn1Deserializer<'de> {
    type Err: Asn1Error;

    type ImplicitDeserializer: Asn1Deserializer<'de, Err=Self::Err>;
    type ExplicitDeserializer: Asn1Deserializer<'de, Err=Self::Err>;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;

    fn deserialize_bit_string<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
    fn deserialize_null<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;

    fn deserialize_object_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;

    fn deserialize_tagged(self, tag: Tag) -> Result<Self::ExplicitDeserializer, Self::Err>;
    fn deserialize_tagged_implicit(self, tag: Tag) -> Result<Self::ImplicitDeserializer, Self::Err>;

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;

    fn deserialize_choice<V>(self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>;
}


pub trait Asn1Visitor<'de>: Sized {
    type Value: Asn1Typed;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(Self::Value::asn1_type())
    }

    fn visit_bool<E>(self, _v: bool) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        Err(E::invalid_type("BOOLEAN"))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        self.visit_i64(v as i64)
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        self.visit_i64(v as i64)
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        self.visit_i64(v as i64)
    }
    fn visit_i64<E>(self, _v: i64) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        Err(E::invalid_type("INTEGER"))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        self.visit_u64(v as u64)
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        self.visit_u64(v as u64)
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        self.visit_u64(v as u64)
    }
    fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        Err(E::invalid_type("INTEGER"))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        self.visit_f64(v as f64)
    }
    fn visit_f64<E>(self, _v: f64) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        Err(E::invalid_type("REAL"))
    }

    fn visit_null<E>(self) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        Err(E::invalid_type("NULL"))
    }

    fn visit_object_identifier<E>(self, _v: Vec<u64>) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        Err(E::invalid_type("OBJECT IDENTIFIER"))
    }

    fn visit_bit_string<E>(self, _v: (u8, Vec<u8>)) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        Err(E::invalid_type("BIT STRING"))
    }

    fn visit_byte_string<E>(self, _v: Vec<u8>) -> Result<Self::Value, E>
        where E: Asn1Error
    {
        Err(E::invalid_type("BIT STRING"))
    }

    fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Err>
        where A: SeqAccess<'de>
    {
        Err(Asn1Error::invalid_type("SEQUENCE or SEQUENCE OF"))
    }

    fn visit_choice<A>(self, _tag: &Tag, _deserializer: A) -> Result<Self::Value, A::Err>
        where A: Asn1Deserializer<'de>
    {
        Err(Asn1Error::invalid_type("CHOICE"))
    }
}

pub trait VariantAccess<'de> {
    type Err: Asn1Error;

    fn peek_tag(self) -> Result<Tag, Self::Err>;
    fn variant<V>(self) -> Result<V, Self::Err> where V: Asn1Deserialize;
}

pub trait SeqAccess<'de> {
    type Err: Asn1Error;

    fn next_field<V>(&mut self) -> Result<V, Self::Err> where V: Asn1Deserialize;
    fn remaining(&self) -> u64;
}

