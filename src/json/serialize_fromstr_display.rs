use std::{str::FromStr, fmt::Display};
use schemars::schema::{SchemaObject, InstanceType, StringValidation};
use serde::{de, Serialize, Deserialize};

pub fn deserialize<'de, T: FromStr<Err = E>, E: Display, D>(deserializer: D) -> Result<T, D::Error> 
where D: serde::Deserializer<'de>
{
    let s = <&str>::deserialize(deserializer)?;
    T::from_str(s).map_err(de::Error::custom)
}

pub fn serialize<T: Display, S>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer
{
    let s = format!("{}", t);
    s.serialize(serializer)
}

pub fn json_schema(pattern: Option<String>) -> schemars::schema::Schema {
    SchemaObject {
        instance_type: Some(InstanceType::String.into()),
        string: Some(Box::new(StringValidation {
            pattern,
            ..Default::default()
        })),
        ..Default::default()
    }
    .into()
}