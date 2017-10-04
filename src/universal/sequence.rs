#[macro_export]
macro_rules! asn1_seq {
    ($ty:ident: $asn1_type:expr, $($args:tt)+) => (
        asn1_info!($ty => $crate::info::TAG_SEQUENCE, $asn1_type);
        asn1_seq_ser!($ty, $($args)+);
        asn1_seq_de!($ty, $($args)+);
    );
}

#[macro_export]
macro_rules! asn1_seq_ser {
    ($ty:ty, $($args:tt)+) => (
        impl $crate::Asn1Serialize for $ty {
            fn asn1_serialize<S: $crate::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
                use $crate::ser::SeqSerializer;

                let mut seq_serializer = serializer.serialize_sequence()?;
                asn1_seq_ser!(__impl { self seq_serializer } $($args)+);
                seq_serializer.finish()
            }
        }
    );
    (__impl { $this:ident $seq:ident } $item:tt; $($args:tt)+) => (
        $seq.serialize_field(&$this.$item)?;
        asn1_seq_ser!(__impl { $this $seq } $($args)*)
    );
    (__impl { $this:ident $seq:ident } $item:tt) => (
        $seq.serialize_field(&$this.$item)?;
    );
}

#[macro_export]
macro_rules! asn1_seq_de {
    ($ty:ident, $($args:tt)+) => (
        impl $crate::Asn1Deserialize for $ty {
            fn asn1_deserialize<'de, D: $crate::Asn1Deserializer<'de>>(deserializer: D)
                                                                -> Result<Self, D::Err> {
                struct SeqVisitor;
                impl<'de> $crate::de::Asn1Visitor<'de> for SeqVisitor {
                    type Value = $ty;

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Err>
                        where A: $crate::de::SeqAccess<'de>
                    {
                        let v = asn1_seq_de!(__visit {$ty seq}, $($args)*);
                        Ok(v)
                    }
                }
                deserializer.deserialize_seq(SeqVisitor)
            }
        }
    );
    (__visit {$ty:ident $seq:ident}, $($args:tt);+) => (
        $ty { $(
            $args: $seq.next_field()?
        ),* }
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use universal::test_helper::ser_deser;

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seq(i32, i32, i32);

    asn1_seq!(
        Seq: "MY SEQ",
        0;
        1;
        2
    );

    #[test]
    fn seq() {
        let new_seq = Seq(0, 15, 65535);
        assert_eq!(new_seq, ser_deser(&new_seq));
    }
}