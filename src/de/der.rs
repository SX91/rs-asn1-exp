use std::{self, io};

use info::{Tag, Len};
use super::read;
use super::traits::{Deserializer as Asn1Deserializer, RawDecoder as Asn1RawDecoder,
                    Error as Asn1Error};


#[derive(Debug)]
pub enum Error {
    InvalidTag,
    TagMismatch,
    InvalidLength,
    LengthMismatch,
    InvalidValue,
    IO(std::io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<read::ReadError> for Error {
    fn from(e: read::ReadError) -> Self {
        use self::read::ReadError;
        match e {
            ReadError::InvalidTag => Error::InvalidTag,
            ReadError::InvalidLength => Error::InvalidLength,
            ReadError::InvalidValue => Error::InvalidValue,
            ReadError::IoError(err) => Error::IO(err),
        }
    }
}

impl Asn1Error for Error {
    // add code here
}


#[derive(Debug)]
pub struct Deserializer<R> {
    inner: R,
}

impl<R: io::Read> Deserializer<R> {
    pub fn new(reader: R) -> Self {
        Deserializer { inner: reader }
    }

    pub fn decode_primitive<T, F>(&mut self, expected_tag: &Tag, mut f: F) -> Result<T, Error>
        where F: FnMut(&mut R, usize) -> Result<T, Error>
    {

        let tag = read::read_tag(&mut self.inner)?;
        let len = read::read_len_def(&mut self.inner)?;

        if tag == *expected_tag {
            f(&mut self.inner, len)
        } else {
            Err(Error::TagMismatch)
        }
    }
}

impl<'de, R: io::Read> Asn1RawDecoder<'de> for Deserializer<R> {
    type Err = Error;

    fn decode_tag(&mut self) -> Result<Tag, Self::Err> {
        let tag = read::read_tag(&mut self.inner)?;
        Ok(tag)
    }

    fn decode_length(&mut self) -> Result<Len, Self::Err> {
        let len = read::read_len(&mut self.inner)?;
        Ok(len)
    }

    fn decode_byte(&mut self) -> Result<u8, Self::Err> {
        let v = read::read_byte(&mut self.inner)?;
        Ok(v)
    }

    fn decode_base128(&mut self) -> Result<u64, Self::Err> {
        let v = read::read_base128(&mut self.inner)?;
        Ok(v)
    }
}

impl<'de, R: io::Read> Asn1Deserializer<'de> for &'de mut Deserializer<R> {
    type Err = Error;

    type RawDecoder = Deserializer<R>;
    type NestedDeserializer = Self;

    fn deserialize_boolean(self, expected_tag: &Tag) -> Result<bool, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_boolean(r, len).map_err(|e| e.into()))
    }

    fn deserialize_i8(self, expected_tag: &Tag) -> Result<i8, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_i8(r, len).map_err(|e| e.into()))
    }

    fn deserialize_i16(self, expected_tag: &Tag) -> Result<i16, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_i16(r, len).map_err(|e| e.into()))
    }
    fn deserialize_i32(self, expected_tag: &Tag) -> Result<i32, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_i32(r, len).map_err(|e| e.into()))
    }
    fn deserialize_i64(self, expected_tag: &Tag) -> Result<i64, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_i64(r, len).map_err(|e| e.into()))
    }
    fn deserialize_isize(self, expected_tag: &Tag) -> Result<isize, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_isize(r, len).map_err(|e| e.into()))
    }

    fn deserialize_u8(self, expected_tag: &Tag) -> Result<u8, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_u8(r, len).map_err(|e| e.into()))
    }
    fn deserialize_u16(self, expected_tag: &Tag) -> Result<u16, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_u16(r, len).map_err(|e| e.into()))
    }
    fn deserialize_u32(self, expected_tag: &Tag) -> Result<u32, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_u32(r, len).map_err(|e| e.into()))
    }
    fn deserialize_u64(self, expected_tag: &Tag) -> Result<u64, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_u64(r, len).map_err(|e| e.into()))
    }
    fn deserialize_usize(self, expected_tag: &Tag) -> Result<usize, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_usize(r, len).map_err(|e| e.into()))
    }

    fn deserialize_f32(self, tag: &Tag, value: f32) -> Result<f32, Self::Err> {
        unimplemented!()
    }
    fn deserialize_f64(self, tag: &Tag, value: f64) -> Result<f64, Self::Err> {
        unimplemented!()
    }

    fn deserialize_bit_string(self, expected_tag: &Tag) -> Result<(u8, Vec<u8>), Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_bit_string(r, len).map_err(|e| e.into()))
    }

    fn deserialize_bytes(self, expected_tag: &Tag) -> Result<Vec<u8>, Self::Err> {
        self.decode_primitive(expected_tag,
                              |r, len| read::read_octet_string(r, len).map_err(|e| e.into()))
    }
    fn deserialize_null(self, expected_tag: &Tag) -> Result<(), Self::Err> {
        let (tag, len) = read::read_header(&mut self.inner)?;
        if tag != *expected_tag {
            Err(Error::TagMismatch)
        } else if len != Len::Def(1) {
            Err(Error::LengthMismatch)
        } else {
            Ok(())
        }
    }

    fn deserialize_raw(self) -> Result<Self::RawDecoder, Self::Err> {
        unimplemented!()
    }
    fn deserialize_constructed(self,
                               expected_tag: &Tag)
                               -> Result<Self::NestedDeserializer, Self::Err> {
        unimplemented!()
    }
}

