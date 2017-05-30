pub use self::Class::*; // export Class constructors

pub type TagNum = u64;
pub type LenNum = usize;

/// ASN.1 BER/DER tag class.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Class {
    /// ASN.1 UNIVERSAL class
    Universal       = 0x00,
    /// ASN.1 APPLICATION class
    Application     = 0x01,
    /// ASN.1 CONTEXT SPECIFIC class
    ContextSpecific = 0x02,
    /// ASN.1 PRIVATE class
    Private         = 0x03,
}

impl From<u8> for Class {
    fn from(i: u8) -> Self {
        match i {
            0x00 => Class::Universal,
            0x01 => Class::Application,
            0x02 => Class::ContextSpecific,
            0x03 => Class::Private,
            _    => panic!("invalid ASN.1 tag class"),
        }
    }
}

/// ASN.1 Tag.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Tag {
    class: Class,
    tagnum: TagNum,
    constructed: bool,
}

impl Tag {
    /// Create new ASN.1 tag.
    pub fn new(class: Class, tagnum: u64, constructed: bool) -> Self {
        Tag {
            class: class,
            tagnum: tagnum,
            constructed: constructed,
        }
    }

    /// Create new const ASN.1 tag.
    #[inline]
    pub const fn const_new(class: Class, tagnum: u64, constructed: bool) -> Self {
        Tag {
            class: class,
            tagnum: tagnum,
            constructed: constructed,
        }
    }

    /// Create new const empty tag (0x00).
    #[inline]
    pub const fn zero() -> Self {
        Tag { class: Class::Universal, tagnum: 0, constructed: false }
    }

    /// Get class byte (with constructed flag set).
    #[inline]
    pub fn tag_byte(&self) -> u8 {
        if self.constructed {
            (self.class as u8) << 6 | 0x20
        } else {
            (self.class as u8) << 6
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

    /// Check if Tag can be represented in one byte.
    #[inline]
    pub fn is_short(&self) -> bool {
        self.tagnum < 31
    }

    /// Check if Tag represents a constructed ASN.1 value.
    #[inline]
    pub fn is_constructed(&self) -> bool {
        self.constructed
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
            None    => Len::Indef,
        }
    }
}

impl From<LenNum> for Len {
    fn from(l: LenNum) -> Self {
        Len::Def(l)
    }
}
