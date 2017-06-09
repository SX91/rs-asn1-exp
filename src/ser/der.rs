use std::io::{Error as IoError, Write};

use info::{Tag, Len};
use info::universal::*;

use super::traits::{self as ser, SeqSerializer};
use super::write;

#[derive(Debug)]
pub enum EncodeError {
    InvalidTag,
    InvalidLength,
    InvalidValue,
    PrimUntagged,
    Custom(String),
    GeneralIO(IoError),
}


impl From<IoError> for EncodeError {
    fn from(e: IoError) -> Self {
        EncodeError::GeneralIO(e)
    }
}

impl ser::Error for EncodeError {
    fn invalid_tag() -> Self {
        EncodeError::InvalidTag
    }

    fn invalid_value() -> Self {
        EncodeError::InvalidValue
    }

    fn prim_untagged() -> Self {
        EncodeError::PrimUntagged
    }

    fn custom(message: &str) -> Self {
        EncodeError::Custom(String::from(message))
    }
}

pub struct StructSerializer<W> {
    serializer: Vec<u8>,
    out_encoder: Serializer<W>,
}

impl<W: Write> StructSerializer<W> {
    pub fn next<'a>(&'a mut self) -> Serializer<&'a mut Vec<u8>> {
        Serializer {
            writer: &mut self.serializer,
            implicit_tag: None,
        }
    }
}

impl<W: Write> SeqSerializer for StructSerializer<W> {
    type Ok = ();
    type Err = EncodeError;

    fn serialize_field<V: ser::Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Self::Err> {
        value.asn1_serialize(self.next())
    }

    fn finish(mut self) -> Result<Self::Ok, Self::Err> {
        let serializer = self.serializer;
        let len = serializer.len();
        let chunk = serializer.as_slice();

        self.out_encoder.override_tag(&TAG_SEQUENCE, |w, tag| {
            write::write_header(w, tag, &Len::Def(len))?;
            w.write_all(chunk)?;
            Ok(())
        })
    }
}

#[derive(Debug)]
pub struct Serializer<W> {
    writer: W,
    implicit_tag: Option<Tag>,
}

impl<'a, W: 'a> Serializer<W> {
    pub fn new(inner: W) -> Self {
        Serializer {
            writer: inner,
            implicit_tag: None,
        }
    }

    pub fn with_tag(self, tag: Tag) -> Self {
        Serializer {
            writer: self.writer,
            implicit_tag: Some(tag),
        }
    }

    fn override_tag<T, F>(&mut self, tag: &Tag, f: F) -> T
        where F: FnOnce(&mut W, &Tag) -> T
    {
        let tag = if let Some(ref tag) = self.implicit_tag {
            tag
        } else {
            tag
        };
        f(&mut self.writer, tag)
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl Serializer<Vec<u8>> {
    pub fn len(&self) -> usize {
        self.writer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.writer.is_empty()
    }

    pub fn clear(&mut self) {
        self.writer.clear()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.writer.as_slice()
    }
}

impl<'a, W: Write> ser::RawEncoder for Serializer<W> {
    type Ok = ();
    type Err = EncodeError;

    fn write_tag(&mut self, tag: &Tag) -> Result<Self::Ok, Self::Err> {
        write::write_tag(&mut self.writer, tag).map_err(|e| e.into())
    }
    fn write_length(&mut self, len: &Len) -> Result<Self::Ok, Self::Err> {
        write::write_len(&mut self.writer, len).map_err(|e| e.into())
    }

    fn write_base128(&mut self, v: u64) -> Result<Self::Ok, Self::Err> {
        write::write_base128(&mut self.writer, v).map_err(|e| e.into())
    }

    fn write_byte(&mut self, byte: u8) -> Result<Self::Ok, Self::Err> {
        write::write_byte(&mut self.writer, byte).map_err(|e| e.into())
    }
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Err> {
        self.writer.write_all(bytes)?;
        Ok(())
    }
}

impl<W: Write> ser::Serializer for Serializer<W> {
    type Ok = ();
    type Err = EncodeError;

    // type RawEncoder = Self;
    type SeqSerializer = StructSerializer<W>;

    type ImplicitSerializer = Serializer<W>;
    type ExplicitSerializer = Serializer<W>;

    fn serialize_boolean(mut self, value: bool) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_BOOLEAN,
                          |w, tag| write::write_boolean(w, tag, value).map_err(|e| e.into()))
    }

    fn serialize_i8(mut self, value: i8) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_i8(w, tag, value).map_err(|e| e.into()))
    }
    fn serialize_i16(mut self, value: i16) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_i16(w, tag, value).map_err(|e| e.into()))
    }
    fn serialize_i32(mut self, value: i32) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_i32(w, tag, value).map_err(|e| e.into()))
    }
    fn serialize_i64(mut self, value: i64) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_i64(w, tag, value).map_err(|e| e.into()))
    }
    fn serialize_isize(mut self, value: isize) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_isize(w, tag, value).map_err(|e| e.into()))
    }

    fn serialize_u8(mut self, value: u8) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_u8(w, tag, value).map_err(|e| e.into()))
    }
    fn serialize_u16(mut self, value: u16) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_u16(w, tag, value).map_err(|e| e.into()))
    }
    fn serialize_u32(mut self, value: u32) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_u32(w, tag, value).map_err(|e| e.into()))
    }
    fn serialize_u64(mut self, value: u64) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_u64(w, tag, value).map_err(|e| e.into()))
    }
    fn serialize_usize(mut self, value: usize) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_INTEGER,
                          |w, tag| write::write_usize(w, tag, value).map_err(|e| e.into()))
    }

    fn serialize_f32(mut self, value: f32) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_REAL, |w, tag| unimplemented!())
    }
    fn serialize_f64(mut self, value: f64) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_REAL, |w, tag| unimplemented!())
    }

    fn serialize_bit_string(mut self, value: (u8, &[u8])) -> Result<Self::Ok, Self::Err> {
        let (unused, bytes) = value;
        if unused < 8 {
            self.override_tag(&TAG_BIT_STRING, |w, tag| {
                write::write_bit_string(w, tag, unused, bytes).map_err(|e| e.into())
            })
        } else {
            Err(EncodeError::InvalidValue)
        }
    }

    fn serialize_bytes(mut self, value: &[u8]) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_OCTET_STRING,
                          |w, tag| write::write_octet_string(w, tag, value).map_err(|e| e.into()))
    }

    fn serialize_null(mut self) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_NULL,
                          |w, tag| write::write_null(w, tag).map_err(|e| e.into()))
    }

    fn serialize_object_identifier(mut self, value: &[u64]) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&TAG_OBJECT_IDENTIFIER, |w, tag| if value.len() < 128 {
            write::write_short_object_identifier(w, tag, value).map_err(|e| e.into())
        } else {
            write::write_object_identifier(w, tag, value).map_err(|e| e.into())
        })
    }

    // fn serialize_raw(self) -> Result<Self::RawEncoder, Self::Err> {
    //     Ok(self)
    // }

    fn serialize_tagged(self, tag: &Tag) -> Result<Self::ExplicitSerializer, Self::Err> {
        unimplemented!()
    }

    fn serialize_implicit(self, tag: &Tag) -> Result<Self::ImplicitSerializer, Self::Err> {
        Ok(self.with_tag(*tag))
    }

    fn serialize_sequence(self) -> Result<Self::SeqSerializer, Self::Err> {
        Ok(StructSerializer {
               serializer: Vec::with_capacity(128),
               out_encoder: self,
           })
    }
}

