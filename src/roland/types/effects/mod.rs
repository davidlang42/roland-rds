use super::numeric::Parameter;
use crate::json::{serialize_default_terminated_array, validation::merge_all_fixed};
use crate::json::validation::validate_boxed_array;
use schemars::JsonSchema;
use validator::{Validate, ValidationErrors};

pub mod discrete;
pub mod parameters;
pub mod chorus;
pub mod reverb;
pub mod mfx;

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
