use super::tag::{Tag, Class, ContentType};
use self::ContentType::*;

// TODO: Make use of const_fn feature.

pub const TAG_BOOLEAN: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x01,
    content_type: Primitive,
};
pub const TAG_INTEGER: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x02,
    content_type: Primitive,
};
pub const TAG_BIT_STRING: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x03,
    content_type: Primitive,
};
pub const TAG_OCTET_STRING: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x04,
    content_type: Primitive,
};
pub const TAG_NULL: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x05,
    content_type: Primitive,
};
pub const TAG_OBJECT_IDENTIFIER: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x06,
    content_type: Primitive,
};
pub const TAG_REAL: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x09,
    content_type: Primitive,
};
pub const TAG_SEQUENCE: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x10,
    content_type: Constructed,
};
pub const TAG_SET: Tag = Tag {
    class: Class::Universal,
    tagnum: 0x11,
    content_type: Constructed,
};

pub const TYPE_BOOLEAN: &str = "BOOLEAN";
pub const TYPE_INTEGER: &str = "INTEGER";
pub const TYPE_BIT_STRING: &str = "BIT STRING";
pub const TYPE_OCTET_STRING: &str = "OCTET STRING";
pub const TYPE_NULL: &str = "NULL";
pub const TYPE_OBJECT_IDENTIFIER: &str = "OBJECT IDENTIFIER";
pub const TYPE_REAL: &str = "REAL";
pub const TYPE_SEQUENCE: &str = "SEQUENCE";
pub const TYPE_SEQUENCE_OF: &str = "SEQUENCE OF";
pub const TYPE_SET: &str = "SET";
pub const TYPE_SET_OF: &str = "SET OF";

