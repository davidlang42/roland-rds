use std::fmt::Display;
use strum::IntoEnumIterator;

use super::enums::FilterType;
use super::numeric::Parameter;
use crate::json::{serialize_default_terminated_array, validation::merge_all_fixed};
use crate::json::validation::{valid_boxed_elements, validate_boxed_array};
use schemars::JsonSchema;
use strum_macros::EnumIter;
use validator::{Validate, ValidationErrors};

trait Parameters<const N: usize> : Validate + From<[Parameter; N]> {
    fn parameters(&self) -> [Parameter; N];
}


#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct UnusedParameters<const N: usize> {
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, {N}>")]
    unused: Box<[Parameter; N]>
}

impl<const N: usize> From<[Parameter; N]> for UnusedParameters<N> {
    fn from(value: [Parameter; N]) -> Self {
        Self {
            unused: Box::new(value)
        }
    }
}

impl<const N: usize> Parameters<N> for UnusedParameters<N> {
    fn parameters(&self) -> [Parameter; N] {
        *self.unused
    }
}

impl<const N: usize> Validate for UnusedParameters<N> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut r = Ok(());
        r = merge_all_fixed(r, "unused", validate_boxed_array(&self.unused));
        r
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ChorusType { // 0-3
    Off(UnusedParameters<20>),
    Chorus(ChorusParameters),
    Delay,
    Gm2Chorus
}

impl ChorusType {
    pub fn from(number: u8, parameters: [Parameter; 20]) -> Self {
        match number {
            0 => Self::Off(parameters.into()),
            1 => Self::Chorus(parameters.into()),
            2 => Self::Delay, //TODO paramters
            3 => Self::Gm2Chorus, //TODO parameters
            _ => panic!("Invalid chorus type: {}", number)
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::Off(_) => 0,
            Self::Chorus(_) => 1,
            Self::Delay => 2,
            Self::Gm2Chorus => 3
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Off(_) => "Off",
            Self::Chorus(_) => "Chorus",
            Self::Delay => "Delay",
            Self::Gm2Chorus => "Gm2Chorus"
        }
    }
    
    pub fn parameters(&self) -> [Parameter; 20] {
        match self {
            Self::Off(u) => u.parameters(),
            Self::Chorus(c) => c.parameters(),
            Self::Delay => todo!(), //TODO parameters
            Self::Gm2Chorus => todo!() //TODO parameters
        }
    }

    pub fn is_off(&self) -> bool {
        match self {
            Self::Off(_) => true,
            _ => false
        }
    }
}

impl Default for ChorusType {
    fn default() -> Self {
        Self::from(0, [Parameter::default(); 20])
    }
}

impl Validate for ChorusType {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Off(u) => u.validate(),
            Self::Chorus(c) => c.validate(),
            Self::Delay => Ok(()), //TODO parameters
            Self::Gm2Chorus => Ok(()) //TODO parameters
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct ChorusParameters {//TODO add validation
    filter_type: FilterType,
    cutoff_frequency: LogFrequency<800>,
    pre_delay: LogMilliseconds, //TODO default 2.0
    rate_mode: TimingMode,
    rate_hz: LinearFrequency, //TODO default 1
    rate_note: NoteLength,//default whole note
    #[validate(range(max = 127))]
    depth: u8, //TODO default 40
    #[validate(range(max = 180))]
    phase_degrees: u8, //TODO default 180deg
    #[validate(range(max = 127))]
    feedback: u8, //TODO default 8
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 11>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 11]>
}

impl From<[Parameter; 20]> for ChorusParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            filter_type: p.next().unwrap().into(),
            cutoff_frequency: p.next().unwrap().into(),
            pre_delay: p.next().unwrap().into(),
            rate_mode: p.next().unwrap().into(),
            rate_hz: p.next().unwrap().into(),
            rate_note: p.next().unwrap().into(),
            depth: p.next().unwrap().0 as u8,
            phase_degrees: p.next().unwrap().0 as u8,
            feedback: p.next().unwrap().0 as u8,
            unused_parameters: Box::new(p.collect::<Vec<_>>().try_into().unwrap())
        }
    }
}

impl Parameters<20> for ChorusParameters {
    fn parameters(&self) -> [Parameter; 20] {
        todo!()//TODO
    }
}

//TODO move things below this into enums/numeric/etc

trait DiscreteValues<T: PartialEq + Display> {
    const VALUES: Vec<T>;

    fn value_from(parameter: Parameter) -> T {
        if parameter.0 < 0 || parameter.0 as usize >= Self::VALUES.len() {
            panic!("Parameter out of range: {} (expected 0-{})", parameter.0, Self::VALUES.len()-1)
        }
        *Self::VALUES.iter().nth(parameter.0 as usize).unwrap()
    }

    fn into_parameter(value: T) -> Parameter {
        if let Some(position) = Self::VALUES.iter().position(|v| *v == value) {
            return Parameter(position as i16);
        } else {
            panic!("Invalid discrete value: {}", value);
        }
    }
}

impl<T: RepeatingValues> DiscreteValues<u16> for T {
    const VALUES: Vec<u16> = Self::enumerate_values();
}

trait RepeatingValues : DiscreteValues<u16> {
    const MIN: u16;
    const MAX: u16;
    const DEFAULT: u16;
    const BASE_VALUES: Vec<u16>;

    fn enumerate_values() -> Vec<u16> {
        let mut factor = 1;
        let mut v = Vec::new();
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
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)] //TODO DiscreteValues schema
struct LogFrequency<const DEFAULT: u16>(u16); // 0-16 (200-8000Hz)

impl<const D: u16> RepeatingValues for LogFrequency<D> {
    const BASE_VALUES: Vec<u16> = vec![200, 250, 315, 400, 500, 630, 800, 1000, 1250, 1600];
    const MIN: u16 = 200;
    const MAX: u16 = 8000;
    const DEFAULT: u16 = D;
}

impl<const D: u16> From<Parameter> for LogFrequency<D> {
    fn from(parameter: Parameter) -> Self {
        Self(Self::value_from(parameter))
    }
}

impl<const D: u16> Into<Parameter> for LogFrequency<D> {
    fn into(self) -> Parameter {
        Self::into_parameter(self.0)
    }
}

//#[derive(Serialize, Deserialize, Debug, JsonSchema)]
enum LogFrequencyOrByPass<const DEFAULT: u16> { // 0-17 (200-8000Hz, BYPASS)
    LogFrequency(LogFrequency<DEFAULT>),
    ByPass
}

impl<const D: u16> From<Parameter> for LogFrequencyOrByPass<D> {
    fn from(value: Parameter) -> Self {
        let index = value.0 as usize;
        if value.0 < 0 || value.0 > LogFrequency::<D>::VALUES.len() as i16 {
            panic!("Parameter out of range: {} (expected 0-{})", value.0, LogFrequency::<D>::VALUES.len())
        } else if value.0 == LogFrequency::<D>::VALUES.len() as i16 {
            Self::ByPass
        } else {
            Self::LogFrequency(value.into())
        }
    }
}

impl<const D: u16> Into<Parameter> for LogFrequencyOrByPass<D> {
    fn into(self) -> Parameter {
        match self {
            Self::LogFrequency(u) => u.into(),
            Self::ByPass => Parameter(LogFrequency::<D>::VALUES.len() as i16)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)] //TODO DiscreteValues schema
struct LinearFrequency(f64); // 0-? (0.05-10 by 0.05)

impl DiscreteValues<f64> for LinearFrequency {
    const VALUES: Vec<f64> = enumerate(0.05, 10.0, 0.05);
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

#[derive(Serialize, Deserialize, Debug, JsonSchema)] //TODO DiscreteValues schema
struct LogMilliseconds(f64); // 0-? (0-5 by 0.1, 5-10 by 0.5, 10-50 by 1, 50-100 by 2)

impl DiscreteValues<f64> for LogMilliseconds {
    const VALUES: Vec<f64> = flatten(vec![
        enumerate(0.0, 4.9, 0.1),
        enumerate(5.0, 9.5, 0.5),
        enumerate(10.0, 49.0, 1.0),
        enumerate(50.0, 100.0, 2.0)
    ]);
}

fn enumerate(start: f64, end: f64, step: f64) -> Vec<f64> {
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, PartialEq)]
enum TimingMode {
    Hertz,
    Note
}

impl From<Parameter> for TimingMode {
    fn from(value: Parameter) -> Self {
        Self::iter().nth(Into::<u16>::into(value) as usize).unwrap()
    }
}

impl Into<Parameter> for TimingMode {
    fn into(self) -> Parameter {
        (Self::iter().position(|s| s == self).unwrap() as u16).into()
    }
}

impl Default for TimingMode {
    fn default() -> Self {
        Self::from(Parameter::default())
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
enum NoteLength {
    SixtyFourthNoteTriplet,
    SixtyFourthNote,
    ThirtySecondNoteTriplet,
    ThirtySecondNote,
    SixteenthNoteTriplet,
    DottedThirtySecondNote,
    SixteenthNote,
    EighthNoteTriplet,
    DottedSixteenthNote,
    EighthNote,
    QuarterNoteTriplet,
    DottedEighthNote,
    QuarterNote,
    HalfNoteTriplet,
    DottedQuarterNote,
    HalfNote,
    WholeNoteTriplet,
    DottedHalfNote,
    WholeNote,
    DoubleNoteTriplet,
    DottedWholeNote,
    DoubleNote
}

impl From<Parameter> for NoteLength {
    fn from(value: Parameter) -> Self {
        todo!() //TODO
    }
}

//TODO future

// struct DelayParameters {
//     delay_left_mode: DelayMode,//default note
//     delay_left_ms: u16, //1-1000 by 1, default 200
//     delay_left_note: NoteLength, // default eighth note triplet
//     delay_right_mode: DelayMode,//default note
//     delay_right_ms: u16, //1-1000 by 1, default 400
//     delay_right_note: NoteLength,//default quarternote triplet
//     delay_centre_mode: DelayMode,//default note
//     delay_centre_ms: u16, //1-1000 by 1, default 600
//     delay_centre_note: NoteLength, // default quarternote
//     centre_feedback: u8, //-98% to 98% by 2, default +20
//     hf_damp: u16, //200-800 logarthimic LogarithmicFrequency + bypass default bypass
//     left_level: u8, //0-127, default 127
//     right_level: u8, //0-127, default 127
//     centre_level: u8, //0-127, default 127
//     #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
//     #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
//     #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 6>")]
//     #[validate(custom = "valid_boxed_elements")]
//     unused_parameters: Box<[Parameter; 6]>
// }

// enum DelayMode { //TODO better name
//     Milliseconds,
//     Note
// }

// struct Gm2ChorusParameters {
//     pre_lpf: u8,//0-7
//     level: u8: //0-127, default 64
//     feedback: u8, //0-127, default 8
//     delay: u8, //0-127, default 80
//     rate: u8, //0-127, default 3
//     depth: u8, //0-127, default 19
//     send_to_reverb: u8, //0-127
//     #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
//     #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
//     #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 13>")]
//     #[validate(custom = "valid_boxed_elements")]
//     unused_parameters: Box<[Parameter; 13]>
// }