use std::{collections::HashMap, hash::Hash};
use serde::{Serialize, Deserialize, ser::SerializeMap};

pub fn deserialize<'de, D, K: Deserialize<'de> + Eq + Hash, V: Deserialize<'de>>(deserializer: D) -> Result<HashMap<K, V>, D::Error> 
where D: serde::Deserializer<'de>
{
    HashMap::<K, V>::deserialize(deserializer)
}

pub fn serialize<S, K: Serialize + Ord + Eq + Hash, V: Serialize>(t: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer
{
    let mut pairs: Vec<(_, _)> = t.iter().collect();
    pairs.sort_by(|(a, _), (b, _)| a.cmp(&b));
    let mut map = serializer.serialize_map(Some(pairs.len()))?;
    for (k, v) in pairs {
        map.serialize_entry(k, v)?;
    }
    map.end()
}
