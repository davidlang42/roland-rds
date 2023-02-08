use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};

use super::tones::ToneNumber;
use super::{max, in_range};

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalLayer {
    volume: u8, // max 127
    pan: u8, // max 127 (L64 - 63R)
    chorus: u8, // max 127
    reverb: u8, // max 127
    range_lower: u8,  // max 87
    range_upper: u8, // max 87
    velocity_range_lower: u8, // 1-127
    velocity_range_upper: u8, // 1-127
    velocity_sensitivity: u8, // 1-127 (-63 - +63)
    velocity_max: u8, // 1-127
    transpose: u8, // 16-112 (-48 - +48)
    enable: bool,
    damper: bool,
    fc1: bool,
    fc2: bool,
    modulation: bool,
    bender: bool,
    control_slider: [bool; 4], // UPPER1, UPPER2, LOWER1, LOWER2
    s1: bool,
    s2: bool,
    // flags below are not editable on the keyboard
    receive_bank_select: bool,
    receive_program_change: bool,
    receive_bender: bool,
    receive_modulation: bool,
    receive_volume: bool,
    receive_pan: bool,
    receive_hold_1: bool,
    receive_expression: bool,
    unused: Bits<15>
}

impl Bytes<14> for InternalLayer {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        let mut bits = BitStream::new();
        bits.set_u8::<7>(self.volume);
        bits.set_u8::<7>(self.pan);
        bits.set_u8::<7>(self.chorus);
        bits.set_u8::<7>(self.reverb);
        bits.set_u8::<7>(max(self.range_lower, 87));
        bits.set_u8::<7>(max(self.range_upper, 87));
        bits.set_u8::<7>(in_range(self.velocity_range_lower, 1, 127));
        bits.set_u8::<7>(in_range(self.velocity_range_upper, 1, 127));
        bits.set_u8::<7>(in_range(self.velocity_sensitivity, 1, 127));
        bits.set_u8::<7>(in_range(self.velocity_max, 1, 127));
        bits.set_u8::<7>(in_range(self.transpose, 16, 112));
        bits.set_bool(self.enable);
        bits.set_bool(self.damper);
        bits.set_bool(self.fc1);
        bits.set_bool(self.fc2);
        bits.set_bool(self.modulation);
        bits.set_bool(self.bender);
        for value in self.control_slider {
            bits.set_bool(value);
        }
        bits.set_bool(self.s1);
        bits.set_bool(self.s2);
        bits.set_bool(self.receive_bank_select);
        bits.set_bool(self.receive_program_change);
        bits.set_bool(self.receive_bender);
        bits.set_bool(self.receive_modulation);
        bits.set_bool(self.receive_volume);
        bits.set_bool(self.receive_pan);
        bits.set_bool(self.receive_hold_1);
        bits.set_bool(self.receive_expression);
        bits.set_bits(&self.unused);
        bits.reset();
        Box::new(bits.get_bytes())
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        let mut data = BitStream::read(bytes);
        Ok(Self {
            volume: data.get_u8::<7>(),
            pan: data.get_u8::<7>(),
            chorus: data.get_u8::<7>(),
            reverb: data.get_u8::<7>(),
            range_lower: data.get_u8::<7>(),
            range_upper: data.get_u8::<7>(),
            velocity_range_lower: data.get_u8::<7>(),
            velocity_range_upper: data.get_u8::<7>(),
            velocity_sensitivity: data.get_u8::<7>(),
            velocity_max: data.get_u8::<7>(),
            transpose: data.get_u8::<7>(),
            enable: data.get_bool(),
            damper: data.get_bool(),
            fc1: data.get_bool(),
            fc2: data.get_bool(),
            modulation: data.get_bool(),
            bender: data.get_bool(),
            control_slider: [data.get_bool(),data.get_bool(),data.get_bool(),data.get_bool()],
            s1: data.get_bool(),
            s2: data.get_bool(),
            receive_bank_select: data.get_bool(),
            receive_program_change: data.get_bool(),
            receive_bender: data.get_bool(),
            receive_modulation: data.get_bool(),
            receive_volume: data.get_bool(),
            receive_pan: data.get_bool(),
            receive_hold_1: data.get_bool(),
            receive_expression: data.get_bool(),
            unused: data.get_bits()
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
    unused: Bits<10>
}

impl Bytes<12> for ToneLayer {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        let mut bits = BitStream::new();
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
        bits.reset();
        Box::new(bits.get_bytes())
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        let mut data = BitStream::read(bytes);
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