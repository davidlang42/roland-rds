use std::{collections::HashMap, hash::Hash, marker::PhantomData, fmt::Display};
use schemars::{JsonSchema, schema::{SchemaObject, InstanceType, ObjectValidation, Schema}, Map, Set};
use serde::{Serialize, Deserialize, ser::SerializeMap};
use strum::IntoEnumIterator;

use super::type_name_pretty;

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

pub struct OptionalMapSchema<K: IntoEnumIterator + Display, V: JsonSchema>(PhantomData<K>, PhantomData<V>);

impl<K: IntoEnumIterator + Display, V: JsonSchema> JsonSchema for OptionalMapSchema<K, V> {
    fn schema_name() -> String {
        format!("Optional_map_of_{}_to_{}", type_name_pretty::<K>(), V::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        map_schema::<K, V>(gen, false)
    }
}

pub struct RequiredMapSchema<K: IntoEnumIterator + Display, V: JsonSchema>(PhantomData<K>, PhantomData<V>);

impl<K: IntoEnumIterator + Display, V: JsonSchema> JsonSchema for RequiredMapSchema<K, V> {
    fn schema_name() -> String {
        format!("Required_map_of_{}_to_{}", type_name_pretty::<K>(), V::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        map_schema::<K, V>(gen, true)
    }
}

fn map_schema<K: IntoEnumIterator + Display, V: JsonSchema>(gen: &mut schemars::gen::SchemaGenerator, include_required: bool) -> schemars::schema::Schema {
    let mut properties = Map::<String, Schema>::new();
    let mut required = Set::<String>::new();
    for key in K::iter() {
        properties.insert(key.to_string(), gen.subschema_for::<V>());
        if include_required {
            required.insert(key.to_string());
        }
    }
    SchemaObject {
        instance_type: Some(InstanceType::Object.into()),
        object: Some(Box::new(ObjectValidation {
            properties,
            required,
            additional_properties: Some(Box::new(Schema::Bool(false))),
            ..Default::default()
        })),
        ..Default::default()
    }
    .into()
}