use std::{fmt::Display, ops::AddAssign};
use schemars::JsonSchema;
use serde_json::Value;
use crate::json::{schema::enum_schema, type_name_pretty};

use super::super::numeric::Parameter;
use serde::{Serialize, Deserialize, de};

//TODO refactor DiscreteValues to make use of derive macros
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
    const PRE_VALUES: [u16; 4] = [16, 20, 25, 32];
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

/// Parameter(0-8) === FineFrequency(50-125Hz)
#[derive(Debug, Copy, Clone)]
pub struct FineFrequency(pub u8);

impl DiscreteValues<u8, 0> for FineFrequency {
    fn values() -> Vec<u8> {
        vec![50, 56, 63, 71, 80, 90, 100, 112, 125]
    }

    fn format(value: u8) -> String {
        format!("{}Hz", value)
    }
}

impl JsonSchema for FineFrequency {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for FineFrequency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FineFrequency {
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

impl From<Parameter> for FineFrequency {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for FineFrequency {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}

/// Parameter(0-2) === FilterSlope(-12, -24, -36)
#[derive(Debug, Copy, Clone)]
pub struct FilterSlope(pub i8);

impl DiscreteValues<i8, 0> for FilterSlope {
    fn values() -> Vec<i8> {
        vec![-12, -24, -36]
    }

    fn format(value: i8) -> String {
        format!("{}dB", value)
    }
}

impl JsonSchema for FilterSlope {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for FilterSlope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FilterSlope {
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

impl From<Parameter> for FilterSlope {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for FilterSlope {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}

/// Parameter(0-100) === Balance(D100:0W to D0:100W)
#[derive(Debug, Copy, Clone)]
pub struct Balance(pub u8); // value is the EFFECT (W) percentage

impl DiscreteValues<u8, 0> for Balance {
    fn values() -> Vec<u8> {
        enumerate(0, 100, 1)
    }

    fn format(value: u8) -> String {
        format!("D{}:{}W", value, 100-value)
    }
}

impl JsonSchema for Balance {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for Balance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Balance {
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

impl From<Parameter> for Balance {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for Balance {
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

/// Parameter(1-18) === LogFrequencyOrByPassOffByOne(200-8000Hz, BYPASS)
#[derive(Serialize, Deserialize, Debug, JsonSchema, Copy, Clone)]
pub enum LogFrequencyOrByPassOffByOne<const MIN: u16, const MAX: u16> {
    Frequency(LogFrequency<MIN, MAX>),
    ByPass
}

impl<const L: u16, const H: u16> From<Parameter> for LogFrequencyOrByPassOffByOne<L, H> {
    fn from(value: Parameter) -> Self {
        let values = LogFrequency::<L, H>::values();
        if value.0 < LogFrequency::<L, H>::OFFSET + 1 || value.0 > LogFrequency::<L, H>::OFFSET + values.len() as i16 + 1 {
            panic!("Parameter out of range: {} (expected {}-{})", value.0, LogFrequency::<L, H>::OFFSET + 1, LogFrequency::<L, H>::OFFSET + values.len() as i16 + 1)
        } else if value.0 == values.len() as i16 + 1 {
            Self::ByPass
        } else {
            Self::Frequency(Parameter(value.0 - 1).into())
        }
    }
}

impl<const L: u16, const H: u16> Into<Parameter> for LogFrequencyOrByPassOffByOne<L, H> {
    fn into(self) -> Parameter {
        match self {
            Self::Frequency(f) => Parameter(Into::<Parameter>::into(f).0 + 1),
            Self::ByPass => Parameter(LogFrequency::<L, H>::OFFSET + LogFrequency::<L, H>::values().len() as i16 + 1)
        }
    }
}

/// Parameter(0-17) === ByPassOrLogFrequency(BYPASS, 200-8000Hz)
#[derive(Serialize, Deserialize, Debug, JsonSchema, Copy, Clone)]
pub enum ByPassOrLogFrequency<const MIN: u16, const MAX: u16> {
    ByPass,
    Frequency(LogFrequency<MIN, MAX>)
}

impl<const L: u16, const H: u16> From<Parameter> for ByPassOrLogFrequency<L, H> {
    fn from(value: Parameter) -> Self {
        let values = LogFrequency::<L, H>::values();
        if value.0 == 0 {
            Self::ByPass
        } else if value.0 < LogFrequency::<L, H>::OFFSET + 1 || value.0 > LogFrequency::<L, H>::OFFSET + values.len() as i16 + 1 {
            panic!("Parameter out of range: {} (expected {}-{})", value.0, LogFrequency::<L, H>::OFFSET + 1, LogFrequency::<L, H>::OFFSET + values.len() as i16 + 1)
        } else {
            Self::Frequency(Parameter(value.0 - 1).into())
        }
    }
}

impl<const L: u16, const H: u16> Into<Parameter> for ByPassOrLogFrequency<L, H> {
    fn into(self) -> Parameter {
        match self {
            Self::ByPass => Parameter(0),
            Self::Frequency(f) => Parameter(Into::<Parameter>::into(f).0 + 1),
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

/// Parameter(1-200) === StepLinearFrequency(0.1-20.0 by 0.1)
#[derive(Debug, Copy, Clone)]
pub struct StepLinearFrequency(pub f64);

impl DiscreteValues<f64, 1> for StepLinearFrequency {
    fn values() -> Vec<f64> {
        enumerate_f64(0.1, 20.0, 0.1)
    }

    fn format(value: f64) -> String {
        format!("{:.1}ms", value)
    }
}

impl JsonSchema for StepLinearFrequency {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for StepLinearFrequency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StepLinearFrequency {
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

impl From<Parameter> for StepLinearFrequency {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for StepLinearFrequency {
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
pub struct EvenPercent<const LIMIT: u8>(pub i8);

impl<const LIMIT: u8> DiscreteValues<i8, 0> for EvenPercent<LIMIT> {
    fn values() -> Vec<i8> {
        if LIMIT > i8::MAX as u8 {
            panic!("Invalid EvenPercent limit: {}", LIMIT);
        }
        enumerate(-(LIMIT as i8), LIMIT as i8, 2)
    }

    fn format(value: i8) -> String {
        format!("{}%", value)
    }
}

impl<const LIMIT: u8> JsonSchema for EvenPercent<LIMIT> {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl<const LIMIT: u8> Serialize for EvenPercent<LIMIT> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de, const LIMIT: u8> Deserialize<'de> for EvenPercent<LIMIT> {
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

impl<const LIMIT: u8> From<Parameter> for EvenPercent<LIMIT> {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl<const LIMIT: u8> Into<Parameter> for EvenPercent<LIMIT> {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}

pub type Feedback = EvenPercent<98>;

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

/// Parameter(0-1) === HumFrequency(50, 60)
#[derive(Debug, Copy, Clone)]
pub struct HumFrequency(pub u8);

impl DiscreteValues<u8, 0> for HumFrequency {
    fn values() -> Vec<u8> {
        vec![50, 60]
    }

    fn format(value: u8) -> String {
        format!("{}Hz", value)
    }
}

impl JsonSchema for HumFrequency {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        enum_schema(Self::values().into_iter().map(Self::format).collect())
    }
}

impl Serialize for HumFrequency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        Self::format(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HumFrequency {
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

impl From<Parameter> for HumFrequency {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl Into<Parameter> for HumFrequency {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}