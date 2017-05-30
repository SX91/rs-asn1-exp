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
    (__impl $ty:ty, $reader:expr, $len:expr, 1, true) => {
        if $len == 1 {
            read_byte($reader).map(|out| out as $ty).map_err(|e| e.into())
        } else {
            Err(ReadError::InvalidLength)
        }
    };
    (__impl $ty:ty, $reader:expr, $len:expr, 1, false) => {
        match $len {
            1 => read_byte($reader).map_err(|e| e.into()),
            2 => {
                let null = read_byte($reader)?;
                if null != 0 {
                    Err(ReadError::InvalidValue)
                } else {
                    read_byte($reader).map_err(|e| e.into())
                }
            }
            _ => Err(ReadError::InvalidLength)
        }
    };
    (__impl $ty:ty, $reader:expr, $len:expr, $size:expr, true) => {
        if 0 < $len && $len <= $size {
            let mut data: [u8; $size] = [0; $size];
            let offset = $size - $len;
            $reader.read_exact(&mut data[offset..])?;
            let val: $ty = unsafe { (*(data.as_ptr() as *const $ty)).to_be() };
            let shift = offset * 8;

            Ok((val << shift) >> shift)
        } else {
            Err(ReadError::InvalidLength)
        }
    };
    (__impl $ty:ty, $reader:expr, $len:expr, $size:expr, false) => {
        if 0 < $len && $len <= ($size + 1) {
            let mut out: [u8; $size + 1] = [0; $size + 1];
            let offset = $size + 1 - $len;
            $reader.read_exact(&mut out[offset..])?;

            let mut u = 0;
            for &x in &out[offset..] {
                u <<= 8;
                u |= x as $ty;
            }
            Ok(u)
        } else {
            Err(ReadError::InvalidLength)
        }
    };
}

read_integer!(read_i8: i8, 1, true);
read_integer!(read_i16: i16, 2, true);
read_integer!(read_i32: i32, 4, true);
read_integer!(read_i64: i64, 8, true);
read_integer!(read_isize: isize, 8, true);

read_integer!(read_u8: u8, 1, false);
read_integer!(read_u16: u16, 2, false);
read_integer!(read_u32: u32, 4, false);
read_integer!(read_u64: u64, 8, false);
read_integer!(read_usize: usize, 8, false);

#[inline]
pub fn read_base128<R: IoRead>(r: &mut R) -> Result<u64, ReadError> {
    let mut i = 0;

    loop {
        let byte = read_byte(r)?;
        if byte & 0x80 == 0x80 {
            i += (byte & 0x7f) as u64;
            i <<= 7;
        } else {
            i += byte as u64;
            break;
        }
    }
    Ok(i)
}

#[inline]
pub fn read_tag<R: IoRead>(r: &mut R) -> Result<Tag, ReadError> {
    let tag_byte = read_byte(r)?;
    let class_num = (tag_byte & 0xc0) >> 6;
    let constructed = tag_byte & 0x20 == 0x20;

    let tagnum = if (tag_byte & 0x1f) == 0x1f {
        read_base128(r)?
    } else {
        (tag_byte & 0x1f) as u64
    };

    Ok(Tag::new(class_num.into(), tagnum, constructed))
}

#[inline]
pub fn read_len<R: IoRead>(r: &mut R) -> Result<Len, ReadError> {
    let head = read_byte(r)?;
    let len: LenNum = head as LenNum & 0x7f;

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

    if l == 0x80 {
        Err(ReadError::InvalidLength)
    } else if l & 0x80 == 0x80 {
        read_usize(r, (l & 0x7f) as LenNum)
    } else {
        Ok(l as LenNum)
    }
}

pub fn read_header<R: IoRead>(r: &mut R) -> Result<(Tag, Len), ReadError> {
    let tag = read_tag(r)?;
    let len = read_len(r)?;
    Ok((tag, len))
}

pub fn read_boolean<R: IoRead>(r: &mut R, len: LenNum) -> Result<bool, ReadError> {
    if len == 1 {
        let byte = read_byte(r)?;
        Ok(byte != 0)
    } else {
        Err(ReadError::InvalidLength)
    }
}

pub fn read_bit_string<R: IoRead>(r: &mut R, len: LenNum) -> Result<(u8, Vec<u8>), ReadError> {
    if len < 2 {
        return Err(ReadError::InvalidLength);
    }

    let unused: u8 = read_u8(r, 1)?;
    if unused > 7 {
        return Err(ReadError::InvalidValue);
    }

    let mut buf: Vec<u8> = Vec::with_capacity(len);
    r.take((len - 1) as u64).read_to_end(&mut buf)?;

    Ok((unused, buf))
}

pub fn read_octet_string<R: IoRead>(r: &mut R, len: LenNum) -> Result<Vec<u8>, ReadError> {
    let mut buf: Vec<u8> = Vec::with_capacity(len);
    r.take(len as u64).read_to_end(&mut buf)?;
    Ok(buf)
}


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::io::{self, Error as IoError, ErrorKind as IoErrorKind};

    use super::*;
    use super::ReadError;
    use info::{Tag, Class, Len};
    use ser::write::*;

    #[quickcheck]
    fn i8_read(i: i8) -> bool {
        let mut buf = Vec::new();
        let tag = Tag::new(Class::Universal, 0x02, false);

        write_i8(&mut buf, &tag, i).unwrap();

        let mut cur = &mut &buf[..];
        let tag = read_tag(&mut cur).expect("asn.1 tag");
        let len = read_len_def(&mut cur).expect("asn.1 length");
        let new_i = read_i8(&mut cur, len).expect("asn.1 content");
        i == new_i
    }

    #[quickcheck]
    fn i16_read(i: i16) -> bool {
        let mut buf = Vec::new();
        let tag = Tag::new(Class::Universal, 0x02, false);

        write_i16(&mut buf, &tag, i).unwrap();

        let mut cur = &mut &buf[..];
        let tag = read_tag(&mut cur).expect("asn.1 tag");
        let len = read_len_def(&mut cur).expect("incorrect length");
        let new_i = read_i16(&mut cur, len).expect("incorrect content");
        i == new_i
    }

    #[quickcheck]
    fn i32_read(i: i32) -> bool {
        let mut buf = Vec::new();
        let tag = Tag::new(Class::Universal, 0x02, false);

        write_i32(&mut buf, &tag, i).unwrap();

        let mut cur = &mut &buf[..];
        let tag = read_tag(&mut cur).expect("asn.1 tag");
        let len = read_len_def(&mut cur).expect("incorrect length");
        let new_i = read_i32(&mut cur, len).expect("incorrect content");
        i == new_i
    }

    #[quickcheck]
    fn i64_read(i: i64) -> bool {
        let mut buf = Vec::new();
        let tag = Tag::new(Class::Universal, 0x02, false);

        write_i64(&mut buf, &tag, i).unwrap();

        let mut cur = &mut &buf[..];
        let tag = read_tag(&mut cur).expect("asn.1 tag");
        let len = read_len_def(&mut cur).expect("incorrect length");
        let new_i = read_i64(&mut cur, len).expect("incorrect content");
        i == new_i
    }

    #[quickcheck]
    fn u8_read(u: u8) -> bool {
        let mut buf = Vec::new();
        let tag = Tag::new(Class::Universal, 0x02, false);

        write_u8(&mut buf, &tag, u).unwrap();

        let mut cur = &mut &buf[..];
        let tag = read_tag(&mut cur).expect("asn.1 tag");
        let len = read_len_def(&mut cur).expect("incorrect length");
        let new_u = read_u8(&mut cur, len).expect("incorrect content");
        u == new_u
    }

    #[quickcheck]
    fn u16_read(u: u16) -> bool {
        let mut buf = Vec::new();
        let tag = Tag::new(Class::Universal, 0x02, false);

        write_u16(&mut buf, &tag, u).unwrap();

        let mut cur = &mut &buf[..];
        let tag = read_tag(&mut cur).expect("asn.1 tag");
        let len = read_len_def(&mut cur).expect("incorrect length");
        let new_u = read_u16(&mut cur, len).expect("incorrect content");
        u == new_u
    }

    #[quickcheck]
    fn u32_read(u: u32) -> bool {
        let mut buf = Vec::new();
        let tag = Tag::new(Class::Universal, 0x02, false);

        write_u32(&mut buf, &tag, u).unwrap();

        let mut cur = &mut &buf[..];
        let tag = read_tag(&mut cur).expect("asn.1 tag");
        let len = read_len_def(&mut cur).expect("incorrect length");
        let new_u = read_u32(&mut cur, len).expect("incorrect content");
        u == new_u
    }

    #[quickcheck]
    fn u64_read(u: u64) -> bool {
        let mut buf = Vec::new();
        let tag = Tag::new(Class::Universal, 0x02, false);

        write_u64(&mut buf, &tag, u).unwrap();

        let mut cur = &mut &buf[..];
        let tag = read_tag(&mut cur).expect("asn.1 tag");
        let len = read_len_def(&mut cur).expect("incorrect length");
        let new_u = read_u64(&mut cur, len).expect("incorrect content");
        u == new_u
    }

    #[quickcheck]
    fn base128_read(i: u64) -> bool {
        let mut buf = Vec::new();
        write_base128(&mut buf, i).unwrap();
        let mut cur = &mut &buf[..];
        i == read_base128(&mut cur).expect("base128")
    }

    #[quickcheck]
    fn length_read(i: usize) -> bool {
        let mut buf = Vec::new();
        write_len_def(&mut buf, i).unwrap();

        let mut reader = io::Cursor::new(buf);
        i == read_len_def(&mut reader).expect("incorrect length")
    }

    #[test]
    fn tag_simple() {
        let bytes = b"\x02\x00";
        let tag = Tag::new(0u8.into(), 2, false);
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
        let tag = Tag::new(1u8.into(), 1, false);
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
        let tag = Tag::new(1u8.into(), 33, false);
        let len: Len = Some(16).into();
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn tag_constructed() {
        let bytes = b"\x30\x12";
        let tag = Tag::new(0u8.into(), 16, true);
        let len: Len = Some(18).into();
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn tag_indefinite() {
        let bytes = b"\x30\x80";
        let tag = Tag::new(0u8.into(), 16, true);
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
        let tag = Tag::new(0u8.into(), 16, true);
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
        let tag = Tag::new(0u8.into(), 16, true);
        let len: Len = Some(129).into();
        assert_eq!(read_header(&mut &bytes[..]).unwrap(), (tag, len));
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &tag, &len).unwrap();
        assert_eq!(&buf, bytes);
    }

    #[test]
    fn tag_ridiculous() {
        let bytes = b"\x7f\x81\x80\x01\x85\x80\x00\x00\x00\x01";
        let tag = Tag::new(1u8.into(), 0x4001, true);
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
    use ser::write::*;

    #[inline]
    fn bench_helper<F, G>(b: &mut test::Bencher, mut f: F, mut g: G)
        where F: FnMut(&mut Vec<u8>),
              G: FnMut(&[u8])
    {
        let mut buf = Vec::with_capacity(32);
        f(&mut buf);

        b.iter(|| {
                   let mut cur = &buf[..];
                   g(&mut cur);
               })
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
        let tag = Tag::new(Class::Universal, 0x02, false);

        bench_helper(b, |buf| write_i64(buf, &tag, i).unwrap(), |mut cur| {
            let len = read_len_def(&mut cur).unwrap();
            read_i64(&mut cur, len).unwrap();
        })
    }

    #[bench]
    fn long_uint(b: &mut test::Bencher) {
        let i = 0x7fffffffffffffff;
        let tag = Tag::new(Class::Universal, 0x02, false);

        bench_helper(b, |buf| write_u64(buf, &tag, i).unwrap(), |mut cur| {
            let len = read_len_def(&mut cur).unwrap();
            read_u64((&mut cur), len).unwrap();
        })
    }
}

