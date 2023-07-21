use std::collections::HashMap;
use std::fmt::Debug;

use schemars::JsonSchema;
use strum::IntoEnumIterator;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::serialize_map_keys_in_order;
use crate::roland::types::numeric::OffsetU8;
use crate::roland::types::notes::PianoKey;
use crate::roland::types::enums::Layer;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ExternalLayer {
    range_lower: PianoKey, // max 87 (A0-C8)
    range_upper: PianoKey, // max 87 (A0-C8), must be >= lower
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
    control_mfx_switch: bool,
    #[serde(deserialize_with = "serialize_map_keys_in_order::deserialize")]
    #[serde(serialize_with = "serialize_map_keys_in_order::serialize")]
    #[schemars(with = "serialize_map_keys_in_order::RequiredMapSchema::<Layer, bool>")]
    control_slider: HashMap<Layer, bool>,
    transmit_midi_messages: Bits<177>,
    s1: bool,
    s2: bool,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<1>::zero")]
    unused: Bits<1>
}

impl Bytes<30> for ExternalLayer {
    fn to_bytes(&self) -> Result<Box<[u8; 30]>, BytesError> {
        BitStream::write_fixed(|bits| {
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
            bits.set_bool(self.control_mfx_switch);
            for key in Layer::iter() {
                bits.set_bool(*self.control_slider.get(&key).unwrap());
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
            let control_mfx_switch = data.get_bool();
            let mut control_slider = HashMap::new();
            for key in Layer::iter() {
                control_slider.insert(key, data.get_bool());
            }
            Ok(Self {
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
                control_mfx_switch,
                control_slider,
                transmit_midi_messages: data.get_bits(),
                s1: data.get_bool(),
                s2: data.get_bool(),
                unused: data.get_bits()
            })
        })
    }
}
