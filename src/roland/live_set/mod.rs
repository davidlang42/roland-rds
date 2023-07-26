use std::fmt::Debug;
use schemars::JsonSchema;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError};
use crate::json::validation::valid_boxed_elements;
use self::chorus::Chorus;
use self::common::Common;
use self::mfx::Mfx;
use self::resonance::Resonance;
use self::reverb::Reverb;
use self::song_rhythm::SongRhythm;

use super::layers::{LogicalLayer, ToneWheelLayer, EPianoLayer, InternalLayer, ExternalLayer, ToneLayer, PianoLayer};

mod common;
mod chorus;
mod reverb;
mod song_rhythm;
mod mfx;
mod resonance;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]//TODO validate that the piano_tone_number and tone_number match (as per https://github.com/davidlang42/roland-rds/issues/7)
pub struct LiveSet {
    #[validate]
    common: Common, // 56 bytes
    #[validate]
    song_rhythm: SongRhythm, // 6 bytes
    #[validate]
    chorus: Chorus, // 42 bytes
    #[validate]
    reverb: Reverb, // 42 bytes
    #[validate]
    mfx: Mfx, // 76 bytes
    #[validate(custom = "valid_boxed_elements")]
    unused_mfx: Box<[Mfx; 7]>, // 532 bytes
    //#[validate]
    unused_resonance: Resonance, // 76 bytes
    #[validate(custom = "valid_boxed_elements")]
    layers: Box<[LogicalLayer; 3]>, // 332*3=996 bytes
    #[validate]
    unused_layer: LogicalLayer, // 332 bytes
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<8>::unit")]
    padding: Bits<8>, // 1 byte
    // checksum: 1 byte
}

impl LiveSet {
    pub fn name_string(&self) -> String {
        self.common.name_string()
    }
}

impl Bytes<2160> for LiveSet {
    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> {
        BitStream::read_fixed(bytes, |data| {
            let common = Common::from_bytes(data.get_bytes())?;
            let song_rhythm = SongRhythm::from_bytes(data.get_bytes())?;
            let chorus = Chorus::from_bytes(data.get_bytes())?;
            let reverb = Reverb::from_bytes(data.get_bytes())?;
            let mfx = Mfx::from_bytes(data.get_bytes())?;
            let unused_mfx = Mfx::array_from_bytes(data)?;
            let unused_resonance = Resonance::from_bytes(data.get_bytes())?;
            let internal_layers = InternalLayer::array_from_bytes(data)?;
            let unused_internal_layer = InternalLayer::from_bytes(data.get_bytes())?;
            let external_layers = ExternalLayer::array_from_bytes(data)?;
            let unused_external_layer = ExternalLayer::from_bytes(data.get_bytes())?;
            let tone_layers = ToneLayer::array_from_bytes(data)?;
            let unused_tone_layer = ToneLayer::from_bytes(data.get_bytes())?;
            let piano_layers = PianoLayer::array_from_bytes(data)?;
            let unused_piano_layer = PianoLayer::from_bytes(data.get_bytes())?;
            let e_piano_layers = EPianoLayer::array_from_bytes(data)?;
            let unused_e_piano_layer = EPianoLayer::from_bytes(data.get_bytes())?;
            let tone_wheel_layers = ToneWheelLayer::array_from_bytes(data)?;
            let unused_tone_wheel_layer = ToneWheelLayer::from_bytes(data.get_bytes())?;
            let layers = LogicalLayer::from_layers(internal_layers, external_layers, tone_layers, piano_layers, e_piano_layers, tone_wheel_layers);
            let unused_layer = LogicalLayer {
                internal: unused_internal_layer,
                external: unused_external_layer,
                tone: unused_tone_layer,
                piano: unused_piano_layer,
                unused_e_piano: unused_e_piano_layer,
                unused_tone_wheel: unused_tone_wheel_layer,
            };
            let padding = data.get_bits();
            let expected_sum_to_zero = sum_to_zero(data.sum_previous_bytes());
            let found_sum_to_zero = data.get_full_u8();
            if found_sum_to_zero != expected_sum_to_zero {
                return Err(BytesError::IncorrectCheckSum {
                    expected: vec![expected_sum_to_zero],
                    found: vec![found_sum_to_zero]
                });
            }
            Ok(Self {
                common,
                song_rhythm,
                chorus,
                reverb,
                mfx,
                unused_mfx,
                unused_resonance,
                layers,
                unused_layer,
                padding
            })
        })
    }

    fn to_bytes(&self) -> Result<Box<[u8; 2160]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_bytes(self.common.to_bytes()?);
            bs.set_bytes(self.song_rhythm.to_bytes()?);
            bs.set_bytes(self.chorus.to_bytes()?);
            bs.set_bytes(self.reverb.to_bytes()?);
            bs.set_bytes(self.mfx.to_bytes()?);
            for mfx in self.unused_mfx.iter() {
                bs.set_bytes(mfx.to_bytes()?);
            }
            bs.set_bytes(self.unused_resonance.to_bytes()?);
            for layer in self.layers.iter() {
                bs.set_bytes(layer.internal.to_bytes()?);
            }
            bs.set_bytes(self.unused_layer.internal.to_bytes()?);
            for layer in self.layers.iter() {
                bs.set_bytes(layer.external.to_bytes()?);
            }
            bs.set_bytes(self.unused_layer.external.to_bytes()?);
            for layer in self.layers.iter() {
                bs.set_bytes(layer.tone.to_bytes()?);
            }
            bs.set_bytes(self.unused_layer.tone.to_bytes()?);
            for layer in self.layers.iter() {
                bs.set_bytes(layer.piano.to_bytes()?);
            }
            bs.set_bytes(self.unused_layer.piano.to_bytes()?);
            for layer in self.layers.iter() {
                bs.set_bytes(layer.unused_e_piano.to_bytes()?);
            }
            bs.set_bytes(self.unused_layer.unused_e_piano.to_bytes()?);
            for layer in self.layers.iter() {
                bs.set_bytes(layer.unused_tone_wheel.to_bytes()?);
            }
            bs.set_bytes(self.unused_layer.unused_tone_wheel.to_bytes()?);
            bs.set_bits(&self.padding);
            let sum_to_zero = sum_to_zero(bs.sum_previous_bytes());
            bs.set_full_u8(sum_to_zero);
            Ok(())
        })
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
            ("mfx".to_string(), self.mfx.to_structured_json()),
            ("unused_mfx".to_string(), StructuredJson::from_collection(self.unused_mfx.as_slice(), |_| None)),
            ("unused_resonance".to_string(), self.unused_resonance.to_structured_json()),
            ("layers".to_string(), StructuredJson::from_collection(self.layers.as_slice(), |l| Some(l.tone.tone_name()))),
            ("unused_layer".to_string(), self.unused_layer.to_structured_json())
        ])
    }

    fn from_structured_json(mut structured_json: StructuredJson) -> Result<Self, StructuredJsonError> {
        let common = structured_json.extract("ls_common")?.to()?;
        let song_rhythm = structured_json.extract("song_rhythm")?.to()?;
        let chorus = structured_json.extract("chorus")?.to()?;
        let reverb = structured_json.extract("reverb")?.to()?;
        let mfx = structured_json.extract("mfx")?.to()?;
        let unused_mfx = structured_json.extract("unused_mfx")?.to_array()?;
        let unused_resonance = structured_json.extract("unused_resonance")?.to()?;
        let layers = structured_json.extract("layers")?.to_array()?;
        let unused_layer = structured_json.extract("unused_layer")?.to()?;
        structured_json.done()?;
        Ok(Self {
            common,
            song_rhythm,
            chorus,
            reverb,
            mfx,
            unused_mfx,
            unused_resonance,
            layers,
            unused_layer,
            padding: Bits::unit()
        })
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn from_json(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}

fn sum_to_zero(sum: u16) -> u8 {
    (u8::MAX - sum.to_be_bytes()[1]).wrapping_add(1)
}