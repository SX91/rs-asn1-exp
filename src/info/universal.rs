use super::tag::{Tag, Class};

pub const TAG_BOOLEAN: Tag = Tag::const_new(Class::Universal, 0x01, false);
pub const TAG_INTEGER: Tag = Tag::const_new(Class::Universal, 0x02, false);
pub const TAG_BIT_STRING: Tag = Tag::const_new(Class::Universal, 0x03, false);
pub const TAG_OCTET_STRING: Tag = Tag::const_new(Class::Universal, 0x04, false);
pub const TAG_NULL: Tag = Tag::const_new(Class::Universal, 0x05, false);
pub const TAG_OBJECT_IDENTIFIER: Tag = Tag::const_new(Class::Universal, 0x06, false);
pub const TAG_REAL: Tag = Tag::const_new(Class::Universal, 0x09, false);
pub const TAG_SEQUENCE: Tag = Tag::const_new(Class::Universal, 0x10, true);
pub const TAG_SET: Tag = Tag::const_new(Class::Universal, 0x11, true);

