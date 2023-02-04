use serde::{de, Serialize, Deserialize};

pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[char; N], D::Error> 
where D: serde::Deserializer<'de>
{
    let s = <&str>::deserialize(deserializer)?;
    let chars: Vec<char> = s.chars().collect();
    if chars.len() != N {
        Err(de::Error::custom(format!("Expected {} chars, but got {}", N, chars.len())))
    } else {
        Ok(chars.try_into().unwrap())
    }
}

pub fn serialize<S, const N: usize>(chars: &[char; N], serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer
{
    let s: String = chars.iter().collect();
    s.serialize(serializer)
}
