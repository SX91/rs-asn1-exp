use std::io::{Result as IoResult, Read as IoRead, Error as IoError};

use info::{Tag, Len, LenNum};

#[derive(Debug)]
pub enum ReadError {
    InvalidTag,
    InvalidLength,
    InvalidValue,
    IoError(IoError),
}

impl From<IoError> for ReadError {
    fn from(e: IoError) -> Self {
        ReadError::IoError(e)
    }
}

pub fn read_byte<R: IoRead>(r: &mut R) -> IoResult<u8> {
    let mut buf: [u8; 1] = [0; 1];
    r.read_exact(&mut buf[..])?;
    Ok(buf[0])
}

macro_rules! read_integer {
    ($ident:ident: $ty:ty, $($args:tt)*) => {
        #[inline]
        pub fn $ident<R: IoRead>(r: &mut R, len: LenNum) -> Result<$ty, ReadError> {
            read_integer!(__impl $ty, r, len, $($args)*)
        }
    };
    (__impl $ty:ty, $reader:expr, $len:expr, $size:expr, true) => {
        if $len == 0 {
            Err(ReadError::InvalidLength)
        } else if $len == 1 {
            read_byte($reader).map(|out| out as i8 as $ty).map_err(|e| e.into())
        } else if $len <= $size {
            fn _read_transmute<R: IoRead>(r: &mut R, len: usize) -> IoResult<$ty> {
                use std::mem;

                let mut out: [u8; $size] = [0; $size];
                let offset = $size - len;
                r.read_exact(&mut out[offset..])?;

                let v: $ty = <$ty>::from_be(unsafe { mem::transmute(out) });
                let shift = offset * 8;

                Ok((v << shift) >> shift)
            }
            _read_transmute($reader, $len).map_err(|e| e.into())
        } else {
            Err(ReadError::InvalidLength)
        }
    };
    (__impl $ty:ty, $reader:expr, $len:expr, $size:expr, false) => {
        {
            use std::mem;

            fn _read_transmute<R: IoRead>(r: &mut R, len: usize) -> IoResult<$ty> {
                let mut out: [u8; $size] = [0; $size];
                let offset = $size - len;
                r.read_exact(&mut out[offset..])?;

                let v: $ty = <$ty>::from_be(unsafe { mem::transmute(out) });

                Ok(v)
            }

            if $len == 0 {
                Err(ReadError::InvalidLength)
            } else if $len == 1 {
                read_byte($reader)
                    .map(|v| v as $ty)
                    .map_err(|e| e.into())
            } else if $len <= $size {
                _read_transmute($reader, $len).map_err(|e| e.into())
            } else if $len == $size + 1 {
                match read_byte($reader)? {
                    0 => _read_transmute($reader, $size).map_err(|e| e.into()),
                    _ => Err(ReadError::InvalidValue)
                }
            } else {
                Err(ReadError::InvalidLength)
            }
        }
    };
}

read_integer!(read_i8: i8, 1, true);
read_integer!(read_i16: i16, 2, true);
read_integer!(read_i32: i32, 4, true);
read_integer!(read_i64: i64, 8, true);

read_integer!(read_u8: u8, 1, false);
read_integer!(read_u16: u16, 2, false);
read_integer!(read_u32: u32, 4, false);
read_integer!(read_u64: u64, 8, false);
read_integer!(read_usize: usize, 8, false);

#[inline]
pub fn read_f32<R: IoRead>(_r: &mut R, _len: usize) -> Result<f32, ReadError> {
    unimplemented!()
}

#[inline]
pub fn read_f64<R: IoRead>(_r: &mut R, _len: usize) -> Result<f64, ReadError> {
    unimplemented!()
}

#[inline]
pub fn read_base128<R: IoRead>(r: &mut R) -> Result<u64, ReadError> {
    let mut i = 0;

    loop {
        let byte = read_byte(r)?;
        if byte & 0x80 == 0x80 {
            i |= (byte & 0x7f) as u64;
            i <<= 7;
        } else {
            i |= byte as u64;
            break;
        }
    }
    Ok(i)
}

#[inline]
pub fn read_tag<R: IoRead>(r: &mut R) -> Result<Tag, ReadError> {
    let tag_byte = read_byte(r)?;

    let class_num = tag_byte & 0xc0;
    let content_type = tag_byte & 0x20;

    let tagnum = if (tag_byte & 0x1f) != 0x1f {
        (tag_byte & 0x1f) as u64
    } else {
        read_base128(r)?
    };

    Ok(Tag::new(class_num.into(), tagnum, content_type.into()))
}

#[inline]
#[allow(dead_code)]
pub fn read_len<R: IoRead>(r: &mut R) -> Result<Len, ReadError> {
    let head = read_byte(r)?;
    let len = (head & 0x7f) as usize;

    if head & 0x80 == 0 {
        Ok(Len::Def(len))
    } else if head == 0x80 {
        Ok(Len::Indef)
    } else {
        read_usize(r, len).map(Len::Def)
    }
}

#[inline]
pub fn read_len_def<R: IoRead>(r: &mut R) -> Result<LenNum, ReadError> {
    let l = read_byte(r)?;

    if l & 0x80 == 0 {
        Ok((l & 0x7f) as usize)
    } else if l > 0x80 {
        read_usize(r, (l & 0x7f) as LenNum)
    } else {
        Err(ReadError::InvalidLength)
    }
}

#[inline]
#[allow(dead_code)]
pub fn read_header<R: IoRead>(r: &mut R) -> Result<(Tag, Len), ReadError> {
    let tag = read_tag(r)?;
    let len = read_len(r)?;
    Ok((tag, len))
}

#[inline]
pub fn read_boolean<R: IoRead>(r: &mut R, len: LenNum) -> Result<bool, ReadError> {
    if len == 1 {
        let byte = read_byte(r)?;
        Ok(byte != 0)
    } else {
        Err(ReadError::InvalidLength)
    }
}

pub fn read_bit_string<R: IoRead>(r: &mut R, len: LenNum) -> Result<(u8, Vec<u8>), ReadError> {
    if len == 0 {
        return Err(ReadError::InvalidLength);
    }

    let unused: u8 = read_u8(r, 1)?;
    if unused == 0 && len == 1 {
        return Ok((unused, vec![]));
    } else if unused > 7 || (unused > 0 && len == 1) {
        return Err(ReadError::InvalidValue);
    }

    let mut buf: Vec<u8> = Vec::with_capacity(len);
    r.take((len - 1) as u64).read_to_end(&mut buf)?;

    Ok((unused, buf))
}

pub fn read_octet_string<R: IoRead>(r: &mut R, len: LenNum) -> Result<Vec<u8>, ReadError> {
    if len == 0 {
        return Ok(Vec::new());
    }

    let mut buf: Vec<u8> = Vec::with_capacity(len);
    r.take(len as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn read_object_identifier<R: IoRead>(r: &mut R, len: LenNum) -> Result<Vec<u64>, ReadError> {
    let mut buf: Vec<u64> = Vec::new();
    let mut nested = r.take(len as u64);

    let i0 = read_byte(&mut nested)?;

    buf.push((i0 / 40) as u64);
    buf.push((i0 % 40) as u64);

    while nested.limit() > 0 {
        buf.push(read_base128(&mut nested)?)
    }

    Ok(buf)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::io::ErrorKind as IoErrorKind;

    use quickcheck::{Arbitrary, Gen};

    use super::*;
    use super::ReadError;
    use info::{Tag, ContentType, Class, Len};
    use der::write::*;

    impl Arbitrary for Class {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let all = [Class::Universal,
                       Class::Application,
                       Class::ContextSpecific,
                       Class::Private];
            let value = g.choose(&all).unwrap();
            *value
        }
    }

    impl Arbitrary for ContentType {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            if g.gen() {
                ContentType::Constructed
            } else {
                ContentType::Primitive
            }
        }
    }

    impl Arbitrary for Tag {
        fn arbitrary<G: Gen>(g: &mut G) -> Tag {
            Tag::new(Arbitrary::arbitrary(g),
                     Arbitrary::arbitrary(g),
                     Arbitrary::arbitrary(g))
        }
    }

    fn read_helper<T, R, W>(tag: &Tag, v: &T, w: W, r: R) -> bool
        where T: PartialEq,
              W: FnOnce(&mut Vec<u8>, &Tag, &T),
              R: FnOnce(&mut &[u8], usize) -> T
    {
        let mut buf = Vec::new();
        w(&mut buf, tag, v);

        let mut cur = &mut buf.as_slice();
        let new_tag = read_tag(&mut cur).expect("asn.1 tag");
        assert_eq!(new_tag, *tag);

        let len = read_len_def(&mut cur).expect("asn.1 length");
        let new_v = r(&mut cur, len);

        *v == new_v
    }

    #[quickcheck]
    fn base128(i: u64) -> bool {
        let mut buf = Vec::new();
        write_base128(&mut buf, i).unwrap();
        let mut cur = &mut buf.as_slice();
        i == read_base128(&mut cur).expect("base128")
    }

    #[quickcheck]
    fn tag(tag: Tag) -> bool {
        let mut buf = Vec::new();
        write_tag(&mut buf, &tag).unwrap();

        tag == read_tag(&mut buf.as_slice()).expect("incorrect tag")
    }

    #[quickcheck]
    fn length(i: usize) -> bool {
        let mut buf = Vec::new();
        write_len_def(&mut buf, i).unwrap();

        i == read_len_def(&mut buf.as_slice()).expect("incorrect length")
    }

    #[quickcheck]
    fn boolean(b: bool) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x01),
                    &b,
                    |w, tag, v| write_boolean(w, tag, *v).unwrap(),
                    |r, len| read_boolean(r, len).unwrap())
    }

    #[quickcheck]
    fn i8(i: i8) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x02),
                    &i,
                    |w, tag, v| write_i8(w, tag, *v).unwrap(),
                    |r, len| read_i8(r, len).unwrap())
    }

    #[quickcheck]
    fn i16(i: i16) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x02),
                    &i,
                    |w, tag, v| write_i16(w, tag, *v).unwrap(),
                    |r, len| read_i16(r, len).unwrap())
    }

    #[quickcheck]
    fn i32(i: i32) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x02),
                    &i,
                    |w, tag, v| write_i32(w, tag, *v).unwrap(),
                    |r, len| read_i32(r, len).unwrap())
    }

    #[quickcheck]
    fn i64(i: i64) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x02),
                    &i,
                    |w, tag, v| write_i64(w, tag, *v).unwrap(),
                    |r, len| read_i64(r, len).unwrap())
    }

    #[quickcheck]
    fn u8(u: u8) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x02),
                    &u,
                    |w, tag, v| write_u8(w, tag, *v).unwrap(),
                    |r, len| read_u8(r, len).unwrap())
    }

    #[quickcheck]
    fn u16(u: u16) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x02),
                    &u,
                    |w, tag, v| write_u16(w, tag, *v).unwrap(),
                    |r, len| read_u16(r, len).unwrap())
    }

    #[quickcheck]
    fn u32(u: u32) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x02),
                    &u,
                    |w, tag, v| write_u32(w, tag, *v).unwrap(),
                    |r, len| read_u32(r, len).unwrap())
    }

    #[quickcheck]
    fn u64(u: u64) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x02),
                    &u,
                    |w, tag, v| write_u64(w, tag, *v).unwrap(),
                    |r, len| read_u64(r, len).unwrap())
    }

    #[quickcheck]
    fn octet_string(buf: Vec<u8>) -> bool {
        read_helper(&Tag::primitive(Class::Universal, 0x04),
                    &buf,
                    |w, tag, v| write_octet_string(w, tag, v.as_slice()).unwrap(),
                    |r, len| read_octet_string(r, len).unwrap())
    }

    #[quickcheck]
    fn oid(buf: Vec<u64>) -> bool {
        let mut oid: Vec<u64> = vec![1, 3];
        oid.extend(buf);

        read_helper(&Tag::primitive(Class::Universal, 0x06),
                    &oid,
                    |w, tag, v| write_object_identifier(w, tag, v.as_slice()).unwrap(),
                    |r, len| read_object_identifier(r, len).unwrap())
    }

    #[test]
    fn tag_simple() {
        let bytes = b"\x02\x00";
        let tag = Tag::primitive(0u8.into(), 2);
        let len: Len = Len::new(0);
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn high_tag_class_1() {
        let short_bytes = b"\x41\x10";
        let long_bytes = b"\x5f\x01\x10";
        let tag = Tag::primitive(1u8.into(), 1);
        let len: Len = Len::Def(16);

        assert_eq!(read_header(&mut &short_bytes[..]).unwrap(), (tag, len));
        assert_eq!(read_header(&mut &long_bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, short_bytes);
    }

    #[test]
    fn high_tag_class_2() {
        let bytes = b"\x5f\x21\x10";
        let tag = Tag::primitive(1u8.into(), 33);
        let len: Len = Some(16).into();
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn tag_constructed() {
        let bytes = b"\x30\x12";
        let tag = Tag::constructed(0u8.into(), 16);
        let len: Len = Some(18).into();
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn tag_indefinite() {
        let bytes = b"\x30\x80";
        let tag = Tag::constructed(0u8.into(), 16);
        let len: Len = None.into();
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn tag_long_len_1() {
        let long_bytes = b"\x30\x81\x11";
        let short_bytes = b"\x30\x11";
        let tag = Tag::constructed(0u8.into(), 16);
        let len: Len = Some(17).into();
        assert_eq!(read_header(&mut &short_bytes[..]).unwrap(), (tag, len));
        assert_eq!(read_header(&mut &long_bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, short_bytes);
    }

    #[test]
    fn tag_long_len_2() {
        let bytes = b"\x30\x81\x81";
        let tag = Tag::constructed(0u8.into(), 16);
        let len: Len = Some(129).into();
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn tag_ridiculous() {
        let bytes = b"\x7f\x81\x80\x01\x85\x80\x00\x00\x00\x01";
        let tag = Tag::constructed(1u8.into(), 0x4001);
        let len: Len = Some(549755813889).into();
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn tag_missing_bytes() {
        let bytes = b"";
        let res = read_header(&mut &bytes[..]);
        match res {
            Err(ReadError::IoError(ref err)) if err.kind() == IoErrorKind::UnexpectedEof => {}
            _ => panic!("Expected UnexpectedEOf, got {:?}", res.unwrap_err()),
        }
    }

    #[test]
    fn tag_missing_tag_bytes() {
        let res = read_header(&mut &b"\x1f"[..])
            .or(read_header(&mut &b"\x1f\x80"[..]))
            .or(read_header(&mut &b"\x1f\x80\x82"[..]));
        match res {
            Err(ReadError::IoError(ref err)) if err.kind() == IoErrorKind::UnexpectedEof => {}
            _ => panic!("Expected UnexpectedEOf, got {:?}", res.unwrap_err()),
        }
    }

    #[test]
    fn tag_missing_len_bytes() {
        let res = read_header(&mut &b"\x30"[..])
            .or(read_header(&mut &b"\x30\x81"[..]))
            .or(read_header(&mut &b"\x30\x83\x01\x03"[..]));
        match res {
            Err(ReadError::IoError(ref err)) if err.kind() == IoErrorKind::UnexpectedEof => {}
            _ => panic!("Expected UnexpectedEOf, got {:?}", res.unwrap_err()),
        }
    }
}

#[cfg(test)]
mod benches {
    use test;

    use super::*;
    use info::{Tag, Class};
    use der::write::*;

    #[inline]
    fn bench_helper<F, G>(b: &mut test::Bencher, mut f: F, mut g: G)
        where F: FnMut(&mut Vec<u8>),
              G: FnMut(&[u8])
    {
        let mut buf = Vec::with_capacity(32);
        f(&mut buf);

        b.iter(|| {
                   let mut cur = buf.as_slice();
                   g(&mut cur);
               })
    }

    #[bench]
    fn short_tag(b: &mut test::Bencher) {
        let t = Tag::primitive(Class::Universal, 0x02);
        bench_helper(b,
                     |buf| write_tag(buf, &t).unwrap(),
                     |mut buf| { read_tag(&mut buf).unwrap(); })
    }

    #[bench]
    fn short_length(b: &mut test::Bencher) {
        let l = 0x01;
        bench_helper(b,
                     |buf| write_len_def(buf, l).unwrap(),
                     |mut buf| { read_len_def(&mut buf).unwrap(); })
    }

    #[bench]
    fn long_length(b: &mut test::Bencher) {
        let l = 0x7fffffffffffffff;
        bench_helper(b,
                     |buf| write_len_def(buf, l).unwrap(),
                     |mut buf| { read_len_def(&mut buf).unwrap(); })
    }

    #[bench]
    fn long_int(b: &mut test::Bencher) {
        let i = -0x7fffffffffffffff;
        let tag = Tag::primitive(Class::Universal, 0x02);

        bench_helper(b, |buf| write_i64(buf, &tag, i).unwrap(), |mut cur| {
            let _ = read_tag(&mut cur).unwrap();
            let len = read_len_def(&mut cur).unwrap();
            read_i64(&mut cur, len).unwrap();
        })
    }

    #[bench]
    fn long_uint(b: &mut test::Bencher) {
        let i = 0x7fffffffffffffff;
        let tag = Tag::primitive(Class::Universal, 0x02);

        bench_helper(b, |buf| write_u64(buf, &tag, i).unwrap(), |mut cur| {
            let _ = read_tag(&mut cur).unwrap();
            let len = read_len_def(&mut cur).unwrap();
            read_u64((&mut cur), len).unwrap();
        })
    }
}

