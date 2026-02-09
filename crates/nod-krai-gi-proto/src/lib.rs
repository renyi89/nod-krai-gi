pub use prost::DecodeError as ProtobufDecodeError;
pub use prost::Message as Protobuf;

pub mod dy_parser;
pub mod gm;
pub mod normal;
pub mod packet;
pub mod packet_head;
pub mod raw_packet;
pub mod retcode;
pub mod server_only;

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
mod base64 {
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64_simd::STANDARD.encode_to_string(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64_simd::STANDARD
            .decode_to_vec(&*base64)
            .map_err(|e| serde::de::Error::custom(e))
    }
}

mod u64_string {
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S: Serializer>(v: &u64, s: S) -> Result<S::Ok, S::Error> {
        String::serialize(&v.to_string(), s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        let str = String::deserialize(d)?;
        u64::from_str(&*str).map_err(|e| serde::de::Error::custom(e))
    }
}

mod u64_repeated_string {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;

    pub fn serialize<S: Serializer>(v: &Vec<u64>, s: S) -> Result<S::Ok, S::Error> {
        let string_vec: Vec<String> = v.iter().map(|num| num.to_string()).collect();
        string_vec.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u64>, D::Error> {
        let string_vec: Vec<String> = Vec::deserialize(d)?;
        string_vec
            .into_iter()
            .map(|s| u64::from_str(&s).map_err(serde::de::Error::custom))
            .collect()
    }
}

#[allow(dead_code)]
mod u64_map_key_string {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    use std::str::FromStr;

    #[allow(unused_imports)]
    pub fn serialize<S: Serializer, T: Serialize>(
        map: &HashMap<u64, T>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        let string_key_map: HashMap<String, &T> =
            map.iter().map(|(k, v)| (k.to_string(), v)).collect();
        string_key_map.serialize(s)
    }

    #[allow(unused_imports)]
    pub fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        d: D,
    ) -> Result<HashMap<u64, T>, D::Error> {
        let string_key_map: HashMap<String, T> = HashMap::deserialize(d)?;
        string_key_map
            .into_iter()
            .map(|(k, v)| {
                u64::from_str(&k)
                    .map(|key| (key, v))
                    .map_err(serde::de::Error::custom)
            })
            .collect()
    }
}

#[allow(dead_code)]
mod u64_map_value_string {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    #[allow(unused_imports)]
    use std::fmt::Display;
    use std::hash::Hash;
    use std::str::FromStr;

    #[allow(dead_code)]
    pub fn serialize<S: Serializer, K: Serialize + Eq + Hash>(
        map: &HashMap<K, u64>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        let string_value_map: HashMap<&K, String> =
            map.iter().map(|(k, v)| (k, v.to_string())).collect();
        string_value_map.serialize(s)
    }

    #[allow(dead_code)]
    pub fn deserialize<'de, D: Deserializer<'de>, K: Deserialize<'de> + Eq + Hash>(
        d: D,
    ) -> Result<HashMap<K, u64>, D::Error> {
        let string_value_map: HashMap<K, String> = HashMap::deserialize(d)?;
        string_value_map
            .into_iter()
            .map(|(k, v)| {
                u64::from_str(&v)
                    .map(|num| (k, num))
                    .map_err(serde::de::Error::custom)
            })
            .collect()
    }
}

#[allow(dead_code)]
mod u64_map_both_string {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    use std::str::FromStr;

    #[allow(dead_code)]
    pub fn serialize<S: Serializer>(map: &HashMap<u64, u64>, s: S) -> Result<S::Ok, S::Error> {
        let string_both_map: HashMap<String, String> = map
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        string_both_map.serialize(s)
    }

    #[allow(dead_code)]
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<HashMap<u64, u64>, D::Error> {
        let string_both_map: HashMap<String, String> = HashMap::deserialize(d)?;
        string_both_map
            .into_iter()
            .map(|(k, v)| {
                let key = u64::from_str(&k).map_err(serde::de::Error::custom)?;
                let value = u64::from_str(&v).map_err(serde::de::Error::custom)?;
                Ok((key, value))
            })
            .collect()
    }
}
