use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};

use super::super::tones::ToneNumber;
use super::super::{max, in_range};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToneLayer {
    tone_number: ToneNumber,
    course_tune: u8, // 16-112 (-48 - +48)
    fine_tune: u8, // 14-114 (-50 - + 50)
    mono_poly: u8, // 0=Mono, 1=Poly, 2=Mono/Legato
    pitch_bend_range: u8, // max 24
    portamento_switch: bool,
    portamento_time: u8, // max 127
    cutoff: u8, // max 127 (-63 - +63)
    resonance: u8, // max 127 (-63 - +63)
    attack_time: u8, // max 127 (-63 - +63)
    decay_time: u8, // max 127 (-63 - +63)
    release_time: u8, // max 127 (-63 - +63)
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused: Bits<10>
}

impl Bytes<12> for ToneLayer {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        BitStream::write_fixed(|bits| {
            let tone = self.tone_number.details();
            bits.set_u8::<7>(tone.msb);
            bits.set_u8::<7>(tone.lsb);
            bits.set_u8::<7>(tone.pc);
            bits.set_u8::<7>(in_range(self.course_tune, 16, 112));
            bits.set_u8::<7>(in_range(self.fine_tune, 14, 114));
            bits.set_u8::<2>(max(self.mono_poly, 2));
            bits.set_u8::<5>(max(self.pitch_bend_range, 24));
            bits.set_bool(self.portamento_switch);
            bits.set_u8::<8>(max(self.portamento_time, 127));
            bits.set_u8::<7>(self.cutoff);
            bits.set_u8::<7>(self.resonance);
            bits.set_u8::<7>(self.attack_time);
            bits.set_u8::<7>(self.decay_time);
            bits.set_u8::<7>(self.release_time);
            bits.set_bits(&self.unused);
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let msb = data.get_u8::<7>();
            let lsb = data.get_u8::<7>();
            let pc = data.get_u8::<7>();
            Ok(Self {
                tone_number: ToneNumber::find(msb, lsb, pc).expect(&format!("Tone not found: MSB({}) LSB({}) PC({})", msb, lsb, pc)),
                course_tune: data.get_u8::<7>(),
                fine_tune: data.get_u8::<7>(),
                mono_poly: data.get_u8::<2>(),
                pitch_bend_range: data.get_u8::<5>(),
                portamento_switch: data.get_bool(),
                portamento_time: data.get_u8::<8>(),
                cutoff: data.get_u8::<7>(),
                resonance: data.get_u8::<7>(),
                attack_time: data.get_u8::<7>(),
                decay_time: data.get_u8::<7>(),
                release_time: data.get_u8::<7>(),
                unused: data.get_bits()
            })
        })
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
