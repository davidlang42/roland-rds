use super::{UnusedParameters, Parameters};
use super::parameters::{FilterType, RateMode, NoteLength, DelayMode};
use super::super::numeric::Parameter;
use super::discrete::{LogFrequency, LogMilliseconds, LinearFrequency, LogFrequencyOrByPass, EvenPercent};
use crate::json::serialize_default_terminated_array;
use crate::json::validation::valid_boxed_elements;
use schemars::JsonSchema;
use validator::Validate;

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
    cutoff_frequency: LogFrequency<200, 8000>,
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
            filter_type: FilterType::Off,
            cutoff_frequency: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Hertz,
            rate_hz: LinearFrequency(1.0),
            rate_note: NoteLength::WholeNote,
            depth: 40,
            phase_degrees: 180,
            feedback: 8,
            unused_parameters: Default::default() }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct DelayParameters {
    delay_left_mode: DelayMode,
    #[validate(range(min = 1, max = 1000))]
    delay_left_ms: u16,
    delay_left_note: NoteLength,
    delay_right_mode: DelayMode,
    #[validate(range(min = 1, max = 1000))]
    delay_right_ms: u16,
    delay_right_note: NoteLength,
    delay_centre_mode: DelayMode,
    #[validate(range(min = 1, max = 1000))]
    delay_centre_ms: u16,
    delay_centre_note: NoteLength,
    centre_feedback_percent: EvenPercent,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    #[validate(range(max = 127))]
    left_level: u8,
    #[validate(range(max = 127))]
    right_level: u8,
    #[validate(range(max = 127))]
    centre_level: u8,
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
            centre_feedback_percent: p.next().unwrap().into(),
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
        p.push(self.centre_feedback_percent.into());
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

impl Default for DelayParameters {
    fn default() -> Self {
        Self {
            delay_left_mode: DelayMode::Note,
            delay_left_ms: 200,
            delay_left_note: NoteLength::EighthNoteTriplet,
            delay_right_mode: DelayMode::Note,
            delay_right_ms: 400,
            delay_right_note: NoteLength::QuarterNoteTriplet,
            delay_centre_mode: DelayMode::Note,
            delay_centre_ms: 600,
            delay_centre_note: NoteLength::QuarterNote,
            centre_feedback_percent: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            left_level: 127,
            right_level: 127,
            centre_level: 127,
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Gm2ChorusParameters {
    #[validate(range(max = 7))]
    pre_lpf: u8,
    #[validate(range(max = 127))]
    level: u8,
    #[validate(range(max = 127))]
    feedback: u8,
    #[validate(range(max = 127))]
    delay: u8,
    #[validate(range(max = 127))]
    rate: u8,
    #[validate(range(max = 127))]
    depth: u8,
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

impl Default for Gm2ChorusParameters {
    fn default() -> Self {
        Self {
            pre_lpf: Default::default(),
            level: 64,
            feedback: 8,
            delay: 80,
            rate: 3,
            depth: 19,
            send_to_reverb: Default::default(),
            unused_parameters: Default::default()
        }
    }
}
