use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};
use self::common::Common;

use super::layers::{InternalLayer, ToneLayer, ExternalLayer, PianoLayer, EPianoLayer, ToneWheelLayer};
use super::parse_many;

mod common;

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveSet {
    common: Common, // 56 bytes
    other: Bits<6192>, // 774 bytes
    // Song/Rhythm
    // Chorus
    // Reverb
    // MFX x8
    // Resonance
    internal_layers: Box<[InternalLayer; 4]>, // 56 bytes
    external_layers: Box<[ExternalLayer; 4]>, // 120 bytes
    tone_layers: Box<[ToneLayer; 4]>, // 48 bytes
    piano_layers: Box<[PianoLayer; 4]>, // 1056 bytes
    e_piano_layers: Box<[EPianoLayer; 4]>, // 24 bytes
    tone_wheel_layers: Box<[ToneWheelLayer; 4]>, // 24 bytes
    unused: Bits<8>, // 1 byte
    // checksum: 1 byte
}

impl LiveSet {
    pub fn name_string(&self) -> String {
        self.common.name_string()
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
        let common = Common::from_bytes(Box::new(data.get_bytes()))?;
        let other = data.get_bits();
        let internal_layers = parse_many(&mut data)?;
        let external_layers = parse_many(&mut data)?;
        let tone_layers = parse_many(&mut data)?;
        let piano_layers = parse_many(&mut data)?;
        let e_piano_layers = parse_many(&mut data)?;
        let tone_wheel_layers = parse_many(&mut data)?;
        let unused = data.get_bits();
        let found_check_sum = data.get_u8::<8>();
        let live_set = Self {
            common,
            other: other,
            internal_layers,
            external_layers,
            tone_layers,
            piano_layers,
            e_piano_layers,
            tone_wheel_layers,
            unused
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
        let mut bytes = self.common.to_bytes().to_vec();
        bytes.append(&mut self.other.to_bytes());
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
        for piano_layer in self.piano_layers.iter() {
            for byte in *piano_layer.to_bytes() {
                bytes.push(byte);
            }
        }
        for e_piano_layer in self.e_piano_layers.iter() {
            for byte in *e_piano_layer.to_bytes() {
                bytes.push(byte);
            }
        }
        for tone_wheel_layer in self.tone_wheel_layers.iter() {
            for byte in *tone_wheel_layer.to_bytes() {
                bytes.push(byte);
            }
        }
        bytes.append(&mut self.unused.to_bytes());
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
