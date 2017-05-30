use std::slice;
use std::io::{Result as IoResult, Write};

use info::tag::{Tag, Len, LenNum};



pub unsafe fn _rwrite_base128(ptr: *mut u8, pos: usize, n: u64) -> usize {
    let mut n = n;
    let mut pos = pos as isize;

    *ptr.offset(pos) = n as u8 & 0x7f;
    n >>= 7;

    while n > 0 {
        pos -= 1;
        *ptr.offset(pos) = n as u8 | 0x80;
        n >>= 7;
    }

    pos as usize
}

#[inline]
pub fn write_byte<W: Write>(w: &mut W, byte: u8) -> IoResult<()> {
    w.write_all(&[byte])
}

#[inline]
pub fn write_base128<W: Write>(w: &mut W, u: u64) -> IoResult<()> {
    use std::mem;

    let mut buf: [u8; 9] = unsafe { mem::uninitialized() };
    let from = unsafe { _rwrite_base128(buf.as_mut_ptr(), 8, u) };

    w.write_all(&buf[from..])
}

#[inline]
pub fn write_tag<W: Write>(w: &mut W, tag: &Tag) -> IoResult<()> {
    let tag_byte = tag.tag_byte();

    if tag.is_short() {
        write_byte(w, tag_byte | tag.tagnum() as u8)
    } else {
        write_byte(w, tag_byte | 0x1f)?;
        write_base128(w, tag.tagnum() as u64)
    }
}

#[inline]
pub fn write_len_def<W: Write>(w: &mut W, n: usize) -> IoResult<()> {
    if n < 128 {
        write_byte(w, n as u8)
    } else {
        let n = n.to_be();
        let chunk = unsafe {
            let ptr = &n as *const usize as *const u8;
            slice::from_raw_parts(ptr, 8)
        };

        let offset = chunk.iter().position(|&x| x != 0).unwrap(); // Ok here.
        write_byte(w, (8 - offset) as u8 | 0x80)?;
        w.write_all(&chunk[offset..])
    }
}

#[inline]
pub fn write_len_indef<W: Write>(w: &mut W) -> IoResult<()> {
    write_byte(w, 0x80)
}

#[inline]
pub fn write_len<W: Write>(w: &mut W, length: &Len) -> IoResult<()> {
    match *length {
        Len::Def(l) => write_len_def(w, l),
        Len::Indef => write_len_indef(w),
    }
}

#[inline]
pub fn _write_header<W: Write>(w: &mut W, tag: &Tag, len: LenNum) -> IoResult<()> {
    write_tag(w, tag)?;
    write_len_def(w, len)
}

#[inline]
pub fn write_header<W: Write>(w: &mut W, tag: &Tag, len: &Len) -> IoResult<()> {
    write_tag(w, tag)?;
    write_len(w, len)
}

#[inline]
pub fn write_primitive<W: Write>(w: &mut W, tag: &Tag, slice: &[u8]) -> IoResult<()> {
    _write_header(w, tag, slice.len())?;
    w.write_all(slice)
}

#[inline]
pub fn write_boolean<W: Write>(w: &mut W, tag: &Tag, value: bool) -> IoResult<()> {
    _write_header(w, tag, 1)?;
    write_byte(w, if value { 0xff } else { 0x00 })
}

macro_rules! integer_write {
    ($ident:ident, $ty:ty, 1) => {
        #[inline]
        pub fn $ident<W: Write>(w: &mut W, tag: &Tag, value: $ty) -> IoResult<()> {
            if value < 0x80 {
                write_primitive(w, tag, &[value])
            } else {
                write_primitive(w, tag, &[0x00, value])
            }
        }
    };
    ($ident:ident, $ty:ty, $size:expr) => {
        pub fn $ident<W: Write>(w: &mut W, tag: &Tag, value: $ty) -> IoResult<()> {
            static NEG_MASK: $ty = (0x80 << (8 * ($size - 1)));

            if value < 0x80 {
                _write_header(w, tag, 1)?;
                write_byte(w, value as u8)
            } else if value & NEG_MASK > 0 {
                let n = value.to_be();
                let chunk = unsafe { slice::from_raw_parts(&n as *const $ty as *const u8, $size) };

                _write_header(w, tag, $size + 1)?;
                write_byte(w, 0)?;
                w.write_all(chunk)
            } else {
                integer_write!(__chunk $ty, $size, (w, tag, value))
            }
        }
    };
    ($ident:ident, $ty:ty, $nat_ty:ty, 1) => {
        #[inline]
        pub fn $ident<W: Write>(w: &mut W, tag: &Tag, value: $ty) -> IoResult<()> {
            _write_header(w, tag, 1)?;
            write_byte(w, value as u8)
        }
    };
    ($ident:ident, $ty:ty, $nat_ty:ty, $size:expr) => {
        pub fn $ident<W: Write>(w: &mut W, tag: &Tag, value: $ty) -> IoResult<()> {
            if -0x80 < value && value < 0x80 {
                _write_header(w, tag, 1)?;
                write_byte(w, value as u8)
            } else {
                integer_write!(__chunk $nat_ty, $size, (w, tag, value))
            }
        }
    };
    (__chunk $ty:ty, $size:expr, ($w:expr, $tag:expr, $value:expr)) => {
        {
            static MASK: $ty = 0x1ff << (8 * ($size - 1) - 1);

            let mut n = $value as $ty;
            let mut size = $size;

            while ((n & MASK) == 0 || (n & MASK) == MASK) && size > 1 {
                size -= 1;
                n <<= 8;
            }

            n = n.to_be();
            let chunk = unsafe { slice::from_raw_parts(&n as *const $ty as *const u8, size) };

            write_primitive($w, $tag, chunk)
        }
    };
}

integer_write!(write_i8, i8, u8, 1);
integer_write!(write_i16, i16, u16, 2);
integer_write!(write_i32, i32, u32, 4);
integer_write!(write_i64, i64, u64, 8);
integer_write!(write_isize, isize, usize, 8);

integer_write!(write_u8, u8, 1);
integer_write!(write_u16, u16, 2);
integer_write!(write_u32, u32, 4);
integer_write!(write_u64, u64, 8);
integer_write!(write_usize, usize, 8);

pub fn write_bit_string<W: Write>(w: &mut W,
                                  tag: &Tag,
                                  unused_bits: u8,
                                  bytes: &[u8])
                                  -> IoResult<()> {
    assert!(unused_bits < 8);

    _write_header(w, tag, bytes.len() + 1)?;
    write_byte(w, unused_bits)?;
    w.write_all(bytes)
}


pub fn write_octet_string<W: Write>(w: &mut W, tag: &Tag, value: &[u8]) -> IoResult<()> {
    _write_header(w, tag, value.len())?;
    w.write_all(value)
}


pub fn write_null<W: Write>(w: &mut W, tag: &Tag) -> IoResult<()> {
    _write_header(w, tag, 0)
}

// TODO: use this as optimization for short oids
pub fn write_short_object_identifier<W: Write>(w: &mut W,
                                               tag: &Tag,
                                               value: &[u64])
                                               -> IoResult<()> {
    assert!(value.len() >= 2 && value.len() <= 130);

    const MAX_LEN: usize = 128;
    const MAX_BUF_SIZE: usize = MAX_LEN * 9;

    let (head, tail) = value.split_at(2);

    let mut _write = |v: &[u8]| -> IoResult<()> {
        _write_header(w, tag, v.len() + 1)?;
        write_byte(w, head[0] as u8 * 40 + head[1] as u8)?;
        w.write_all(v)
    };

    if tail.is_empty() {
        _write(&[])
    } else {
        let mut buf: [u8; MAX_BUF_SIZE] = [0; MAX_BUF_SIZE];
        let ptr = buf.as_mut_ptr();
        let mut pos = buf.len();

        for &u in tail.iter().rev() {
            pos = unsafe { _rwrite_base128(ptr, pos - 1, u) }
        }

        _write(&buf[pos..])
    }
}

pub fn write_object_identifier<W: Write>(w: &mut W, tag: &Tag, value: &[u64]) -> IoResult<()> {
    assert!(value.len() >= 2);

    let mut buf: Vec<u8> = Vec::with_capacity(1280);
    let (head, tail) = value.split_at(2);

    for &oi in tail {
        write_base128(&mut buf, oi)?
    }

    _write_header(w, tag, buf.len() + 1)?;
    write_byte(w, (head[0] as u8) * 40 + (head[1] as u8))?;
    w.write_all(buf.as_slice())
}

#[cfg(test)]
mod tests {
    use test;
    use super::*;
    use info::Class;

    fn buffer_eq_test<T, F>(test_set: &[(T, Vec<u8>)], mut f: F)
        where F: FnMut(&mut Vec<u8>, &T) -> IoResult<()>
    {
        let mut buf: Vec<u8> = Vec::new();

        for &(ref data, ref expected) in test_set.iter() {
            f(&mut buf, data).unwrap();
            assert_eq!(buf.as_slice(), expected.as_slice());
            buf.clear()
        }
    }

    #[test]
    fn length_write() {
        let test_set =
            [(0x00, vec![0x00]),
             (0x7f, vec![0x7f]),
             (0xff, vec![0x81, 0xff]),
             (0x7fff, vec![0x82, 0x7f, 0xff]),
             (0xffff, vec![0x82, 0xff, 0xff]),
             (0x7fffffff, vec![0x84, 0x7f, 0xff, 0xff, 0xff]),
             (0xffffffff, vec![0x84, 0xff, 0xff, 0xff, 0xff]),
             (0x7fffffffffffffff, vec![0x88, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
             (0xffffffffffffffff, vec![0x88, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff])];

        buffer_eq_test(&test_set[..], |buf, x| write_len_def(buf, *x));
    }

    #[test]
    fn i64_write() {
        let tag = Tag::new(Class::Universal, 0x02, false);
        let test_set = [(0x00, vec![0x02, 0x01, 0x00]),
                        (0x7f, vec![0x02, 0x01, 0x7f]),
                        (0xff, vec![0x02, 0x02, 0x00, 0xff]),
                        (0x7fff, vec![0x02, 0x02, 0x7f, 0xff]),
                        (0xffff, vec![0x02, 0x03, 0x00, 0xff, 0xff]),
                        (0x7fffffff, vec![0x02, 0x04, 0x7f, 0xff, 0xff, 0xff]),
                        (0xffffffff, vec![0x02, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff]),
                        (-0x01, vec![0x02, 0x01, 0xff]),
                        (-0x7f, vec![0x02, 0x01, 0x81]),
                        (-0xff, vec![0x02, 0x02, 0xff, 0x01]),
                        (-0x0100, vec![0x02, 0x02, 0xff, 0x00]),
                        (-0x7f00, vec![0x02, 0x02, 0x81, 0x00]),
                        (-0xffff, vec![0x02, 0x03, 0xff, 0x00, 0x01]),
                        (-0x7fffffffffffffff,
                         vec![0x02, 0x08, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01])];

        buffer_eq_test(&test_set[..], |buf, x| write_i64(buf, &tag, *x))
    }

    #[test]
    fn u64_write() {
        let tag = Tag::new(Class::Universal, 0x02, false);
        let test_set = [(0x00, vec![0x02, 0x01, 0x00]),
                        (0x7f, vec![0x02, 0x01, 0x7f]),
                        (0xff, vec![0x02, 0x02, 0x00, 0xff]),
                        (0x7fff, vec![0x02, 0x02, 0x7f, 0xff]),
                        (0xffff, vec![0x02, 0x03, 0x00, 0xff, 0xff]),
                        (0x7fffffff, vec![0x02, 0x04, 0x7f, 0xff, 0xff, 0xff]),
                        (0xffffffff, vec![0x02, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff]),
                        (0x7fffffffffffffff,
                         vec![0x02, 0x08, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
                        (0xffffffffffffffff,
                         vec![0x02, 0x09, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff])];

        buffer_eq_test(&test_set[..], |buf, x| write_u64(buf, &tag, *x))
    }

    #[test]
    fn bits_write() {
        let test_set = [((vec![0xff, 0xff, 0xf0], 4), vec![0x03, 0x04, 0x04, 0xff, 0xff, 0xf0]),
                        ((vec![0xff, 0xff], 0), vec![0x03, 0x03, 0x00, 0xff, 0xff])];
        let bit_string_tag = Tag::new(Class::Universal, 0x03, false);

        buffer_eq_test(&test_set[..], |buf, &(ref bits, unused)| {
            write_bit_string(buf, &bit_string_tag, unused, bits)
        })
    }

    #[test]
    fn oid_write() {
        let test_set = [(vec![0x01, 0x03], vec![0x04, 0x01, 0x2b]),
                        (vec![0x00, 0x03, 0x7f, 0x7f], vec![0x04, 0x03, 0x03, 0x7f, 0x7f]),
                        (vec![0x01, 0x03, 0x7fff, 0x7fff],
                         vec![0x04, 0x07, 0x2b, 0x81, 0xff, 0x7f, 0x81, 0xff, 0x7f])];
        let oid_tag = Tag::new(Class::Universal, 0x04, false);

        buffer_eq_test(&test_set[..],
                       |buf, x| write_object_identifier(buf, &oid_tag, x))
    }

    #[bench]
    fn bench_tag(b: &mut test::Bencher) {

        let mut writer: Vec<u8> = Vec::with_capacity(128);
        const V: Tag = Tag::const_new(Class::Universal, 0x02, false);

        b.iter(|| {
                   write_tag(&mut writer, &V).unwrap();
                   unsafe { writer.set_len(0) }
               })
    }

    #[bench]
    fn bench_length(b: &mut test::Bencher) {

        let mut writer: Vec<u8> = Vec::with_capacity(128);
        let v: usize = 0x7f;

        b.iter(|| {
                   write_len_def(&mut writer, v).unwrap();
                   unsafe { writer.set_len(0) }
               })
    }
}

