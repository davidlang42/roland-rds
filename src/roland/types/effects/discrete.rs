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

    fn equal(a: &T, b: &T) -> bool {
        a == b
    }
}

const EPSILON: f64 = 0.000001;

fn enumerate_f64(start: f64, end: f64, step: f64) -> Vec<f64> {
    let mut values = Vec::new();
    let mut i: usize = 0;
    let mut v = start;
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

/// Parameter(0-8) === FineFrequency(50-125Hz)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct FineFrequency(pub u8);

impl DiscreteValues<u8, 0> for FineFrequency {
    fn values() -> Vec<u8> {
        vec![50, 56, 63, 71, 80, 90, 100, 112, 125]
    }

    fn format(value: u8) -> String {
        format!("{}Hz", value)
    }
}

/// Parameter(0-2) === FilterSlope(-12, -24, -36)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct FilterSlope(pub i8);

impl DiscreteValues<i8, 0> for FilterSlope {
    fn values() -> Vec<i8> {
        vec![-12, -24, -36]
    }

    fn format(value: i8) -> String {
        format!("{}dB", value)
    }
}

/// Parameter(0-100) === Balance(D100:0W to D0:100W)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct Balance(pub u8); // value is the EFFECT (W) percentage

impl DiscreteValues<u8, 0> for Balance {
    fn values() -> Vec<u8> {
        enumerate(0, 100, 1)
    }

    fn format(value: u8) -> String {
        format!("D{}:{}W", value, 100-value)
    }
}

/// Parameter(1-200) === LinearFrequency(0.05-10.00 by 0.05)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct LinearFrequency(pub f64);

impl DiscreteValues<f64, 1> for LinearFrequency {
    fn values() -> Vec<f64> {
        enumerate_f64(0.05, 10.0, 0.05)
    }

    fn format(value: f64) -> String {
        format!("{:.2}ms", value)
    }

    fn equal(a: &f64, b: &f64) -> bool {
        (a - b).abs() < EPSILON
    }
}

/// Parameter(1-200) === StepLinearFrequency(0.1-20.0 by 0.1)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct StepLinearFrequency(pub f64);

impl DiscreteValues<f64, 1> for StepLinearFrequency {
    fn values() -> Vec<f64> {
        enumerate_f64(0.1, 20.0, 0.1)
    }

    fn format(value: f64) -> String {
        format!("{:.1}ms", value)
    }

    fn equal(a: &f64, b: &f64) -> bool {
        (a - b).abs() < EPSILON
    }
}

/// Parameter(0-?) === LogMilliseconds(0-5 by 0.1, 5-10 by 0.5, 10-50 by 1, 50-100 by 2)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
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

    fn equal(a: &f64, b: &f64) -> bool {
        (a - b).abs() < EPSILON
    }
}

/// Parameter(0-?) === EvenPercent(-98% to 98% by 2)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
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

pub type Feedback = EvenPercent<98>;

/// Parameter(0-4) === QFactor(0.5, 1.0, 2.0, 4.0, 8.0)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct QFactor(pub f64);

impl DiscreteValues<f64, 0> for QFactor {
    fn values() -> Vec<f64> {
        vec![0.5, 1.0, 2.0, 4.0, 8.0]
    }

    fn format(value: f64) -> String {
        format!("{:.1}", value)
    }
}

/// Parameter(0-1) === HumFrequency(50, 60)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct HumFrequency(pub u8);

impl DiscreteValues<u8, 0> for HumFrequency {
    fn values() -> Vec<u8> {
        vec![50, 60]
    }

    fn format(value: u8) -> String {
        format!("{}Hz", value)
    }
}

/// Parameter(0-90) === Phase(0-180)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct Phase(pub u8);

impl DiscreteValues<u8, 0> for Phase {
    fn values() -> Vec<u8> {
        enumerate(0, 180, 2)
    }

    fn format(value: u8) -> String {
        format!("{}deg", value)
    }
}

/// Parameter(0-?) === GateTime(5-500)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct GateTime(pub u16);

impl DiscreteValues<u16, 0> for GateTime {
    fn values() -> Vec<u16> {
        enumerate(5, 500, 5)
    }

    fn format(value: u16) -> String {
        format!("{}ms", value)
    }
}

/// Parameter(0-16) === LogFrequency(200-8000Hz)
#[derive(Debug, Copy, Clone, DiscreteValuesSerialization)]
pub struct LogFrequency<const MIN: u16, const MAX: u16>(pub u16);

impl<const MIN: u16, const MAX: u16> LogFrequency<MIN, MAX> {
    const PRE_VALUES: [u16; 4] = [16, 20, 25, 32];
    const BASE_VALUES: [u16; 10] = [40, 50, 63, 80, 100, 125, 160, 200, 250, 315];
}

impl<const MIN: u16, const MAX: u16> DiscreteValues<u16, 0> for LogFrequency<MIN, MAX> {
    fn values() -> Vec<u16> {
        let mut factor = 1;
        let mut v = Vec::new();
        for pre_value in Self::PRE_VALUES {
            if pre_value >= MIN {
                if pre_value <= MAX {
                    v.push(pre_value);
                } else {
                    break;
                }
            }
        }
        loop {
            for base_value in Self::BASE_VALUES {
                let current = base_value * factor;
                if current >= MIN {
                    if current <= MAX {
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