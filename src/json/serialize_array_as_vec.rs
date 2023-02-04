use std::fmt::Debug;
use serde::{de, Serialize, Deserialize};

pub fn deserialize<'de, D, T: Deserialize<'de> + Debug, const N: usize>(deserializer: D) -> Result<Box<[T; N]>, D::Error> 
where D: serde::Deserializer<'de>
{
    let vec = Vec::<T>::deserialize(deserializer)?;
    if vec.len() != N {
        Err(de::Error::custom(format!("Expected {} {}, but got {}", N, std::any::type_name::<T>(), vec.len())))
    } else {
        Ok(Box::new(vec.try_into().unwrap()))
    }
}

pub fn serialize<S, T: Serialize, const N: usize>(t: &Box<[T; N]>, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer
{
    let vec: Vec<&T> = t.iter().collect();
    vec.serialize(serializer)
}
