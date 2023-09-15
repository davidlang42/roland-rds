use std::borrow::Cow;

use schemars::JsonSchema;
use validator::{Validate, ValidationError, ValidationErrors};
use crate::{roland::rd300nx::RD300NX, json::{schema::{array_schema, u8_schema, object_schema}, validation::{out_of_range_err, merge_all_fixed}}};

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

impl Validate for ToneRemain {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Always(_) => Ok(()),
            Self::BySet(s) => s.validate()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct ToneRemainSets {
    #[validate]
    pub user_sets: BySet<{RD300NX::USER_SETS}>,
    #[validate]
    pub piano: BySet<{RD300NX::PIANO_SETS}>,
    #[validate]
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
    ByIndex(ToneRemainIndicies<N>),
    ByRange(ToneRemainRanges<N>)
}

impl<const N: usize> BySet<N> {
    pub fn any(&self) -> bool {
        match self {
            Self::Always(b) => *b,
            Self::ByIndex(indicies) => indicies.any(),
            Self::ByRange(ranges) => ranges.any()
        }
    }

    pub fn includes(&self, index: usize) -> bool {
        match self {
            Self::Always(b) => *b,
            Self::ByIndex(indicies) => indicies.contains(index),
            Self::ByRange(ranges) => ranges.contains(index)
        }
    }
}

impl<const N: usize> Validate for BySet<N> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Always(_) => Ok(()),
            Self::ByIndex(i) => i.validate(),
            Self::ByRange(r) => r.validate()
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

impl<const N: usize> Validate for ToneRemainIndicies<N> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let max = N - 2;
        merge_all_fixed(Ok(()), "0", self.0.iter().map(|i| {
            if *i > max {
                Err(out_of_range_err("0", &0, &max))
            } else {
                Ok(())
            }
        }).collect())
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ToneRemainRanges<const N: usize>(Vec<PatchRange<N>>);

impl<const N: usize> ToneRemainRanges<N> {
    pub fn any(&self) -> bool {
        self.0.iter().any(|pr| pr.any())
    }

    pub fn contains(&self, index: usize) -> bool {
        self.0.iter().any(|pr| pr.contains(index))
    }
}

impl<const N: usize> Validate for ToneRemainRanges<N> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        merge_all_fixed(Ok(()), "0", self.0.iter().map(|r| r.validate()).collect())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchRange<const N: usize> {
    from: usize,
    to: usize
}

impl<const N: usize> PatchRange<N> {
    fn check_valid(&self) {
        if self.to > N || self.from > N || self.from < 1 || self.to <= self.from {
            panic!("Invalid PatchRange<{}>: {}-{}", N, self.from, self.to)
        }
    }

    pub fn any(&self) -> bool {
        self.check_valid();
        self.to > self.from
    }

    pub fn contains(&self, index: usize) -> bool {
        self.check_valid();
        let patch = index + 1;
        patch >= self.from && patch < self.to
    }
}

impl<const N: usize> Default for PatchRange<N> {
    fn default() -> Self {
        Self {
            from: 1,
            to: N
        }
    }
}

impl<const N: usize> JsonSchema for PatchRange<N> {
    fn schema_name() -> String {
        format!("PatchRange_for_{}", N)
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        object_schema(vec![
            ("from", u8_schema(1, N as u8)),
            ("to", u8_schema(1, N as u8))
        ], Some(serde_json::to_value(Self::default()).unwrap()))
    }
}

impl<const N: usize> Validate for PatchRange<N> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.from < 1 || self.from > N {
            return Err(out_of_range_err("from", &1, &N));
        }
        if self.to < 1 || self.to > N {
            return Err(out_of_range_err("to", &1, &N));
        }
        if self.to <= self.from {
            let mut e = ValidationErrors::new();
            let mut range_e = ValidationError::new("to is less than from");
            range_e.add_param(Cow::from("to"), &self.to);
            range_e.add_param(Cow::from("from"), &self.from);
            e.add("to", range_e);
            return Err(e);
        }
        Ok(())
    }
}