use std::fmt::Debug;
use serde::{de, Serialize, Deserialize};

pub fn deserialize<'de, D, T: Deserialize<'de> + Debug + Default, const N: usize>(deserializer: D) -> Result<Box<[T; N]>, D::Error> 
where D: serde::Deserializer<'de>
{
    let mut vec = Vec::<T>::deserialize(deserializer)?;
    if vec.len() > N {
        Err(de::Error::custom(format!("Expected a max of {} {}, but got {}", N, std::any::type_name::<T>(), vec.len())))
    } else {
        while vec.len() < N {
            vec.push(T::default());
        }
        Ok(Box::new(vec.try_into().unwrap()))
    }
}

pub fn serialize<S, T: Serialize + Default + PartialEq, const N: usize>(t: &Box<[T; N]>, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer
{
    let mut vec: Vec<&T> = t.iter().collect();
    while vec.len() > 0 && *vec[vec.len() - 1] == T::default() {
        vec.remove(vec.len() - 1);
    }
    vec.serialize(serializer)
}
