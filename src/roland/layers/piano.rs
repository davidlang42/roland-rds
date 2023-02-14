use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson};
use crate::json::serialize_array_as_vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct PianoLayer {
    tone_number: u8, // max 8
    stereo_width: u8, // max 63
    nuance: u8, // max 2 (TYPE1, TYPE2, TYPE3)
    duplex_scale_level: u8, // max 127
    hammer_noise_level: u8, // MI is wrong: "62-66 (-2 - +2)"
    damper_noise_level: u8, // max 127
    string_resonance_level: u8, // max 127
    key_off_resonance_level: u8, // max 127
    sound_lift: u8, // max 127
    tone_character: u8, // MI is wrong: "59-69 (-5 - +5)"
    stretch_tune_type: u8, // max 2 (OFF, PRESET, USER)
    #[serde(with = "serialize_array_as_vec")]
    micro_tune: Box<[u16; 128]>, // index=midi note?, each 12-1012 (-50.0 - +50.0)
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused: Bits<5>
}

impl Bytes<264> for PianoLayer {
    fn to_bytes(&self) -> Result<Box<[u8; 264]>, BytesError> {
        BitStream::write_fixed(|bits| {
            bits.set_u8::<7>(self.tone_number, 0, 8)?;
            bits.set_u8::<6>(self.stereo_width, 0, 63)?;
            bits.set_u8::<2>(self.nuance, 0, 2)?;
            bits.set_u8::<7>(self.duplex_scale_level, 0, 127)?;
            bits.set_u8::<3>(self.hammer_noise_level, 0, 255)?; // MI is wrong: "62-66 (-2 - +2)"
            bits.set_u8::<7>(self.damper_noise_level, 0, 127)?;
            bits.set_u8::<7>(self.string_resonance_level, 0, 127)?;
            bits.set_u8::<7>(self.key_off_resonance_level, 0, 127)?;
            bits.set_u8::<7>(self.sound_lift, 0, 127)?;
            bits.set_u8::<4>(self.tone_character, 0, 255)?; // MI is wrong: "59-69 (-5 - +5)"
            bits.set_u8::<2>(self.stretch_tune_type, 0, 2)?;
            for value in *self.micro_tune {
                bits.set_u16::<16>(value, 12, 1012)?;
            }
            bits.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let tone_number = data.get_u8::<7>(0, 8)?;
            let stereo_width = data.get_u8::<6>(0, 63)?;
            let nuance = data.get_u8::<2>(0, 2)?;
            let duplex_scale_level = data.get_u8::<7>(0, 127)?;
            let hammer_noise_level = data.get_u8::<3>(0, 255)?; // MI is wrong: "62-66 (-2 - +2)"
            let damper_noise_level = data.get_u8::<7>(0, 127)?;
            let string_resonance_level = data.get_u8::<7>(0, 127)?;
            let key_off_resonance_level = data.get_u8::<7>(0, 127)?;
            let sound_lift = data.get_u8::<7>(0, 127)?;
            let tone_character = data.get_u8::<4>(0, 255)?; // MI is wrong: "59-69 (-5 - +5)"
            let stretch_tune_type = data.get_u8::<2>(0, 2)?;
            let mut micro_tune = [0; 128];
            for i in 0..micro_tune.len() {
                micro_tune[i] = data.get_u16::<16>(12, 1012)?;
            }
            Ok(Self {
                tone_number,
                stereo_width,
                nuance,
                duplex_scale_level,
                hammer_noise_level,
                damper_noise_level,
                string_resonance_level,
                key_off_resonance_level,
                sound_lift,
                tone_character,
                stretch_tune_type,
                micro_tune: Box::new(micro_tune),
                unused: data.get_bits()
            })
        })
    }
}

impl Json for PianoLayer {
    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json())
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}