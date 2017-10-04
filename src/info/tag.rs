pub use self::Class::*; // export Class constructors
pub use self::ContentType::*;

pub type TagNum = u64;
pub type LenNum = usize;

/// ASN.1 BER/DER tag class.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Class {
    /// ASN.1 UNIVERSAL class
    Universal = 0b00000000,
    /// ASN.1 APPLICATION class
    Application = 0b01000000,
    /// ASN.1 CONTEXT SPECIFIC class
    ContextSpecific = 0b10000000,
    /// ASN.1 PRIVATE class
    Private = 0b11000000,
}

impl From<u8> for Class {
    fn from(i: u8) -> Self {
        match i {
            0x00 => Class::Universal,
            0x40 | 0x01 => Class::Application,
            0x80 | 0x02 => Class::ContextSpecific,
            0xc0 | 0x03 => Class::Private,
            x => panic!("invalid ASN.1 tag class {:?}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ContentType {
    Primitive = 0b00000000,
    Constructed = 0b00100000,
}

impl From<u8> for ContentType {
    fn from(i: u8) -> Self {
        match i {
            0x00 => ContentType::Primitive,
            0x20 => ContentType::Constructed,
            _ => panic!("invalid ASN.1 constructed flag"),
        }
    }
}

/// ASN.1 Tag.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    pub class: Class,
    pub tagnum: TagNum,
    pub content_type: ContentType,
}

impl Tag {
    /// Create new ASN.1 tag.
    pub fn new(class: Class, tagnum: u64, content_type: ContentType) -> Self {
        Tag {
            class: class,
            tagnum: tagnum,
            content_type: content_type,
        }
    }

    pub fn primitive(class: Class, tagnum: u64) -> Self {
        Tag {
            class: class,
            tagnum: tagnum,
            content_type: ContentType::Primitive,
        }
    }

    pub fn constructed(class: Class, tagnum: u64) -> Self {
        Tag {
            class: class,
            tagnum: tagnum,
            content_type: ContentType::Constructed,
        }
    }

    /// Create new const ASN.1 tag.
    #[inline]
    pub const fn const_new(class: Class, tagnum: u64, content_type: ContentType) -> Self {
        Tag {
            class: class,
            tagnum: tagnum,
            content_type: content_type,
        }
    }

    /// Create new const empty tag (0x00).
    #[inline]
    pub const fn zero() -> Self {
        Tag {
            class: Class::Universal,
            tagnum: 0,
            content_type: ContentType::Primitive,
        }
    }

    /// Get Tag's class.
    #[inline]
    pub fn class(&self) -> Class {
        self.class
    }

    /// Get Tag's tag num.
    #[inline]
    pub fn tagnum(&self) -> TagNum {
        self.tagnum
    }

    #[inline]
    pub fn content_type(&self) -> ContentType {
        self.content_type
    }

    #[inline]
    pub fn tag_byte(&self) -> u8 {
        if self.is_short() {
            self.class as u8 | self.content_type as u8 | self.tagnum as u8
        } else {
            self.class as u8 | self.content_type as u8 | 0x1f
        }
    }

    /// Check if Tag can be represented in one byte.
    #[inline]
    pub fn is_short(&self) -> bool {
        self.tagnum < 31
    }

    /// Check if Tag represents a constructed ASN.1 value.
    #[inline]
    pub fn is_constructed(&self) -> bool {
        ContentType::Constructed == self.content_type
    }
}

/// ASN.1 Length.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Len {
    /// Indefinite length.
    Indef,
    /// Definite length.
    Def(LenNum),
}

impl Len {
    /// Create new definite Length.
    pub fn new(n: LenNum) -> Self {
        Len::Def(n)
    }

    /// Create new indefinite Length.
    pub fn new_indef() -> Self {
        Len::Indef
    }

    /// Convert Length to u64.
    pub fn as_num(self) -> Option<LenNum> {
        self.into()
    }
}

impl From<Len> for Option<LenNum> {
    fn from(len: Len) -> Self {
        match len {
            Len::Def(l) => Some(l),
            Len::Indef => None,
        }
    }
}

impl From<Option<LenNum>> for Len {
    fn from(o: Option<LenNum>) -> Self {
        match o {
            Some(l) => Len::Def(l),
            None => Len::Indef,
        }
    }
}

impl From<LenNum> for Len {
    fn from(l: LenNum) -> Self {
        Len::Def(l)
    }
}

