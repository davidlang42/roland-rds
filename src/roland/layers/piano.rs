use std::collections::HashMap;
use std::fmt::Debug;
use schemars::JsonSchema;
use strum::IntoEnumIterator;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::roland::tones::PianoToneNumber;
use crate::roland::types::enums::{StretchTuneType, NuanceType};
use crate::roland::types::notes::MidiNote;
use crate::roland::types::numeric::{Offset1Dp, OffsetU8};
use crate::json::serialize_map_keys_in_order;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct PianoLayer {
    #[validate]
    pub tone_number: PianoToneNumber,
    #[validate(range(max = 63))]
    stereo_width: u8,
    nuance: NuanceType,
    #[validate(range(max = 127))]
    duplex_scale_level: u8,
    #[validate]
    hammer_noise_level: OffsetU8<4, 2, 6>, // 2-6 (-2 - +2)
    #[validate(range(max = 127))]
    damper_noise_level: u8,
    #[validate(range(max = 127))]
    string_resonance_level: u8,
    #[validate(range(max = 127))]
    key_off_resonance_level: u8,
    #[validate(range(max = 127))]
    sound_lift: u8,
    #[validate]
    tone_character: OffsetU8<8, 3, 13>, // 3-13 (-5 - +5)
    stretch_tune_type: StretchTuneType,
    #[serde(deserialize_with = "serialize_map_keys_in_order::deserialize")]
    #[serde(serialize_with = "serialize_map_keys_in_order::serialize")]
    #[schemars(with = "serialize_map_keys_in_order::OptionalMapSchema::<MidiNote, Offset1Dp<512, 12, 1012>>")]
    #[validate]
    micro_tune_percent: HashMap<MidiNote, Offset1Dp<512, 12, 1012>>, // each 12-1012 (-50.0 - +50.0)
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<5>::zero")]
    unused: Bits<5>
}

impl Bytes<264> for PianoLayer {
    fn to_bytes(&self) -> Result<Box<[u8; 264]>, BytesError> {
        BitStream::write_fixed(|bits| {
            bits.set_u8::<7>(self.tone_number.into(), 0, 8)?;
            bits.set_u8::<6>(self.stereo_width, 0, 63)?;
            bits.set_u8::<2>(self.nuance.into(), 0, 2)?;
            bits.set_u8::<7>(self.duplex_scale_level, 0, 127)?;
            bits.set_u8::<3>(self.hammer_noise_level.into(), 2, 6)?;
            bits.set_u8::<7>(self.damper_noise_level, 0, 127)?;
            bits.set_u8::<7>(self.string_resonance_level, 0, 127)?;
            bits.set_u8::<7>(self.key_off_resonance_level, 0, 127)?;
            bits.set_u8::<7>(self.sound_lift, 0, 127)?;
            bits.set_u8::<4>(self.tone_character.into(), 3, 13)?;
            bits.set_u8::<2>(self.stretch_tune_type.into(), 0, 2)?;
            for note in MidiNote::iter() {
                let value = match self.micro_tune_percent.get(&note) {
                    Some(value) => *value,
                    None => Offset1Dp::default()
                };
                bits.set_u16::<16>(value.into(), 12, 1012)?;
            }
            bits.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let tone_number = data.get_u8::<7>(0, 8)?.into();
            let stereo_width = data.get_u8::<6>(0, 63)?;
            let nuance = data.get_u8::<2>(0, 2)?.into();
            let duplex_scale_level = data.get_u8::<7>(0, 127)?;
            let hammer_noise_level = data.get_u8::<3>(2, 6)?.into();
            let damper_noise_level = data.get_u8::<7>(0, 127)?;
            let string_resonance_level = data.get_u8::<7>(0, 127)?;
            let key_off_resonance_level = data.get_u8::<7>(0, 127)?;
            let sound_lift = data.get_u8::<7>(0, 127)?;
            let tone_character = data.get_u8::<4>(3, 13)?.into();
            let stretch_tune_type = data.get_u8::<2>(0, 2)?.into();
            let mut micro_tune_percent = HashMap::new();
            for note in MidiNote::iter() {
                let value = data.get_u16::<16>(12, 1012)?.into();
                if value != Offset1Dp::default() {
                    micro_tune_percent.insert(note, value);
                }
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
                micro_tune_percent,
                unused: data.get_bits()
            })
        })
    }
}
