use super::numeric::Parameter;
use crate::json::serialize_default_terminated_array;
use schemars::JsonSchema;
use validator::Validate;

pub mod discrete;
pub mod parameters;
pub mod chorus;
pub mod reverb;
pub mod mfx;

trait Parameters<const N: usize> : Validate + From<[Parameter; N]> + Default {
    fn parameters(&self) -> [Parameter; N];
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct UnusedParameters<const N: usize> {
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, {N}>")]
    #[validate]
    unused: [Parameter; N]
}

impl<const N: usize> From<[Parameter; N]> for UnusedParameters<N> {
    fn from(value: [Parameter; N]) -> Self {
        Self {
            unused: value
        }
    }
}

impl<const N: usize> Parameters<N> for UnusedParameters<N> {
    fn parameters(&self) -> [Parameter; N] {
        self.unused
    }
}

impl<const N: usize> Default for UnusedParameters<N> {
    fn default() -> Self {
        Self {
            unused: [Default::default(); N]
        }
    }
}
