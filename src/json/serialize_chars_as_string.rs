use schemars::{schema::{StringValidation, SchemaObject, InstanceType}, JsonSchema};
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

pub struct StringSchema<const N: usize>();

impl<const N: usize> JsonSchema for StringSchema<N> {
    fn is_referenceable() -> bool {
        false
    }
    
    fn schema_name() -> String {
        format!("String_size_{}", N)
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let required_size = N.try_into().unwrap();
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            string: Some(Box::new(StringValidation {
                min_length: Some(required_size),
                max_length: Some(required_size),
                pattern: Some("[ -~]*".to_owned()), // ascii chars 32-127
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}