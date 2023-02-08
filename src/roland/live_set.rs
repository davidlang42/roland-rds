use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};
use crate::json::serialize_chars_as_string;
use super::layers::{InternalLayer, ToneLayer, ExternalLayer};
use super::{validate, parse_many};

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveSet {
    // Common
    #[serde(with = "serialize_chars_as_string")]
    name: [char; 16], // 14 bytes
    other1: Bits<6528>, // 816 bytes
    // Song/Rhythm
    // Chorus
    // Reverb
    // MFX x8
    // Resonance
    // Internal Layer x4
    internal_layers: Box<[InternalLayer; 4]>, // 56 bytes
    // External Layer x4
    external_layers: Box<[ExternalLayer; 4]>, // 120 bytes
    // Tone x4
    tone_layers: Box<[ToneLayer; 4]>, // 48 bytes
    // Piano x4
    other2: Bits<8840>, // 1105 bytes
    // E.Piano x4
    // ToneWheel x4
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
        (u8::MAX - sum).wrapping_add(1)
    }
}

impl Bytes<2160> for LiveSet {
    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> {
        let mut data = BitStream::read(bytes);
        let mut name = [char::default(); 16];
        for i in 0..name.len() {
            name[i] = validate(data.get_char())?;
        }
        let other1 = data.get_bits();
        let internal_layers = parse_many(&mut data)?;
        let external_layers = parse_many(&mut data)?;
        let tone_layers = parse_many(&mut data)?;
        let other2 = data.get_bits();
        let found_check_sum = data.get_u8::<8>();
        let live_set = Self {
            name,
            other1,
            internal_layers,
            external_layers,
            tone_layers,
            other2
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

    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        let mut bytes = Bits::<7>::compress(self.name).to_bytes();
        bytes.append(&mut self.other1.to_bytes());
        for internal_layer in self.internal_layers.iter() {
            for byte in *internal_layer.to_bytes() {
                bytes.push(byte);
            }
        }
        for external_layer in self.external_layers.iter() {
            for byte in *external_layer.to_bytes() {
                bytes.push(byte);
            }
        }
        for tone_layer in self.tone_layers.iter() {
            for byte in *tone_layer.to_bytes() {
                bytes.push(byte);
            }
        }
        bytes.append(&mut self.other2.to_bytes());
        let check_sum = Self::check_sum(&bytes);
        bytes.push(check_sum);
        bytes.try_into().unwrap()
    }

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json()) //TODO split this up once I've made all the live set components
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json()) //TODO split this up once I've made all the live set components
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}
