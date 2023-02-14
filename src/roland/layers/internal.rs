use std::collections::HashMap;
use std::fmt::Debug;

use strum::IntoEnumIterator;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::roland::types::enums::{Pan, Layer};
use crate::roland::types::notes::PianoKey;
use crate::roland::types::numeric::OffsetU8;

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalLayer {
    volume: u8, // max 127
    pan: Pan,
    chorus: u8, // max 127
    reverb: u8, // max 127
    range_lower: PianoKey,
    range_upper: PianoKey,
    velocity_range_lower: u8, // 1-127
    velocity_range_upper: u8, // 1-127
    velocity_sensitivity: OffsetU8<64>, // 1-127 (-63 - +63)
    velocity_max: u8, // 1-127
    transpose: OffsetU8<64>, // 16-112 (-48 - +48)
    enable: bool,
    damper: bool,
    fc1: bool,
    fc2: bool,
    modulation: bool,
    bender: bool,
    control_slider: HashMap<Layer, bool>,
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
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused: Bits<15>
}

impl Bytes<14> for InternalLayer {
    fn to_bytes(&self) -> Result<Box<[u8; 14]>, BytesError> {
        BitStream::write_fixed(|bits| {
            bits.set_u8::<7>(self.volume, 0, 127)?;
            bits.set_u8::<7>(self.pan.into(), 0, 127)?;
            bits.set_u8::<7>(self.chorus, 0, 127)?;
            bits.set_u8::<7>(self.reverb, 0, 127)?;
            bits.set_u8::<7>(self.range_lower.into(), 0, 87)?;
            bits.set_u8::<7>(self.range_upper.into(), self.range_lower.into(), 87)?;
            bits.set_u8::<7>(self.velocity_range_lower, 1, 127)?;
            bits.set_u8::<7>(self.velocity_range_upper, 1, 127)?;
            bits.set_u8::<7>(self.velocity_sensitivity.into(), 1, 127)?;
            bits.set_u8::<7>(self.velocity_max, 1, 127)?;
            bits.set_u8::<7>(self.transpose.into(), 16, 112)?;
            bits.set_bool(self.enable);
            bits.set_bool(self.damper);
            bits.set_bool(self.fc1);
            bits.set_bool(self.fc2);
            bits.set_bool(self.modulation);
            bits.set_bool(self.bender);
            for key in Layer::iter() {
                bits.set_bool(*self.control_slider.get(&key).unwrap());
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
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let volume = data.get_u8::<7>(0, 127)?;
            let pan = data.get_u8::<7>(0, 127)?.into();
            let chorus = data.get_u8::<7>(0, 127)?;
            let reverb = data.get_u8::<7>(0, 127)?;
            let range_lower: PianoKey = data.get_u8::<7>(0, 87)?.into();
            let range_upper = data.get_u8::<7>(range_lower.into(), 87)?.into();
            let velocity_range_lower = data.get_u8::<7>(1, 127)?;
            let velocity_range_upper = data.get_u8::<7>(1, 127)?;
            let velocity_sensitivity = data.get_u8::<7>(1, 127)?.into();
            let velocity_max = data.get_u8::<7>(1, 127)?;
            let transpose = data.get_u8::<7>(16, 112)?.into();
            let enable = data.get_bool();
            let damper = data.get_bool();
            let fc1 = data.get_bool();
            let fc2 = data.get_bool();
            let modulation = data.get_bool();
            let bender = data.get_bool();
            let mut control_slider = HashMap::new();
            for key in Layer::iter() {
                control_slider.insert(key, data.get_bool());
            }
            Ok(Self {
                volume,
                pan,
                chorus,
                reverb,
                range_lower,
                range_upper,
                velocity_range_lower,
                velocity_range_upper,
                velocity_sensitivity,
                velocity_max,
                transpose,
                enable,
                damper,
                fc1,
                fc2,
                modulation,
                bender,
                control_slider,
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
        })
    }
}

