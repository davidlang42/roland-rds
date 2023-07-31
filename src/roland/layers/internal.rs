use std::collections::HashMap;
use std::fmt::Debug;

use schemars::JsonSchema;
use strum::IntoEnumIterator;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::serialize_map_keys_in_order;
use crate::json::validation::{contains_all_keys, LayerRanges, valid_key_range, valid_velocity_range};
use crate::roland::types::enums::{Pan, Layer, PedalFunction};
use crate::roland::types::notes::PianoKey;
use crate::roland::types::numeric::OffsetU8;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[validate(schema(function = "valid_key_range"))]
#[validate(schema(function = "valid_velocity_range"))]
pub struct InternalLayer {
    #[validate(range(max = 127))]
    volume: u8,
    #[validate]
    pan: Pan,
    #[validate(range(max = 127))]
    pub chorus: u8,
    #[validate(range(max = 127))]
    pub reverb: u8,
    range_lower: PianoKey,
    range_upper: PianoKey,
    #[validate(range(min = 1, max = 127))]
    velocity_range_lower: u8,
    #[validate(range(min = 1, max = 127))]
    velocity_range_upper: u8,
    #[validate]
    velocity_sensitivity: OffsetU8<64, 1, 127>, // 1-127 (-63 - +63)
    #[validate(range(min = 1, max = 127))]
    velocity_max: u8,
    #[validate]
    transpose: OffsetU8<64, 16, 112>, // 16-112 (-48 - +48)
    enable: bool,
    damper: bool,
    fc1: bool,
    fc2: bool,
    modulation: bool,
    bender: bool,
    #[serde(deserialize_with = "serialize_map_keys_in_order::deserialize")]
    #[serde(serialize_with = "serialize_map_keys_in_order::serialize")]
    #[schemars(with = "serialize_map_keys_in_order::RequiredMapSchema::<Layer, bool>")]
    #[validate(custom = "contains_all_keys")]
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
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<15>::zero")]
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

impl LayerRanges for InternalLayer {
    fn get_range_upper(&self) -> PianoKey {
        self.range_upper
    }

    fn get_range_lower(&self) -> PianoKey {
        self.range_lower
    }

    fn get_velocity_upper(&self) -> u8 {
        self.velocity_range_upper
    }

    fn get_velocity_lower(&self) -> u8 {
        self.velocity_range_lower
    }
}

impl InternalLayer {
    pub fn tone_remain_warning(id: &Layer, a: &Self, b: &Self, chorus_active: bool, reverb_active: bool, a_fc1: &PedalFunction, b_fc1: &PedalFunction, a_fc2: &PedalFunction, b_fc2: &PedalFunction) -> Option<String> {
        if !a.enable {
            None // if this layer wasn't on to begin with then it can't have any tone which needs remaining
        } else if a.range_lower == a.range_upper && (a.range_lower == PianoKey::A0 || a.range_lower == PianoKey::C8) {
            None // by convention, a one note range at the top or bottom of the keyboard is considered no range
        } else if !b.enable {
            Some(format!("Layer[{}] turns OFF", id))
        } else if a.volume != b.volume {
            Some(format!("Layer[{}] volume changes from {} to {}", id, a.volume, b.volume))
        } else if a.pan != b.pan {
            Some(format!("Layer[{}] pan moves from {:?} to {:?}", id, a.pan, b.pan))
        } else if chorus_active && a.chorus != b.chorus {
            Some(format!("Layer[{}] chorus level changes from {} to {}", id, a.chorus, b.chorus))
        } else if reverb_active && a.reverb != b.reverb {
            Some(format!("Layer[{}] reverb level changes from {} to {}", id, a.reverb, b.reverb))
        } else if a.damper && !b.damper {
            Some(format!("Layer[{}] damper pedal STOPS working", id))
        } else if a.modulation && !b.modulation {
            Some(format!("Layer[{}] modulation STOPS working", id))
        } else if a.bender && !b.bender {
            Some(format!("Layer[{}] bend STOPS working", id))
        } else if let Some(fc1_reason) = PedalFunction::tone_remain_warning(a_fc1, b_fc1, a.fc1, b.fc1) {
            Some(format!("Layer[{}] fc1 pedal {}", id, fc1_reason))
        } else if let Some(fc2_reason) = PedalFunction::tone_remain_warning(a_fc2, b_fc2, a.fc2, b.fc2) {
            Some(format!("Layer[{}] fc2 pedal {}", id, fc2_reason))
        } else {
            None
        }
    }
}
