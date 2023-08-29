use super::{UnusedParameters, Parameters};
use super::parameters::{FilterType, RateMode, NoteLength, DelayMode, Level, Phase, LinearMilliseconds, PreLpf};
use super::super::numeric::Parameter;
use super::discrete::{LogFrequency, LogMilliseconds, LinearFrequency, LogFrequencyOrByPass, EvenPercent};
use crate::json::serialize_default_terminated_array;
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
    depth: Level,
    phase_degrees: Phase,
    feedback: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 11>")]
    #[validate]
    unused_parameters: [Parameter; 11]
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
            depth: p.next().unwrap().into(),
            phase_degrees: p.next().unwrap().into(),
            feedback: p.next().unwrap().into(),
            unused_parameters: p.collect::<Vec<_>>().try_into().unwrap()
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
        p.push(self.depth.into());
        p.push(self.phase_degrees.into());
        p.push(self.feedback.into());
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
            depth: Level(40),
            phase_degrees: Phase(180),
            feedback: Level(8),
            unused_parameters: Default::default() }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct DelayParameters {
    delay_left_mode: DelayMode,
    delay_left_ms: LinearMilliseconds<1000>,
    delay_left_note: NoteLength,
    delay_right_mode: DelayMode,
    delay_right_ms: LinearMilliseconds<1000>,
    delay_right_note: NoteLength,
    delay_centre_mode: DelayMode,
    delay_centre_ms: LinearMilliseconds<1000>,
    delay_centre_note: NoteLength,
    centre_feedback_percent: EvenPercent,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    left_level: Level,
    right_level: Level,
    centre_level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 6>")]
    #[validate]
    unused_parameters: [Parameter; 6]
}

impl From<[Parameter; 20]> for DelayParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            delay_left_mode: p.next().unwrap().into(),
            delay_left_ms: p.next().unwrap().into(),
            delay_left_note: p.next().unwrap().into(),
            delay_right_mode: p.next().unwrap().into(),
            delay_right_ms: p.next().unwrap().into(),
            delay_right_note: p.next().unwrap().into(),
            delay_centre_mode: p.next().unwrap().into(),
            delay_centre_ms: p.next().unwrap().into(),
            delay_centre_note: p.next().unwrap().into(),
            centre_feedback_percent: p.next().unwrap().into(),
            hf_damp: p.next().unwrap().into(),
            left_level: p.next().unwrap().into(),
            right_level: p.next().unwrap().into(),
            centre_level: p.next().unwrap().into(),
            unused_parameters: p.collect::<Vec<_>>().try_into().unwrap()
        }
    }
}

impl Parameters<20> for DelayParameters {
    fn parameters(&self) -> [Parameter; 20] {
        let mut p: Vec<Parameter> = Vec::new();
        p.push(self.delay_left_mode.into());
        p.push(self.delay_left_ms.into());
        p.push(self.delay_left_note.into());
        p.push(self.delay_right_mode.into());
        p.push(self.delay_right_ms.into());
        p.push(self.delay_right_note.into());
        p.push(self.delay_centre_mode.into());
        p.push(self.delay_centre_ms.into());
        p.push(self.delay_centre_note.into());
        p.push(self.centre_feedback_percent.into());
        p.push(self.hf_damp.into());
        p.push(self.left_level.into());
        p.push(self.right_level.into());
        p.push(self.centre_level.into());
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
            delay_left_ms: LinearMilliseconds(200),
            delay_left_note: NoteLength::EighthNoteTriplet,
            delay_right_mode: DelayMode::Note,
            delay_right_ms: LinearMilliseconds(400),
            delay_right_note: NoteLength::QuarterNoteTriplet,
            delay_centre_mode: DelayMode::Note,
            delay_centre_ms: LinearMilliseconds(600),
            delay_centre_note: NoteLength::QuarterNote,
            centre_feedback_percent: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            left_level: Level(127),
            right_level: Level(127),
            centre_level: Level(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct Gm2ChorusParameters {
    pre_lpf: PreLpf,
    level: Level,
    feedback: Level,
    delay: Level,
    rate: Level,
    depth: Level,
    send_to_reverb: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 13>")]
    #[validate]
    unused_parameters: [Parameter; 13]
}

impl Default for Gm2ChorusParameters {
    fn default() -> Self {
        Self {
            pre_lpf: PreLpf(0),
            level: Level(64),
            feedback: Level(8),
            delay: Level(80),
            rate: Level(3),
            depth: Level(19),
            send_to_reverb: Level(0),
            unused_parameters: Default::default()
        }
    }
}
