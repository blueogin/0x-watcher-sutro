use crate::prelude::*;
use serde::{de, ser};
use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct Bytes(Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // OPT: Avoid allocations
        serializer.serialize_str(&format!("0x{}", hex::encode(&self.0)))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Bytes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a hexadecimal string")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let str = if str.starts_with("0x") {
                    &str[2..]
                } else {
                    str
                };
                let vec = hex::decode(str).map_err::<E, _>(de::Error::custom)?;
                Ok(Bytes(vec))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::prelude::{assert_eq, *};
    use serde_json::{from_value, json, to_value};

    #[test]
    fn test_serialize_default() {
        let obj = Bytes::default();
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x"));
        let de: Bytes = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_serialize_random() {
        let obj = Bytes(b"random".to_vec());
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x72616e646f6d"));
        let de: Bytes = from_value(json).unwrap();
        assert_eq!(de, obj);
    }
}
