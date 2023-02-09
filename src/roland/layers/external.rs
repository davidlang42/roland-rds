use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};

use super::super::{max, in_range};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalLayer {
    range_lower: u8, // max 87 (A0-C8)
    range_upper: u8, // max 87 (A0-C8), must be >= lower
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
    control_mfx_switch: bool,
    control_slider: [bool; 4], // index=layer (UPPER1, UPPER2, LOWER1, LOWER2)
    transmit_other: Bits<177>, //TODO these are well defined by the 700NX midi implementation, but CBF doing the boilerplate rn
    s1: bool,
    s2: bool,
    unused: Bits<1>
}

impl Bytes<30> for ExternalLayer {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        let mut bits = BitStream::new();
        bits.set_u8::<7>(max(self.range_lower, 87));
        bits.set_u8::<7>(in_range(self.range_upper, self.range_lower, 87));
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
        bits.set_bool(self.control_mfx_switch);
        for value in self.control_slider {
            bits.set_bool(value);
        }
        bits.set_bits(&self.transmit_other);
        bits.set_bool(self.s1);
        bits.set_bool(self.s2);
        bits.set_bits(&self.unused);
        bits.reset();
        Box::new(bits.get_bytes())
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        let mut data = BitStream::read(bytes);
        Ok(Self {
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
            control_mfx_switch: data.get_bool(),
            control_slider: [data.get_bool(), data.get_bool(), data.get_bool(), data.get_bool()],
            transmit_other: data.get_bits(),
            s1: data.get_bool(),
            s2: data.get_bool(),
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