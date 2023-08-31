use schemars::JsonSchema;
use validator::Validate;
use crate::json::serialize_default_terminated_array;

use crate::roland::types::numeric::Parameter;
use super::discrete::{LogMilliseconds, DiscreteValues, LogFrequency, LogFrequencyOrByPass};
use super::parameters::{ReverbCharacter, Gm2ReverbCharacter, Level, PreLpf, Size, UInt, Int, DampGain};
use super::{UnusedParameters, Parameters};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ReverbType { // 0-6
    Off(UnusedParameters<20>),
    Reverb(ReverbParameters),
    Room(CharacterParameters<50, 64>), // 50 = 5ms
    Hall(CharacterParameters<76, 70>), // 76 = 26ms
    Plate(CharacterParameters<66, 64>), // 66 = 16ms
    Gm2Reverb(Gm2ReverbParameters),
    Cathedral(CathedralParameters)
}

impl ReverbType {
    pub fn from(number: u8, parameters: [Parameter; 20]) -> Self {
        match number {
            0 => Self::Off(parameters.into()),
            1 => Self::Reverb(parameters.into()),
            2 => Self::Room(parameters.into()),
            3 => Self::Hall(parameters.into()),
            4 => Self::Plate(parameters.into()),
            5 => Self::Gm2Reverb(parameters.into()),
            6 => Self::Cathedral(parameters.into()),
            _ => panic!("Invalid reverb type: {}", number)
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::Off(_) => 0,
            Self::Reverb(_) => 1,
            Self::Room(_) => 2,
            Self::Hall(_) => 3,
            Self::Plate(_) => 4,
            Self::Gm2Reverb(_) => 5,
            Self::Cathedral(_) => 6
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Off(_) => "Off",
            Self::Reverb(_) => "Reverb",
            Self::Room(_) => "Room",
            Self::Hall(_) => "Hall",
            Self::Plate(_) => "Plate",
            Self::Gm2Reverb(_) => "Gm2Reverb",
            Self::Cathedral(_) => "Cathedral"
        }
    }
    
    pub fn parameters(&self) -> [Parameter; 20] {
        match self {
            Self::Off(u) => u.parameters(),
            Self::Reverb(re) => re.parameters(),
            Self::Room(ro) => ro.parameters(),
            Self::Hall(h) => h.parameters(),
            Self::Plate(p) => p.parameters(),
            Self::Gm2Reverb(g) => g.parameters(),
            Self::Cathedral(c) => c.parameters()
        }
    }

    #[allow(dead_code)] // used by tests, potentially useful if using this as a library
    pub fn default(&self) -> Self {
        match self {
            Self::Off(_) => Self::Off(Default::default()),
            Self::Reverb(_) => Self::Reverb(Default::default()),
            Self::Room(_) => Self::Room(Default::default()),
            Self::Hall(_) => Self::Hall(Default::default()),
            Self::Plate(_) => Self::Plate(Default::default()),
            Self::Gm2Reverb(_) => Self::Gm2Reverb(Default::default()),
            Self::Cathedral(_) => Self::Cathedral(Default::default())
        }
    }

    pub fn is_off(&self) -> bool {
        match self {
            Self::Off(_) => true,
            _ => false
        }
    }
}

impl Default for ReverbType {
    fn default() -> Self {
        Self::from(0, [Parameter::default(); 20])
    }
}

impl Validate for ReverbType {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Off(u) => u.validate(),
            Self::Reverb(re) => re.validate(),
            Self::Room(ro) => ro.validate(),
            Self::Hall(h) => h.validate(),
            Self::Plate(p) => p.validate(),
            Self::Gm2Reverb(g) => g.validate(),
            Self::Cathedral(c) => c.validate()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ReverbParameters {
    character: ReverbCharacter,
    time: Level,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_feedback: Level,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 15>")]
    #[validate]
    unused_parameters: [Parameter; 15]
}

impl Default for ReverbParameters {
    fn default() -> Self {
        Self {
            character: ReverbCharacter::Stage2,
            time: UInt(84),
            hf_damp: LogFrequencyOrByPass::Frequency(LogFrequency(8000)),
            delay_feedback: UInt(0),
            level: UInt(64),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct CharacterParameters<const DEFAULT_MS: usize, const DEFAULT_TIME: u16> {
    pre_delay: LogMilliseconds,
    time: Level,
    size: Size,
    high_cut: LogFrequencyOrByPass<160, 12500>, // technically this mislabels 320 as 315 and 640 as 630, but it really doesn't matter
    density: Level,
    diffusion: Level,
    lf_damp_freq: LogFrequency<50, 4000>, // technically this mislabels 64 as 63 and 320 as 315, but it really doesn't matter
    lf_damp_gain: DampGain,
    hf_damp_freq: LogFrequency<4000, 12500>, // technically this mislabels 6400 as 6300, but it really doesn't matter
    hf_damp_gain: DampGain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 9>")]
    #[validate]
    unused_parameters: [Parameter; 9]
}

impl<const DMS: usize, const DT: u16> Default for CharacterParameters<DMS, DT> {
    fn default() -> Self {
        Self {
            pre_delay: LogMilliseconds(LogMilliseconds::values().into_iter().nth(DMS).unwrap()),
            time: UInt(DT),
            size: Int(8),
            high_cut: LogFrequencyOrByPass::Frequency(LogFrequency(12500)),
            density: UInt(127),
            diffusion: UInt(127),
            lf_damp_freq: LogFrequency(4000),
            lf_damp_gain: Int(0),
            hf_damp_freq: LogFrequency(4000),
            hf_damp_gain: Int(0),
            level: UInt(64),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct Gm2ReverbParameters {
    character: Gm2ReverbCharacter,
    pre_lpf: PreLpf,
    level: Level,
    time: Level,
    delay_feedback: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 15>")]
    #[validate]
    unused_parameters: [Parameter; 15]
}

impl Default for Gm2ReverbParameters {
    fn default() -> Self {
        Self {
            character: Gm2ReverbCharacter::Hall2,
            pre_lpf: UInt(0),
            level: UInt(64),
            time: UInt(64),
            delay_feedback: UInt(0),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct CathedralParameters {
    pre_lpf: PreLpf,
    level: Level,
    time: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 17>")]
    #[validate]
    unused_parameters: [Parameter; 17]
}

impl Default for CathedralParameters {
    fn default() -> Self {
        Self {
            pre_lpf: UInt(3),
            level: UInt(64),
            time: UInt(54),
            unused_parameters: Default::default()
        }
    }
}
