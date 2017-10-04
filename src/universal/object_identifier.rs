// Module for OBJECT IDENTIFIER
use std::{default, fmt, error};
use std::str::FromStr;

use info;
use ser;
use de::{self, Asn1Visitor, Asn1Error};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ObjectIdentifier(Vec<u64>);

impl ObjectIdentifier {
    pub fn new(data: Vec<u64>) -> Self {
        ObjectIdentifier(data)
    }

    pub fn into_inner(self) -> Vec<u64> {
        self.0
    }

    pub fn from_slice(slice: &[u64]) -> Self {
        ObjectIdentifier(Vec::from(slice))
    }

    pub fn as_slice(&self) -> &[u64] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u64] {
        self.0.as_mut_slice()
    }
}

#[derive(Debug)]
pub struct ParseObjectIdError {
    _priv: (),
}

impl ParseObjectIdError {
    pub fn new() -> ParseObjectIdError {
        ParseObjectIdError { _priv: () }
    }
}

impl From<()> for ParseObjectIdError {
    fn from(_: ()) -> ParseObjectIdError {
        ParseObjectIdError { _priv: () }
    }
}

impl fmt::Display for ParseObjectIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string is not a valid Object Identifier (dot separated natural numbers)".fmt(f)
    }
}

impl error::Error for ParseObjectIdError {
    fn description(&self) -> &str {
        "provided string is not a valid Object Identifier (dot separated natural numbers)"
    }
}

impl FromStr for ObjectIdentifier {
    type Err = ParseObjectIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out: Result<Vec<u64>, Self::Err> = s.split(".")
            .map(|x| u64::from_str(x).map_err(|_| ().into()))
            .collect();
        out.map(ObjectIdentifier::new)
    }
}

impl fmt::Display for ObjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut has_fields = false;
        for i in self.0.iter() {
            let prefix = if has_fields {
                "."
            } else {
                ""
            };
            write!(f, "{}{}", prefix, i)?;
            has_fields = true;
        }
        Ok(())
    }
}

impl default::Default for ObjectIdentifier {
    fn default() -> ObjectIdentifier {
        Self::new(vec![0, 0])
    }
}

asn1_info!(ObjectIdentifier => info::TAG_OBJECT_IDENTIFIER, info::TYPE_OBJECT_IDENTIFIER);

impl ser::Asn1Serialize for ObjectIdentifier {
    fn asn1_serialize<S: ser::Asn1Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_object_identifier(self.as_slice())
    }
}

impl de::Asn1Deserialize for ObjectIdentifier {
    fn asn1_deserialize<'de, D: de::Asn1Deserializer<'de>>(deserializer: D)
                                                           -> Result<Self, D::Err> {
        struct BytesVisitor;
        impl<'de> Asn1Visitor<'de> for BytesVisitor {
            type Value = ObjectIdentifier;

            fn visit_object_identifier<E: Asn1Error>(self, v: Vec<u64>) -> Result<Self::Value, E> {
                Ok(ObjectIdentifier::new(v))
            }
        }
        deserializer.deserialize_object_identifier(BytesVisitor)
    }
}


#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::ObjectIdentifier;
    use universal::test_helper::ser_deser;

    impl Arbitrary for ObjectIdentifier {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let i0 = g.gen_range(0, 2);
            let i1 = g.gen_range(0, 39);

            let mut oid: Vec<u64> = vec![i0, i1];
            let tail: Vec<u64> = Arbitrary::arbitrary(g);
            oid.extend(tail);

            ObjectIdentifier::new(oid)
        }
    }

    #[quickcheck]
    fn object_identifier(v: ObjectIdentifier) -> bool {
        v == ser_deser(&v)
    }
}

