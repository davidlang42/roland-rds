use std::fmt::Debug;

use schemars::{schema::{Schema, SchemaObject, InstanceType, NumberValidation, SubschemaValidation, ObjectValidation, Metadata}, Set, Map, JsonSchema};
use serde_json::Value;
use serde::Serialize;
use strum::IntoEnumIterator;

pub fn one_of_schema(sub_schemas: Vec<Schema>) -> Schema {
    SchemaObject {
        subschemas: Some(Box::new(SubschemaValidation {
            one_of: Some(sub_schemas),
            ..Default::default()
        })),
        ..Default::default()
    }
    .into()
}

pub fn u16_schema(min: u16, max: u16) -> Schema {
    integer_schema(min, max, "uint16")
}

pub fn u8_schema(min: u8, max: u8) -> Schema {
    integer_schema(min, max, "uint8")
}

pub fn i16_schema(min: i16, max: i16) -> Schema {
    integer_schema(min, max, "int16")
}

pub fn i8_schema(min: i8, max: i8) -> Schema {
    integer_schema(min, max, "int8")
}

fn integer_schema<T: Into<f64>>(min: T, max: T, format: &str) -> Schema {
    SchemaObject {
        instance_type: Some(InstanceType::Integer.into()),
        number: Some(Box::new(NumberValidation {
            minimum: Some(min.into()),
            maximum: Some(max.into()),
            ..Default::default()
        })),
        format: Some(format.into()),
        ..Default::default()
    }.into()
}

pub fn double_schema(min: f64, max: f64, multiple_of: f64) -> Schema {
    SchemaObject {
        instance_type: Some(InstanceType::Number.into()),
        number: Some(Box::new(NumberValidation {
            multiple_of: Some(multiple_of),
            minimum: Some(min),
            maximum: Some(max),
            ..Default::default()
        })),
        format: Some("double".into()),
        ..Default::default()
    }
    .into()
}

pub fn enum_schema(strings: Vec<String>) -> Schema {
    SchemaObject {
        instance_type: Some(InstanceType::String.into()),
        enum_values: Some(strings.into_iter().map(Value::String).collect()),
        ..Default::default()
    }.into()
}

pub fn single_property_schema_of<T: Default + Serialize + JsonSchema>(property: &str, gen: &mut schemars::gen::SchemaGenerator) -> Schema {
    let mut map = serde_json::Map::new();
    map.insert(property.to_string(), serde_json::to_value(T::default()).unwrap());
    let value = Value::Object(map);
    _single_property_schema(property, gen.subschema_for::<T>(), Some(value))
}

pub fn single_property_schema(property: &str, schema: Schema) -> Schema {
    _single_property_schema(property, schema, None)
}

fn _single_property_schema(property: &str, schema: Schema, default: Option<Value>) -> Schema {
    let mut required = Set::new();
    required.insert(property.into());
    let mut properties = Map::new();
    properties.insert(property.into(), schema);
    SchemaObject {
        instance_type: Some(InstanceType::Object.into()),
        object: Some(Box::new(ObjectValidation {
            required,
            properties,
            additional_properties: Some(Box::new(Schema::Bool(false))),
            ..Default::default()
        })),
        metadata: Some(Box::new(Metadata {
            default,
            ..Default::default()
        })),
        ..Default::default()
    }.into()
}

pub fn enum_except_one_schema<T: IntoEnumIterator + Debug>(string_to_ignore: &str) -> Schema {
    let mut strings = Vec::new();
    for value in T::iter() {
        let string = format!("{:?}", value);
        if string != string_to_ignore {
            strings.push(string);
        }
    }
    enum_schema(strings)
}
