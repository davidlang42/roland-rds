use schemars::JsonSchema;
use validator::Validate;

use crate::{roland::types::numeric::Parameter, json::serialize_default_terminated_array};
use crate::json::validation::valid_boxed_elements;

use super::{UnusedParameters, Parameters, discrete::{LogFrequency, LogFrequencyOrByPass}, parameters::ReverbOption};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ReverbType { // 0-6
    Off(UnusedParameters<20>),
    Reverb(ReverbParameters),
    Room(UnusedParameters<20>),
    Hall(UnusedParameters<20>),
    Plate(UnusedParameters<20>),
    Gm2Reverb(UnusedParameters<20>),
    Cathedral(UnusedParameters<20>)
    //TODO other reverb types
    // Room(RoomParameters),
    // Hall(HallParameters),
    // Plate(PlateParameters),
    // Gm2Reverb(Gm2ReverbParameters),
    // Cathedral(CathedralParameters)
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
    type_option: ReverbOption,
    #[validate(range(max = 127))]
    time: u8,
    hf_damp: LogFrequencyOrByPass,
    #[validate(range(max = 127))]
    delay_feedback: u8,
    #[validate(range(max = 127))]
    level: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 11>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 15]>
}

impl From<[Parameter; 20]> for ReverbParameters {
    fn from(value: [Parameter; 20]) -> Self {
        let mut p = value.into_iter();
        Self {
            type_option: p.next().unwrap().into(),
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
        p.push(self.type_option.into());
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
            type_option: ReverbOption::Stage2,
            time: 84,
            hf_damp: LogFrequencyOrByPass::Frequency(LogFrequency(8000)),
            delay_feedback: 0,
            level: 64,
            unused_parameters: Default::default()
        }
    }
}
