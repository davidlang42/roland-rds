use schemars::JsonSchema;

pub mod numeric;
pub mod enums;
pub mod notes;
pub mod effects;
pub mod metadata;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema)]
pub struct StateMap<T> {
    pub on: T,
    pub off: T
}