pub mod tag;
pub mod universal;


pub use self::tag::{Class, Tag, Len, TagNum, LenNum};
pub use self::tag::Class::*;
pub use self::universal::*;

pub trait Asn1Tagged {
    fn asn1_tag() -> &'static tag::Tag;
}

pub trait Asn1Typed {
    fn asn1_type() -> &'static str;
}

pub trait Asn1Info: Asn1Tagged + Asn1Typed {}

#[macro_export]
macro_rules! asn1_info {
    ($rs_type:ty: ($($gen:tt)+) => [$($args:tt)+], $asn_type:expr) => (
        asn1_tagged!($rs_type: ($($gen)+), [$($args)+]);
        asn1_typed!($rs_type: ($($gen)*), $asn_type);
    );
    ($rs_type:ty: ($($gen:tt)+) => $class:ident $tagnum:expr, $asn_type:expr) => (
        asn1_tagged!($rs_type: ($($gen)+), $class $tagnum);
        asn1_typed!($rs_type: ($($gen)+), $asn_type);
    );
    ($rs_type:ty => [$($args:tt)*], $asn_type:expr) => {
        asn1_tagged!($rs_type, [$($args)*]);
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
    ($rs_type:ty, $($args:tt)*) => (
        impl $crate::info::Asn1Tagged for $rs_type {
            asn1_tagged!{__impl $($args)*}
        }
    );
    (__impl $($args:tt)*) => (
        fn asn1_tag() -> &'static $crate::info::Tag {
            static TAG: $crate::info::Tag = asn1_spec_tag!($($args)*);
            &TAG
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
        $crate::info::Tag::const_new($crate::info::Universal, $tagnum, true);
    );
    ([CONTEXT $tagnum:expr]) => (
        $crate::info::Tag::const_new($crate::info::ContextSpecific, $tagnum, true);
    );
    ([APPLICATION $tagnum:expr]) => (
        $crate::info::Tag::const_new($crate::info::Application, $tagnum, true);
    );
    ([PRIVATE $tagnum:expr]) => (
        $crate::info::Tag::const_new($crate::info::Private, $tagnum, true);
    );
    (UNIVERSAL $tagnum:expr) => (
        $crate::info::Tag::const_new($crate::info::Universal, $tagnum, false);
    );
    (CONTEXT $tagnum:expr) => (
        $crate::info::Tag::const_new($crate::info::ContextSpecific, $tagnum, false);
    );
    (APPLICATION $tagnum:expr) => (
        $crate::info::Tag::const_new($crate::info::Application, $tagnum, false);
    );
    (PRIVATE $tagnum:expr) => (
        $crate::info::Tag::const_new($crate::info::Private, $tagnum, false);
    );
}

