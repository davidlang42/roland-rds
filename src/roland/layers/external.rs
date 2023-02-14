use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};

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
    transmit_midi_messages: Bits<177>,
    s1: bool,
    s2: bool,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused: Bits<1>
}

impl Bytes<30> for ExternalLayer {
    fn to_bytes(&self) -> Result<Box<[u8; 30]>, BytesError> {
        BitStream::write_fixed(|bits| {
            bits.set_u8::<7>(self.range_lower, 0, 87)?;
            bits.set_u8::<7>(self.range_upper, self.range_lower, 87)?;
            bits.set_u8::<7>(self.velocity_range_lower, 1, 127)?;
            bits.set_u8::<7>(self.velocity_range_upper, 1, 127)?;
            bits.set_u8::<7>(self.velocity_sensitivity, 1, 127)?;
            bits.set_u8::<7>(self.velocity_max, 1, 127)?;
            bits.set_u8::<7>(self.transpose, 16, 112)?;
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
            bits.set_bits(&self.transmit_midi_messages);
            bits.set_bool(self.s1);
            bits.set_bool(self.s2);
            bits.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let range_lower = data.get_u8::<7>(0, 87)?;
            Ok(Self {
                range_lower,
                range_upper: data.get_u8::<7>(range_lower, 87)?,
                velocity_range_lower: data.get_u8::<7>(1, 127)?,
                velocity_range_upper: data.get_u8::<7>(1, 127)?,
                velocity_sensitivity: data.get_u8::<7>(1, 127)?,
                velocity_max: data.get_u8::<7>(1, 127)?,
                transpose: data.get_u8::<7>(16, 112)?,
                enable: data.get_bool(),
                damper: data.get_bool(),
                fc1: data.get_bool(),
                fc2: data.get_bool(),
                modulation: data.get_bool(),
                bender: data.get_bool(),
                control_mfx_switch: data.get_bool(),
                control_slider: [data.get_bool(), data.get_bool(), data.get_bool(), data.get_bool()],
                transmit_midi_messages: data.get_bits(),
                s1: data.get_bool(),
                s2: data.get_bool(),
                unused: data.get_bits()
            })
        })
    }
}
