use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson};
use self::chorus::Chorus;
use self::common::Common;
use self::mfx::Mfx;
use self::resonance::Resonance;
use self::reverb::Reverb;
use self::song_rhythm::SongRhythm;

use super::layers::LogicalLayer;
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
    layers: Box<[LogicalLayer; 4]>, // 332*4=1328 bytes
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::unit")]
    padding: Bits<8>, // 1 byte
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
            let layers = LogicalLayer::from_layers(internal_layers, external_layers, tone_layers, piano_layers, e_piano_layers, tone_wheel_layers);
            let padding = data.get_bits();
            let found_check_sum = data.get_u8::<8>();
            let live_set = Self {
                common,
                song_rhythm,
                chorus,
                reverb,
                mfx,
                resonance,
                layers,
                padding
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
        for layer in self.layers.iter() {
            for byte in *layer.internal.to_bytes() {
                bytes.push(byte);
            }
        }
        for layer in self.layers.iter() {
            for byte in *layer.external.to_bytes() {
                bytes.push(byte);
            }
        }
        for layer in self.layers.iter() {
            for byte in *layer.tone.to_bytes() {
                bytes.push(byte);
            }
        }
        for layer in self.layers.iter() {
            for byte in *layer.piano.to_bytes() {
                bytes.push(byte);
            }
        }
        for layer in self.layers.iter() {
            for byte in *layer.e_piano.to_bytes() {
                bytes.push(byte);
            }
        }
        for layer in self.layers.iter() {
            for byte in *layer.tone_wheel.to_bytes() {
                bytes.push(byte);
            }
        }
        bytes.append(&mut self.padding.to_bytes());
        let check_sum = Self::check_sum(&bytes);
        bytes.push(check_sum);
        bytes.try_into().expect("Wrong number of bytes")
    }
}

impl Json for LiveSet {
    fn to_structured_json(&self) -> StructuredJson {
        if !self.padding.is_unit() {
            panic!("Cannot split JSON with non-standard padding");
        }
        StructuredJson::NestedCollection(vec![
            ("ls_common".to_string(), self.common.to_structured_json()),
            ("song_rhythm".to_string(), self.song_rhythm.to_structured_json()),
            ("chorus".to_string(), self.chorus.to_structured_json()),
            ("reverb".to_string(), self.reverb.to_structured_json()),
            ("mfx".to_string(), StructuredJson::from_collection(self.mfx.as_slice(), |_| None)),
            ("resonance".to_string(), self.resonance.to_structured_json()),
            ("layers".to_string(), StructuredJson::from_collection(self.layers.as_slice(), |l| Some(l.tone.tone_name())))
        ])
    }

    fn from_structured_json(mut structured_json: StructuredJson) -> Self {
        let common = structured_json.extract("ls_common").to();
        let song_rhythm = structured_json.extract("song_rhythm").to();
        let chorus = structured_json.extract("chorus").to();
        let reverb = structured_json.extract("reverb").to();
        let mfx = structured_json.extract("mfx").to_array();
        let resonance = structured_json.extract("resonance").to();
        let layers = structured_json.extract("layers").to_array();
        structured_json.done();
        Self {
            common,
            song_rhythm,
            chorus,
            reverb,
            mfx,
            resonance,
            layers,
            padding: Bits::unit()
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}
