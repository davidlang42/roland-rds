use std::collections::HashMap;
use std::fmt::Debug;
use schemars::JsonSchema;
use strum::IntoEnumIterator;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, serialize_chars_as_string, StructuredJsonError};
use crate::json::validation::{valid_chars, contains_all_keys};
use crate::roland::types::StateMap;
use crate::roland::types::enums::{Layer, SliderSelect, KeyOffPosition, KeyTouchVelocity, KeyTouchCurveType, VoiceReserve, HarmonicBar, MidiChannel, ButtonFunction, PedalFunction, SliderFunction, SoundFocusType};
use crate::roland::types::numeric::OffsetU8;
use crate::json::serialize_map_keys_in_order;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[schemars(rename = "LiveSetCommon")]
pub struct Common {
    #[serde(deserialize_with = "serialize_chars_as_string::deserialize")]
    #[serde(serialize_with = "serialize_chars_as_string::serialize")]
    #[schemars(with = "serialize_chars_as_string::StringSchema::<16>")]
    #[validate(custom = "valid_chars")]
    name: [char; 16], // 32-127 (ascii)
    #[serde(deserialize_with = "serialize_map_keys_in_order::deserialize")]
    #[serde(serialize_with = "serialize_map_keys_in_order::serialize")]
    #[schemars(with = "serialize_map_keys_in_order::OptionalMapSchema::<MidiChannel, VoiceReserve>")]
    #[validate]
    voice_reserve: HashMap<MidiChannel, VoiceReserve>,
    #[validate(range(min = 10, max = 500))]
    live_set_tempo: u16,
    #[validate]
    fc1_assign: PedalFunction, // 0-144
    #[validate]
    fc2_assign: PedalFunction, // 0-144
    sound_focus_switch: bool,
    #[validate]
    sound_focus_type: SoundFocusType,
    #[validate(range(max = 127))]
    sound_focus_value: u8,
    s1_assign: ButtonFunction, // 0-17
    s2_assign: ButtonFunction, // 0-17
    s1_state: bool,
    s2_state: bool,
    unused_eq_settings: Bits<68>,
    #[validate]
    key_touch_velocity: KeyTouchVelocity,
    key_touch_curve_type: KeyTouchCurveType,
    #[validate]
    key_touch_curve_offset: OffsetU8<10, 0, 19>, // 0-19 (-10 - +9)
    #[validate]
    key_touch_velocity_delay_sense: OffsetU8<64, 1, 127>, // 1-127 (-63 - +63)
    #[validate]
    key_touch_velocity_key_follow: OffsetU8<64, 1, 127>, // 1-127 (-63 - +63)
    key_off_position: KeyOffPosition,
    slider_select: SliderSelect,
    #[serde(deserialize_with = "serialize_map_keys_in_order::deserialize")]
    #[serde(serialize_with = "serialize_map_keys_in_order::serialize")]
    #[schemars(with = "serialize_map_keys_in_order::RequiredMapSchema::<Layer, SliderFunction>")]
    #[validate]
    #[validate(custom = "contains_all_keys")]
    slider_assign: HashMap<Layer, SliderFunction>,
    pub split_switch_internal: bool,
    pub split_switch_external: bool,
    #[serde(deserialize_with = "serialize_map_keys_in_order::deserialize")]
    #[serde(serialize_with = "serialize_map_keys_in_order::serialize")]
    #[schemars(with = "serialize_map_keys_in_order::RequiredMapSchema::<Layer, StateMap<HarmonicBar>>")]
    #[validate(custom = "contains_all_keys")]
    unused_harmonic_bar_assign: HashMap<Layer, StateMap<HarmonicBar>>, // index=(LOWER2:ON, LOWER2:OFF, LOWER1:ON, LOWER1:OFF, UPPER2:ON, UPPER2:OFF, UPPER1:ON, UPPER1:OFF)
    unused_mfx_control_destination: Layer,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<7>::zero")]
    unused: Bits<7>
}

impl Bytes<56> for Common {
    fn to_bytes(&self) -> Result<Box<[u8; Self::BYTE_SIZE]>, BytesError> {
        BitStream::write_fixed(|bits| {
            for value in self.name {
                bits.set_char::<7>(value)?;
            }
            for channel in MidiChannel::iter() {
                let value = match self.voice_reserve.get(&channel) {
                    Some(value) => *value,
                    None => VoiceReserve::default()
                };
                bits.set_u8::<7>(value.into(), 0, 64)?;
            }
            bits.set_u16::<9>(self.live_set_tempo, 10, 500)?;
            bits.set_u8::<8>(self.fc1_assign.into(), 0, 144)?;
            bits.set_u8::<8>(self.fc2_assign.into(), 0, 144)?;
            bits.set_bool(self.sound_focus_switch);
            bits.set_u8::<5>(self.sound_focus_type.into(), 0, 31)?;
            bits.set_u8::<7>(self.sound_focus_value, 0, 127)?;
            bits.set_u8::<5>(self.s1_assign.into(), 0, 17)?;
            bits.set_u8::<5>(self.s2_assign.into(), 0, 17)?;
            bits.set_bool(self.s1_state);
            bits.set_bool(self.s2_state);
            bits.set_bits(&self.unused_eq_settings);
            bits.set_u8::<7>(self.key_touch_velocity.into(), 0, 127)?;
            bits.set_u8::<3>(self.key_touch_curve_type.into(), 1, 5)?;
            bits.set_u8::<5>(self.key_touch_curve_offset.into(), 0, 19)?;
            bits.set_u8::<7>(self.key_touch_velocity_delay_sense.into(), 1, 127)?;
            bits.set_u8::<7>(self.key_touch_velocity_key_follow.into(), 1, 127)?;
            bits.set_bool(self.key_off_position.into());
            bits.set_bool(self.slider_select.into());
            for layer in Layer::iter() {
                bits.set_u8::<8>((*self.slider_assign.get(&layer).unwrap()).into(), 0, 133)?;
            }
            bits.set_bool(self.split_switch_internal);
            bits.set_bool(self.split_switch_external);
            for layer in Layer::iter().rev() {
                let state_map = self.unused_harmonic_bar_assign.get(&layer).unwrap();
                bits.set_u8::<4>(state_map.on.into(), 1, 9)?;
                bits.set_u8::<4>(state_map.off.into(), 1, 9)?;
            }
            bits.set_u8::<2>(self.unused_mfx_control_destination.into(), 0, 3)?;
            bits.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let mut name = [char::default(); 16];
            for i in 0..name.len() {
                name[i] = data.get_char::<7>()?;
            }
            let mut voice_reserve = HashMap::new();
            for channel in MidiChannel::iter() {
                let value = data.get_u8::<7>(0, 64)?.into();
                if value != VoiceReserve::default() {
                    voice_reserve.insert(channel, value);
                }
            }
            let live_set_tempo = data.get_u16::<9>(10, 500)?;
            let fc1_assign = data.get_u8::<8>(0, 144)?.into();
            let fc2_assign = data.get_u8::<8>(0, 144)?.into();
            let sound_focus_switch = data.get_bool();
            let sound_focus_type = data.get_u8::<5>(0, 31)?.into();
            let sound_focus_value = data.get_u8::<7>(0, 127)?;
            let s1_assign = data.get_u8::<5>(0, 17)?.into();
            let s2_assign = data.get_u8::<5>(0, 17)?.into();
            let s1_state = data.get_bool();
            let s2_state = data.get_bool();
            let eq_settings = data.get_bits();
            let key_touch_velocity = data.get_u8::<7>(0, 127)?.into();
            let key_touch_curve_type = data.get_u8::<3>(1, 5)?.into();
            let key_touch_curve_offset = data.get_u8::<5>(0, 19)?.into();
            let key_touch_velocity_delay_sense = data.get_u8::<7>(1, 127)?.into();
            let key_touch_velocity_key_follow = data.get_u8::<7>(1, 127)?.into();
            let key_off_position = data.get_bool().into();
            let slider_select = data.get_bool().into();
            let mut slider_assign = HashMap::new();
            for layer in Layer::iter() {
                slider_assign.insert(layer, data.get_u8::<8>(0, 133)?.into());
            }
            let split_switch_internal = data.get_bool();
            let split_switch_external = data.get_bool();
            let mut harmonic_bar_assign = HashMap::new();
            for layer in Layer::iter().rev() {
                harmonic_bar_assign.insert(layer, StateMap {
                    on: data.get_u8::<4>(1, 9)?.into(),
                    off: data.get_u8::<4>(1, 9)?.into()
                });
            }
            Ok(Self {
                name,
                voice_reserve,
                live_set_tempo,
                fc1_assign,
                fc2_assign,
                sound_focus_switch,
                sound_focus_type,
                sound_focus_value,
                s1_assign,
                s2_assign,
                s1_state,
                s2_state,
                unused_eq_settings: eq_settings,
                key_touch_velocity,
                key_touch_curve_type,
                key_touch_curve_offset,
                key_touch_velocity_delay_sense,
                key_touch_velocity_key_follow,
                key_off_position,
                slider_select,
                slider_assign,
                split_switch_internal,
                split_switch_external,
                unused_harmonic_bar_assign: harmonic_bar_assign,
                unused_mfx_control_destination: data.get_u8::<2>(0, 3)?.into(),
                unused: data.get_bits()
            })
        })
    }
}

impl Json for Common {
    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Result<Self, StructuredJsonError> {
        Self::from_json(structured_json.to_single_json()?).map_err(|e| e.into())
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn from_json(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}

impl Common {
    pub fn name_string(&self) -> String {
        self.name.iter().collect()
    }
}