use std::{fmt::Debug, marker::PhantomData};
use schemars::{JsonSchema, schema::{InstanceType, SchemaObject, ArrayValidation}};
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

pub struct DefaultTerminatedArraySchema<T: JsonSchema, const N: usize>(PhantomData<T>);

impl<T: JsonSchema, const N: usize> JsonSchema for DefaultTerminatedArraySchema<T, N> {
    fn is_referenceable() -> bool {
        false
    }
    
    fn schema_name() -> String {
        format!("Default_terminated_array_size_{}_of_{}", N, T::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let items = vec![gen.subschema_for::<T>()];
        let required_size = N.try_into().unwrap();
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(items.into()),
                max_items: Some(required_size),
                min_items: Some(0),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}