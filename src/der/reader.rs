use std::{self, io};
use std::io::Read;

use info::{self, Tag, Len};
use de::{self, Asn1Visitor, Asn1Deserialize, Asn1Deserializer, Asn1Error};

use super::read;

pub trait ReadCountExt: io::Read {
    fn bytes_read(&self) -> usize;
}


#[derive(Debug)]
pub enum DecodeError {
    InvalidType(&'static str),
    InvalidTag(&'static str),
    TagMismatch(Tag, Tag),
    InvalidLength(&'static str),
    LengthMismatch(Len, Len),
    InvalidValue(&'static str),
    ConstructedNotConsumed,
    Custom(String),
    IO(std::io::Error),
}

impl From<io::Error> for DecodeError {
    fn from(e: io::Error) -> Self {
        DecodeError::IO(e)
    }
}

impl From<read::ReadError> for DecodeError {
    fn from(e: read::ReadError) -> Self {
        use self::read::ReadError;
        match e {
            ReadError::InvalidTag => DecodeError::InvalidTag("bad tag encoding"),
            ReadError::InvalidLength => DecodeError::InvalidLength("bad length encoding"),
            ReadError::InvalidValue => DecodeError::InvalidValue("bad value encoding"),
            ReadError::IoError(err) => DecodeError::IO(err),
        }
    }
}

impl Asn1Error for DecodeError {
    fn custom<T>(msg: T) -> Self
        where T: std::fmt::Display
    {

        DecodeError::Custom(msg.to_string())
    }

    fn invalid_type(msg: &'static str) -> Self {
        DecodeError::InvalidType(msg)
    }

    fn invalid_tag(msg: &'static str) -> Self {
        DecodeError::InvalidTag(msg)
    }

    fn invalid_length(msg: &'static str) -> Self {
        DecodeError::InvalidLength(msg)
    }

    fn invalid_value(msg: &'static str) -> Self {
        DecodeError::InvalidValue(msg)
    }

    fn tag_mismatch(expected: Tag, got: Tag) -> Self {
        DecodeError::TagMismatch(expected, got)
    }

    fn length_mismatch(expected: Len, got: Len) -> Self {
        DecodeError::LengthMismatch(expected, got)
    }
}


#[derive(Debug)]
pub struct Deserializer<R: io::Read> {
    inner: R,
    peeked_tag: Option<Tag>,
    implicit_tag: Option<Tag>,
}

#[derive(Debug)]
struct SeqAccessor<R: io::Read> {
    inner: io::Take<R>,
}

impl<'de, R: io::Read + 'de> SeqAccessor<R> {
    fn new(nested: io::Take<R>) -> Self {
        SeqAccessor { inner: nested }
    }

    fn next(&mut self) -> Deserializer<&mut io::Take<R>> {
        Deserializer::new(&mut self.inner)
    }
}

impl<'de, R: io::Read> de::SeqAccess<'de> for SeqAccessor<R> {
    type Err = DecodeError;

    fn next_field<V>(&mut self) -> Result<V, Self::Err>
        where V: Asn1Deserialize
    {
        V::asn1_deserialize(self.next())
    }

    fn remaining(&self) -> u64 {
        self.inner.limit()
    }
}

impl<'de, R: io::Read> Deserializer<R> {
    pub fn new(reader: R) -> Self {
        Deserializer {
            inner: reader,
            peeked_tag: None,
            implicit_tag: None,
        }
    }

    pub fn with_tag(self, tag: Tag) -> Self {
        Deserializer {
            inner: self.inner,
            peeked_tag: self.peeked_tag,
            implicit_tag: Some(tag),
        }
    }

    pub fn override_tag<T, F>(&mut self, default_tag: Tag, f: F) -> Result<T, DecodeError>
        where F: FnOnce(&mut Self, Tag) -> Result<T, DecodeError>
    {
        let tag = if let Some(tag) = self.implicit_tag {
            tag
        } else {
            default_tag
        };
        f(self, tag)
    }

    #[inline]
    pub fn decode_primitive<T, F>(&mut self, expected_tag: Tag, f: F) -> Result<T, DecodeError>
        where F: FnOnce(&mut R, usize) -> Result<T, DecodeError>
    {
        let tag = self.read_tag()?;
        let len = self.read_length_def()?;

        if tag == expected_tag {
            f(&mut self.inner, len)
        } else {
            Err(DecodeError::TagMismatch(expected_tag, tag))
        }
    }

    fn peek_tag(&mut self) -> Result<Tag, DecodeError> {
        if let Some(tag) = self.peeked_tag {
            Ok(tag)
        } else {
            let tag = read::read_tag(&mut self.inner)?;
            self.peeked_tag = Some(tag);
            Ok(tag)
        }
    }

    fn read_tag(&mut self) -> Result<Tag, DecodeError> {
        if let Some(tag) = self.peeked_tag {
            Ok(tag)
        } else {
            let tag = read::read_tag(&mut self.inner)?;
            Ok(tag)
        }
    }

    fn read_length_def(&mut self) -> Result<usize, DecodeError> {
        read::read_len_def(&mut self.inner).map_err(|e| e.into())
    }
}

impl<'de, R: io::Read + 'de> Asn1Deserializer<'de> for Deserializer<R> {
    type Err = DecodeError;

    type ExplicitDeserializer = Self;
    type ImplicitDeserializer = Self;

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_BOOLEAN, |d, tag| {
            d.decode_primitive(tag,
                               |r, len| read::read_boolean(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_bool(v))
        })
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_INTEGER, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_i8(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_i8(v))
        })
    }
    fn deserialize_i16<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_INTEGER, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_i16(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_i16(v))
        })
    }
    fn deserialize_i32<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_INTEGER, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_i32(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_i32(v))
        })
    }
    fn deserialize_i64<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_INTEGER, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_i64(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_i64(v))
        })
    }

    fn deserialize_u8<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_INTEGER, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_u8(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_u8(v))
        })
    }
    fn deserialize_u16<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_INTEGER, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_u16(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_u16(v))
        })
    }
    fn deserialize_u32<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_INTEGER, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_u32(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_u32(v))
        })
    }
    fn deserialize_u64<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_INTEGER, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_u64(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_u64(v))
        })
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_REAL, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_f32(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_f32(v))
        })
    }
    fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_REAL, |d, tag| {
            d.decode_primitive(tag, |r, len| read::read_f64(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_f64(v))
        })
    }

    fn deserialize_bit_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_BIT_STRING, |d, tag| {
            d.decode_primitive(tag,
                               |r, len| read::read_bit_string(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_bit_string(v))
        })
    }
    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_OCTET_STRING, |d, tag| {
            d.decode_primitive(tag,
                               |r, len| read::read_octet_string(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_byte_string(v))
        })
    }
    fn deserialize_null<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_NULL, |d, tag| {
            d.decode_primitive(tag, |_r, len| if len == 0 {
                visitor.visit_null()
            } else {
                Err(Asn1Error::invalid_length("non-zero length"))
            })
        })
    }

    fn deserialize_object_identifier<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_OBJECT_IDENTIFIER, |d, tag| {
            d.decode_primitive(tag,
                               |r, len| read::read_object_identifier(r, len).map_err(|e| e.into()))
                .and_then(|v| visitor.visit_object_identifier(v))
        })
    }

    fn deserialize_tagged(self, tag: Tag) -> Result<Self::ExplicitDeserializer, Self::Err> {
        // This is DER decoder, so all tagging MUST be implicit
        self.deserialize_tagged_implicit(tag)
    }

    fn deserialize_tagged_implicit(self,
                                   tag: Tag)
                                   -> Result<Self::ImplicitDeserializer, Self::Err> {
        Ok(self.with_tag(tag))
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        self.override_tag(info::TAG_SEQUENCE, |d, expected_tag| {
            let tag = d.read_tag()?;
            let len = d.read_length_def()?;

            if tag == expected_tag {
                visitor.visit_seq(SeqAccessor::new(d.inner.by_ref().take(len as u64)))
            } else {
                Err(DecodeError::TagMismatch(expected_tag, tag))
            }
        })
    }

    fn deserialize_choice<V>(mut self, visitor: V) -> Result<V::Value, Self::Err>
        where V: Asn1Visitor<'de>
    {
        let tag = self.peek_tag()?;
        visitor.visit_choice(&tag, self.with_tag(tag))
    }
}

