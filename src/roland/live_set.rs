use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};
use crate::json::serialize_chars_as_string;
use super::validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveSet {
    #[serde(with = "serialize_chars_as_string")]
    name: [char; 16], // 14 bytes
    other: Bits<17160>, // 2145 bytes
    // checksum: 1 byte
}

impl LiveSet {
    pub fn name_string(&self) -> String {
        self.name.iter().collect()
    }

    fn check_sum(bytes_without_checksum: &Vec<u8>) -> u8 {
        let mut sum: u8 = 0;
        for byte in bytes_without_checksum {
            sum = sum.wrapping_add(*byte);
        }
        u8::MAX - sum + 1
    }
}

impl Bytes<2160> for LiveSet {
    fn from_bytes(bytes: [u8; Self::BYTE_SIZE]) -> Result<Self, BytesError> {
        let mut data = BitStream::read(bytes);
        let mut name = [char::default(); 16];
        for i in 0..name.len() {
            name[i] = validate(data.get_char())?;
        }
        let other = data.get_bits();
        let found_check_sum = data.get_u8::<8>();
        let live_set = Self {
            name,
            other
        };
        let bytes = live_set.to_bytes();
        let expected_check_sum = bytes[bytes.len() - 1];
        if found_check_sum != expected_check_sum {
            return Err(BytesError::IncorrectCheckSum {
                expected: vec![expected_check_sum],
                found: vec![found_check_sum]
            });
        }
        Ok(live_set)
    }

    fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        let mut bytes = Bits::<7>::compress(self.name).to_bytes();
        bytes.append(&mut self.other.to_bytes());
        let check_sum = Self::check_sum(&bytes);
        bytes.push(check_sum);
        bytes.try_into().unwrap()
    }

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json())
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}
