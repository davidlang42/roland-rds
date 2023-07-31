use crate::bytes::{Bytes, BytesError, BitStream};
use crate::json::validation::{validate_boxed_array, merge_all_fixed};
use crate::json::warnings::{Warnings, tone_remain_warnings};
use crate::json::{StructuredJson, Json, StructuredJsonError, serialize_array_as_vec};
use super::live_set::LiveSet;
use super::system::System;
use schemars::JsonSchema;
use validator::{Validate, ValidationErrors};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RD300NX {
    #[serde(deserialize_with = "serialize_array_as_vec::deserialize")]
    #[serde(serialize_with = "serialize_array_as_vec::serialize")]
    #[schemars(with = "serialize_array_as_vec::ArraySchema::<LiveSet, {Self::USER_SETS}>")]
    pub user_sets: Box<[LiveSet; Self::USER_SETS]>,
    pub piano: Box<[LiveSet; Self::PIANO_SETS]>,
    pub e_piano: Box<[LiveSet; Self::E_PIANO_SETS]>,
    system: System
    // checksum: 2 bytes
}

impl Validate for RD300NX {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut r = Ok(());
        r = merge_all_fixed(r, "user_sets", validate_boxed_array(&self.user_sets));
        r = merge_all_fixed(r, "piano", validate_boxed_array(&self.piano));
        r = merge_all_fixed(r, "e_piano", validate_boxed_array(&self.e_piano));
        r = ValidationErrors::merge(r, "system", self.system.validate());
        r
    }
}

impl Warnings for RD300NX {
    fn warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        for (i, live_set) in self.user_sets.iter().enumerate() {
            for warning in live_set.warnings() {
                warnings.push(format!("User #{}: {}", i+1, warning));
            }
        }
        for (i, live_set) in self.piano.iter().enumerate() {
            for warning in live_set.warnings() {
                warnings.push(format!("Piano #{}: {}", i+1, warning));
            }
        }
        for (i, live_set) in self.e_piano.iter().enumerate() {
            for warning in live_set.warnings() {
                warnings.push(format!("EPiano #{}: {}", i+1, warning));
            }
        }
        if self.system.common.tone_remain {
            for i in 0..(self.user_sets.len() - 1) {
                let reasons = tone_remain_warnings(&self.user_sets[i], &self.user_sets[i+1]);
                for reason in reasons {
                    warnings.push(format!("User #{}-#{}: Tone remain may malfunction because {}", i+1, i+2, reason));
                }
            }
        }
        warnings
    }
}

impl RD300NX {
    const USER_SETS: usize = 60;
    const PIANO_SETS: usize = 10;
    const E_PIANO_SETS: usize = 15;

    pub fn all_live_sets(&self) -> Vec<&LiveSet> {
        self.user_sets.iter().chain(self.piano.iter()).chain(self.e_piano.iter()).collect()
    }
}

impl Bytes<183762> for RD300NX {
    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> {
        BitStream::read_fixed(bytes, |data| {
            let user_sets = LiveSet::array_from_bytes(data)?;
            let piano = LiveSet::array_from_bytes(data)?;
            let e_piano = LiveSet::array_from_bytes(data)?;
            let system = System::from_bytes(data.get_bytes())?;
            let expected_sum = data.sum_previous_bytes().to_be_bytes();
            let found_sum = data.get_full_u16().to_be_bytes();
            if found_sum != expected_sum {
                return Err(BytesError::IncorrectCheckSum {
                    expected: expected_sum.into_iter().collect(),
                    found: found_sum.into_iter().collect()
                });
            }
            Ok(Self {
                user_sets,
                piano,
                e_piano,
                system
            })
        })
    }

    fn to_bytes(&self) -> Result<Box<[u8; Self::BYTE_SIZE]>, BytesError> {
        BitStream::write_fixed(|bs| {
            for live_set in self.all_live_sets() {
                bs.set_bytes(live_set.to_bytes()?);
            }
            bs.set_bytes(self.system.to_bytes()?);
            let check_sum = bs.sum_previous_bytes();           
            bs.set_full_u16(check_sum);
            Ok(())
        })
    }
}

impl Json for RD300NX {
    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::NestedCollection(vec![
            ("user_sets".to_string(), StructuredJson::from_collection(self.user_sets.as_slice(), |ls| Some(ls.name_string()))),
            ("piano".to_string(), StructuredJson::from_collection(self.piano.as_slice(), |ls| Some(ls.name_string()))),
            ("e_piano".to_string(), StructuredJson::from_collection(self.e_piano.as_slice(), |ls| Some(ls.name_string()))),
            ("system".to_string(), self.system.to_structured_json())
        ])
    }

    fn from_structured_json(mut structured_json: StructuredJson) -> Result<Self, StructuredJsonError> {
        let user_sets = structured_json.extract("user_sets")?.to_array()?;
        let piano = structured_json.extract("piano")?.to_array()?;
        let e_piano = structured_json.extract("e_piano")?.to_array()?;
        let system = structured_json.extract("system")?.to()?;
        structured_json.done()?;
        Ok(Self {
            user_sets,
            piano,
            e_piano,
            system
        })
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn from_json(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
