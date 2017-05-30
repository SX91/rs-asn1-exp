use std::io::{Error as IoError, Write};

use info::{Tag, Len};
use super::traits as ser;
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

pub struct StructSerializer<'a, W: 'a> {
    serializer: Serializer<Vec<u8>>,
    out_encoder: &'a mut Serializer<W>,
}

impl<'s, W: Write> ser::StructSerializer for StructSerializer<'s, W> {
    type Ok = ();
    type Err = EncodeError;

    fn serialize_field<V: ser::Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Self::Err> {
        value.asn1_serialize(&mut self.serializer)
    }

    fn finish(mut self, tag: &Tag) -> Result<Self::Ok, Self::Err> {
        use self::ser::RawEncoder;

        self.out_encoder
            .encode_header(tag, &Len::Def(self.serializer.len()))?;
        self.out_encoder.encode_bytes(self.serializer.as_slice())
    }
}

#[derive(Debug)]
pub struct Serializer<W> {
    writer: W,
}

impl Serializer<Vec<u8>> {
    pub fn new() -> Self {
        // TODO: remove the hardcoded capacity
        Serializer { writer: Vec::with_capacity(128) }
    }

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

impl<'a> Serializer<&'a mut Vec<u8>> {
    pub fn from_vec(writer: &'a mut Vec<u8>) -> Self {
        Serializer { writer: writer }
    }
}

impl<'s, W: Write> ser::RawEncoder for &'s mut Serializer<W> {
    type Ok = ();
    type Err = EncodeError;

    fn encode_tag(&mut self, tag: &Tag) -> Result<Self::Ok, Self::Err> {
        write::write_tag(&mut self.writer, tag)
            .map_err(|e| e.into())
    }
    fn encode_len(&mut self, len: &Len) -> Result<Self::Ok, Self::Err> {
        write::write_len(&mut self.writer, len)
            .map_err(|e| e.into())
    }

    fn encode_base128(&mut self, v: u64) -> Result<Self::Ok, Self::Err> {
        write::write_base128(&mut self.writer, v)
            .map_err(|e| e.into())
    }

    fn encode_byte(&mut self, byte: u8) -> Result<Self::Ok, Self::Err> {
        write::write_byte(&mut self.writer, byte)
            .map_err(|e| e.into())
    }
    fn encode_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Err> {
        self.writer.write_all(bytes)?;
        Ok(())
    }
}

impl<'s, W: Write> ser::Serializer for &'s mut Serializer<W> {
    type Ok = ();
    type Err = EncodeError;

    type RawEncoder = Self;
    type StructSerializer = StructSerializer<'s, W>;

    fn serialize_boolean(self, tag: &Tag, value: bool) -> Result<Self::Ok, Self::Err> {
        write::write_boolean(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }

    fn serialize_i8(self, tag: &Tag, value: i8) -> Result<Self::Ok, Self::Err> {
        write::write_i8(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }
    fn serialize_i16(self, tag: &Tag, value: i16) -> Result<Self::Ok, Self::Err> {
        write::write_i16(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }
    fn serialize_i32(self, tag: &Tag, value: i32) -> Result<Self::Ok, Self::Err> {
        write::write_i32(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }
    fn serialize_i64(self, tag: &Tag, value: i64) -> Result<Self::Ok, Self::Err> {
        write::write_i64(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }
    fn serialize_isize(self, tag: &Tag, value: isize) -> Result<Self::Ok, Self::Err> {
        write::write_isize(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }

    fn serialize_u8(self, tag: &Tag, value: u8) -> Result<Self::Ok, Self::Err> {
        write::write_u8(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }
    fn serialize_u16(self, tag: &Tag, value: u16) -> Result<Self::Ok, Self::Err> {
        write::write_u16(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }
    fn serialize_u32(self, tag: &Tag, value: u32) -> Result<Self::Ok, Self::Err> {
        write::write_u32(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }
    fn serialize_u64(self, tag: &Tag, value: u64) -> Result<Self::Ok, Self::Err> {
        write::write_u64(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }
    fn serialize_usize(self, tag: &Tag, value: usize) -> Result<Self::Ok, Self::Err> {
        write::write_usize(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }

    fn serialize_f32(self, tag: &Tag, value: f32) -> Result<Self::Ok, Self::Err> {
        unimplemented!()
    }
    fn serialize_f64(self, tag: &Tag, value: f64) -> Result<Self::Ok, Self::Err> {
        unimplemented!()
    }

    fn serialize_bit_string(self, tag: &Tag, value: (u8, &[u8])) -> Result<Self::Ok, Self::Err> {
        let (unused, bytes) = value;
        if unused < 8 {
            write::write_bit_string(&mut self.writer, tag, unused, bytes)?;
            Ok(())
        } else {
            Err(EncodeError::InvalidValue)
        }
    }

    fn serialize_bytes(self, tag: &Tag, value: &[u8]) -> Result<Self::Ok, Self::Err> {
        write::write_octet_string(&mut self.writer, tag, value)
            .map_err(|e| e.into())
    }

    fn serialize_null(self, tag: &Tag) -> Result<Self::Ok, Self::Err> {
        write::write_null(&mut self.writer, tag)
            .map_err(|e| e.into())
    }

    fn serialize_object_identifier(self, tag: &Tag, value: &[u64]) -> Result<Self::Ok, Self::Err> {
        if value.len() < 128 {
            write::write_short_object_identifier(&mut self.writer, tag, value)
                .map_err(|e| e.into())
        } else {
            write::write_object_identifier(&mut self.writer, tag, value)
                .map_err(|e| e.into())
        }
    }

    fn serialize_raw(self) -> Result<Self::RawEncoder, Self::Err> {
        Ok(self)
    }

    fn serialize_constructed(self) -> Result<Self::StructSerializer, Self::Err> {
        Ok(StructSerializer {
               serializer: Serializer::new(),
               out_encoder: self,
           })
    }
}

