use schemars::JsonSchema;
use validator::Validate;

use crate::roland::types::numeric::OffsetU8;
use crate::{roland::types::numeric::Parameter, json::serialize_default_terminated_array};
use crate::json::validation::valid_boxed_elements;

use super::discrete::LogMilliseconds;
use super::parameters::{ReverbCharacter, Gm2ReverbCharacter};
use super::{UnusedParameters, Parameters, discrete::{LogFrequency, LogFrequencyOrByPass}};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ReverbType { // 0-6
    Off(UnusedParameters<20>),
    Reverb(ReverbParameters),
    Room(CharacterParameters),
    Hall(CharacterParameters),
    Plate(CharacterParameters),
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct ReverbParameters {
    character: ReverbCharacter,
    #[validate(range(max = 127))]
    time: u8,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    #[validate(range(max = 127))]
    delay_feedback: u8,
    #[validate(range(max = 127))]
    level: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 15>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 15]>
}

impl From<[Parameter; 20]> for ReverbParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            character: p.next().unwrap().into(),
            time: p.next().unwrap().0 as u8,
            hf_damp: p.next().unwrap().into(),
            delay_feedback: p.next().unwrap().0 as u8,
            level: p.next().unwrap().0 as u8,
            unused_parameters: Box::new(p.collect::<Vec<_>>().try_into().unwrap())
        }
    }
}

impl Parameters<20> for ReverbParameters {
    fn parameters(&self) -> [Parameter; 20] {
        let mut p: Vec<Parameter> = Vec::new();
        p.push(self.character.into());
        p.push(Parameter(self.time as i16));
        p.push(self.hf_damp.into());
        p.push(Parameter(self.delay_feedback as i16));
        p.push(Parameter(self.level as i16));
        for unused_parameter in self.unused_parameters.iter() {
            p.push(*unused_parameter);
        }
        p.try_into().unwrap()
    }
}

impl Default for ReverbParameters {
    fn default() -> Self {
        Self {
            character: ReverbCharacter::Stage2,
            time: 84,
            hf_damp: LogFrequencyOrByPass::Frequency(LogFrequency(8000)),
            delay_feedback: 0,
            level: 64,
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct CharacterParameters {
    pre_delay: LogMilliseconds,
    #[validate(range(max = 127))]
    time: u8,
    #[validate(range(min = 1, max = 8))]
    size: u8,
    high_cut: LogFrequencyOrByPass<160, 12500>, // technically this mislabels 320 as 315 and 640 as 630, but it really doesn't matter
    #[validate(range(max = 127))]
    density: u8,
    #[validate(range(max = 127))]
    diffusion: u8,
    lf_damp_freq: LogFrequency<50, 4000>, // technically this mislabels 64 as 63 and 320 as 315, but it really doesn't matter
    lf_damp_gain: OffsetU8<36, 0, 36>,
    hf_damp_freq: LogFrequency<4000, 12500>, // technically this mislabels 6400 as 6300, but it really doesn't matter
    hf_damp_gain: OffsetU8<36, 0, 36>,
    #[validate(range(max = 127))]
    level: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 9>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 9]>
}

impl From<[Parameter; 20]> for CharacterParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            pre_delay: p.next().unwrap().into(),
            time: p.next().unwrap().0 as u8,
            size: p.next().unwrap().0 as u8,
            high_cut: p.next().unwrap().into(),
            density: p.next().unwrap().0 as u8,
            diffusion: p.next().unwrap().0 as u8,
            lf_damp_freq: p.next().unwrap().into(),
            lf_damp_gain: (p.next().unwrap().0 as u8).into(),
            hf_damp_freq: p.next().unwrap().into(),
            hf_damp_gain: (p.next().unwrap().0 as u8).into(),
            level: p.next().unwrap().0 as u8,
            unused_parameters: Box::new(p.collect::<Vec<_>>().try_into().unwrap())
        }
    }
}

impl Parameters<20> for CharacterParameters {
    fn parameters(&self) -> [Parameter; 20] {
        let mut p: Vec<Parameter> = Vec::new();
        p.push(self.pre_delay.into());
        p.push(Parameter(self.time as i16));
        p.push(Parameter(self.size as i16));
        p.push(self.high_cut.into());
        p.push(Parameter(self.density as i16));
        p.push(Parameter(self.diffusion as i16));
        p.push(self.lf_damp_freq.into());
        p.push(Parameter(Into::<u8>::into(self.lf_damp_gain) as i16));
        p.push(self.hf_damp_freq.into());
        p.push(Parameter(Into::<u8>::into(self.hf_damp_gain) as i16));
        p.push(Parameter(self.level as i16));
        for unused_parameter in self.unused_parameters.iter() {
            p.push(*unused_parameter);
        }
        p.try_into().unwrap()
    }
}

//TODO add generics for default
// impl Default for CharacterParameters {
//     fn default() -> Self {
//         Self {
//             character: Gm2ReverbCharacter::Hall2,
//             pre_lpf: 0,
//             level: 64,
//             time: 64,
//             delay_feedback: 0,
//             unused_parameters: Default::default()
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Gm2ReverbParameters {
    character: Gm2ReverbCharacter,
    #[validate(range(max = 7))]
    pre_lpf: u8,
    #[validate(range(max = 127))]
    level: u8,
    #[validate(range(max = 127))]
    time: u8,
    #[validate(range(max = 127))]
    delay_feedback: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 15>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 15]>
}

impl From<[Parameter; 20]> for Gm2ReverbParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            character: p.next().unwrap().into(),
            pre_lpf: p.next().unwrap().0 as u8,
            level: p.next().unwrap().0 as u8,
            time: p.next().unwrap().0 as u8,
            delay_feedback: p.next().unwrap().0 as u8,
            unused_parameters: Box::new(p.collect::<Vec<_>>().try_into().unwrap())
        }
    }
}

impl Parameters<20> for Gm2ReverbParameters {
    fn parameters(&self) -> [Parameter; 20] {
        let mut p: Vec<Parameter> = Vec::new();
        p.push(self.character.into());
        p.push(Parameter(self.pre_lpf as i16));
        p.push(Parameter(self.level as i16));
        p.push(Parameter(self.time as i16));
        p.push(Parameter(self.delay_feedback as i16));
        for unused_parameter in self.unused_parameters.iter() {
            p.push(*unused_parameter);
        }
        p.try_into().unwrap()
    }
}

impl Default for Gm2ReverbParameters {
    fn default() -> Self {
        Self {
            character: Gm2ReverbCharacter::Hall2,
            pre_lpf: 0,
            level: 64,
            time: 64,
            delay_feedback: 0,
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct CathedralParameters {
    #[validate(range(max = 7))]
    pre_lpf: u8,
    #[validate(range(max = 127))]
    level: u8,
    #[validate(range(max = 127))]
    time: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 17>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 17]>
}

impl From<[Parameter; 20]> for CathedralParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            pre_lpf: p.next().unwrap().0 as u8,
            level: p.next().unwrap().0 as u8,
            time: p.next().unwrap().0 as u8,
            unused_parameters: Box::new(p.collect::<Vec<_>>().try_into().unwrap())
        }
    }
}

impl Parameters<20> for CathedralParameters {
    fn parameters(&self) -> [Parameter; 20] {
        let mut p: Vec<Parameter> = Vec::new();
        p.push(Parameter(self.pre_lpf as i16));
        p.push(Parameter(self.level as i16));
        p.push(Parameter(self.time as i16));
        for unused_parameter in self.unused_parameters.iter() {
            p.push(*unused_parameter);
        }
        p.try_into().unwrap()
    }
}

impl Default for CathedralParameters {
    fn default() -> Self {
        Self {
            pre_lpf: 3,
            level: 64,
            time: 54,
            unused_parameters: Default::default()
        }
    }
}
