use schemars::JsonSchema;
use crate::{roland::rd300nx::RD300NX, json::schema::{array_schema, u8_schema}};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ToneRemain {
    Always(bool),
    BySet(ToneRemainSets)
}

impl ToneRemain {
    pub fn any(&self) -> bool {
        match self {
            Self::Always(b) => *b,
            Self::BySet(set) => set.any()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ToneRemainSets {
    pub user_sets: BySet<{RD300NX::USER_SETS}>,
    pub piano: BySet<{RD300NX::PIANO_SETS}>,
    pub e_piano: BySet<{RD300NX::E_PIANO_SETS}>
}

impl ToneRemainSets {
    pub fn any(&self) -> bool {
        self.user_sets.any() || self.piano.any() || self.e_piano.any()
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum BySet<const N: usize> {
    Always(bool),
    ByIndex(ToneRemainIndicies<N>)
    //TODO ByRange(ToneRemainRanges<N>)
}

impl<const N: usize> BySet<N> {
    pub fn any(&self) -> bool {
        match self {
            Self::Always(b) => *b,
            Self::ByIndex(indicies) => indicies.any()
        }
    }

    pub fn includes(&self, index: usize) -> bool {
        match self {
            Self::Always(b) => *b,
            Self::ByIndex(indicies) => indicies.contains(index)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToneRemainIndicies<const N: usize>(Vec<usize>);

impl<const N: usize> ToneRemainIndicies<N> {
    pub fn any(&self) -> bool {
        self.0.len() > 0
    }

    pub fn contains(&self, index: usize) -> bool {
        self.0.contains(&index)
    }
}

impl<const N: usize> JsonSchema for ToneRemainIndicies<N> {
    fn schema_name() -> String {
        format!("ToneRemainIndicies_for_{}", N)
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        array_schema(u8_schema(0, N as u8 - 2))
    }
}