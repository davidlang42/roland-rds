use super::{UnusedParameters, Parameters};
use super::parameters::{FilterType, RateMode, NoteLength, DelayMode, Level, Phase, LinearMilliseconds, PreLpf, UInt};
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
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

impl Default for ChorusParameters {
    fn default() -> Self {
        Self {
            filter_type: FilterType::Off,
            cutoff_frequency: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Hertz,
            rate_hz: LinearFrequency(1.0),
            rate_note: NoteLength::WholeNote,
            depth: UInt(40),
            phase_degrees: UInt(180),
            feedback: UInt(8),
            unused_parameters: Default::default() }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
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

impl Default for DelayParameters {
    fn default() -> Self {
        Self {
            delay_left_mode: DelayMode::Note,
            delay_left_ms: UInt(200),
            delay_left_note: NoteLength::EighthNoteTriplet,
            delay_right_mode: DelayMode::Note,
            delay_right_ms: UInt(400),
            delay_right_note: NoteLength::QuarterNoteTriplet,
            delay_centre_mode: DelayMode::Note,
            delay_centre_ms: UInt(600),
            delay_centre_note: NoteLength::QuarterNote,
            centre_feedback_percent: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            left_level: UInt(127),
            right_level: UInt(127),
            centre_level: UInt(127),
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
            pre_lpf: UInt(0),
            level: UInt(64),
            feedback: UInt(8),
            delay: UInt(80),
            rate: UInt(3),
            depth: UInt(19),
            send_to_reverb: UInt(0),
            unused_parameters: Default::default()
        }
    }
}
