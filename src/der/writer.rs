use std::io::{Error as IoError, Write};

use info::{self, Tag, Len};
use ser::{self, SeqSerializer};

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
        Serializer::new(&mut self.serializer)
    }
}

impl<W: Write> SeqSerializer for StructSerializer<W> {
    type Ok = ();
    type Err = EncodeError;

    fn serialize_field<V: ser::Asn1Serialize + ?Sized>(&mut self,
                                                       value: &V)
                                                       -> Result<(), Self::Err> {
        value.asn1_serialize(self.next())
    }

    fn finish(mut self) -> Result<Self::Ok, Self::Err> {
        let len = self.serializer.len();
        let chunk = self.serializer.as_slice();

        self.out_encoder
            .override_tag(&info::TAG_SEQUENCE, |w, tag| {
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

impl<W: Write> ser::Asn1Serializer for Serializer<W> {
    type Ok = ();
    type Err = EncodeError;

    type SeqSerializer = StructSerializer<W>;
    type ImplicitSerializer = Serializer<W>;
    type ExplicitSerializer = Serializer<W>;

    fn serialize_bool(mut self, value: bool) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_BOOLEAN, |w, tag| {
            write::write_boolean(w, tag, value)?;
            Ok(())
        })
    }

    fn serialize_i8(mut self, value: i8) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_i8(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_i16(mut self, value: i16) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_i16(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_i32(mut self, value: i32) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_i32(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_i64(mut self, value: i64) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_i64(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_isize(mut self, value: isize) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_isize(w, tag, value)?;
            Ok(())
        })
    }

    fn serialize_u8(mut self, value: u8) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_u8(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_u16(mut self, value: u16) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_u16(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_u32(mut self, value: u32) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_u32(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_u64(mut self, value: u64) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_u64(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_usize(mut self, value: usize) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_INTEGER, |w, tag| {
            write::write_usize(w, tag, value)?;
            Ok(())
        })
    }

    fn serialize_f32(mut self, value: f32) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_REAL, |w, tag| {
            write::write_real32(w, tag, value)?;
            Ok(())
        })
    }
    fn serialize_f64(mut self, value: f64) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_REAL, |w, tag| {
            write::write_real64(w, tag, value)?;
            Ok(())
        })
    }

    fn serialize_bit_string(mut self, value: (u8, &[u8])) -> Result<Self::Ok, Self::Err> {
        let (unused, bytes) = value;
        if unused < 8 {
            self.override_tag(&info::TAG_BIT_STRING, |w, tag| {
                write::write_bit_string(w, tag, unused, bytes)?;
                Ok(())
            })
        } else {
            Err(EncodeError::InvalidValue)
        }
    }

    fn serialize_bytes(mut self, value: &[u8]) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_OCTET_STRING, |w, tag| {
            write::write_octet_string(w, tag, value)?;
            Ok(())
        })
    }

    fn serialize_null(mut self) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_NULL, |w, tag| {
            write::write_null(w, tag)?;
            Ok(())
        })
    }

    fn serialize_object_identifier(mut self, value: &[u64]) -> Result<Self::Ok, Self::Err> {
        self.override_tag(&info::TAG_OBJECT_IDENTIFIER,
                          |w, tag| if value.len() < 128 {
                              {
                                  write::write_short_object_identifier(w, tag, value)?;
                                  Ok(())
                              }
                          } else {
                              {
                                  write::write_object_identifier(w, tag, value)?;
                                  Ok(())
                              }
                          })
    }

    fn serialize_tagged(self, tag: Tag) -> Result<Self::ExplicitSerializer, Self::Err> {
        self.serialize_implicit(tag)
    }

    fn serialize_implicit(self, tag: Tag) -> Result<Self::ImplicitSerializer, Self::Err> {
        Ok(self.with_tag(tag))
    }

    fn serialize_sequence(self) -> Result<Self::SeqSerializer, Self::Err> {
        Ok(StructSerializer {
               serializer: Vec::with_capacity(128),
               out_encoder: self,
           })
    }
}

