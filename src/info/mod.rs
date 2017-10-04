#[macro_use]
pub mod tag;
pub mod universal;

use std::fmt::{self, Display};

pub use self::tag::{Class, Tag, Len, ContentType, TagNum, LenNum};
pub use self::tag::Class::*;
pub use self::universal::*;

pub trait Asn1Tagged {
    fn asn1_tag() -> tag::Tag;
}

pub trait Asn1Typed {
    fn asn1_type() -> &'static str;
}

pub trait Asn1Info: Asn1Tagged + Asn1Typed {}

pub trait Asn1DisplayExt: Display {
    fn asn1_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T: Display + Asn1Typed> Asn1DisplayExt for T {
    fn asn1_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", T::asn1_type(), self)
    }
}

#[macro_export]
macro_rules! asn1_info {
    ($rs_type:ty: ($($gen:tt)+) => [$($args:tt)+], $asn_type:expr) => (
        asn1_tagged!($rs_type: ($($gen)+), [$($args)+]);
        asn1_typed!($rs_type: ($($gen)*), $asn_type);
    );
    ($rs_type:ty: ($($gen:tt)+) => $tag:tt, $asn_type:tt) => (
        asn1_tagged!($rs_type: ($($gen)+): $tag);
        asn1_typed!($rs_type: ($($gen)+), $asn_type);
    );
    ($rs_type:ty: ($($gen:tt)+) => $class:ident $tagnum:expr, $asn_type:expr) => (
        asn1_tagged!($rs_type: ($($gen)+), $class $tagnum);
        asn1_typed!($rs_type: ($($gen)+), $asn_type);
    );
    ($rs_type:ty => [$($args:tt)*], $asn_type:expr) => {
        asn1_tagged!($rs_type, [$($args)*]);
        asn1_typed!($rs_type, $asn_type);
    };
    ($rs_type:ty => $tag:expr, $asn_type:expr) => {
        asn1_tagged!($rs_type: $tag);
        asn1_typed!($rs_type, $asn_type);
    };
    ($rs_type:ty => $tagclass:ident $tagnum:expr, $asn_type:expr) => {
        asn1_tagged!($rs_type, $tagclass $tagnum);
        asn1_typed!($rs_type, $asn_type);
    };
}

#[macro_export]
/// ASN.1 tagged object specification.
macro_rules! asn1_tagged {
    ($rs_type:ty: ($($gen:tt)+), $($args:tt)*) => (
        impl<$($gen)+> $crate::info::Asn1Tagged for $rs_type {
            asn1_tagged!{__impl $($args)*}
        }
    );
    ($rs_type:ty: ($($gen:tt)+): $tag:path) => (
        impl<$($gen)+> $crate::info::Asn1Tagged for $rs_type {
            asn1_tagged!{__static $tag}
        }
    );
    ($rs_type:ty, $($args:tt)*) => (
        impl $crate::info::Asn1Tagged for $rs_type {
            asn1_tagged!{__impl $($args)*}
        }
    );
    ($rs_type:ty: $tag:tt) => (
        impl $crate::info::Asn1Tagged for $rs_type {
            asn1_tagged!{__static $tag}
        }
    );
    (__static $tag:tt) => (
        fn asn1_tag() -> $crate::info::Tag {
            $tag
        }
    );
    (__impl $($args:tt)*) => (
        fn asn1_tag() -> $crate::info::Tag {
            static TAG: $crate::info::Tag = asn1_spec_tag!($($args)*);
            TAG
        }
    );
}

#[macro_export]
/// ASN.1 tagged object specification.
macro_rules! asn1_typed {
    ($rs_type:ty: ($($gen:tt)+), $asn_type:expr) => (
        impl<$($gen)+> $crate::info::Asn1Typed for $rs_type {
            asn1_typed!{__impl $asn_type}
        }
    );
    ($rs_type:ty, $asn_type:expr) => (
        impl $crate::info::Asn1Typed for $rs_type {
            asn1_typed!{__impl $asn_type}
        }
    );
    (__impl $asn_type:expr) => (
        fn asn1_type() -> &'static str {
            $asn_type
        }
    );
}

#[macro_export]
/// This macro parses an ASN.1 tag specification, and returns the appropriate Tag.
macro_rules! asn1_spec_tag {
    ({ $count:ident }) => (
        asn1_spec_tag!([])
    );
    ({ $count:ident } []) => ({
        let _count = $count;
        $count += 1;
        asn1_spec_tag!([CONTEXT _count])
    });
    ({ $count:ident } [$($args:tt)*]) => (
        asn1_spec_tag!([$($args)*])
    );
    ([$tagnum:expr]) => (
        asn1_spec_tag!(CONTEXT $tagnum);
    );
    ([UNIVERSAL $tagnum:expr]) => (
        $crate::info::Tag {
            class: $crate::info::Universal,
            tagnum: $tagnum,
            content_type: $crate::info::ContentType::Constructed
        }
    );
    ([CONTEXT $tagnum:expr]) => (
        $crate::info::Tag {
            class: $crate::info::ContextSpecific,
            tagnum: $tagnum,
            content_type: $crate::info::ContentType::Constructed
        }
    );
    ([APPLICATION $tagnum:expr]) => (
        $crate::info::Tag {
            class: $crate::info::Application,
            tagnum: $tagnum,
            content_type: $crate::info::ContentType::Constructed
        }
    );
    ([PRIVATE $tagnum:expr]) => (
        $crate::info::Tag {
            class: $crate::info::Private,
            tagnum: $tagnum,
            content_type: $crate::info::ContentType::Constructed
        }
    );
    (UNIVERSAL $tagnum:expr) => (
        $crate::info::Tag {
            class: $crate::info::Universal,
            tagnum: $tagnum,
            content_type: $crate::info::ContentType::Primitive
        }
    );
    (CONTEXT $tagnum:expr) => (
        $crate::info::Tag {
            class: $crate::info::ContextSpecific,
            tagnum: $tagnum,
            content_type: $crate::info::ContentType::Primitive
        }
    );
    (APPLICATION $tagnum:expr) => (
        $crate::info::Tag {
            class: $crate::info::Application,
            tagnum: $tagnum,
            content_type: $crate::info::ContentType::Primitive
        }
    );
    (PRIVATE $tagnum:expr) => (
        $crate::info::Tag {
            class: $crate::info::Private,
            tagnum: $tagnum,
            content_type: $crate::info::ContentType::Primitive
        }
    );
}

