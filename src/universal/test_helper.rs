use der;

use ser::Asn1Serialize;
use de::Asn1Deserialize;

#[allow(dead_code)]
pub fn ser_deser<T>(v: &T) -> T
    where T: Asn1Serialize + for<'de> Asn1Deserialize
{
    let mut buf: Vec<u8> = Vec::new();
    {
        let serializer = der::Serializer::new(&mut buf);
        v.asn1_serialize(serializer).unwrap();
    }
    let deserializer = der::Deserializer::new(&buf[..]);
    T::asn1_deserialize(deserializer).unwrap()
}

