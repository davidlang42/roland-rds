use std::{fmt::Debug, marker::PhantomData};
use schemars::{JsonSchema, schema::{SchemaObject, InstanceType, ArrayValidation}};
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

pub struct ArraySchema<T: JsonSchema, const N: usize>(PhantomData<T>);

impl<T: JsonSchema, const N: usize> JsonSchema for ArraySchema<T, N> {
    fn schema_name() -> String {
        format!("Array_size_{}_of_{}", N, T::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let items = vec![gen.subschema_for::<T>()];
        let required_size = N.try_into().unwrap();
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(items.into()),
                max_items: Some(required_size),
                min_items: Some(required_size),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}