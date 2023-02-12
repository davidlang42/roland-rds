use crate::bits::BitStream;
use crate::bytes::{Bytes, BytesError, StructuredJson};
use crate::json::serialize_array_as_vec;
use super::live_set::LiveSet;
use super::parse_many;
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
            let user_sets = parse_many(data)?;
            let piano = parse_many(data)?;
            let e_piano = parse_many(data)?;
            let system = System::from_bytes(Box::new(data.get_bytes()))?;
            let found_check_sum = [
                data.get_u8::<8>(),
                data.get_u8::<8>(),
            ];
            let rds = Self {
                user_sets,
                piano,
                e_piano,
                system
            };
            let bytes = rds.to_bytes();
            let expected_check_sum: [u8; 2] = bytes[(bytes.len()-2)..bytes.len()].try_into().unwrap();
            if found_check_sum != expected_check_sum {
                return Err(BytesError::IncorrectCheckSum {
                    expected: expected_check_sum.into_iter().collect(),
                    found: found_check_sum.into_iter().collect()
                });
            }
            Ok(rds)
        })
    }

    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        let mut bytes = Vec::new();
        for live_set in self.all_live_sets() {
            for byte in *live_set.to_bytes() {
                bytes.push(byte);
            }
        }
        for byte in *self.system.to_bytes() {
            bytes.push(byte);
        }
        let check_sum = Self::check_sum(&bytes);
        for byte in check_sum.to_be_bytes() {
            bytes.push(byte);
        }
        bytes.try_into().unwrap()
    }

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::NestedCollection(vec![
            ("user_sets".to_string(), StructuredJson::from_collection(self.user_sets.as_slice(), Some(LiveSet::name_string))),
            ("piano".to_string(), StructuredJson::from_collection(self.piano.as_slice(), Some(LiveSet::name_string))),
            ("e_piano".to_string(), StructuredJson::from_collection(self.e_piano.as_slice(), Some(LiveSet::name_string))),
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
        serde_json::to_string(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}

impl RD300NX {
    const USER_SETS: usize = 60;
    const PIANO_SETS: usize = 10;
    const E_PIANO_SETS: usize = 15;

    fn check_sum(bytes_without_checksum: &Vec<u8>) -> u16 {
        let mut sum: u16 = 0;
        for byte in bytes_without_checksum {
            sum = sum.wrapping_add(*byte as u16);
        }
        sum
    }

    pub fn all_live_sets(&self) -> Vec<&LiveSet> {
        self.user_sets.iter().chain(self.piano.iter()).chain(self.e_piano.iter()).collect()
    }
}
