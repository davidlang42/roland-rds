use crate::bytes::{Bytes, BytesError, BitStream};
use crate::json::{StructuredJson, Json};
use crate::json::serialize_array_as_vec;
use super::live_set::LiveSet;
use super::system::System;

#[derive(Serialize, Deserialize)]
pub struct RD300NX {
    #[serde(with = "serialize_array_as_vec")]
    pub user_sets: Box<[LiveSet; Self::USER_SETS]>,
    pub piano: Box<[LiveSet; Self::PIANO_SETS]>,
    pub e_piano: Box<[LiveSet; Self::E_PIANO_SETS]>,
    system: System
    // checksum: 2 bytes
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

    fn from_structured_json(mut structured_json: StructuredJson) -> Self {
        let user_sets = structured_json.extract("user_sets").to_array();
        let piano = structured_json.extract("piano").to_array();
        let e_piano = structured_json.extract("e_piano").to_array();
        let system = structured_json.extract("system").to();
        structured_json.done();
        Self {
            user_sets,
            piano,
            e_piano,
            system
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
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
