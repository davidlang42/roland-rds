use std::{fmt::Display, ops::AddAssign};
use schemars::JsonSchema;
use serde_json::Value;
use crate::json::{schema::enum_schema, type_name_pretty};

use super::super::numeric::Parameter;
use serde::{Serialize, Deserialize, de};

pub trait DiscreteValues<T: PartialEq + Display, const OFFSET: i16> {
    const OFFSET: i16 = OFFSET;

    fn values() -> Vec<T>;

    fn format(value: T) -> String;

    fn value_from(parameter: Parameter) -> T {
        let values = Self::values();
        if parameter.0 < OFFSET || parameter.0 >= OFFSET + values.len() as i16 {
            panic!("Parameter out of range: {} (expected {}-{})", parameter.0, OFFSET, OFFSET + values.len() as i16 - 1)
        }
        values.into_iter().nth((parameter.0 as i16 - OFFSET) as usize).unwrap()
    }

    fn into_parameter(value: T) -> Parameter {
        if let Some(position) = Self::values().iter().position(|v| *v == value) {
            return Parameter(position as i16 + OFFSET);
        } else {
            panic!("Invalid discrete value: {}", value);
        }
    }
}

/// Parameter(0-16) === LogFrequency(200-8000Hz)
#[derive(Debug, Copy, Clone)]
pub struct LogFrequency<const MIN: u16, const MAX: u16>(pub u16);

impl<const L: u16, const H: u16> LogFrequency<L, H> {
    const PRE_VALUES: [u16; 3] = [20, 25, 32];
    const BASE_VALUES: [u16; 10] = [40, 50, 63, 80, 100, 125, 160, 200, 250, 315];
    const MIN: u16 = L;
    const MAX: u16 = H;
}

impl<const L: u16, const H: u16> DiscreteValues<u16, 0> for LogFrequency<L, H> {
    fn values() -> Vec<u16> {
        let mut factor = 1;
        let mut v = Vec::new();
        for pre_value in Self::PRE_VALUES {
            if pre_value >= Self::MIN {
                if pre_value <= Self::MAX {
                    v.push(pre_value);
                } else {
                    break;
                }
            }
        }
        loop {
            for base_value in Self::BASE_VALUES {
                let current = base_value * factor;
                if current >= Self::MIN {
                    if current <= Self::MAX {
                        v.push(current);
                    } else {
                        return v;
                    }
                }
            }
            factor *= 10;
        }
    }

    fn format(value: u16) -> String {
        format!("{}Hz", value)
    }
}

impl<const L: u16, const H: u16> JsonSchema for LogFrequency<L, H> {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl<const L: u16, const H: u16> Serialize for LogFrequency<L, H> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de, const L: u16, const H: u16> Deserialize<'de> for LogFrequency<L, H> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => {
                for v in Self::values().into_iter() {
                    if s == Self::format(v) {
                        return Ok(Self(v));
                    }
                }
                Err(de::Error::custom(format!("String is not a valid discrete value: {}", s)))
            }
            _ => Err(de::Error::custom(format!("Expected string")))
        }
    }
}

impl<const L: u16, const H: u16> From<Parameter> for LogFrequency<L, H> {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl<const L: u16, const H: u16> Into<Parameter> for LogFrequency<L, H> {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}

/// Parameter(0-17) === LogFrequencyOrByPass(200-8000Hz, BYPASS)
#[derive(Serialize, Deserialize, Debug, JsonSchema, Copy, Clone)]
pub enum LogFrequencyOrByPass<const MIN: u16, const MAX: u16> {
    Frequency(LogFrequency<MIN, MAX>),
    ByPass
}

impl<const L: u16, const H: u16> From<Parameter> for LogFrequencyOrByPass<L, H> {
    fn from(value: Parameter) -> Self {
        let values = LogFrequency::<L, H>::values();
        if value.0 < LogFrequency::<L, H>::OFFSET || value.0 > LogFrequency::<L, H>::OFFSET + values.len() as i16 {
            panic!("Parameter out of range: {} (expected {}-{})", value.0, LogFrequency::<L, H>::OFFSET, LogFrequency::<L, H>::OFFSET + values.len() as i16)
        } else if value.0 == values.len() as i16 {
            Self::ByPass
        } else {
            Self::Frequency(value.into())
        }
    }
}

impl<const L: u16, const H: u16> Into<Parameter> for LogFrequencyOrByPass<L, H> {
    fn into(self) -> Parameter {
        match self {
            Self::Frequency(f) => f.into(),
            Self::ByPass => Parameter(LogFrequency::<L, H>::OFFSET + LogFrequency::<L, H>::values().len() as i16)
        }
    }
}

/// Parameter(1-200) === LinearFrequency(0.05-10.00 by 0.05)
#[derive(Debug, Copy, Clone)]
pub struct LinearFrequency(pub f64);

impl DiscreteValues<f64, 1> for LinearFrequency {
    fn values() -> Vec<f64> {
        enumerate_f64(0.05, 10.0, 0.05)
    }

    fn format(value: f64) -> String {
        format!("{:.2}ms", value)
    }
}

impl JsonSchema for LinearFrequency {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for LinearFrequency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LinearFrequency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => {
                for v in Self::values().into_iter() {
                    if s == Self::format(v) {
                        return Ok(Self(v));
                    }
                }
                Err(de::Error::custom(format!("String is not a valid discrete value: {}", s)))
            }
            _ => Err(de::Error::custom(format!("Expected string")))
        }
    }
}

impl From<Parameter> for LinearFrequency {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for LinearFrequency {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}

/// Parameter(0-?) === LogMilliseconds(0-5 by 0.1, 5-10 by 0.5, 10-50 by 1, 50-100 by 2)
#[derive(Debug, Copy, Clone)]
pub struct LogMilliseconds(pub f64);

impl DiscreteValues<f64, 0> for LogMilliseconds {
    fn values() -> Vec<f64> {
        flatten(vec![
            enumerate_f64(0.0, 4.9, 0.1),
            enumerate_f64(5.0, 9.5, 0.5),
            enumerate_f64(10.0, 49.0, 1.0),
            enumerate_f64(50.0, 100.0, 2.0)
        ])
    }

    fn format(value: f64) -> String {
        format!("{:.1}ms", value)
    }
}

impl JsonSchema for LogMilliseconds {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for LogMilliseconds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LogMilliseconds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => {
                for v in Self::values().into_iter() {
                    if s == Self::format(v) {
                        return Ok(Self(v));
                    }
                }
                Err(de::Error::custom(format!("String is not a valid discrete value: {}", s)))
            }
            _ => Err(de::Error::custom(format!("Expected string")))
        }
    }
}

fn enumerate_f64(start: f64, end: f64, step: f64) -> Vec<f64> {
    let mut values = Vec::new();
    let mut i: usize = 0;
    let mut v = start;
    const EPSILON: f64 = 0.000001;
    while v <= end + EPSILON {
        values.push(v);
        i += 1;
        v = start + i as f64 * step;
    }
    values
}

fn enumerate<T: PartialOrd + AddAssign + Copy>(start: T, end: T, step: T) -> Vec<T> {
    let mut values = Vec::new();
    let mut v = start;
    while v <= end {
        values.push(v);
        v += step;
    }
    values
}

fn flatten<T>(vectors: Vec<Vec<T>>) -> Vec<T> {
    let mut output = Vec::new();
    for vec in vectors.into_iter() {
        for t in vec.into_iter() {
            output.push(t);
        }
    }
    output
}

impl From<Parameter> for LogMilliseconds {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for LogMilliseconds {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}

/// Parameter(0-?) === EvenPercent(-98% to 98% by 2)
#[derive(Debug, Copy, Clone)]
pub struct EvenPercent(pub i8);

impl DiscreteValues<i8, 0> for EvenPercent {
    fn values() -> Vec<i8> {
        enumerate(-98, 98, 2)
    }

    fn format(value: i8) -> String {
        format!("{}%", value)
    }
}

impl JsonSchema for EvenPercent {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for EvenPercent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EvenPercent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => {
                for v in Self::values().into_iter() {
                    if s == Self::format(v) {
                        return Ok(Self(v));
                    }
                }
                Err(de::Error::custom(format!("String is not a valid discrete value: {}", s)))
            }
            _ => Err(de::Error::custom(format!("Expected string")))
        }
    }
}

impl From<Parameter> for EvenPercent {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for EvenPercent {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}

/// Parameter(0-4) === QFactor(0.5, 1.0, 2.0, 4.0, 8.0)
#[derive(Debug, Copy, Clone)]
pub struct QFactor(pub f64);

impl DiscreteValues<f64, 0> for QFactor {
    fn values() -> Vec<f64> {
        vec![0.5, 1.0, 2.0, 4.0, 8.0]
    }

    fn format(value: f64) -> String {
        format!("{:.1}", value)
    }
}

impl JsonSchema for QFactor {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for QFactor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for QFactor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => {
                for v in Self::values().into_iter() {
                    if s == Self::format(v) {
                        return Ok(Self(v));
                    }
                }
                Err(de::Error::custom(format!("String is not a valid discrete value: {}", s)))
            }
            _ => Err(de::Error::custom(format!("Expected string")))
        }
    }
}

impl From<Parameter> for QFactor {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for QFactor {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}