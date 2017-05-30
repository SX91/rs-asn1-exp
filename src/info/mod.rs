pub mod tag;


pub use self::tag::{Class, Tag, Len, TagNum, LenNum};
pub use self::tag::Class::*;


pub trait Asn1Info {
    fn asn1_tag() -> Option<&'static tag::Tag>;
    fn asn1_type() -> &'static str;

    fn is_primitive(&self) -> bool {
        if let Some(tag) = Self::asn1_tag() {
            !tag.is_constructed()
        } else {
            false  // primitives should be tagged?
        }
    }
}

#[macro_export]
/// ASN.1 object specification.
macro_rules! asn1_info {
  ($rs_type:ty: ($($gen:tt)+), $($args:tt)*) => (
    impl<$($gen)+> $crate::Asn1Info for $rs_type {
      asn1_info!{__impl $($args)*}
    }
  );
  (__impl [$($args:tt)*], $asn1_ty:expr) => (
    fn asn1_tag() -> Option<&'static $crate::info::Tag> {
        static TAG: $crate::info::Tag = asn1_spec_tag!([$($args)*]);
        Some(&TAG)
    }
    asn1_info!(__type $asn1_ty);
  );
  (__impl $class:expr, $tagnum:expr, $constructed:expr, $asn1_ty:expr) => (
    fn asn1_tag() -> Option<&'static $crate::info::Tag> {
        static TAG: $crate::info::Tag = $crate::info::Tag::const_new($class, $tagnum, $constructed);
        Some(&TAG)
    }
    asn1_info!(__type $asn1_ty);
  );
  (__impl $asn1_ty:expr) => (
    fn asn1_tag() -> Option<&'static $crate::info::Tag> {
      None
    }
    asn1_info!(__type $asn1_ty);
  );
  (__type $asn1_ty:expr) => (
    fn asn1_type() -> &'static str {
        $asn1_ty
    }
  );
  (__impl $class:ident $tagnum:expr, $asn1_ty:expr) => (
    fn asn1_tag() -> Option<&'static $crate::info::Tag> {
        static TAG: $crate::info::Tag = asn1_spec_tag!($class $tagnum);
        Some(&TAG)
    }
    asn1_info!(__type $asn1_ty);
  );
  ($rs_type:ty, $($args:tt)*) => (
    impl $crate::Asn1Info for $rs_type {
      asn1_info!{__impl $($args)*}
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