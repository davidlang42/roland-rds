use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};
use self::chorus::Chorus;
use self::common::Common;
use self::mfx::Mfx;
use self::resonance::Resonance;
use self::reverb::Reverb;
use self::song_rhythm::SongRhythm;

use super::layers::{InternalLayer, ToneLayer, ExternalLayer, PianoLayer, EPianoLayer, ToneWheelLayer};
use super::parse_many;

mod common;
mod chorus;
mod reverb;
mod song_rhythm;
mod mfx;
mod resonance;

//TODO modify live set to separate real and unused layers/parameters in 300nx vs 700nx

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveSet {
    common: Common, // 56 bytes
    song_rhythm: SongRhythm, // 6 bytes
    chorus: Chorus, // 42 bytes
    reverb: Reverb, // 42 bytes
    mfx: Box<[Mfx; 8]>, // 608 bytes
    resonance: Resonance, // 76 bytes
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
        BitStream::read_fixed(bytes, |data| {
            let common = Common::from_bytes(Box::new(data.get_bytes()))?;
            let song_rhythm = SongRhythm::from_bytes(Box::new(data.get_bytes()))?;
            let chorus = Chorus::from_bytes(Box::new(data.get_bytes()))?;
            let reverb = Reverb::from_bytes(Box::new(data.get_bytes()))?;
            let mfx = parse_many(data)?;
            let resonance = Resonance::from_bytes(Box::new(data.get_bytes()))?;
            let internal_layers = parse_many(data)?;
            let external_layers = parse_many(data)?;
            let tone_layers = parse_many(data)?;
            let piano_layers = parse_many(data)?;
            let e_piano_layers = parse_many(data)?;
            let tone_wheel_layers = parse_many(data)?;
            let unused = data.get_bits();
            let found_check_sum = data.get_u8::<8>();
            let live_set = Self {
                common,
                song_rhythm,
                chorus,
                reverb,
                mfx,
                resonance,
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
        })
    }

    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        let mut bytes = self.common.to_bytes().to_vec();
        for byte in *self.song_rhythm.to_bytes() {
            bytes.push(byte);
        }
        for byte in *self.chorus.to_bytes() {
            bytes.push(byte);
        }
        for byte in *self.reverb.to_bytes() {
            bytes.push(byte);
        }
        for mfx in self.mfx.iter() {
            for byte in *mfx.to_bytes() {
                bytes.push(byte);
            }
        }
        for byte in *self.resonance.to_bytes() {
            bytes.push(byte);
        }
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
        bytes.try_into().expect("Wrong number of bytes")
    }

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json()) //TODO (NEXT) split this up once I've made all the live set components
        //TODO (NEXT) make sure every file/folder has different names unless they are meant to be the identical type (and interchangeable) across entire project
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json()) //TODO (NEXT) split this up once I've made all the live set components
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}
