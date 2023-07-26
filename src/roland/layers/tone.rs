use std::fmt::Debug;
use schemars::JsonSchema;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::roland::types::enums::MonoPoly;
use crate::roland::types::numeric::OffsetU8;

use super::super::tones::ToneNumber;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct ToneLayer {
    #[validate]
    tone_number: ToneNumber,
    #[validate]
    course_tune_semitones: OffsetU8<64, 16, 112>, // 16-112 (-48 - +48)
    #[validate]
    fine_tune_percent: OffsetU8<64, 14, 114>, // 14-114 (-50 - + 50)
    mono_poly: MonoPoly, // 0=Mono, 1=Poly, 2=Mono/Legato
    #[validate(range(max = 24))]
    pitch_bend_range_semitones: u8,
    portamento_switch: bool,
    #[validate(range(max = 127))]
    portamento_time: u8,
    #[validate]
    cutoff: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    #[validate]
    resonance: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    #[validate]
    attack_time: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    #[validate]
    decay_time: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    #[validate]
    release_time: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<10>::zero")]
    unused: Bits<10>
}

impl ToneLayer {
    pub fn tone_name(&self) -> String {
        self.tone_number.details().name.to_owned()
    }
}

impl Bytes<12> for ToneLayer {
    fn to_bytes(&self) -> Result<Box<[u8; 12]>, BytesError> {
        BitStream::write_fixed(|bits| {
            let tone = self.tone_number.details();
            bits.set_u8::<7>(tone.msb, 0, 127)?;
            bits.set_u8::<7>(tone.lsb, 0, 127)?;
            bits.set_u8::<7>(tone.pc, 0, 127)?;
            bits.set_u8::<7>(self.course_tune_semitones.into(), 16, 112)?;
            bits.set_u8::<7>(self.fine_tune_percent.into(), 14, 114)?;
            bits.set_u8::<2>(self.mono_poly.into(), 0, 2)?;
            bits.set_u8::<5>(self.pitch_bend_range_semitones, 0, 24)?;
            bits.set_bool(self.portamento_switch);
            bits.set_u8::<8>(self.portamento_time, 0, 127)?;
            bits.set_u8::<7>(self.cutoff.into(), 0, 127)?;
            bits.set_u8::<7>(self.resonance.into(), 0, 127)?;
            bits.set_u8::<7>(self.attack_time.into(), 0, 127)?;
            bits.set_u8::<7>(self.decay_time.into(), 0, 127)?;
            bits.set_u8::<7>(self.release_time.into(), 0, 127)?;
            bits.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let msb = data.get_u8::<7>(0, 127)?;
            let lsb = data.get_u8::<7>(0, 127)?;
            let pc = data.get_u8::<7>(0, 127)?;
            Ok(Self {
                tone_number: ToneNumber::find(msb, lsb, pc).ok_or(BytesError::InvalidTone { msb, lsb, pc })?,
                course_tune_semitones: data.get_u8::<7>(16, 112)?.into(),
                fine_tune_percent: data.get_u8::<7>(14, 114)?.into(),
                mono_poly: data.get_u8::<2>(0, 2)?.into(),
                pitch_bend_range_semitones: data.get_u8::<5>(0, 24)?,
                portamento_switch: data.get_bool(),
                portamento_time: data.get_u8::<8>(0, 127)?,
                cutoff: data.get_u8::<7>(0, 127)?.into(),
                resonance: data.get_u8::<7>(0, 127)?.into(),
                attack_time: data.get_u8::<7>(0, 127)?.into(),
                decay_time: data.get_u8::<7>(0, 127)?.into(),
                release_time: data.get_u8::<7>(0, 127)?.into(),
                unused: data.get_bits()
            })
        })
    }
}
