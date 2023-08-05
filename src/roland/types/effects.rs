use super::enums::FilterType;
use super::numeric::Parameter;
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
    cutoff_frequency: Frequency,
    pre_delay: Milliseconds,
    rate_mode: TimingMode,
    rate_hz: f64,
    rate_note: NoteLength,
    depth: u8,
    phase: u8,
    feedback: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 11>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 11]>
}

impl From<[Parameter; 20]> for ChorusParameters {
    fn from(_value: [Parameter; 20]) -> Self {
        todo!()//TODO
    }
}

impl Parameters<20> for ChorusParameters {
    fn parameters(&self) -> [Parameter; 20] {
        todo!()//TODO
    }
}

//TODO move things below this into enums/numeric/etc
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
enum Frequency {
    //TODO
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
enum Milliseconds {
    //TODO
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
enum TimingMode {
    //TODO
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
enum NoteLength {
    //TODO
}
