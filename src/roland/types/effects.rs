use super::parameters::{FilterType, RateMode, NoteLength, DelayMode};
use super::numeric::Parameter;
use super::discrete::{LogFrequency, LogMilliseconds, LinearFrequency, LogFrequencyOrByPass};
use crate::json::{serialize_default_terminated_array, validation::merge_all_fixed};
use crate::json::validation::{valid_boxed_elements, validate_boxed_array};
use schemars::JsonSchema;
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
    Delay(DelayParameters),
    Gm2Chorus(Gm2ChorusParameters)
}

impl ChorusType {
    pub fn from(number: u8, parameters: [Parameter; 20]) -> Self {
        match number {
            0 => Self::Off(parameters.into()),
            1 => Self::Chorus(parameters.into()),
            2 => Self::Delay(parameters.into()),
            3 => Self::Gm2Chorus(parameters.into()),
            _ => panic!("Invalid chorus type: {}", number)
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::Off(_) => 0,
            Self::Chorus(_) => 1,
            Self::Delay(_) => 2,
            Self::Gm2Chorus(_) => 3
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Off(_) => "Off",
            Self::Chorus(_) => "Chorus",
            Self::Delay(_) => "Delay",
            Self::Gm2Chorus(_) => "Gm2Chorus"
        }
    }
    
    pub fn parameters(&self) -> [Parameter; 20] {
        match self {
            Self::Off(u) => u.parameters(),
            Self::Chorus(c) => c.parameters(),
            Self::Delay(d) => d.parameters(),
            Self::Gm2Chorus(g) => g.parameters()
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
            Self::Delay(d) => d.validate(),
            Self::Gm2Chorus(g) => g.validate()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct ChorusParameters {
    filter_type: FilterType,
    cutoff_frequency: LogFrequency,
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    #[validate(range(max = 127))]
    depth: u8,
    #[validate(range(max = 180))]
    phase_degrees: u8,
    #[validate(range(max = 127))]
    feedback: u8,
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
        let mut p: Vec<Parameter> = Vec::new();
        p.push(self.filter_type.into());
        p.push(self.cutoff_frequency.into());
        p.push(self.pre_delay.into());
        p.push(self.rate_mode.into());
        p.push(self.rate_hz.into());
        p.push(self.rate_note.into());
        p.push(Parameter(self.depth as i16));
        p.push(Parameter(self.phase_degrees as i16));
        p.push(Parameter(self.feedback as i16));
        for unused_parameter in self.unused_parameters.iter() {
            p.push(*unused_parameter);
        }
        p.try_into().unwrap()
    }
}

impl Default for ChorusParameters {
    fn default() -> Self {
        Self {
            filter_type: Default::default(),
            cutoff_frequency: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: Default::default(),
            rate_hz: LinearFrequency(1.0),
            rate_note: NoteLength::WholeNote,
            depth: 40,
            phase_degrees: 180,
            feedback: 8,
            unused_parameters: Default::default() }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct DelayParameters { //TODO Default for DelayParameters
    delay_left_mode: DelayMode,//default note
    #[validate(range(min = 1, max = 1000))]
    delay_left_ms: u16, //default 200
    delay_left_note: NoteLength, // default eighth note triplet
    delay_right_mode: DelayMode,//default note
    #[validate(range(min = 1, max = 1000))]
    delay_right_ms: u16, //default 400
    delay_right_note: NoteLength,//default quarternote triplet
    delay_centre_mode: DelayMode,//default note
    #[validate(range(min = 1, max = 1000))]
    delay_centre_ms: u16, //default 600
    delay_centre_note: NoteLength, // default quarternote
    centre_feedback: u8, //TODO -98% to 98% by 2, default +20
    hf_damp: LogFrequencyOrByPass, //default bypass
    #[validate(range(max = 127))]
    left_level: u8, //default 127
    #[validate(range(max = 127))]
    right_level: u8, //default 127
    #[validate(range(max = 127))]
    centre_level: u8, //default 127
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 6>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 6]>
}

impl From<[Parameter; 20]> for DelayParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            delay_left_mode: p.next().unwrap().into(),
            delay_left_ms: p.next().unwrap().0 as u16,
            delay_left_note: p.next().unwrap().into(),
            delay_right_mode: p.next().unwrap().into(),
            delay_right_ms: p.next().unwrap().0 as u16,
            delay_right_note: p.next().unwrap().into(),
            delay_centre_mode: p.next().unwrap().into(),
            delay_centre_ms: p.next().unwrap().0 as u16,
            delay_centre_note: p.next().unwrap().into(),
            centre_feedback: p.next().unwrap().0 as u8,
            hf_damp: p.next().unwrap().into(),
            left_level: p.next().unwrap().0 as u8,
            right_level: p.next().unwrap().0 as u8,
            centre_level: p.next().unwrap().0 as u8,
            unused_parameters: Box::new(p.collect::<Vec<_>>().try_into().unwrap())
        }
    }
}

impl Parameters<20> for DelayParameters {
    fn parameters(&self) -> [Parameter; 20] {
        let mut p: Vec<Parameter> = Vec::new();
        p.push(self.delay_left_mode.into());
        p.push(Parameter(self.delay_left_ms as i16));
        p.push(self.delay_left_note.into());
        p.push(self.delay_right_mode.into());
        p.push(Parameter(self.delay_right_ms as i16));
        p.push(self.delay_right_note.into());
        p.push(self.delay_centre_mode.into());
        p.push(Parameter(self.delay_centre_ms as i16));
        p.push(self.delay_centre_note.into());
        p.push(Parameter(self.centre_feedback as i16));
        p.push(self.hf_damp.into());
        p.push(Parameter(self.left_level as i16));
        p.push(Parameter(self.right_level as i16));
        p.push(Parameter(self.centre_level as i16));
        for unused_parameter in self.unused_parameters.iter() {
            p.push(*unused_parameter);
        }
        p.try_into().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Gm2ChorusParameters { //TODO Default for Gm2ChorusParameters
    #[validate(range(max = 7))]
    pre_lpf: u8,
    #[validate(range(max = 127))]
    level: u8, //default 64
    #[validate(range(max = 127))]
    feedback: u8, //default 8
    #[validate(range(max = 127))]
    delay: u8, //default 80
    #[validate(range(max = 127))]
    rate: u8, //default 3
    #[validate(range(max = 127))]
    depth: u8, //default 19
    #[validate(range(max = 127))]
    send_to_reverb: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 13>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 13]>
}

impl From<[Parameter; 20]> for Gm2ChorusParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            pre_lpf: p.next().unwrap().0 as u8,
            level: p.next().unwrap().0 as u8,
            feedback: p.next().unwrap().0 as u8,
            delay: p.next().unwrap().0 as u8,
            rate: p.next().unwrap().0 as u8,
            depth: p.next().unwrap().0 as u8,
            send_to_reverb: p.next().unwrap().0 as u8,
            unused_parameters: Box::new(p.collect::<Vec<_>>().try_into().unwrap())
        }
    }
}

impl Parameters<20> for Gm2ChorusParameters {
    fn parameters(&self) -> [Parameter; 20] {
        let mut p: Vec<Parameter> = Vec::new();
        p.push(Parameter(self.pre_lpf as i16));
        p.push(Parameter(self.level as i16));
        p.push(Parameter(self.feedback as i16));
        p.push(Parameter(self.delay as i16));
        p.push(Parameter(self.rate as i16));
        p.push(Parameter(self.depth as i16));
        p.push(Parameter(self.send_to_reverb as i16));
        for unused_parameter in self.unused_parameters.iter() {
            p.push(*unused_parameter);
        }
        p.try_into().unwrap()
    }
}